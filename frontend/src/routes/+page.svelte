<script lang="ts">
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'

	let LeafletContainer;

	onMount(async () => {
		if (browser) {
			LeafletContainer = (await import('../components/map.svelte')).default
		}
	})
</script>

{#if browser}
<svelte:component this={LeafletContainer} />
{/if}