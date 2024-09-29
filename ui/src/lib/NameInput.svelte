<script lang="ts">
    import { set_name, get_name} from "./store";
    let name = get_name();
</script>

<p>Enter name</p>
<input
    bind:value={name}
    on:keyup={() => set_name(name)}
    type="text"
    minlength="1"
    maxlength="30"
    spellcheck="false"
    placeholder="Player1"
/>
