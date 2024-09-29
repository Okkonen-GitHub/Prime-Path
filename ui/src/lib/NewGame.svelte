<script lang="ts">
    import { push } from "svelte-spa-router";
    import { get_name, set_name } from "./store";
    import { create_game } from "./socket"

    import NameInput from "./NameInput.svelte";
    
    let resp;
    async function new_game() {
      let name = get_name();
      if (name === undefined || name === "") {
        alert("You need to enter a name");
        return;
      }
      const response = await fetch(`/new_game?name=${name}`);

      response.text().then((t) => (resp = t));
      create_game(name);
    }
</script>

<NameInput />
<button on:click={new_game}> New Game </button>

{#if resp !== undefined}
    <p>Response: {resp}</p>
{/if}
