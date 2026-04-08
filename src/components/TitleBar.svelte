<!-- svelte-ignore a11y_consider_explicit_label -->
<script lang="ts">
    import type { UnlistenFn } from "@tauri-apps/api/event";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { openUrl } from "@tauri-apps/plugin-opener";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let cpu = $state(0);
    let ram = $state(0);

    const appWindow = getCurrentWindow();

    let isMaximized = $state(false);
    let isDragging = $state(false);

    onMount(() => {
        // Variabel untuk menyimpan fungsi unlisten
        let unlistenResized: UnlistenFn;

        // Definisikan fungsi setup secara terpisah
        const setupListeners = async () => {
            unlistenResized = await appWindow.onResized(async () => {
                isMaximized = await appWindow.isMaximized();
            });
        };

        setupListeners();

        // Svelte bisa memproses cleanup ini karena onMount-nya tidak async
        return () => {
            if (unlistenResized) unlistenResized();
        };
    });

    onMount(() => {
        const updateStats = async () => {
            try {
                const stats: { cpu_usage: number; ram_usage: number } =
                    await invoke("get_system_stats");
                cpu = stats.cpu_usage;
                ram = stats.ram_usage;
            } catch (e) {
                console.error("Failed to fetch stats", e);
            }
        };

        updateStats(); // Ambil pertama kali
        const interval = setInterval(updateStats, 2000); // Update tiap 2 detik
        return () => clearInterval(interval);
    });

    const minimize = () => appWindow.minimize();
    const toggleMaximize = () => appWindow.toggleMaximize();
    const close = () => appWindow.close();

    function handleMouseDown() {
        isDragging = true;
        const handleMouseUp = () => {
            isDragging = false;
            window.removeEventListener("mouseup", handleMouseUp);
        };
        window.addEventListener("mouseup", handleMouseUp);
    }
</script>

<div
    data-tauri-drag-region
    class="titlebar flex h-8 items-center justify-between border-b border-(--sf-border) bg-(--sf-bg-sidebar) select-none transition-all duration-200 ease-[cubic-bezier(0.2,0.8,0.2,1)] z-9999"
    class:shrunk={isDragging && !isMaximized}
    onmousedown={handleMouseDown}
    role="none"
>
    <div
        class="flex items-center pl-4 pointer-events-none"
        data-tauri-drag-region
    >
        <span
            class="text-[10px] font-extrabold tracking-[2px] text-(--sf-accent)"
        >
            SNOWFLAKES
        </span>
    </div>

    <div class="flex h-full items-center gap-4" data-tauri-no-drag>
        <div
            class="flex items-center gap-3 px-3 py-1 rounded-md bg-white/5 border border-white/10 font-mono"
        >
            <div class="flex items-center gap-1.5">
                <span
                    class="text-[8px] text-(--sf-text-secondary) opacity-40 font-bold"
                    >CPU</span
                >
                <span
                    class="text-[9px] font-bold text-(--sf-text-secondary) min-w-[25px]"
                >
                    {cpu}%
                </span>
                <div
                    class="w-12 h-1 bg-white/5 rounded-full overflow-hidden hidden sm:block"
                >
                    <div
                        class="h-full bg-(--sf-accent) transition-all duration-500"
                        style="width: {cpu}%"
                    ></div>
                </div>
            </div>

            <span class="text-(--sf-border) opacity-30">|</span>

            <div class="flex items-center gap-1.5">
                <span
                    class="text-[8px] text-(--sf-text-secondary) opacity-40 font-bold"
                    >RAM</span
                >
                <span
                    class="text-[9px] font-bold text-(--sf-text-secondary) min-w-[25px]"
                >
                    {ram}%
                </span>
                <div
                    class="w-12 h-1 bg-white/5 rounded-full overflow-hidden hidden sm:block"
                >
                    <div
                        class="h-full bg-emerald-500 transition-all duration-500"
                        style="width: {ram}%"
                    ></div>
                </div>
            </div>

            <span class="text-(--sf-border) opacity-30">|</span>

            <span
                class="text-[9px] font-medium text-(--sf-text-secondary) opacity-30"
            >
                v1.0.0
            </span>
        </div>

        <div class="flex h-full">
            <button
                onclick={minimize}
                class="flex w-[45px] h-full items-center justify-center text-(--sf-text-primary) hover:bg-white/5 transition-colors"
            >
                <svg width="10" height="10" viewBox="0 0 12 12"
                    ><rect
                        fill="currentColor"
                        width="10"
                        height="1"
                        x="1"
                        y="6"
                    /></svg
                >
            </button>

            <button
                onclick={toggleMaximize}
                class="flex w-[45px] h-full items-center justify-center text-(--sf-text-primary) hover:bg-white/5 transition-colors"
            >
                {#if isMaximized}
                    <svg width="10" height="10" viewBox="0 0 12 12"
                        ><path
                            fill="currentColor"
                            d="M3 5v5h5V5H3zm1 1h3v3H4V6z M5 2v2h5v5h2V2H5z"
                        /></svg
                    >
                {:else}
                    <svg width="10" height="10" viewBox="0 0 12 12"
                        ><path
                            fill="currentColor"
                            d="M2 2v8h8V2H2zm1 1h6v6H3V3z"
                        /></svg
                    >
                {/if}
            </button>

            <button
                onclick={close}
                class="flex w-[45px] h-full items-center justify-center text-(--sf-text-primary) hover:bg-red-600 hover:text-white transition-colors"
            >
                <svg width="10" height="10" viewBox="0 0 12 12"
                    ><path
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1.2"
                        d="M1 1l10 10M11 1L1 11"
                    /></svg
                >
            </button>
        </div>
    </div>
</div>

<style>
    .titlebar {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        height: 32px;
        background: var(--sf-bg-sidebar);
        display: flex;
        justify-content: space-between;
        align-items: center;
        user-select: none;
        border-bottom: 1px solid var(--sf-border);
        transition:
            transform 0.2s cubic-bezier(0.2, 0.8, 0.2, 1),
            border-radius 0.2s ease,
            margin 0.2s ease;
        z-index: 9999;
    }

    .title {
        padding-left: 15px;
        font-size: 10px;
        font-weight: 800;
        letter-spacing: 1px;
        color: var(--sf-text-secondary);
        pointer-events: none; /* Agar tidak mengganggu drag region */
    }

    .controls {
        display: flex;
        height: 100%;
    }

    .control-btn {
        width: 45px;
        height: 100%;
        background: transparent;
        border: none;
        color: var(--sf-text-primary);
        display: flex;
        justify-content: center;
        align-items: center;
        cursor: default;
    }

    .control-btn:hover {
        background: rgba(255, 255, 255, 0.05);
    }

    .close-btn:hover {
        background: #e81123 !important;
        color: white;
    }
</style>
