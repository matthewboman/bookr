<script lang="ts">
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'
	import { Modal}    from 'flowbite-svelte'

	import About             from '../components/About.svelte'
    import AuthModal         from "../components/AuthModal.svelte"
	import FilterContainer   from '../components/FilterContainer.svelte'
    // import LeafletContainer  from '../components/MapContainer.svelte'
    import Menu              from '../components/Menu.svelte'
    import ContactModal      from '../components/ContactModal.svelte'
    import type { Contact }  from '../types'
    import { authenticated } from "../store"

	const API_URL  = "http://127.0.0.1:8000"
    const CONTACTS = `${API_URL}/contacts`

    let authModal     = false
    let contactModal  = false

	let LeafletContainer: any
    let contactList: any
	let filteredContacts: Contact[] = []
    $: renderedContacts = filteredContacts

    // TODO: send message from modal w which to close
    // TODO: refactor to single function
    function closeAuthModal() {
        authModal = false
    }
    function closeContactModal() {
        contactModal = false
    }

    function setAuthenticated() {
        let token = sessionStorage.getItem('byotoken')

        if (token != null) {
            authenticated.update(() => true)
        }
    }

	// Workaround bc `browser` isn't defined w/ SSR
	async function getMap() {
		if (browser) {
			LeafletContainer = (await import('../components/MapContainer.svelte')).default
		}
	}

    onMount(async () => {
        setAuthenticated()
		getMap()

        contactList = await fetch(CONTACTS).then(r => r.json()) // TODO: move method to API, use token
    })
</script>

{#if contactList}
    <Menu bind:authModal={authModal} bind:contactModal={contactModal} />
    <!-- <LeafletContainer renderedContacts={renderedContacts} /> -->
	<svelte:component this={LeafletContainer} {renderedContacts}/>
    <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList} />
    <About/>

    <Modal bind:open={authModal} size="xs" outsideclose class="w-full above">
        <AuthModal on:close={closeAuthModal}/>
    </Modal>

    <Modal bind:open={contactModal} size="xs" outsideclose class="w-full above">
        <ContactModal on:close={closeContactModal} />
    </Modal>
{:else}
    loading
{/if}

<style>
    /* Leaflet map has a z-index of 400, so set modal over it */
    .above {
        z-index: 500;
    }
</style>