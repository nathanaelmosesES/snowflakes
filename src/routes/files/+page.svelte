<script>
    import Session from "../../components/terminal/Session.svelte";
</script>

<div class="main-container">
    <!-- <Sidebar /> -->
    <Session />
</div>

<style>
    .main-container {
        width: 100vw;
        height: 100vh;
        background-color: var(--sf-bg-app);
        display: flex;
    }
    :global(body) {
        overflow: hidden;
    }
</style>
