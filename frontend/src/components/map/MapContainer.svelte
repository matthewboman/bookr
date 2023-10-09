<script lang="ts">
    import Map        from './Map.svelte'
    import MediaQuery from '../ui/MediaQuery.svelte'
    import { mapOptions }   from '../../store'
    import type { Contact } from '../../types'

    export let renderedContacts: Contact[]= []
</script>

{#key $mapOptions}
    <MediaQuery query="(min-width: 1281px)" let:matches>
        {#if matches}
            <div class="map desktop">
                <Map renderedContacts={renderedContacts} mapOptions={$mapOptions}/>
            </div>
        {/if}
    </MediaQuery>
    <MediaQuery query="(max-width: 1280px)" let:matches>
        {#if matches}
            <div class="map tablet-mobile">    
                <Map renderedContacts={renderedContacts} mapOptions={$mapOptions}/>
            </div>
        {/if}
    </MediaQuery>
{/key}

<style>
    .map {
        display: block;
        margin-left: auto;
        margin-right: auto;
        z-index: 5;
        position: sticky; /* Override: no idea what is setting this to `static` */
    }

	/* Device-specific map layout */
	.desktop {
        height: 800px;
    }
    .tablet-mobile {
        flex-grow: 1;
        height: 100vh;
        width: 100vw;
    }
</style>