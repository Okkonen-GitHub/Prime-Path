const forms = document.getElementsByTagName("form")
console.log(forms)
console.log(forms.length)
for (const form of forms) {
    form.addEventListener("keydown", (event) => {
        if (event.key === "Enter") {
            console.log("clicked enter")
            event.preventDefault()
            console.info(form.childNodes)
            let btn = form.childNodes.values().find((el) => el.tagName === "BUTTON")
            btn.click()
        }
    })
}

let [hasName, name] = ensureHasName(false)
if (hasName) {
    const nameInput = document.getElementById("username")
    nameInput.value = name
}

if (window.location.pathname.startsWith("/game/")) {
    if (hasName) {
        const [socket, sendMessage] = openGameConnection()
        sendMessage(`/connect ${name}`)

    }
    else {
        openPopup("Name missing")
    }
}

function openPopup(message) {
    const main = document.getElementById("main")
    const popup = document.createElement("dialog")
    const txt = document.createElement("p")
    txt.innerText = message
    const form = document.createElement("form")
    form.method = "dialog"
    const btn = document.createElement("button")
    btn.innerText = "Ok"
    form.appendChild(btn)
    popup.append(txt)
    popup.append(form)
    popup.open = true
    main.prepend(popup)
}

function ensureHasName(openDialog) {
    let name = window.localStorage.getItem("name")
    if (name == null || name === " ") {
        if (openDialog) {
            openPopup("Name missing")
        }
        return [false, null];
    }
    return [true, name];
}

function setName() {
    const nameInput = document.getElementById("username")
    const newName = nameInput.value
    if (newName == null || newName === "") {
        openPopup("Name missing")
        return
    }
    console.log(newName)
    nameInput.value = newName
    window.localStorage.setItem("name", newName)
}


async function createGame() {

    const nameInput = document.getElementById("room-name")
    const gameName = nameInput.value
    if (gameName == null || gameName === "") {
        openPopup("Game room name missing")
        return
    }
    let [hasName, name] = ensureHasName(true)
    if (!hasName) { return }
    console.log(gameName)
    let params = new URLSearchParams({"name": gameName, "creator": name })
    let resp = await fetch(`/create_game?${params.toString()}}`, {method: "post", headers: new Headers({"content-type": "application/x-www-form-urlencoded"})})
    if (!resp.ok) {
        openPopup(resp.body)
        return
    }
    window.location = await resp.text()
}
async function joinGame() {
    const codeInput = document.getElementById("game-code")
    const gameCode = codeInput.value
    if (gameCode == null || gameCode === "") {
        openPopup("Game code missing")
        return
    }
    let [hasName, name] = ensureHasName(true)
    if (!hasName) { return }
    let params = new URLSearchParams({"code": gameCode, "name": name})
    let resp = await fetch(`/join_game?${params.toString()}}`, {method: "post", headers: new Headers({"content-type": "application/x-www-form-urlencoded"})})
    if (!resp.ok) {
        openPopup(resp.body)
        return
    }
    window.location = await resp.text()
}

function openGameConnection() {
    if (!window.location.pathname.startsWith("/game")) {
        return
    }
    const socket = new WebSocket( `ws://${window.location.host}/ws`)

    socket.onopen = function(event) {
      // Handle connection open
    };

    socket.onmessage = function(event) {
      // Handle received message
        console.info(`recieved: ${event.data}`)
    };

    socket.onclose = function(event) {
      // Handle connection close
    };

    function sendMessage(message) {
      socket.send(message);
    }
    return [socket, sendMessage]
}
