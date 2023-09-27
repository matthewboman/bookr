<script lang="ts">
    import Map        from './Map.svelte'
    import MediaQuery from '../MediaQuery.svelte'
    import { mapOptions }   from '../../store'
    import type { Contact } from '../../types'

    export let renderedContacts: Contact[]= []
</script>

{#key $mapOptions}
    <MediaQuery query="(min-width: 1281px)" let:matches>
        {#if matches}
            <div class="map computer"> 
                <Map renderedContacts={renderedContacts} mapOptions={$mapOptions}/>   
            </div>
        {/if}
    </MediaQuery>
    <MediaQuery query="(min-width: 481px) and (max-width: 1280px)" let:matches>
        {#if matches}
            <div class="map tablet">    
                <Map renderedContacts={renderedContacts} mapOptions={$mapOptions}/>
            </div>
        {/if}
    </MediaQuery>

    <MediaQuery query="(max-width: 480px)" let:matches>
        {#if matches}
            <div class="map mobile">    
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
        position: sticky; /* no idea what is setting this to `static` */
    }

	/* Device-specific map layout */
	.computer {
        height: 800px;
    }
    .tablet {
        flex-grow: 1;
        height: 100vh;
        width: 100vw;
    }
    .mobile {
        flex-grow: 1;
        height: 100vh;
        width: 100vw;
    }
</style>