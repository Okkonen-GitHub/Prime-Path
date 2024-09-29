
export function set_name(name: string) {
  localStorage.setItem("name", name)
}

export function get_name() {
  return localStorage.getItem("name")
}


