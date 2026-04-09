<script lang="ts">
    import { onMount } from "svelte";
    import { page } from "$app/stores"; // SvelteKit store
    import { Terminal } from "@xterm/xterm";
    import { FitAddon } from "@xterm/addon-fit";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/core";
    import "@xterm/xterm/css/xterm.css";
    import ErrorModal from "../../components/modal/ErrorModal.svelte";
    import {
        client,
        getRecord,
        insertRecord,
        stronghold,
    } from "../../controller/vault";

    // Ambil IP dari query param ?ip=...
    const targetIp = $page.url.searchParams.get("ip") || "unknown";

    let terminalElement: HTMLElement;
    let unlisten: UnlistenFn;
    let unlistenError: UnlistenFn;
    let isOpenerrorMessage = $state(false);
    let errorMessage = $state("");

    onMount(() => {
        const term = new Terminal({
            theme: {
                background: "#1a1b26",
                foreground: "#a9b1d6",
                cursor: "#f7768e",
            },
            cursorBlink: true,
            fontFamily: "JetBrains Mono, monospace",
            fontSize: 13,
        });

        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        term.open(terminalElement);
        fitAddon.fit();
        term.focus();

        const setupSsh = async () => {
            const eventName = `ssh-output-${targetIp.replaceAll(".", "-")}`;
            console.log(`Listening to: ${eventName}, ${targetIp}`);

            unlisten = await listen(eventName, (event) => {
                term.write(event.payload as string);
            });

            const errorEventName = `ssh-error-output-${targetIp.replaceAll(".", "-")}`;

            unlistenError = await listen(errorEventName, (event) => {
                isOpenerrorMessage = true;
                errorMessage = event.payload as string;
            });

            let inputBuffer = "";
            let debounceTimer: ReturnType<typeof setTimeout> | null = null;

            term.onData((data: string) => {
                inputBuffer += data;

                if (debounceTimer) return;

                debounceTimer = setTimeout(() => {
                    const payload = inputBuffer;
                    inputBuffer = "";
                    debounceTimer = null;

                    const res = invoke("send_ssh_input", {
                        input: payload,
                        ip: targetIp,
                    });

                    console.log("sent:", payload);
                    console.log(res);
                }, 70);
            });
        };
        const setupVault = async () => {
            const store = client.getStore();
            const key = "my_key";

            // Insert a record to the store
            insertRecord(store, key, "secret value");

            // Read a record from store
            const value = await getRecord(store, key);
            console.log(value); // 'secret value'

            // Save your updates
            await stronghold.save();

            // Remove a record from store
            await store.remove(key);
        };
        setupSsh();
        setupVault();

        const handleResize = () => fitAddon.fit();
        window.addEventListener("resize", handleResize);

        return () => {
            window.removeEventListener("resize", handleResize);
            if (unlisten) unlisten();
            if (unlistenError) unlistenError();
            term.dispose();
        };
    });
</script>

<div class="flex flex-col h-screen bg-[#1a1b26]">
    <div
        class="px-4 py-2 bg-[#16161e] border-b border-[#24283b] flex justify-between items-center"
    >
        <div class="text-[11px] text-[#565f89] font-mono">
            CONNECTED TO: <span class="text-[#7aa2f7]">{targetIp}</span>
        </div>
        <button
            class="text-[11px] text-[#f7768e] hover:underline"
            onclick={() => history.back()}
        >
            Disconnect
        </button>
    </div>

    <div class="flex-1 p-2">
        <div
            bind:this={terminalElement}
            class="h-[calc(100%-32px)] w-full"
        ></div>
    </div>
    <ErrorModal isOpen={isOpenerrorMessage} {errorMessage} />
</div>

<style>
</style>
