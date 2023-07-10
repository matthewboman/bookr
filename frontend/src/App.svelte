<script lang="ts">
    import { onMount } from "svelte"
    import { Modal}    from 'flowbite-svelte'

	import About            from './components/About.svelte'
    import AuthModal        from "./components/AuthModal.svelte"
	import FilterContainer  from './components/FilterContainer.svelte'
    import LeafletContainer from './components/MapContainer.svelte'
    import Menu             from './components/Menu.svelte'
    import ContactModal     from './components/ContactModal.svelte'
    import type { Contact } from './types'

    const API_URL  = "http://127.0.0.1:8000"
    const CONTACTS = `${API_URL}/contacts`

    let authModal    = false
    let contactModal = false

    let contactList
	let filteredContacts: Contact[] = []
    $: renderedContacts = filteredContacts

    onMount(async () => {
        contactList = await fetch(CONTACTS).then(r => r.json())
    })
</script>

{#if contactList}
    <Menu bind:authModal={authModal} bind:contactModal={contactModal} />
    <!-- TODO: why is the container a higher z value?-->
    <!-- <LeafletContainer renderedContacts={renderedContacts} /> -->
    <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList} />
    <About/>

    <Modal bind:open={authModal} size="xs" autoclose={true} outsideclose class="w-full">
        <AuthModal apiUrl={API_URL} />
    </Modal>

    <Modal bind:open={contactModal} size="xs" autoclose={true} outsideclose class="w-full">
        <ContactModal apiUrl={API_URL} />
    </Modal>
{:else}
    loading
{/if}
  
