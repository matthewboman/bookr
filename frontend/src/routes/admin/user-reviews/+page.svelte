<script lang="ts">
    import { onMount } from 'svelte'

    import AdminDisplayReview from '../../../components/review/AdminDisplayReview.svelte'
    import EditReview         from '../../../components/review/EditReview.svelte'
    import { get }            from "../../../api"
    import type { Review }    from '../../../types'

    let reviews: Review[] = []
    let allowEditReview   = false

    async function getReviewsByUser() {
        const params = new URLSearchParams(window.location.search)
        const userId = params.get("user-id")
        
        reviews = await get(`/admin/user-reviews?userId=${userId}`)
            .then(r => r.json())
    }

    onMount(async() => {
        getReviewsByUser()
    })
</script>

{#if reviews.length}
    Reviews by {reviews[0].email}
{/if}
{#if allowEditReview}
    <EditReview on:disallowEditReview={() => allowEditReview = false}/>
{/if}
{#each reviews as review}
    <AdminDisplayReview review={review} on:allowEdit={() => allowEditReview = true}/>
{/each}
