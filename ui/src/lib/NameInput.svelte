<script lang="ts">
    import { name } from "./store";

    let ns: string;
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
