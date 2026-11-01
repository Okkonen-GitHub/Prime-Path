if (window.location.pathName === "/") {
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

function ensureHasName() {
    let name = window.localStorage.getItem("name")
    if (name == null || name === " ") {
        openPopup("Name missing")
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
    let [hasName, name] = ensureHasName()
    if (!hasName) { return }
    console.log(gameName)
    let params = new URLSearchParams({"name": gameName, "creator": name })
    await fetch(`/create_game?${params.toString()}}`, {method: "post", headers: new Headers({"content-type": "application/x-www-form-urlencoded"})})
}
async function joinGame() {
    const codeInput = document.getElementById("game-code")
    const gameCode = codeInput.value
    if (gameCode == null || gameCode === "") {
        openPopup("Game code missing")
        return
    }
    let [hasName, name] = ensureHasName()
    if (!hasName) { return }
    let params = new URLSearchParams({"code": gameCode, "name": name})
    await fetch(`/join_game?${params.toString()}}`, {method: "post", headers: new Headers({"content-type": "application/x-www-form-urlencoded"})})
}
