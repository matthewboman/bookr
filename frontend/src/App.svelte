<script lang="ts">
    import { onMount }      from "svelte"
	import About            from './components/About.svelte'
	import FilterContainer  from './components/FilterContainer.svelte'
    import LeafletContainer from './components/MapContainer.svelte'
    import type { Contact } from './types'

    const API_URL = "http://127.0.0.1:8000/contacts"

    let contactList
	let filteredContacts: Contact[] = []
    $: renderedContacts = filteredContacts

    onMount(async () => {
        contactList = await fetch(API_URL).then(r => r.json())
    })
</script>

{#if contactList}
    <LeafletContainer renderedContacts={renderedContacts} />
    <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList}/>
    <About/>
{:else}
    loading
{/if}
