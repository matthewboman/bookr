<script lang="ts">
    import { onMount } from 'svelte'

    import { isAdmin, isAuthenticated } from '../functions'
    import { 
        admin, 
        contactReviews, 
        selectedContact,
        userId
    } from '../store'
    import type { Review }   from '../types'
    import UserDisplayReview from './review/UserDisplayReview.svelte'
    import EditReview        from './review/EditReview.svelte'

    export let reviews: Review[]

    let allowAddReview    = false
    let allowDeleteReview = false

    onMount(() => {
        let hasReviewed = $contactReviews
                .map((r: Review) => r.userId)
                .includes($userId)

        if (isAdmin() && !hasReviewed) {
            admin.update(() => true)
            allowDeleteReview = true
            allowAddReview    = true
        } else {
            admin.update(() => false)
        }

        if (isAuthenticated() && !hasReviewed) {
            allowAddReview = true
        }
        allowAddReview = false
    })
</script>

<div class="flex">
    {#if allowAddReview && $selectedContact}
        <EditReview on:disallowEditReview={() => allowAddReview = false}/>
    {/if}
    {#each reviews as review}
        <UserDisplayReview review={review} on:allowEdit={() => allowAddReview = true}/>
    {/each}
</div>