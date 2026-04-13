<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";

    import type { SessionInfo } from "../../types/settings";
    import { loadAllSessions, deleteSessionInfo } from "../../controller/local";
    import { reconnectToSession } from "../../controller/ssh";
    import { deleteSessionPass, loadSessionPass } from "../../controller/vault";
    import { clearTerminalState } from "../../controller/session";

    import * as sftp from "../../controller/sftp";

    import FileSystem from "../../components/file/FileSystem.svelte";
    import NewHostModal from "../../components/modal/NewHostModal.svelte";

    let isModalOpen = $state(false);
    let sessions = $state<SessionInfo[]>([]);
    let prefillSession = $state<SessionInfo | null>(null);
    let isLoading = $state(true);

    let isConnecting = $state(false);
    let connectingStatus = $state("");
    let errorMsg = $state("");

    // SFTP states
    let sftpSession = $state<{ key: string; info: SessionInfo } | null>(null);
    let isSftpConnecting = $state(false);
    let sftpError = $state("");

    onMount(async () => {
        try {
            sessions = await loadAllSessions();
        } catch (err) {
            console.error("[Home] Failed to load sessions:", err);
        } finally {
            isLoading = false;
        }
    });

    /**
     * Handle SSH Connection
     */
    async function handleCardClick(session: SessionInfo) {
        errorMsg = "";
        isConnecting = true;
        try {
            const key = await reconnectToSession(session, (msg) => {
                connectingStatus = msg;
            });
            goto(`/session?key=${key}`, {
                state: {
                    bastion: session.bastionIp,
                    initialUsername: session.username,
                    hostname: session.targetIp,
                },
            });
        } catch (e) {
            errorMsg = String(e);
            isConnecting = false;
        }
    }

    /**
     * Handle SFTP Connection using the new sftp.ts controller
     */
    async function handleSftpClick(e: MouseEvent, session: SessionInfo) {
        e.stopPropagation();
        sftpError = "";
        isSftpConnecting = true;

        try {
            // Menggunakan controller sftp.ts yang baru
            const key = await sftp.connectToSftpSession(session, (status) => {
                // Opsional: Anda bisa menambahkan state sftpStatus jika ingin progress bar
                console.log("[SFTP Status]:", status);
            });

            sftpSession = { key, info: session };
        } catch (err) {
            sftpError = err instanceof Error ? err.message : String(err);
        } finally {
            isSftpConnecting = false;
        }
    }

    async function handleDeleteClick(e: MouseEvent, sessionKey: string) {
        e.stopPropagation();
        if (confirm("Are you sure you want to delete this host?")) {
            try {
                await deleteSessionInfo(sessionKey);
                await deleteSessionPass(sessionKey);
                clearTerminalState(sessionKey);
                sessions = sessions.filter((s) => s.sessionKey !== sessionKey);
            } catch (err) {
                console.error("[Home] Failed to delete session:", err);
            }
        }
    }

    function handleNewHost() {
        prefillSession = null;
        isModalOpen = true;
    }

    function formatDate(ts: number): string {
        return new Date(ts).toLocaleString();
    }

    async function closeSftp() {
        if (sftpSession) {
            try {
                await sftp.disconnectSftp(sftpSession.key);
            } catch (e) {
                console.warn("[SFTP] Disconnect failed during close:", e);
            }
        }
        sftpSession = null;
        sftpError = "";
    }
</script>

<main>
    <div class="app-shell">
        <header class="top-bar">
            <div class="brand">
                <div class="logo-snowflake">❄</div>
                <h1>Snowflakes <span class="version">v1.0</span></h1>
            </div>
            <button class="btn-primary" onclick={handleNewHost}>
                <svg
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2.5"
                >
                    <path d="M12 5v14M5 12h14" />
                </svg>
                New Connection
            </button>
        </header>

        <div class="content-area">
            <div class="table-container">
                <table class="session-table">
                    <thead>
                        <tr>
                            <th>Status</th>
                            <th>Host / Alias</th>
                            <th>Address</th>
                            <th>Last Access</th>
                            <th class="text-right">Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#if isLoading}
                            {#each Array(3) as _}
                                <tr class="skeleton-row">
                                    <td colspan="5"
                                        ><div class="shimmer"></div></td
                                    >
                                </tr>
                            {/each}
                        {:else}
                            {#each sessions as session}
                                <tr
                                    class="session-row"
                                    onclick={() => handleCardClick(session)}
                                >
                                    <td class="td-status">
                                        <div
                                            class="status-indicator online"
                                        ></div>
                                    </td>
                                    <td class="td-info">
                                        <div class="host-main">
                                            <span class="host-icon">🖥</span>
                                            <span class="host-label"
                                                >{session.label ||
                                                    "Untitled Server"}</span
                                            >
                                        </div>
                                    </td>
                                    <td class="td-address">
                                        <code
                                            >{session.username}@{session.targetIp}</code
                                        >
                                    </td>
                                    <td class="td-time">
                                        {formatDate(session.connectedAt)}
                                    </td>
                                    <td class="td-actions">
                                        <div class="action-group">
                                            <button
                                                class="icon-btn sftp"
                                                onclick={(e) =>
                                                    handleSftpClick(e, session)}
                                                title="Open SFTP Browser"
                                            >
                                                <svg
                                                    width="16"
                                                    height="16"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                >
                                                    <path
                                                        d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
                                                    />
                                                </svg>
                                            </button>
                                            <button
                                                class="icon-btn delete"
                                                onclick={(e) =>
                                                    handleDeleteClick(
                                                        e,
                                                        session.sessionKey,
                                                    )}
                                                title="Remove"
                                            >
                                                <svg
                                                    width="16"
                                                    height="16"
                                                    viewBox="0 0 24 24"
                                                    fill="none"
                                                    stroke="currentColor"
                                                    stroke-width="2"
                                                >
                                                    <path
                                                        d="M3 6h18m-2 0v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
                                                    />
                                                </svg>
                                            </button>
                                        </div>
                                    </td>
                                </tr>
                            {/each}
                        {/if}
                    </tbody>
                </table>
            </div>
        </div>
    </div>

    {#if isSftpConnecting || isConnecting}
        <div class="glass-overlay">
            <div class="loader-box">
                <div class="spinner"></div>
                <p>
                    {isSftpConnecting
                        ? "Establishing SFTP Tunnel..."
                        : connectingStatus}
                </p>
                {#if errorMsg || sftpError}
                    <div class="error-bubble">
                        {errorMsg || sftpError}
                        <button
                            onclick={() => {
                                errorMsg = "";
                                sftpError = "";
                                isConnecting = false;
                                isSftpConnecting = false;
                            }}>Close</button
                        >
                    </div>
                {/if}
            </div>
        </div>
    {/if}

    {#if sftpSession}
        <FileSystem
            sessionKey={sftpSession.key}
            sessionInfo={sftpSession.info}
            onClose={closeSftp}
        />
    {/if}

    <NewHostModal
        isOpen={isModalOpen}
        onClose={() => (isModalOpen = false)}
        prefill={prefillSession}
    />
</main>

<style>
    :global(:root) {
        --sf-bg-main: #0b0f14;
        --sf-bg-side: #11161d;
        --sf-border: #1e262f;
        --sf-accent: #4fc3f7;
        --sf-text-dim: #94a3b8;
        --sf-row-hover: rgba(79, 195, 247, 0.04);
    }

    .app-shell {
        width: 100%;
        height: 100vh;
        display: flex;
        flex-direction: column;
        background: var(--sf-bg-main);
        color: white;
    }

    /* Header Styling */
    .top-bar {
        height: 64px;
        padding: 0 32px;
        display: flex;
        align-items: center;
        justify-content: space-between;
        border-bottom: 1px solid var(--sf-border);
        background: var(--sf-bg-side);
    }

    .brand {
        display: flex;
        align-items: center;
        gap: 12px;
    }

    .logo-snowflake {
        font-size: 24px;
        filter: drop-shadow(0 0 8px var(--sf-accent));
    }

    .brand h1 {
        font-size: 18px;
        font-weight: 700;
        margin: 0;
    }

    .version {
        font-size: 10px;
        color: var(--sf-accent);
        vertical-align: top;
        opacity: 0.7;
    }

    /* Button Styling */
    .btn-primary {
        background: var(--sf-accent);
        color: #000;
        border: none;
        padding: 8px 16px;
        border-radius: 6px;
        font-weight: 600;
        font-size: 13px;
        display: flex;
        align-items: center;
        gap: 8px;
        cursor: pointer;
        transition: transform 0.1s;
    }

    .btn-primary:active {
        transform: scale(0.96);
    }

    /* Table Styling */
    .content-area {
        flex: 1;
        padding: 24px 32px;
        overflow-y: auto;
    }

    .table-container {
        background: var(--sf-bg-side);
        border: 1px solid var(--sf-border);
        border-radius: 8px;
    }

    .session-table {
        width: 100%;
        border-collapse: collapse;
        font-size: 13px;
    }

    .session-table th {
        text-align: left;
        padding: 14px 20px;
        color: var(--sf-text-dim);
        font-weight: 500;
        border-bottom: 1px solid var(--sf-border);
        text-transform: uppercase;
        letter-spacing: 0.05em;
        font-size: 11px;
    }

    .session-row {
        border-bottom: 1px solid var(--sf-border);
        cursor: pointer;
        transition: background 0.2s;
    }

    .session-row:hover {
        background: var(--sf-row-hover);
    }

    .session-row td {
        padding: 16px 20px;
        vertical-align: middle;
    }

    .host-main {
        display: flex;
        align-items: center;
        gap: 12px;
    }

    .host-icon {
        font-size: 18px;
        opacity: 0.8;
    }

    .host-label {
        font-weight: 600;
        color: #e2e8f0;
    }

    .td-address code {
        background: rgba(255, 255, 255, 0.05);
        padding: 4px 8px;
        border-radius: 4px;
        color: var(--sf-accent);
        font-family: "JetBrains Mono", monospace;
    }

    /* Status Indicator */
    .status-indicator {
        width: 8px;
        height: 8px;
        border-radius: 50%;
        box-shadow: 0 0 10px currentColor;
    }
    .online {
        color: #10b981;
        background: #10b981;
    }

    /* Action Buttons */
    .action-group {
        display: flex;
        justify-content: flex-end;
        gap: 8px;
    }

    .icon-btn {
        background: #1e293b;
        border: 1px solid var(--sf-border);
        color: var(--sf-text-dim);
        padding: 8px;
        border-radius: 6px;
        cursor: pointer;
        transition: all 0.2s;
    }

    .icon-btn.sftp:hover {
        color: var(--sf-accent);
        border-color: var(--sf-accent);
        background: rgba(79, 195, 247, 0.1);
    }

    .icon-btn.delete:hover {
        color: #ff5252;
        border-color: #ff5252;
        background: rgba(255, 82, 82, 0.1);
    }

    /* Glass Overlay */
    .glass-overlay {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.7);
        backdrop-filter: blur(12px);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
    }

    .loader-box {
        text-align: center;
    }

    .spinner {
        width: 48px;
        height: 48px;
        border: 3px solid rgba(255, 255, 255, 0.1);
        border-top-color: var(--sf-accent);
        border-radius: 50%;
        animation: spin 1s linear infinite;
        margin: 0 auto 16px;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }
</style>
