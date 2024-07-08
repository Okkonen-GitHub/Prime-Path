<script lang="ts">
    let resp: String;
    async function new_game() {
        const response = await fetch("/new_game?Player1");
        response.text().then((t) => (resp = t));
    }
</script>

<button on:click={new_game}> New Game </button>

{#if resp !== undefined}
    <p>Response: {resp}</p>
{/if}
