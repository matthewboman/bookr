<script lang="ts">
    import { Card } from "flowbite-svelte"

    import type { Review } from '../../types'
    import StarRating      from "../ui/StarRating.svelte"

    export let review: Review
    export let displayHeader = false
</script>

<Card class="text-center m-4" size="xl" padding="lg">
    {#if displayHeader}
        <h2>
            Review of { review.contactName }
        </h2>
    {/if}
    <h5 class="mb-2 text-2xl tracking-tight text-gray-900 dark:text-white flex justify-between">
        { review.title }
        <StarRating currentRating={review.rating} active={false} color={'yellow'}></StarRating>
    </h5>
    <p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
        { review.body }
    </p>
    <slot name="actions" />
</Card>