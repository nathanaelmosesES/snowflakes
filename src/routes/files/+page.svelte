<script lang="ts">
    import {
        Folder,
        FolderOpen,
        FileText,
        FileCode,
        Archive,
        Image as ImageIcon,
        Server,
        ChevronRight,
        ChevronDown,
        Upload,
        Plus,
        Search,
        MoreVertical,
        Info,
        HardDrive,
        Download,
        Trash2,
        Home,
    } from "@lucide/svelte";

    // MOCK DATA
    const mockTree = [
        {
            name: "/",
            type: "folder",
            isOpen: true,
            level: 0,
            children: [
                {
                    name: "home",
                    type: "folder",
                    isOpen: true,
                    level: 1,
                    children: [
                        {
                            name: "ubuntu",
                            type: "folder",
                            isOpen: false,
                            level: 2,
                            children: [],
                        },
                        {
                            name: "snowflakes",
                            type: "folder",
                            isOpen: true,
                            level: 2,
                            children: [
                                {
                                    name: "projects",
                                    type: "folder",
                                    isOpen: false,
                                    level: 3,
                                    children: [],
                                },
                                {
                                    name: "config",
                                    type: "folder",
                                    isOpen: false,
                                    level: 3,
                                    children: [],
                                },
                            ],
                        },
                    ],
                },
                {
                    name: "var",
                    type: "folder",
                    isOpen: false,
                    level: 1,
                    children: [],
                },
                {
                    name: "etc",
                    type: "folder",
                    isOpen: false,
                    level: 1,
                    children: [],
                },
            ],
        },
    ];

    const mockFiles = [
        {
            name: "docker-compose.yml",
            type: "file",
            icon: FileCode,
            size: "1.2 KB",
            modified: "Today, 10:42 AM",
            permissions: "-rw-r--r--",
        },
        {
            name: "nginx.conf",
            type: "file",
            icon: FileCode,
            size: "4.5 KB",
            modified: "Yesterday, 3:15 PM",
            permissions: "-rw-r--r--",
        },
        {
            name: ".env",
            type: "file",
            icon: FileText,
            size: "432 B",
            modified: "Oct 15, 2025",
            permissions: "-rw-------",
        },
        {
            name: "data",
            type: "folder",
            icon: Folder,
            size: "--",
            modified: "Oct 12, 2025",
            permissions: "drwxr-xr-x",
        },
        {
            name: "logs",
            type: "folder",
            icon: Folder,
            size: "--",
            modified: "Oct 11, 2025",
            permissions: "drwxr-xr-x",
        },
        {
            name: "backup.tar.gz",
            type: "archive",
            icon: Archive,
            size: "142.5 MB",
            modified: "Oct 10, 2025",
            permissions: "-rw-r--r--",
        },
        {
            name: "schema.png",
            type: "image",
            icon: ImageIcon,
            size: "2.1 MB",
            modified: "Oct 05, 2025",
            permissions: "-rw-r--r--",
        },
    ];

    const breadcrumbs = [
        "Snowflakes (Production)",
        "home",
        "snowflakes",
        "projects",
    ];

    let selectedFile = $state(mockFiles[0]);
    let activeConnection = $state("192.168.1.100");
</script>

<div class="sftp-layout">
    <!-- Header / Toolbar -->
    <header class="sftp-header">
        <div class="header-left">
            <div class="connection-selector">
                <Server size={16} class="text-accent" />
                <span>prod-cluster-1 (192.168.1.100)</span>
                <ChevronDown size={14} class="text-muted" />
            </div>
            <div class="divider"></div>
            <div class="breadcrumbs">
                <Home size={14} class="crumb-icon" />
                {#each breadcrumbs as crumb, i}
                    <span class="crumb-text">{crumb}</span>
                    {#if i < breadcrumbs.length - 1}
                        <ChevronRight size={14} class="crumb-sep" />
                    {/if}
                {/each}
            </div>
        </div>

        <div class="header-right">
            <div class="search-box">
                <Search size={14} class="search-icon" />
                <input type="text" placeholder="Search files..." />
            </div>
            <div class="action-buttons">
                <button class="btn btn-icon" title="New Folder">
                    <Plus size={16} />
                </button>
                <button class="btn btn-primary">
                    <Upload size={16} />
                    <span>Upload</span>
                </button>
            </div>
        </div>
    </header>

    <div class="sftp-body">
        <!-- Tree Sidebar -->
        <aside class="sftp-sidebar">
            <div class="sidebar-section">
                <h3 class="section-title">REMOTE FILESYSTEM</h3>
                <div class="tree-container">
                    <!-- Simplistic static representation of tree for mockup -->
                    <div class="tree-node">
                        <div class="tree-item open">
                            <ChevronDown size={14} class="tree-chevron" />
                            <FolderOpen
                                size={16}
                                class="tree-icon icon-folder"
                            />
                            <span class="tree-label">/</span>
                        </div>
                        <div class="tree-children">
                            <div class="tree-item open">
                                <ChevronDown size={14} class="tree-chevron" />
                                <FolderOpen
                                    size={16}
                                    class="tree-icon icon-folder"
                                />
                                <span class="tree-label">home</span>
                            </div>
                            <div class="tree-children">
                                <div class="tree-item">
                                    <ChevronRight
                                        size={14}
                                        class="tree-chevron"
                                    />
                                    <Folder
                                        size={16}
                                        class="tree-icon icon-folder"
                                    />
                                    <span class="tree-label">ubuntu</span>
                                </div>
                                <div class="tree-item active">
                                    <ChevronDown
                                        size={14}
                                        class="tree-chevron"
                                    />
                                    <FolderOpen
                                        size={16}
                                        class="tree-icon icon-folder"
                                    />
                                    <span class="tree-label">snowflakes</span>
                                </div>
                                <div class="tree-children active-children">
                                    <div class="tree-item selected">
                                        <div class="tree-spacer"></div>
                                        <Folder
                                            size={16}
                                            class="tree-icon icon-folder text-accent"
                                        />
                                        <span class="tree-label">projects</span>
                                    </div>
                                    <div class="tree-item">
                                        <div class="tree-spacer"></div>
                                        <Folder
                                            size={16}
                                            class="tree-icon icon-folder"
                                        />
                                        <span class="tree-label">config</span>
                                    </div>
                                </div>
                            </div>

                            <div class="tree-item">
                                <ChevronRight size={14} class="tree-chevron" />
                                <Folder
                                    size={16}
                                    class="tree-icon icon-folder"
                                />
                                <span class="tree-label">var</span>
                            </div>
                            <div class="tree-item">
                                <ChevronRight size={14} class="tree-chevron" />
                                <Folder
                                    size={16}
                                    class="tree-icon icon-folder"
                                />
                                <span class="tree-label">etc</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="sidebar-section">
                <h3 class="section-title">STORAGE</h3>
                <div class="storage-info">
                    <div class="storage-row">
                        <HardDrive size={14} class="text-secondary" />
                        <span class="storage-label">/dev/sda1</span>
                    </div>
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: 76%"></div>
                    </div>
                    <div class="storage-stats">
                        <span>76 GB used</span>
                        <span>100 GB</span>
                    </div>
                </div>
            </div>
        </aside>

        <!-- Main Content -->
        <main class="sftp-main">
            <div class="list-header">
                <div class="col-name">Name</div>
                <div class="col-size">Size</div>
                <div class="col-date">Modified</div>
                <div class="col-perms">Permissions</div>
            </div>

            <div class="file-list">
                {#each mockFiles as file}
                    <div
                        class="file-row {selectedFile.name === file.name
                            ? 'selected'
                            : ''}"
                        onclick={() => (selectedFile = file)}
                    >
                        <div class="col-name">
                            <svelte:component
                                this={file.icon}
                                size={18}
                                class="file-icon {file.type === 'folder'
                                    ? 'icon-folder'
                                    : file.type === 'archive'
                                      ? 'icon-archive'
                                      : file.type === 'image'
                                        ? 'icon-image'
                                        : 'icon-file'}"
                            />
                            <span class="file-name">{file.name}</span>
                        </div>
                        <div class="col-size">{file.size}</div>
                        <div class="col-date">{file.modified}</div>
                        <div class="col-perms">
                            <span class="perms-badge">{file.permissions}</span>
                        </div>
                    </div>
                {/each}
            </div>
        </main>

        <!-- Details / Preview Pane -->
        <aside class="sftp-details">
            {#if selectedFile}
                <div class="details-header">
                    <div class="preview-box">
                        <svelte:component
                            this={selectedFile.icon}
                            size={48}
                            stroke-width="1.5"
                            class="preview-icon {selectedFile.type === 'folder'
                                ? 'icon-folder'
                                : selectedFile.type === 'archive'
                                  ? 'icon-archive'
                                  : selectedFile.type === 'image'
                                    ? 'icon-image'
                                    : 'icon-file'}"
                        />
                    </div>
                    <h2 class="file-title">{selectedFile.name}</h2>
                    <span class="file-type"
                        >{selectedFile.type.toUpperCase()}</span
                    >
                </div>

                <div class="details-info">
                    <div class="info-row">
                        <span class="info-label">Size</span>
                        <span class="info-value">{selectedFile.size}</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Modified</span>
                        <span class="info-value">{selectedFile.modified}</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Owner</span>
                        <span class="info-value">ubuntu (1000)</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Group</span>
                        <span class="info-value">ubuntu (1000)</span>
                    </div>
                    <div class="info-row">
                        <span class="info-label">Permissions</span>
                        <span class="info-value font-mono text-accent"
                            >{selectedFile.permissions}</span
                        >
                    </div>
                </div>

                <div class="details-actions">
                    <button class="btn btn-outline">
                        <Download size={14} /> Download
                    </button>
                    <button class="btn btn-danger">
                        <Trash2 size={14} /> Delete
                    </button>
                </div>
            {/if}
        </aside>
    </div>
</div>

<style>
    /* ── Utilities ────────────────────────────────────────── */
    .text-accent {
        color: var(--sf-accent) !important;
    }
    .text-muted {
        color: var(--sf-text-muted, #4a6a8a) !important;
    }
    .text-secondary {
        color: var(--sf-text-secondary) !important;
    }
    .font-mono {
        font-family: var(--sf-font-mono);
    }

    .icon-folder {
        color: #f6c177;
        fill: rgba(246, 193, 119, 0.2);
    }
    .icon-archive {
        color: #eb6f92;
    }
    .icon-image {
        color: #9ccfd8;
    }
    .icon-file {
        color: var(--sf-text-secondary);
    }

    /* ── Layout ───────────────────────────────────────────── */
    .sftp-layout {
        display: flex;
        flex-direction: column;
        height: 100vh;
        width: 100%;
        background-color: var(--sf-bg-app);
        overflow: hidden;
    }

    /* ── Header ───────────────────────────────────────────── */
    .sftp-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        height: 56px;
        padding: 0 20px;
        background-color: var(--sf-bg-surface);
        border-bottom: 1px solid var(--sf-border);
        flex-shrink: 0;
    }

    .header-left,
    .header-right {
        display: flex;
        align-items: center;
        gap: 16px;
    }

    .connection-selector {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 6px 12px;
        background: var(--sf-bg-app);
        border: 1px solid var(--sf-border);
        border-radius: var(--sf-radius-md);
        font-family: var(--sf-font-mono);
        font-size: 11px;
        color: var(--sf-text-primary);
        cursor: pointer;
        transition: all 0.2s;
    }

    .connection-selector:hover {
        border-color: var(--sf-border-hover);
        background: var(--sf-bg-hover);
    }

    .divider {
        width: 1px;
        height: 20px;
        background: var(--sf-border);
    }

    .breadcrumbs {
        display: flex;
        align-items: center;
        gap: 6px;
        font-family: var(--sf-font-ui);
        font-size: 13px;
        color: var(--sf-text-secondary);
    }

    .crumb-text {
        cursor: pointer;
        transition: color 0.15s;
    }

    .crumb-text:hover {
        color: var(--sf-text-primary);
    }

    .crumb-text:last-child {
        color: var(--sf-text-primary);
        font-weight: 500;
    }

    .crumb-sep,
    .crumb-icon {
        color: var(--sf-text-muted, #4a6a8a);
    }

    /* ── Search & Buttons ─────────────────────────────────── */
    .search-box {
        position: relative;
        display: flex;
        align-items: center;
    }

    .search-icon {
        position: absolute;
        left: 10px;
        color: var(--sf-text-muted, #4a6a8a);
    }

    .search-box input {
        width: 200px;
        height: 32px;
        background: var(--sf-bg-input);
        border: 1px solid var(--sf-border);
        border-radius: var(--sf-radius-md);
        padding: 0 12px 0 32px;
        color: var(--sf-text-primary);
        font-family: var(--sf-font-ui);
        font-size: 12px;
        outline: none;
        transition: all 0.2s;
    }

    .search-box input:focus {
        border-color: var(--sf-border-focus);
        width: 260px;
    }

    .action-buttons {
        display: flex;
        gap: 8px;
    }

    .btn {
        display: flex;
        align-items: center;
        gap: 6px;
        height: 32px;
        padding: 0 12px;
        font-family: var(--sf-font-ui);
        font-size: 12px;
        font-weight: 500;
        border-radius: var(--sf-radius-md);
        cursor: pointer;
        transition: all 0.2s;
        border: none;
        outline: none;
    }

    .btn-icon {
        padding: 0;
        width: 32px;
        justify-content: center;
        background: transparent;
        color: var(--sf-text-secondary);
        border: 1px solid var(--sf-border);
    }

    .btn-icon:hover {
        background: var(--sf-bg-hover);
        color: var(--sf-text-primary);
        border-color: var(--sf-border-hover);
    }

    .btn-primary {
        background: var(--sf-accent);
        color: var(--sf-text-on-accent, #042c53);
    }

    .btn-primary:hover {
        background: var(--sf-accent-hover, #38b0e0);
        transform: translateY(-1px);
    }

    .btn-outline {
        background: transparent;
        color: var(--sf-text-primary);
        border: 1px solid var(--sf-border);
    }

    .btn-outline:hover {
        background: var(--sf-bg-hover);
    }

    .btn-danger {
        background: rgba(239, 83, 80, 0.1);
        color: var(--sf-status-error);
        border: 1px solid rgba(239, 83, 80, 0.3);
    }

    .btn-danger:hover {
        background: rgba(239, 83, 80, 0.2);
    }

    /* ── Body Layout ──────────────────────────────────────── */
    .sftp-body {
        display: flex;
        flex: 1;
        overflow: hidden;
    }

    /* ── Sidebar ──────────────────────────────────────────── */
    .sftp-sidebar {
        width: 260px;
        background: var(--sf-bg-app);
        border-right: 1px solid var(--sf-border);
        display: flex;
        flex-direction: column;
        flex-shrink: 0;
        overflow-y: auto;
    }

    .sidebar-section {
        padding: 20px 16px;
    }

    .section-title {
        font-family: var(--sf-font-ui);
        font-size: 11px;
        font-weight: 600;
        color: var(--sf-text-muted, #4a6a8a);
        letter-spacing: 0.05em;
        margin: 0 0 12px 0;
    }

    /* ── Tree View ────────────────────────────────────────── */
    .tree-container {
        font-family: var(--sf-font-ui);
        font-size: 13px;
        color: var(--sf-text-secondary);
    }

    .tree-children {
        margin-left: 14px;
        border-left: 1px solid var(--sf-border);
    }

    .tree-item {
        display: flex;
        align-items: center;
        gap: 6px;
        padding: 6px 8px;
        border-radius: var(--sf-radius-sm);
        cursor: pointer;
        transition: all 0.15s;
        margin-left: 4px;
    }

    .tree-item:hover {
        background: var(--sf-bg-hover);
        color: var(--sf-text-primary);
    }

    .tree-item.selected {
        background: rgba(79, 195, 247, 0.1);
        color: var(--sf-text-primary);
    }

    .tree-chevron {
        color: var(--sf-text-muted, #4a6a8a);
    }

    .tree-spacer {
        width: 14px; /* Matches chevron width when no chevron */
    }

    .tree-label {
        font-weight: 500;
        letter-spacing: 0.01em;
    }

    .tree-item.selected .tree-label {
        color: var(--sf-accent);
    }

    /* ── Storage Area ─────────────────────────────────────── */
    .storage-info {
        background: var(--sf-bg-surface);
        padding: 16px;
        border-radius: var(--sf-radius-md);
        border: 1px solid var(--sf-border);
        display: flex;
        flex-direction: column;
        gap: 10px;
    }

    .storage-row {
        display: flex;
        align-items: center;
        gap: 8px;
    }

    .storage-label {
        font-family: var(--sf-font-mono);
        font-size: 12px;
        color: var(--sf-text-primary);
    }

    .progress-bar {
        height: 6px;
        background: var(--sf-bg-app);
        border-radius: var(--sf-radius-pill);
        overflow: hidden;
    }

    .progress-fill {
        height: 100%;
        background: var(--sf-accent);
        border-radius: var(--sf-radius-pill);
    }

    .storage-stats {
        display: flex;
        justify-content: space-between;
        font-size: 11px;
        color: var(--sf-text-secondary);
        font-family: var(--sf-font-ui);
    }

    /* ── Main List ────────────────────────────────────────── */
    .sftp-main {
        flex: 1;
        background: var(--sf-bg-app);
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .list-header {
        display: grid;
        grid-template-columns: 3fr 1fr 2fr 1fr;
        padding: 10px 24px;
        border-bottom: 1px solid var(--sf-border);
        font-family: var(--sf-font-ui);
        font-size: 11px;
        font-weight: 600;
        text-transform: uppercase;
        color: var(--sf-text-muted, #4a6a8a);
        letter-spacing: 0.05em;
    }

    .file-list {
        flex: 1;
        overflow-y: auto;
        padding: 8px;
    }

    .file-list::-webkit-scrollbar {
        width: 6px;
    }
    .file-list::-webkit-scrollbar-thumb {
        background: var(--sf-border);
        border-radius: 4px;
    }

    .file-row {
        display: grid;
        grid-template-columns: 3fr 1fr 2fr 1fr;
        align-items: center;
        padding: 10px 16px;
        border-radius: var(--sf-radius-sm);
        cursor: pointer;
        transition: all 0.15s;
        border: 1px solid transparent;
    }

    .file-row:hover {
        background: var(--sf-bg-surface);
    }

    .file-row.selected {
        background: rgba(79, 195, 247, 0.08);
        border-color: var(--sf-border-focus);
    }

    .col-name {
        display: flex;
        align-items: center;
        gap: 12px;
        font-family: var(--sf-font-ui);
        font-size: 13px;
        font-weight: 500;
        color: var(--sf-text-primary);
    }

    .col-size,
    .col-date {
        font-family: var(--sf-font-ui);
        font-size: 12px;
        color: var(--sf-text-secondary);
    }

    .col-perms {
        display: flex;
        align-items: center;
    }

    .perms-badge {
        font-family: var(--sf-font-mono);
        font-size: 11px;
        color: var(--sf-text-muted, #4a6a8a);
        background: var(--sf-bg-surface);
        padding: 4px 8px;
        border-radius: 4px;
        border: 1px solid var(--sf-border);
    }

    /* ── Details Pane ─────────────────────────────────────── */
    .sftp-details {
        width: 280px;
        background: var(--sf-bg-surface);
        border-left: 1px solid var(--sf-border);
        display: flex;
        flex-direction: column;
        flex-shrink: 0;
        overflow-y: auto;
    }

    .details-header {
        padding: 32px 24px;
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
        border-bottom: 1px solid rgba(26, 51, 82, 0.5);
    }

    .preview-box {
        width: 96px;
        height: 96px;
        background: var(--sf-bg-app);
        border: 1px solid var(--sf-border);
        border-radius: var(--sf-radius-lg);
        display: flex;
        align-items: center;
        justify-content: center;
        margin-bottom: 16px;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
    }

    .file-title {
        font-family: var(--sf-font-ui);
        font-size: 16px;
        font-weight: 600;
        color: var(--sf-text-primary);
        margin: 0 0 6px 0;
        word-break: break-all;
    }

    .file-type {
        font-family: var(--sf-font-ui);
        font-size: 11px;
        font-weight: 600;
        letter-spacing: 0.1em;
        color: var(--sf-text-secondary);
    }

    .details-info {
        padding: 24px;
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .info-row {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .info-label {
        font-family: var(--sf-font-ui);
        font-size: 11px;
        color: var(--sf-text-muted, #4a6a8a);
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }

    .info-value {
        font-family: var(--sf-font-ui);
        font-size: 13px;
        color: var(--sf-text-primary);
    }

    .details-actions {
        padding: 0 24px 24px;
        display: flex;
        flex-direction: column;
        gap: 10px;
        margin-top: auto;
    }
</style>
