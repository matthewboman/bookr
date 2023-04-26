<script>
	// @ts-nocheck
	import { browser } from '$app/environment'
	import { onMount } from 'svelte'
	import { ApolloClient, InMemoryCache, gql } from "@apollo/client/core"
	import { setClient, query } from "svelte-apollo"

	import About           from '../components/About.svelte'
	import FilterContainer from '../components/FilterContainer.svelte'

	let LeafletContainer

	onMount(async () => {
		if (browser) {
			LeafletContainer = (await import('../components/MapContainer.svelte')).default
		}
	})

	// GQL
    // unfortunately this can't be extracted to a service w/o overhead
    // https://github.com/timhall/svelte-apollo/issues/99
    const GQL_URL = "http://127.0.0.1:8000/"
    const client  = new ApolloClient({
        uri:   GQL_URL,
		cache: new InMemoryCache()
	})

	setClient(client)

    const CONTACTS = gql`
        query Contacts {
            contacts {
                contactId,
                displayName,
                address,
                city,
                state,
                zipCode,
                capacity,
                latitude,
                longitude,
                email,
                contactForm,
                ageRange
            }
        }
    `

    const contacts = query(CONTACTS)

	let filteredContacts = []
    $: renderedContacts  = filteredContacts
    $: contactList       = $contacts.data ? $contacts.data.contacts : []
</script>

{#if browser}
	{#if $contacts.loading}
		loading...
	{:else if $contacts.error}
		error...
	{:else}
		<svelte:component this={LeafletContainer} {renderedContacts}/>
		<FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList}/>
		<About/>
	{/if}
{/if}