
export function set_name(name: string) {
  localStorage.setItem("name", name);
}

export function get_name() {
  return localStorage.getItem("name");
}

export function enter_game() {
  localStorage.setItem("is_in_game", true);
}

export function is_in_game() {
  return localStorage.getItem("is_in_game");
}
export function set_game_left() {
  localStorage.setItem("is_in_game", false);
}
