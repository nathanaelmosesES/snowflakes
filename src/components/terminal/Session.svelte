<script lang="ts">
    import { onMount } from "svelte";
    import { Terminal } from "@xterm/xterm";
    import { FitAddon } from "@xterm/addon-fit";
    import { listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/core";
    import "@xterm/xterm/css/xterm.css";

    let terminalElement: HTMLElement;
    let unlisten: UnlistenFn; // Store this for cleanup

    onMount(() => {
        const term = new Terminal({
            theme: { background: "#1a1b26" },
            cursorBlink: true,
        });

        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        term.open(terminalElement);
        fitAddon.fit();

        // 1. Create an internal async function to handle the setup
        const setupSsh = async () => {
            unlisten = await listen("ssh-output", (event) => {
                term.write(event.payload as string);
                console.log(event.payload);
            });

            term.onData((data) => {
                invoke("send_ssh_input", { input: data });
            });
        };

        setupSsh();

        const handleResize = () => fitAddon.fit();
        window.addEventListener("resize", handleResize);

        // 2. Return a synchronous cleanup function
        return () => {
            window.removeEventListener("resize", handleResize);
            if (unlisten) unlisten();
            term.dispose();
        };
    });
</script>

<div class="h-screen w-screen bg-[#1a1b26] p-2">
    <div bind:this={terminalElement} class="h-full w-full"></div>
</div>
