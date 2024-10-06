
<script lang="ts">
    import NameInput from "../lib/NameInput.svelte";
    import { get_name, set_name, is_in_game, enter_game } from "../lib/store";
    import { send_name, join_game, connected} from "../lib/socket";
    export let params = {};
    
    async function sleep(ms) {
      return new Promise(resolve => setTimeout(resolve, ms));
    }

    if (is_in_game()) {
      console.log("yey");

    } else {
      while (true) {

        if (connected) {
          sleep(1000).then(() => {

            console.log("ya");
            let name = get_name();
            send_name(name);
            console.log("yao");
            join_game(params.gameid);
            console.log(`${params.gameid} gameid`)
          });
          break;
        } else {
          sleep(100).then(() => console.log("slept"));

        }
      }
    }

    var is_ready: bool = false;
    
    export function ready() {
      is_ready = !is_ready;
      console.log(is_ready);
    }

</script>

<NameInput />
<p>HEYYY {params.gameid}</p>
{#if !is_ready }
  <button on:click={ready}> Ready! </button>
{:else}
  <button on:click={ready}> Cancel? </button>
{/if}
