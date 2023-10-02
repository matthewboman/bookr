<script lang="ts">
    import { onMount } from 'svelte'

    import AdminDisplayReview from '../../../components/review/AdminDisplayReview.svelte'
    import EditReview         from '../../../components/review/EditReview.svelte'
    import { get }            from '../../../api'
    import type { Review }    from '../../../types'

    const RECENT_REVIEWS_URL = "/admin/recent-reviews"

    let recentReviews: Review[] = []
    let allowEditReview = false

    async function getRecentReviews() {
        recentReviews = await get(RECENT_REVIEWS_URL)
            .then(r => r.json())
    }

    onMount(async() => {
        getRecentReviews()
    })

</script>

<div>
    <h2>
        Recent reviews
    </h2>
    {#if allowEditReview}
        <EditReview on:disallowEditReview={() => allowEditReview = false}/>
    {/if}
    {#each recentReviews as review}
        <AdminDisplayReview review={review} on:allowEdit={() => allowEditReview = true}/>
    {/each}
</div>
