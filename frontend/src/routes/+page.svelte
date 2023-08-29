<script lang="ts">
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'

    import MediaQuery          from '../components/MediaQuery.svelte'
	import FilterContainer     from '../components/FilterContainer.svelte'
    import Menu                from '../components/Menu.svelte'
    import Reviews             from '../components/Reviews.svelte'
    import { get }             from '../api'
    import { getUserId, isAuthenticated } from '../functions'
    import type { Contact }    from '../types'
    import { authenticated, userId, contactReviews }   from "../store"

    const CONTACTS_URL     = "/contacts"
    const PRIVATE_CONTACTS = "/user/private-contacts"

	let LeafletContainer: any
    let contactList: Contact[] = []
	let filteredContacts: Contact[] = []
    $: renderedContacts = filteredContacts

	// Workaround bc `browser` isn't defined w/ SSR
	async function getMap() {
		if (browser) {
			LeafletContainer = (await import('../components/MapContainer.svelte')).default
		}
	}

    async function getPrivateContacts() {
        let contacts = await get(PRIVATE_CONTACTS).then(r => r.json())
        contactList = [...contactList, ...contacts]
    }

    async function getPublicContacts() {
        let contacts = await get(CONTACTS_URL).then(r => r.json())
        contactList = [...contactList, ...contacts]
    }

    onMount(async () => {
        authenticated.update(() => isAuthenticated())
		getMap()

        await getPublicContacts()

        if (isAuthenticated()) {
            userId.update(() => getUserId())
            await getPrivateContacts()        
        }
    })
</script>

{#if contactList.length}
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

    {#if $contactReviews.length}
        <Reviews/>
    {/if}

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