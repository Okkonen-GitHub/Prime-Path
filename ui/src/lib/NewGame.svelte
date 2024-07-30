<script lang="ts">
    import { push } from "svelte-spa-router";
    import { name } from "./store";
    import NameInput from "./NameInput.svelte";

    let resp: string;
    let ns: string;
    name.subscribe((n) => (ns = n));
    async function new_game() {
        if (ns === undefined || ns === "") {
            alert("You need to enter a name");
            return;
        }
        const response = await fetch(`/new_game?name=${ns}`);

        response.text().then((t) => (resp = t));
    }
</script>

<NameInput />
<button on:click={new_game}> New Game </button>

{#if resp !== undefined}
    <p>Response: {resp}</p>
{/if}
