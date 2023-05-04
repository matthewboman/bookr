<script lang="ts">
	import { ApolloClient, InMemoryCache, gql } from "@apollo/client/core/index.js"
	import { setClient, query } from "svelte-apollo"

	import About            from './components/About.svelte'
	import FilterContainer  from './components/FilterContainer.svelte'
    import LeafletContainer from './components/MapContainer.svelte'
    import type { Contact } from './types'

	// GQL
    // unfortunately this can't be extracted to a service w/o overhead
    // https://github.com/timhall/svelte-apollo/issues/99
    const GQL_URL = "http://127.0.0.1:8000/graphql"
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

	let filteredContacts: Contact[] = []
    $: renderedContacts = filteredContacts
    $: contactList      = $contacts.data ? $contacts.data.contacts : []
</script>

{#if $contacts.loading}
    loading...
{:else if $contacts.error}
    error...
{:else}
    <LeafletContainer renderedContacts={renderedContacts} />
    <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList}/>
    <About/>
{/if}
