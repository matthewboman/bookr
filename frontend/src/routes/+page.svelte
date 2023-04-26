<script>
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'

	import MediaQuery from '../components/MediaQuery.svelte'
	import About      from '../components/About.svelte'
    // import LeafletContainer from '../components/Map.svelte';

	let LeafletContainer

	onMount(async () => {
		if (browser) {
			LeafletContainer = (await import('../components/MapContainer.svelte')).default
		}
	})
</script>

<style>
    .map {
        display: inline-block;
    }

	/* Device-specific map layout */
	.computer {
        width: 90vw;
        height: 800px;
    }
    .tablet {
        width: 90vw;
        height: 600px;
    }
    .mobile {
        width: 90vw;
        height: 600px;
    }
</style>
<div>
	{#if browser}
	<MediaQuery query="(min-width: 1281px)" let:matches>
		{#if matches}
			<div class="map computer">
				<svelte:component this={LeafletContainer} />
			</div>
		{/if}
	</MediaQuery>

	<MediaQuery query="(min-width: 481px) and (max-width: 1280px)" let:matches>
		{#if matches}
		<div class="map tablet">    
			<svelte:component this={LeafletContainer} />
		</div>
		{/if}
	</MediaQuery>

	<MediaQuery query="(max-width: 480px)" let:matches>
		{#if matches}
		<div class="map mobile">    
			<svelte:component this={LeafletContainer} />
		</div>
		{/if}
	</MediaQuery>
{/if}
</div>

