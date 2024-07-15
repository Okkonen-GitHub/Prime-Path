<script lang="ts">
    import Router from "svelte-spa-router";

    // Import the list of routes
    import routes from "./routes";
</script>

<main>
    <Router {routes} />
</main>
