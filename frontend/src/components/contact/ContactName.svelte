<script lang="ts">
    import StarRating from "../StarRating.svelte"

    export let className: string
    export let name:      string
    export let rating:    number
    export let starColor: string
    export let starSize:  number
</script>

<div class={className}>
    <h2 class="mr-2 text-xl">{ name }</h2>
    {#if rating}
        <StarRating 
            currentRating={rating} 
            active={false} 
            color={starColor}
            size={starSize}
        />
    {/if}
</div>