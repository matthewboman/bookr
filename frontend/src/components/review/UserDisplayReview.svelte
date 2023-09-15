<script lang="ts">
    import { createEventDispatcher } from "svelte"
    import { Card, GradientButton } from "flowbite-svelte"

    import { post }           from "../../api"
    import { handleResponse } from '../../functions'
    import { 
        admin, 
        authenticated, 
        contactReviews, 
        selectedReview,
        userId
    } from '../../store'
    import type { Review } from '../../types'
    import StarRating      from "../StarRating.svelte"

    export let review: Review

    const dispatch = createEventDispatcher()

    let errorMessage: string | null

    async function deleteReview(review: Review) {
        const url  = $admin ? "/admin/delete-review" : "/user/delete-review"
        const data = {
            userId:   review.userId,
            reviewId: review.reviewId,
        }

        let response = await post(url, review)
        errorMessage = handleResponse(
            response,
            "There was an error deleting the review. Please try again.",
            logout
        )

        if (response.status === 200) {
            dispatch('allowEdit')
            $contactReviews = $contactReviews.filter((r: Review) => r.reviewId !== review.reviewId)
        }
    }

    function editReview(review: Review) {
        dispatch('allowEdit')
        $selectedReview = {
            title:     review.title,
            body:      review.body,
            rating:    review.rating,
            userId:    review.userId,
            reviewId:  review.reviewId,
            contactId: review.contactId
        }
        $contactReviews = $contactReviews.filter((r: Review) => r.reviewId !== review.reviewId)
    }

    function logout() {
        authenticated.update(() => false)
    }
</script>

<Card class="text-center m-4" size="xl" padding="lg">
    <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white flex justify-between">
        { review.title }
        <StarRating currentRating={review.rating} active={false} color={'yellow'}></StarRating>
    </h5>
    <p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
        { review.body }
    </p>
    <div>
        {#if $admin || review.userId === $userId}
            <GradientButton type="submit" class="w-full1" on:click={() => deleteReview(review)}>Delete review</GradientButton>
        {/if}
        {#if $admin || review.userId === $userId}
            <GradientButton type="submit" class="w-full1" on:click={() => editReview(review)}>Edit review</GradientButton>
        {/if}
        {#if $admin && review.email}
            <a href={`/admin/reviews?user-id=${review.userId}`}>{review.email}</a>
        {/if}
    </div>
</Card>