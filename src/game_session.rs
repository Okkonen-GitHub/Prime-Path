// A websocket connection handler thing
//
use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use crate::{game_server, gen_game_id};

// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug)]
pub struct WsGameSession {
    /// unique session id
    pub id: usize,

    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,

    /// joined room
    pub game_id: Option<String>,

    /// peer name
    pub name: Option<String>,

    /// Game server
    pub addr: Addr<game_server::GameServer>,
}

impl WsGameSession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(game_server::Disconnect { id: act.id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WsGameSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(game_server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat game_server
        self.addr.do_send(game_server::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat game_server, we simply send it to peer websocket
impl Handler<game_server::Message> for WsGameSession {
    type Result = ();

    fn handle(&mut self, msg: game_server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsGameSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        // log::debug!("WEBSOCKET MESSAGE: {msg:?}");
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                // we check for /sss type of messages
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    match v[0] {
                        "/join" => {
                            if v.len() == 2 {
                                // v[1].clone_into(&mut self.game_id);

                                // check if there is a game with this code
                                // if yes, send Join to gameserver
                                // if not send client "Not found"
                                let game_id = v[1].to_owned();
                                self.addr
                                    .send(game_server::CheckGameExists {
                                        game_id: game_id.clone(),
                                    })
                                    .into_actor(self)
                                    .then(move |res, act, ctx| {
                                        match res {
                                            Ok(res) => {
                                                if res {
                                                    act.game_id = Some(game_id.clone());

                                                    act.addr.do_send(game_server::Join {
                                                        id: act.id,
                                                        game_id,
                                                    });
                                                    ctx.text("Joined");
                                                } else {
                                                    act.game_id = None;
                                                    ctx.text("No such room");
                                                }
                                            }
                                            Err(why) => {
                                                eprintln!(
                                                "Something is wrong with the game server: {why}"
                                            );
                                                ctx.stop();
                                            }
                                        }
                                        fut::ready(())
                                    })
                                    .wait(ctx);
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }
                        "/name" => {
                            if v.len() == 2 {
                                self.name = Some(v[1].to_owned());
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }
                        "/create" => {
                            let game_id = gen_game_id();

                            self.game_id = Some(game_id.clone());
                            self.addr
                                .send(game_server::ListGames {})
                                .into_actor(self)
                                .then(move |res, act, ctx| {
                                    match res {
                                        Ok(game_ids) => {
                                            let mut next_game_id = game_id.clone();
                                            while game_ids.contains(&next_game_id) {
                                                // generate new ids until a non duplicate is found
                                                next_game_id = gen_game_id();
                                            }
                                            act.game_id = Some(game_id.clone());
                                            act.addr.do_send(game_server::Join {
                                                id: act.id,
                                                game_id: game_id.clone(),
                                            });
                                            ctx.text(format!("/redirect{}", &game_id));
                                        }
                                        Err(_why) => ctx.stop(), // Something is wrong with the
                                                                 // gameserver, stop
                                    }
                                    fut::ready(())
                                })
                                .wait(ctx);
                        }
                        "/ready" => if let Some(game_id) = &self.game_id {},
                        _ => ctx.text(format!("!!! unknown command: {m:?}")),
                    }
                } else {
                    let msg = if let Some(ref name) = self.name {
                        format!("{name}: {m}")
                    } else {
                        m.to_owned()
                    };
                    // send message to chat game_server
                    self.addr.do_send(game_server::ClientMessage {
                        id: self.id,
                        msg,
                        game_id: self.game_id.clone(),
                    })
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
