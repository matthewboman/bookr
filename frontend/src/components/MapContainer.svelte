<script lang="ts">
    import Map        from './Map.svelte'
    import MediaQuery from './MediaQuery.svelte'
    import type { Contact } from '../types'

    export let renderedContacts: Contact[]= []
</script>

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
        /* width: 90%; */
        height: 800px;
    }
    .tablet {
        width: 85vw;
        height: 600px;
    }
    .mobile {
        width: 85vw;
        height: 600px;
    }
</style>

<MediaQuery query="(min-width: 1281px)" let:matches>
    {#if matches}
        <div class="map computer"> 
            <Map renderedContacts={renderedContacts}/>   
        </div>
    {/if}
</MediaQuery>

<MediaQuery query="(min-width: 481px) and (max-width: 1280px)" let:matches>
    {#if matches}
        <div class="map tablet">    
            <Map renderedContacts={renderedContacts}/>
        </div>
    {/if}
</MediaQuery>

<MediaQuery query="(max-width: 480px)" let:matches>
    {#if matches}
        <div class="map mobile">    
            <Map renderedContacts={renderedContacts}/>
        </div>
    {/if}
</MediaQuery>