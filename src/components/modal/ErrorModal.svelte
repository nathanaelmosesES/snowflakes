<script lang="ts">
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";

    let { isOpen, errorMessage, sessionKey } = $props();

    async function onClose() {
        await invoke("disconnect", { sessionKey });
        goto("/");
    }
</script>

{#if isOpen}
    <div class="overlay" role="dialog">
        <div class="modal">
            <div class="modal-header">
                <span class="modal-title">Error</span>
            </div>
            <div class="modal-body">
                <p>{errorMessage}</p>
            </div>
            <div class="modal-footer">
                <button class="btn-primary" onclick={onClose}
                    >Make a new connection</button
                >
            </div>
        </div>
    </div>
{/if}

<style>
    .overlay {
        position: fixed;
        inset: 0;
        background: rgba(5, 15, 28, 0.85);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 50;
    }

    .modal {
        background: var(--sf-bg-surface);
        border: 1px solid var(--sf-border);
        border-radius: 12px;
        width: 420px;
        overflow: hidden;
    }

    .modal-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 16px 18px 12px;
        border-bottom: 1px solid var(--sf-border);
    }

    .modal-title {
        font-size: 14px;
        font-weight: 500;
        color: var(--sf-text-primary);
    }

    .modal-body {
        padding: 16px 18px;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .modal-footer {
        padding: 12px 18px;
        border-top: 1px solid var(--sf-border);
        display: flex;
        justify-content: flex-end;
        gap: 8px;
    }

    .btn-primary {
        background: var(--sf-accent);
        border: none;
        border-radius: 7px;
        padding: 7px 14px;
        font-size: 12px;
        font-weight: 500;
        color: var(--sf-text-on-accent);
        cursor: pointer;
    }

    .btn-primary:hover {
        background: var(--sf-accent-hover);
    }
</style>
