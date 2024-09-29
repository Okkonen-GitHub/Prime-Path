<script lang="ts">
    import NameInput from "../lib/NameInput.svelte";
    import { get_name, set_name } from "../lib/store";
    export let params = {};
    let name = get_name();
    let pressed_join;
    async function join_game() {
      name = get_name();
      if (name === "" || name === undefined) {
        return alert("Enter a name");
      }

      const response = await fetch(
        `/join_game?id=${params.gameid}&name=${name}`,
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

{#if !pressed_join || name === "" || name === undefined}
    2
    <NameInput />
    <button on:click={join_game}> Join Game </button>
{:else}
    <p>HEYYY {params.gameid}</p>
{/if}
