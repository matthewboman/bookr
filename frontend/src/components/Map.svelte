<script lang="ts">
    import { LeafletMap, Marker, Popup, TileLayer } from 'svelte-leafletjs'
    import type { LatLngExpression } from "leaflet"

    import ContactPopup     from './ContactPopup.svelte'
    import type { Contact } from '../types'

    export let renderedContacts: Contact[]

    const mapOptions = {
        center: [ 37.09, -90.71 ] as LatLngExpression,
        zoom:   4,
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

<LeafletMap bind:this={leafletMap} options={mapOptions}>
    <TileLayer url={tileUrl} options={tileLayerOptions}/>
    {#each renderedContacts as contact}
        <Marker latLng={[contact.latitude, contact.longitude]}>
            <Popup>
                <ContactPopup contact={contact}/>
            </Popup>
        </Marker>
    {/each}
</LeafletMap>

