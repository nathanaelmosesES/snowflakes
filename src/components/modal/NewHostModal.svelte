<script lang="ts">
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let hostname = $state("");
    let port = $state("22");
    let username = $state("");
    let password = $state("");
    let label = $state("");
    let showPassword = $state(false);

    let { isOpen, onClose } = $props();

    $effect(() => {
        const u = username || "user";
        const h = hostname || "hostname";
        const p = port !== "22" ? ` -p ${port}` : "";
        sshCommand = `ssh ${u}@${h}${p}`;
    });

    let sshCommand = $state("ssh user@hostname");

    async function handleSubmit() {
        const newHost = {
            id: crypto.randomUUID(),
            label: label || hostname,
            hostname,
            port,
            username,
            password,
            createdAt: Date.now(),
        };

        // Save to localStorage
        const savedHosts = JSON.parse(
            localStorage.getItem("ssh_hosts") || "[]",
        );
        localStorage.setItem(
            "ssh_hosts",
            JSON.stringify([newHost, ...savedHosts]),
        );

        // Jalankan invoke backend kamu
        const res = await invoke("start_ssh_session", {
            bastion: "10.22.77.251",
            initialPassword: password,
            initialUsername: username,
            hostname: hostname,
        });
        console.log(res);
        // Navigate ke session (titik IP diganti dash untuk URL yang bersih)
        const urlSafeIp = hostname.replaceAll(".", "-");
        goto(`/session?ip=${hostname}`);
    }
</script>

{#if isOpen}
    <div class="overlay" role="dialog">
        <div class="modal">
            <div class="modal-header">
                <span class="modal-title">New SSH connection</span>
                <button class="close-btn" onclick={onClose}>✕</button>
            </div>

            <div class="modal-body">
                <div class="field">
                    <label for="label">Label</label>
                    <input
                        id="label"
                        type="text"
                        placeholder="My web server"
                        bind:value={label}
                    />
                </div>

                <div class="row">
                    <div class="field" style="flex:1">
                        <label for="hostname">Hostname / IP</label>
                        <input
                            id="hostname"
                            type="text"
                            placeholder="192.168.1.1"
                            bind:value={hostname}
                        />
                    </div>
                    <div class="field" style="width:80px">
                        <label for="port">Port</label>
                        <input id="port" type="text" bind:value={port} />
                    </div>
                </div>

                <div class="field">
                    <label for="username">Username</label>
                    <input
                        id="username"
                        type="text"
                        placeholder="root"
                        bind:value={username}
                    />
                </div>

                <div class="field">
                    <label for="password">Password</label>
                    <div class="input-wrap">
                        <input
                            id="password"
                            type={showPassword ? "text" : "password"}
                            placeholder="••••••••"
                            bind:value={password}
                        />
                        <button
                            class="eye-btn"
                            onclick={() => (showPassword = !showPassword)}
                        >
                            {showPassword ? "hide" : "show"}
                        </button>
                    </div>
                </div>

                <div class="cmd-label">Generated SSH command</div>
                <div class="cmd-box">
                    <span class="cmd-text">{sshCommand}</span>
                    <button
                        class="copy-btn"
                        onclick={() =>
                            navigator.clipboard.writeText(sshCommand)}
                    >
                        copy
                    </button>
                </div>
            </div>

            <div class="modal-footer">
                <button class="btn-ghost" onclick={onClose}>Cancel</button>
                <button class="btn-primary" onclick={handleSubmit}
                    >Save & connect</button
                >
            </div>
        </div>
    </div>
{/if}

<style>
    .overlay {
        position: fixed;
        margin-top: 32px;
        inset: 0;
        background: rgba(5, 15, 28, 0.85);
        display: flex;
        align-items: center;
        justify-content: center;
        max-height: calc(100% - 32px);
        overflow-y: auto;
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

    .close-btn {
        background: none;
        border: none;
        color: var(--sf-text-hint);
        cursor: pointer;
        font-size: 14px;
        padding: 2px 6px;
    }

    .close-btn:hover {
        color: var(--sf-text-muted);
    }

    .modal-body {
        padding: 16px 18px;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .row {
        display: flex;
        gap: 8px;
    }

    .field {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    label {
        font-size: 11px;
        color: var(--sf-text-primary);
    }

    input {
        background: var(--sf-bg-secondary);
        border: 1px solid var(--sf-border);
        border-radius: 7px;
        padding: 8px 10px;
        font-size: 12px;
        color: var(--sf-text-primary);
        outline: none;
        width: 100%;
        font-family: var(--sf-font-ui);
    }

    input:focus {
        border-color: var(--sf-accent);
    }

    input::placeholder {
        color: var(--sf-text-hint);
    }

    .input-wrap {
        position: relative;
    }

    .input-wrap input {
        padding-right: 52px;
    }

    .eye-btn {
        position: absolute;
        right: 8px;
        top: 50%;
        transform: translateY(-50%);
        background: none;
        border: none;
        font-size: 10px;
        color: var(--sf-text-hint);
        cursor: pointer;
    }

    .eye-btn:hover {
        color: var(--sf-text-muted);
    }

    .cmd-label {
        font-size: 11px;
        color: var(--sf-text-muted);
    }

    .cmd-box {
        background: var(--sf-bg-terminal);
        border: 1px solid var(--sf-border);
        border-radius: 7px;
        padding: 10px 12px;
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 8px;
    }

    .cmd-text {
        font-family: var(--sf-font-mono);
        font-size: 11px;
        color: var(--sf-accent);
        word-break: break-all;
        flex: 1;
    }

    .copy-btn {
        font-size: 10px;
        color: var(--sf-text-hint);
        background: none;
        border: 1px solid var(--sf-border);
        border-radius: 5px;
        padding: 3px 8px;
        cursor: pointer;
        white-space: nowrap;
    }

    .copy-btn:hover {
        color: var(--sf-accent);
        border-color: var(--sf-border-hover);
    }

    .modal-footer {
        padding: 12px 18px;
        border-top: 1px solid var(--sf-border);
        display: flex;
        justify-content: flex-end;
        gap: 8px;
    }

    .btn-ghost {
        background: transparent;
        border: 1px solid var(--sf-border);
        border-radius: 7px;
        padding: 7px 14px;
        font-size: 12px;
        color: var(--sf-text-muted);
        cursor: pointer;
    }

    .btn-ghost:hover {
        border-color: var(--sf-border-hover);
        color: var(--sf-text);
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
