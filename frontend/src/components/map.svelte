<script>
    import { LeafletMap, Marker, Popup, TileLayer } from 'svelte-leafletjs'
    import { ApolloClient, InMemoryCache, gql } from "@apollo/client/core"
	import { setClient, query } from "svelte-apollo"

    // import { locations } from "../store.js"

    const GQL_URL = "http://127.0.0.1:8000/"

    const client = new ApolloClient({
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
                capacity,
                latitude,
                longitude,
                email,
                contactForm
            }
        }
    `

    const contacts = query(CONTACTS)

    function reload() {
        contacts.refetch()
    }

    $: contacts.refetch()

    const mapOptions = {
        center: [37.09, -90.71],
        zoom: 3.5,
    }
    const tileUrl = "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
    const tileLayerOptions = {
        minZoom: 0,
        maxZoom: 20,
        maxNativeZoom: 19,
        attribution: "© OpenStreetMap contributors",
    }


    let leafletMap
</script>

<style>
    .map-container {
        height: 500px;
        width: 800px;
    }
</style>

<div class="map-container">
    {#if $contacts.loading}
    loading
    {:else if $contacts.error}
    error
    {:else}
    <LeafletMap bind:this={leafletMap} options={mapOptions}>
        <TileLayer url={tileUrl} options={tileLayerOptions}/>
        {#each $contacts.data.contacts as contact}
            <Marker latLng={[contact.latitude, contact.longitude]}>
                <Popup>{contact.displayName}
                
                </Popup>
            </Marker>
        {/each}
    </LeafletMap>
    {/if}
</div>