<script lang="ts">
    import NewGame from "../lib/NewGame.svelte";
    import JoinGame from "../lib/JoinGame.svelte";

    // Import the list of routes
</script>

<main>
    <h1>Prime Path</h1>
    <div class="card">
        <NewGame />
        <JoinGame />
    </div>

    <p>
        Check out the source code at <a
            href="https://github.com/okkonen-github/prime-path"
            target="_blank"
            rel="noreferrer">Github</a
        >.
    </p>
</main>

<style>
    h1 {
        color: #77996e;
        text-align: center;
    }
</style>
