


<script lang="ts">
    import NameInput from "../lib/NameInput.svelte";
    import { get_name, set_name, is_in_game, enter_game } from "../lib/store";
    import { send_name, join_game, connected, init_ws } from "../lib/socket";
    export let params = {};
    
    async function sleep(ms) {
      return new Promise(resolve => setTimeout(resolve, ms));
    }
    let is_connected = false;
    connected.subscribe((val) => {
      console.log(`${val} is new val`);
      is_connected = val;
    })
    init_ws();
    async function try_connect() {
      let i = 1;
      while (true) {
        console.log(is_connected, "conn");
        if (is_connected) {
          console.log("ya");
          let name = get_name();
          send_name(name);
          console.log("yao");
          join_game(params.gameid);
          console.log(`${params.gameid} gameid`)
          break;
        }
        await sleep(100*i);
        console.log(`slept for ${i*100} ms`);
        i++;
      }
    }
    if (is_in_game()) {
      console.log("already in game");

    } else {
      try_connect();
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
