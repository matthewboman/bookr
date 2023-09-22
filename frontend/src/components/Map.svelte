<script lang="ts">
    import { LeafletMap, Marker, Popup, TileLayer } from 'svelte-leafletjs'

    import ContactPopup       from './ContactPopup.svelte'
    import { get }            from '../api'
    import type { Contact, MapOptions }        from '../types'
    import { contactReviews, selectedContact } from '../store'

    export let renderedContacts: Contact[]
    export let mapOptions: MapOptions

    const REVIEWS_URL = "/reviews?contactId="
    const tileUrl = "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
    const tileLayerOptions = {
        minZoom: 0,
        maxZoom: 20,
        maxNativeZoom: 19,
        attribution: "© OpenStreetMap contributors",
    }

    let leafletMap

    async function showReviews(contact: Contact) {
        let res = await get(`${REVIEWS_URL}${contact.contactId}`)
            .then(r => r.json())

        $contactReviews  = res.reviews // intentionally overwrite
        $selectedContact = contact // intentionally overwrite
    }
</script>

<LeafletMap bind:this={leafletMap} options={mapOptions}>
    <TileLayer url={tileUrl} options={tileLayerOptions}/>
    {#each renderedContacts as contact, index (index)}
        <Marker latLng={[contact.latitude, contact.longitude]} events={['click']} on:click={() => showReviews(contact)}>
            <Popup events={['popupclose']}>
                <ContactPopup contact={contact} />
            </Popup>
        </Marker>
    {/each}
</LeafletMap>

