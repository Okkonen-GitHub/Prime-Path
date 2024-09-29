<script lang="ts">
    import { link, push } from "svelte-spa-router";
    import { set_name, get_name } from "./store";
    export let game_code: string = "";

    export async function join_game(code: string) {
        if (code === undefined || code === "") {
            return;
        }
        let name = get_name();
        if (name === undefined || name === "") {
            return alert("Name missing");
        }
        const response = await fetch(`/join_game?id=${game_code}&name=${name}`, {
            method: "post",
        });
        if (response.ok) {
            push(`#/${game_code}`);
        } else {
            let code = response.status;
            response.text().then((t) => alert(`${code}: ${t}`));
        }
    }
</script>

<p>Enter a game code</p>
<input
    bind:value={game_code}
    type="text"
    minlength="5"
    maxlength="30"
    spellcheck="false"
    placeholder="pj65mr"
/>
<button on:click={(t) => join_game(game_code)}> Join Game </button>

<style>
    button {
        background-color: hsl(126, 80%, 80%);
    }
</style>
