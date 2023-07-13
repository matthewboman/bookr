<script lang="ts">
    import { onMount } from "svelte"
    import { Modal}    from 'flowbite-svelte'

	import About             from './components/About.svelte'
    import AuthModal         from "./components/AuthModal.svelte"
	import FilterContainer   from './components/FilterContainer.svelte'
    import LeafletContainer  from './components/MapContainer.svelte'
    import Menu              from './components/Menu.svelte'
    import ContactModal      from './components/ContactModal.svelte'
    import type { Contact }  from './types'
    import { authenticated } from "./store"

    const API_URL  = "http://127.0.0.1:8000"
    const CONTACTS = `${API_URL}/contacts`

    let authModal     = false
    let contactModal  = false

    let contactList
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
            console.log('updating')
            authenticated.update(() => true)
        }
    }

    onMount(async () => {
        setAuthenticated()

        contactList = await fetch(CONTACTS).then(r => r.json()) // TODO: move method to API, use token
    })
</script>

{#if contactList}
    <Menu bind:authModal={authModal} bind:contactModal={contactModal} />
    <!-- TODO: why is the container a higher z value?-->
    <!-- <LeafletContainer renderedContacts={renderedContacts} /> -->
    <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList} />
    <About/>

    <Modal bind:open={authModal} size="xs" outsideclose class="w-full">
        <AuthModal on:close={closeAuthModal}/>
    </Modal>

    <Modal bind:open={contactModal} size="xs" outsideclose class="w-full">
        <ContactModal on:close={closeContactModal} />
    </Modal>
{:else}
    loading
{/if}
  
