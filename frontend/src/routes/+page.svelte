<script lang="ts">
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'

    import FilterContainer  from '../components/FilterContainer.svelte'
    import MediaQuery       from '../components/ui/MediaQuery.svelte'
    import SelectedContact  from '../components/contact/SelectedContact.svelte'
    import Toggle           from '../components/ui/Toggle.svelte'
    import { get }          from '../api'
    import { getUserId, isAuthenticated } from '../functions'
    import type { Contact } from '../types'
    import { 
        authenticated,
        contactList,
        genres,
        mapOptions,
        selectedContact,
        userId
    } from "../store"

    const CONTACTS_URL     = "/contacts"
    const GENRES_URL       = "/genres"
    const PRIVATE_CONTACTS = "/user/private-contacts"

	let LeafletMap: any
	let filteredContacts: Contact[] = []
    $: renderedContacts = filteredContacts

    async function getGenres() {
        $genres = await get(GENRES_URL).then(r => r.json())
    }

	// Workaround bc `browser` isn't defined w/ SSR
	async function getMap() {
		if (browser) {
			LeafletMap = (await import('../components/map/Map.svelte')).default
		}
	}

    async function getPrivateContacts() {
        let contacts = await get(PRIVATE_CONTACTS).then(r => r.json())
        $contactList = [...$contactList, ...contacts]
    }

    async function getPublicContacts() {
        let contacts = await get(CONTACTS_URL).then(r => r.json())
        $contactList = [...$contactList, ...contacts]
    }

    onMount(async () => {
        authenticated.update(() => isAuthenticated())
		getMap()
        getGenres()

        await getPublicContacts()

        if (isAuthenticated()) {
            userId.update(() => getUserId())
            await getPrivateContacts()        
        }
    })
</script>

{#if $contactList.length}
    <!-- Desktop -->
    <MediaQuery query="(min-width: 1281px)" let:matches>
        {#if matches}
            <div class="main-container mx-auto">
                <div class="desktop-filter px-4 py-4">
                    <FilterContainer bind:filteredContacts={filteredContacts} />
                </div>
                <div class="map desktop">
                    <svelte:component this={LeafletMap} {renderedContacts} mapOptions={$mapOptions}/>
                </div>
            </div>
            {#if $selectedContact}
                <SelectedContact/>
            {/if}
        {/if}
    </MediaQuery>

    <!-- Tablet && Mobile--> 
    <MediaQuery query="(max-width: 1280px)" let:matches>
        {#if matches}
            <div class="map tablet-mobile">
                <svelte:component this={LeafletMap} {renderedContacts} mapOptions={$mapOptions}/>
            </div>
            <Toggle >
                <FilterContainer bind:filteredContacts={filteredContacts} />
            </Toggle>
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
    .map {
        display: block;
        margin-left: auto;
        margin-right: auto;
        z-index: 5;
        position: sticky; /* Override: no idea what is setting this to `static` */
    }
    .desktop {
        height: 800px;
        min-width: 65vw;
        flex-grow: 1;
    }
    .tablet-mobile {
        flex-grow: 1;
        height: 100vh;
        width: 100vw;
    }
</style>