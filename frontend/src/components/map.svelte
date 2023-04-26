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

    // Filters
    let minCapacity         = 0
    let maxCapacity         = 1000
    let allowNullCapacity   = true
    let allAges             = true
    let eighteenPlus        = true
    let twentyonePlus       = true
    let allowNullAgeRange   = true

    $: filteredContacts = $contacts.data ? $contacts.data.contacts : []

    const capacityFilter = contact => {
        if (allowNullCapacity && contact.capacity == null) return true
        if (!allowNullCapacity && contact.capacity == null) return false

        if (contact.capacity >= minCapacity && contact.capacity <= maxCapacity) return true
        
        return false
    }

    const ageRangeFilter = contact => {
        if (allAges && contact.ageRange == 'all') return true
        if (eighteenPlus && contact.ageRange == '18+') return true
        if (twentyonePlus && contact.ageRange == '21+') return true

        if (allowNullAgeRange && contact.ageRange == null) return true

        return false
    }

    function update() {
        filteredContacts = $contacts.data.contacts
            .filter(c => capacityFilter(c))
            .filter(c => ageRangeFilter(c))
    }
</script>

<style>

    .filter-container {
        display: flex;
        flex-direction: row;
		font-family: sans-serif;
    }

    .filter {
        padding: 10px;
        flex-grow: 1;
    }

    .filter-title {
        margin-bottom: 10px;
    }

    .filter-input {
        margin-bottom: 0.5em;
    }

    /* Checkboxes */
    .checkbox-container {
        line-height: 1.1;
        display: grid;
        grid-template-columns: 1em auto;
        gap: 0.5em;
    }
    .checkbox-container:hover {
        cursor: pointer;
    }
    input[type="checkbox"] {
        -webkit-appearance: none;
        appearance: none;
        background-color: #fff;
        margin: 0;
        font: inherit;
        color: currentColor;
        width: 1.15em;
        height: 1.15em;
        border: 0.1em solid currentColor;
        border-radius: 0.15em;
        transform: translateY(-0.075em);
        display: grid;
        place-content: center;
    }
    input[type="checkbox"]::before {
        content: "";
        width: 0.65em;
        height: 0.65em;
        transform: scale(0);
        transition: 120ms transform ease-in-out;
        box-shadow: inset 1em 1em rgb(49, 46, 232);
        background-color: CanvasText;
    }
    input[type="checkbox"]:checked::before {
        transform: scale(1);
    }
    input:focus {
        outline: max(2px, 0.15em) solid grey;
        outline-offset: max(2px, 0.15em);
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
    <div class="filter">
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
            <label for="null-capacity" class="checkbox-container">
                <input id="null-capacity" type="checkbox" bind:checked={allowNullCapacity} on:change={update}/>                     
                allow for venues with unknown capacity
            </label>
        </div>
    </div>
    <div class="filter">
        <div class="filter-title">
            Filter venues by age range
        </div>
        <div class="filter-input">
            <label class="checkbox-container">
                <input type=checkbox class="checkbox" bind:checked={allAges} on:change={update}>
                All Ages
            </label>
        </div>
        <div class="filter-input">
            <label class="checkbox-container">
                <input type=checkbox class="checkbox" bind:checked={eighteenPlus} on:change={update}>
                18+
            </label>
        </div>
        <div class="filter-input">
            <label class="checkbox-container">
                <input type=checkbox class="checkbox" bind:checked={twentyonePlus} on:change={update}>
                21+
            </label>
        </div>
        <div class="filter-input">
            <label class="checkbox-container">
                <input type=checkbox class="checkbox" bind:checked={allowNullAgeRange} on:change={update}>
                allow for venues with unknown age range
            </label>
        </div>
    </div>
</div>



