<script lang="ts">
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'

    import MediaQuery          from '../components/MediaQuery.svelte'
	import FilterContainer     from '../components/FilterContainer.svelte'
    import Menu                from '../components/Menu.svelte'
    import { get }             from '../api'
    import { isAuthenticated } from '../functions'
    import type { Contact }    from '../types'
    import { authenticated }   from "../store"

    const CONTACTS_URL = "/contacts"

	let LeafletContainer: any
    let contactList: any
	let filteredContacts: Contact[] = []
    $: renderedContacts = filteredContacts

	// Workaround bc `browser` isn't defined w/ SSR
	async function getMap() {
		if (browser) {
			LeafletContainer = (await import('../components/MapContainer.svelte')).default
		}
	}

    onMount(async () => {
        authenticated.update(() => isAuthenticated())
		getMap()

        //
        //
        //
        // FOR TESTING PURPOSES
        contactList = {}
        //
        // contactList = await get(CONTACTS_URL).then(r => r.json())
        //
        //
    })
</script>

{#if contactList}
    <Menu/>
    <!-- Desktop -->
    <MediaQuery query="(min-width: 1281px)" let:matches>
        {#if matches}
            <div class="main-container mx-auto">
                <div class="desktop-filter px-4 py-4">
                    <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList} />
                </div>
                <div class="desktop-map">
                    <svelte:component this={LeafletContainer} {renderedContacts}/> 
                </div>
            </div>
        {/if}
    </MediaQuery>

    <!-- Tablet --> 
    <MediaQuery query="(min-width: 481px) and (max-width: 1280px)" let:matches>
        {#if matches}
            <svelte:component this={LeafletContainer} {renderedContacts}/> 
            <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList} />
        {/if}
    </MediaQuery>
    
    <!-- Mobile -->
    <MediaQuery query="(max-width: 480px)" let:matches>
        {#if matches}
            <svelte:component this={LeafletContainer} {renderedContacts}/> 
            <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList} />
        {/if}
    </MediaQuery>

{:else}
    loading
{/if}

<style>
    .main-container {
        display: flex;
        width: 90vw;
    }

    .desktop-filter {
        
    }

    .desktop-map {
        min-width: 65vw;
    }
</style>