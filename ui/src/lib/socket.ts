import { push } from "svelte-spa-router";
import { writable } from "svelte/store";

let socket = new WebSocket("ws://localhost:8080/ws");
export let connected = writable(false);


export function init_ws() {
  
  socket.onopen = function(e) {
    console.log("[open] Connection established");
    console.log(socket);
    connected.set(true);
  };

  socket.onmessage = onmsg;

  socket.onclose = function(event) {
    if (event.wasClean) {
      console.log(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
    } else {
      // e.g. server process killed or network down
      // event.code is usually 1006 in this case
      console.log('[close] Connection died');
    }
    connected.set(false);
  };

  socket.onerror = function(error) {
    console.log(`[error: ${error}]`);
  };

}

function onmsg(event) {
  let msg = event.data;
  console.log(`[message] Data received from server: ${event.data}`);
  if (msg.startsWith("/redirect")) {
    on_game_create(msg.slice(9));
  } else if (msg.startsWith("/begin")) {
    on_ready(msg.slice(6));
  } else if (msg.startsWith("/turn")) {
    // turn 0 means it's your turn
    // turn 1 means it's the enemys turn
    on_turn_change(msg.slice(5));
    
  } else if (msg.startsWith("/win")) {
    on_win(msg.slice(4));
  }
}

function on_win(winner: string) {
  if (winner === "0") {
    console.log("You win!");
  } else {
    console.log("Enemy won");
  }
}

function on_turn_change(turn: string) {
  if (turn === "0") {
    console.log("it's your turn");
  } else {
    console.log(`It's enemy's turn ${turn}`);
  }
}

export function join_game(game_id: string) {
  socket.send(`/join ${game_id}`);
}

export function on_game_create(game_id: string) {
  console.log(`Ws got ${game_id}`);
  push(`#/${game_id}`);
}

export function on_ready(time: string) {
  console.log(`game starting in ${time}`)
}

export function create_game(name: string) {
  socket.send("/create");
  
}

export function send_name(name: string) {
  socket.send(`/name ${name}`)
  
}
