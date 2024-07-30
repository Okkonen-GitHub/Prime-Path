<script lang="ts">
    // FIX: When you refresh the site everything is lost
    // Store playername in a cookie
    import NameInput from "../lib/NameInput.svelte";
    import { name } from "../lib/store";
    let ns: string = "";
    name.subscribe((n: string) => (ns = n));
    export let params = {};
    let pressed_join: boolean = ns === "";
    async function join_game() {
        if (ns === "" || ns === undefined) {
            return alert("Enter a name");
        }

        const response = await fetch(
            `/join_game?id=${params.gameid}&name=${ns}`,
            {
                method: "post",
            },
        );
        if (!response.ok) {
            let code = response.status;
            response.text().then((t) => alert(`${code}: ${t}`));
            return;
        }
        pressed_join = true;
    }
</script>

{#if !pressed_join || ns === "" || ns === undefined}
    2
    <NameInput />
    <button on:click={join_game}> Join Game </button>
{:else}
    <p>HEYYY {params.gameid}</p>
{/if}
