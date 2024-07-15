<script lang="ts">
    import { link, push } from "svelte-spa-router";
    import { name } from "./store";
    let game_code: string = "";
    let ns: string;
    name.subscribe((n: string) => (ns = n));

    async function join_game() {
        if (game_code === undefined || game_code === "") {
            return;
        }
        const response = await fetch(`/join_game?id=${game_code}&name=${ns}`, {
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
<button on:click={join_game}> Join Game </button>

<style>
    button {
        background-color: hsl(126, 80%, 80%);
    }
</style>
