<script>
// @ts-nocheck

    import { LeafletMap, Marker, Popup, TileLayer } from 'svelte-leafletjs'
    import { ApolloClient, InMemoryCache, gql } from "@apollo/client/core"
	import { setClient, query } from "svelte-apollo"

    import ContactPopup from './ContactPopup.svelte'

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
                contactForm
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

    // Filters
    let minCapacity = 0
    let maxCapacity = 1000
    let allowNull   = true

    $: filteredContacts = $contacts.data ? $contacts.data.contacts : []

    function update() {
        filteredContacts = $contacts.data.contacts.filter(c => {
            if (allowNull && c.capacity == null) return true
            if (!allowNull && c.capacity == null) return false

            if (c.capacity >= minCapacity && c.capacity <= maxCapacity) return true
           
            return false
        })
    }
</script>

<style>
    .filter-container {

    }

    .capacity-filter {

    }

    .filter-title {
        margin-bottom: 10px;
    }

    .filter-input {

    }
</style>

{#if $contacts.loading}
    loading...
{:else if $contacts.error}
    error...
{:else}
    <LeafletMap bind:this={leafletMap} options={mapOptions}>
        <TileLayer url={tileUrl} options={tileLayerOptions}/>
        {#each filteredContacts as contact}
            <Marker latLng={[contact.latitude, contact.longitude]}>
                <Popup>
                    <ContactPopup contact={contact}/>
                </Popup>
            </Marker>
        {/each}
    </LeafletMap>
{/if}

<div class="filter-container">
    <div class="capacity-filter">
        <div class="filter-title">
            Filter venues by capacity
        </div>
        <div class="filter-input">
            <label for="min">min</label>
            <input id="min" type="number" bind:value={minCapacity} on:change={update}/>
        </div>
        <div class="filter-input">
            <label for="max">max:</label>
            <input id="max" type="numer" bind:value={maxCapacity} on:change={update}/>       
        </div>
        <div class="filter-input">
            <input id="null-capacity" type="checkbox" bind:checked={allowNull} on:change={update}/>          
            <label for="null-capacity">
                allow for venues with unknown capacity
            </label>
        </div>
    </div>
</div>