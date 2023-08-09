<script lang="ts">
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'

	import About             from '../components/About.svelte'
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
	<svelte:component this={LeafletContainer} {renderedContacts}/> 
    <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList} />
{:else}
    loading
{/if}
<About/>