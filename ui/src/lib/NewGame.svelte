<script lang="ts">
    import { push } from "svelte-spa-router";
    import { name } from "./store";

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
    function update_name() {
        name.update((old) => (old = ns));
    }
</script>

<p>Enter name</p>
<input
    bind:value={ns}
    on:keyup={update_name}
    type="text"
    minlength="1"
    maxlength="30"
    spellcheck="false"
    placeholder="Player1"
/>
<button on:click={new_game}> New Game </button>

{#if resp !== undefined}
    <p>Response: {resp}</p>
{/if}
