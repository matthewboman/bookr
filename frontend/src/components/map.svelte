<script>
    import { LeafletMap, Marker, Popup, TileLayer } from 'svelte-leafletjs'
    import { ApolloClient, InMemoryCache, gql } from "@apollo/client/core"
	import { setClient, query } from "svelte-apollo"

    import ContactPopup from './ContactPopup.svelte'
    import MediaQuery from './MediaQuery.svelte'

    // GQL
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
                contactForm
            }
        }
    `
    const contacts = query(CONTACTS)

    function reload() {
        contacts.refetch()
    }

    $: contacts.refetch()

    // Map
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
        text-align: center;
    }
    .map {
        display: inline-block;
    }

    .computer {
        width: 90vw;
        height: 800px;
    }

    .tablet {
        width: 90vw;
        height: 600px;
    }

    .mobile {
        width: 90vw;
        height: 600px;
    }
</style>

<div class="map-container">
{#if $contacts.loading}
loading...
{:else if $contacts.error}
error...
{:else}
    <MediaQuery query="(min-width: 1281px)" let:matches>
        {#if matches}
            <div class="map computer">    
                <LeafletMap bind:this={leafletMap} options={mapOptions}>
                    <TileLayer url={tileUrl} options={tileLayerOptions}/>
                    {#each $contacts.data.contacts as contact}
                        <Marker latLng={[contact.latitude, contact.longitude]}>
                            <Popup>
                                <ContactPopup contact={contact}/>
                            </Popup>
                        </Marker>
                    {/each}
                </LeafletMap>
            </div>
        {/if}
    </MediaQuery>
    
    <MediaQuery query="(min-width: 481px) and (max-width: 1280px)" let:matches>
        {#if matches}
        <div class="map tablet">    
            <LeafletMap bind:this={leafletMap} options={mapOptions}>
                <TileLayer url={tileUrl} options={tileLayerOptions}/>
                {#each $contacts.data.contacts as contact}
                    <Marker latLng={[contact.latitude, contact.longitude]}>
                        <Popup>
                            <ContactPopup contact={contact}/>
                        </Popup>
                    </Marker>
                {/each}
            </LeafletMap>
        </div>
        {/if}
    </MediaQuery>
    
    <MediaQuery query="(max-width: 480px)" let:matches>
        {#if matches}
        <div class="map mobile">    
            <LeafletMap bind:this={leafletMap} options={mapOptions}>
                <TileLayer url={tileUrl} options={tileLayerOptions}/>
                {#each $contacts.data.contacts as contact}
                    <Marker latLng={[contact.latitude, contact.longitude]}>
                        <Popup>
                            <ContactPopup contact={contact}/>
                        </Popup>
                    </Marker>
                {/each}
            </LeafletMap>
        </div>
        {/if}
    </MediaQuery>
    {/if}
</div>
