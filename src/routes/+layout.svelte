<script>
	import { page } from "$app/stores";
	import {
		House,
		Folders,
		ShieldCheck,
		ArrowLeftRight,
		Settings,
	} from "@lucide/svelte";
	import SidebarElement from "../components/home/SidebarElement.svelte";
	import "../layout.css";
	import TitleBar from "../components/TitleBar.svelte";
	const menus = [
		{ icon: House, text: "HOME", href: "/" },
		{ icon: Folders, text: "FILES", href: "/files" },
		{ icon: ShieldCheck, text: "KNOWN HOSTS", href: "/hosts" },
		{ icon: ArrowLeftRight, text: "PORT FORWARDING", href: "/ports" },
		{ icon: Settings, text: "SETTINGS", href: "/settings" },
	];

	let showSidebar = menus.some((menu) => menu.href === $page.url.pathname);
</script>

<TitleBar />
<div class="app-layout">
	{#if showSidebar}
		<aside class="sidebar-container">
			<div class="brand-section">
				<h1 class="brand-title">SNOWFLAKES</h1>
				<p class="brand-subtitle">SSH MANAGER</p>
			</div>

			<nav class="nav-menu">
				{#each menus as menu}
					<SidebarElement
						text={menu.text}
						icon={menu.icon}
						href={menu.href}
						isActive={$page.url.pathname === menu.href}
					/>
				{/each}
			</nav>
		</aside>
	{/if}

	<main class="content">
		<slot />
	</main>
</div>

<style>
	.app-layout {
		display: flex;
		min-height: 100vh;
		overflow: hidden;
	}

	.content {
		flex: 1;
		overflow-y: auto;
	}

	.sidebar-container {
		height: 100vh;
		width: var(--sf-sidebar-width);
		background-color: var(--sf-bg-sidebar);
		border-right: 1px solid var(--sf-border);
		display: flex;
		flex-direction: column;
		padding-top: var(--sf-space-xl);
		position: sticky;
		top: 32px;
	}

	/* ... Sisanya copy paste dari style sidebar lama Anda ... */
	.brand-section {
		padding: 0 var(--sf-space-lg);
		margin-bottom: var(--sf-space-xl);
	}
	.brand-title {
		font-family: var(--sf-font-title);
		font-size: 1.25rem;
		font-weight: 800;
		color: var(--sf-accent);
		margin: 0;
	}
	.brand-subtitle {
		font-family: var(--sf-font-ui);
		font-size: 0.7rem;
		font-weight: 700;
		color: var(--sf-text-secondary);
		margin: 4px 0 0 0;
		text-transform: uppercase;
	}
	.nav-menu {
		display: flex;
		flex-direction: column;
		gap: 4px;
		padding: 0 8px;
	}
</style>
