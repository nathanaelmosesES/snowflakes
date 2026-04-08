<script lang="ts">
  import { onMount } from "svelte";
  import NewHostModal from "../components/modal/NewHostModal.svelte";
  import Session from "../components/terminal/Session.svelte";
  import type { Host } from "../types/host";

  let isModalOpen = $state(false);

  let hosts = $state<Host[]>([]);

  onMount(() => {
    let hosts_str = localStorage.getItem("hosts");
    let hosts_parsed: Host[] = [];
    if (hosts_str) {
      hosts_parsed = JSON.parse(hosts_str) as Host[];
    }
    hosts = hosts_parsed;
  });
</script>

<main>
  <div class="main-container">
    <div class="host-container">
      <div class="grid">
        <!-- Add New Card -->
        <button class="card card-new" onclick={() => (isModalOpen = true)}>
          <div class="new-icon">
            <svg
              width="20"
              height="20"
              viewBox="0 0 20 20"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
            >
              <path d="M10 4v12M4 10h12" />
            </svg>
          </div>
          <span class="new-label">New host</span>
        </button>

        <!-- Host Cards -->
        {#each hosts as host}
          <div class="card group">
            <div class="card-top">
              <div class="card-title-row">
                <div
                  class="status-dot"
                  class:online={host.isOnline}
                  class:offline={!host.isOnline}
                ></div>
                <h3 class="hostname">{host.hostname}</h3>
              </div>
              <span class="label-pill">{host.label}</span>
            </div>

            <div class="card-meta">
              <code class="user-ip">{host.user}@{host.ip}</code>
              <div class="last-seen">
                <svg
                  width="12"
                  height="12"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <circle cx="12" cy="12" r="10" /><path d="M12 6v6l4 2" />
                </svg>
                Last seen: {host.lastSeen}
              </div>
            </div>

            <div class="card-footer">
              <div class="dots">
                <div class="dot"></div>
                <div class="dot"></div>
              </div>
              <span class="connect-hint">
                Connect
                <svg
                  width="12"
                  height="12"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <path d="M5 12h14M12 5l7 7-7 7" />
                </svg>
              </span>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>
  <NewHostModal isOpen={isModalOpen} onClose={() => (isModalOpen = false)} />
</main>

<style>
  .main-container {
    margin-top: 32px;
    width: 100%;
    height: 100vh;
    background-color: var(--sf-bg-app);
    display: flex;
  }

  .host-container {
    flex: 1;
    padding: 32px;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 20px;
    max-width: 80%;
  }

  .card {
    background: var(--sf-bg-surface);
    border-left: 2px solid transparent;
    padding: 24px;
    transition:
      border-color 0.15s,
      background 0.15s;
    position: relative;
    cursor: pointer;
    text-align: left;
  }

  .card:hover {
    border-left-color: var(--sf-accent);
    background: var(--sf-bg-hover);
  }

  .card:hover .connect-hint {
    opacity: 1;
  }

  /* New host card */
  .card-new {
    border: 1px dashed var(--sf-border);
    border-left: 2px solid transparent;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    min-height: 160px;
  }

  .card-new:hover {
    border-color: var(--sf-accent);
    border-left-color: var(--sf-accent);
    border-style: solid;
  }

  .new-icon {
    color: var(--sf-text-hint);
    transition: color 0.15s;
  }

  .card-new:hover .new-icon {
    color: var(--sf-accent);
  }

  .new-label {
    font-size: 11px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--sf-text-hint);
    transition: color 0.15s;
  }

  .card-new:hover .new-label {
    color: var(--sf-accent);
  }

  /* Card internals */
  .card-top {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 16px;
  }

  .card-title-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    flex-shrink: 0;
  }

  .status-dot.online {
    background: var(--sf-online);
  }

  .status-dot.offline {
    background: var(--sf-offline);
  }

  .hostname {
    font-size: 14px;
    font-weight: 500;
    color: var(--sf-text);
    letter-spacing: -0.01em;
  }

  .label-pill {
    font-family: var(--sf-font-mono);
    font-size: 10px;
    color: var(--sf-accent);
    background: rgba(79, 195, 247, 0.05);
    border: 1px solid rgba(79, 195, 247, 0.2);
    padding: 2px 8px;
  }

  .card-meta {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 24px;
  }

  .user-ip {
    font-family: var(--sf-font-mono);
    font-size: 12px;
    color: var(--sf-text-muted);
  }

  .last-seen {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 10px;
    color: var(--sf-text-hint);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    font-weight: 500;
  }

  .last-seen svg {
    color: var(--sf-text-hint);
  }

  .card-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .dots {
    display: flex;
    gap: 4px;
  }

  .dot {
    width: 6px;
    height: 6px;
    background: var(--sf-border);
  }

  .connect-hint {
    opacity: 0;
    transition: opacity 0.15s;
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--sf-accent);
  }
</style>
