//! `GameServer` is an actor. It maintains a list of connection client sessions.
//! It also manages game rooms. Peers send messages to other peers in same
//! room through `GameServer`.

use actix::prelude::*;
use core::num::NonZeroU128;
use rand::{rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};

use crate::GameId;

/// Game server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub game_id: Option<String>,
}

/// Join room, if room does not exists create new one.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client ID
    pub id: usize,

    /// Game id (or name, later on?)
    pub game_id: GameId,
}

#[derive(Message)]
#[rtype(result(bool))]
pub struct CheckGameExists {
    pub game_id: GameId,
}
#[derive(Message)]
#[rtype(result = "Vec<GameId>")]
pub struct ListGames {}

#[derive(Message)]
#[rtype(result = "Vec<GameId>")]
pub struct Ready {}

#[derive(Debug, Default)]
pub enum GameStatus {
    #[default]
    Waiting,
    Starting,
    InProcess {
        current_number: NonZeroU128,
    },
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Player {
    pub name: String, // controlled by /name
}

#[derive(Default, Debug)]
pub struct Game {
    /// at first limited to 2, but adding multiperson games and spectators
    /// later is supported
    pub players: HashMap<usize, Player>,
    /// the id of the player whose turn it is
    pub turn_id: usize,
    /// for checking if certain game actions is valid at a given time
    pub status: GameStatus,
}

/// `GameServer` manages chat rooms and responsible for coordinating chat session.
///
/// Implementation is very naïve.
#[derive(Debug)]
pub struct GameServer {
    sessions: HashMap<usize, Recipient<Message>>,
    games: HashMap<GameId, Game>,
    rng: ThreadRng,
    // visitor_count: Arc<AtomicUsize>,
}

impl GameServer {
    pub fn new() -> GameServer {
        GameServer {
            sessions: HashMap::new(),
            games: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl GameServer {
    /// Send message to all users in the room
    fn send_message(&self, room: Option<&str>, message: &str, skip_id: usize) {
        if let Some(room) = room {
            if let Some(sessions) = self.games.get(room) {
                for (id, _name) in &sessions.players {
                    if *id != skip_id {
                        if let Some(addr) = self.sessions.get(&id) {
                            addr.do_send(Message(message.to_owned()));
                        }
                    }
                }
            }
        }
    }
}

/// Make actor from `GameServer`
impl Actor for GameServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for GameServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // notify all users in same room
        // self.send_message("main", "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions
            .insert(id, msg.addr)
            .map(|_| panic!("Somehow got a usize collision"));

        // auto join session to main room
        // self.rooms.entry("main".to_owned()).or_default().insert(id);

        // let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        // self.send_message("main", &format!("Total visitors {count}"), 0);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.games {
                if sessions.players.remove(&msg.id).is_some() {
                    rooms.push(name.to_owned());
                }
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(Some(&room), "Someone disconnected", 0);
        }
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(msg.game_id.as_deref(), msg.msg.as_str(), msg.id);
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _ctx: &mut Context<Self>) {
        let Join { id, game_id } = msg;
        dbg!("joined room", &game_id);
        let mut rooms = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.games {
            if sessions.players.remove(&id).is_some() {
                rooms.push(n.to_owned());
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(Some(&room), "Someone disconnected", 0);
        }

        self.games
            .entry(game_id.clone())
            .or_default()
            .players
            .insert(
                id,
                Player {
                    name: "Anon".to_owned(),
                },
            );

        self.send_message(Some(&game_id), "Someone connected", id);
        self.sessions
            .get(&id)
            .expect("Just joined, must exist?")
            .do_send(Message("Hello from here".to_owned()));
    }
}

impl Handler<CheckGameExists> for GameServer {
    type Result = bool;

    fn handle(&mut self, msg: CheckGameExists, _: &mut Context<Self>) -> Self::Result {
        let CheckGameExists { game_id } = msg;
        self.games.contains_key(&game_id)
    }
}
impl Handler<ListGames> for GameServer {
    type Result = Vec<GameId>;

    fn handle(&mut self, _msg: ListGames, _: &mut Context<Self>) -> Self::Result {
        Vec::from_iter(self.games.keys().map(ToOwned::to_owned))
    }
}
