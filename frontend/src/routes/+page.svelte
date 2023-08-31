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
        
        contactList = await get(CONTACTS_URL).then(r => r.json())
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
            <div class="filter-container p-4 border-primary-700">
                <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList} />
            </div>
        {/if}
    </MediaQuery>
    
    <!-- Mobile -->
    <MediaQuery query="(max-width: 480px)" let:matches>
        {#if matches}
            <svelte:component this={LeafletContainer} {renderedContacts}/> 
            <div class="filter-container p-4 border-primary-700">
                <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList} />
            </div>
        {/if}
    </MediaQuery>

{:else}
    loading
{/if}

<style>
    @font-face { font-family: 'Comfortaa'; src: local("Comfortaa"), url('Comfortaa-VariableFont_wght.ttf')  format('truetype');}

    * {
        font-family: Comfortaa;
    }

    .main-container {
        display: flex;
        width: 100vw;
    }

    .desktop-filter {
        
    }

    .filter-container {
        backdrop-filter: blur(2px);
        border-style: solid;
        border-top-width: 4px;
        position: absolute;
        bottom: 0px;
        z-index: 10;
        background-color: rgba(31, 41, 55,0.9);
        width: 100vw;
    }

    .desktop-map {
        min-width: 65vw;
        flex-grow: 1;
    }
</style>