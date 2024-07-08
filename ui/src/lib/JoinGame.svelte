<script lang="ts">
    import { link, push } from "svelte-spa-router";
    let game_code: string = "";
</script>

<p>Enter a game code</p>
<input bind:value={game_code} />
<button on:click={() => push(`#/${game_code}`)}> Join Game </button>

<style>
    button {
        background-color: hsl(126, 80%, 80%);
    }
</style>
