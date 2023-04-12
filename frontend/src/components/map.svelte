<script>
    import { Icon, LeafletMap, Marker, Popup, TileLayer } from 'svelte-leafletjs'

    import { locations } from "../store.js"

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
    <LeafletMap bind:this={leafletMap} options={mapOptions}>
        <TileLayer url={tileUrl} options={tileLayerOptions}/>
        {#each locations as location}
            <Marker latLng={location.latLng}>
                <Popup>{location.text}</Popup>
            </Marker>
        {/each}
    </LeafletMap>
</div>