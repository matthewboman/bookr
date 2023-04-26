<script>
    // @ts-nocheck
	import { onMount } from 'svelte'
	import { browser } from '$app/environment'


    import { LeafletMap, Marker, Popup, TileLayer } from 'svelte-leafletjs'
    import { ApolloClient, InMemoryCache, gql } from "@apollo/client/core"
	import { setClient, query } from "svelte-apollo"

    import ContactPopup    from './ContactPopup.svelte'
    import FilterContainer from './FilterContainer.svelte';
	import About           from './About.svelte'

    let Map

	onMount(async () => {
		if (browser) {
			Map = (await import('./Map.svelte')).default
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

    // Map
    const mapOptions = {
        center: [ 37.09, -90.71 ],
        zoom:   3.5,
    }
    const tileUrl = "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
    const tileLayerOptions = {
        minZoom: 0,
        maxZoom: 20,
        maxNativeZoom: 19,
        attribution: "© OpenStreetMap contributors",
    }

    let leafletMap

    // Filtering
    let filteredContacts = []
    $: renderedContacts  = filteredContacts
    $: contactList       = $contacts.data ? $contacts.data.contacts : []
</script>

{#if $contacts.loading}
    loading...
{:else if $contacts.error}
    error...
{:else}
    <!-- <LeafletMap bind:this={leafletMap} options={mapOptions}>
        <TileLayer url={tileUrl} options={tileLayerOptions}/>
        {#each renderedContacts as contact}
            <Marker latLng={[contact.latitude, contact.longitude]}>
                <Popup>
                    <ContactPopup contact={contact}/>
                </Popup>
            </Marker>
        {/each}
    </LeafletMap> -->
    {#if browser}
    <Map renderedContacts={filteredContacts} />
    {/if}
    <FilterContainer bind:filteredContacts={filteredContacts} contactList={contactList}/>
{/if}

<About />