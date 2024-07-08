<script lang="ts">
    import Router from "svelte-spa-router";

    // Import the list of routes
    import routes from "./routes";
</script>

<main>
    <h1>Prime Path</h1>
    <Router {routes} />
</main>

<style>
    h1 {
        color: #77996e;
    }
</style>
