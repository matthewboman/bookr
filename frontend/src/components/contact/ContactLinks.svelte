<script lang="ts">
    import { isAuthenticated } from "../../functions"

    export let className: string
    export let email:     string | null
    export let link:      string | null
</script>

<div class={className}>
    {#if link}
        <a target="_blank" href="{link}" class="text-orange-400 leaflet-link-override">
            { link }
        </a>
    {/if}
    {#if email && isAuthenticated()}
        <div>{ email }</div>
    {/if}
</div>

<style>
    .leaflet-link-override {
        color: rgb(255 138 76);
    }
</style>