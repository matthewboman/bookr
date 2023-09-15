<script lang="ts">
    import { createEventDispatcher } from "svelte"
    import { Card, GradientButton }  from "flowbite-svelte"

    import { post }           from "../../api"
    import { handleResponse } from '../../functions'
    import {
        authenticated, 
        adminReviews, 
        selectedReview
    } from '../../store'
    import type { Review } from '../../types'
    import StarRating      from "../StarRating.svelte"

    export let review: Review

    const dispatch   = createEventDispatcher()
    const DELETE_URL = "/admin/delete-review"

    let errorMessage: string | null

    async function deleteReview(review: Review) {
        const data   = {
            userId:   review.userId,
            reviewId: review.reviewId,
        }
        let response = await post(DELETE_URL, review)
        errorMessage = handleResponse(
            response,
            "There was an error deleting the review. Please try again.",
            logout
        )

        if (response.status === 200) {
            dispatch('allowEdit')
            $adminReviews = $adminReviews.filter((r: Review) => r.reviewId !== review.reviewId)
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
        $adminReviews = $adminReviews.filter((r: Review) => r.reviewId !== review.reviewId)
    }

    function logout() {
        authenticated.update(() => false)
    }
</script>

<Card class="text-center m-4" size="xl" padding="lg">
    <div class="flex flex-row">
        <div class="basis-1/4 gap-4">
            { review.contactName }
            <a href={`/admin/reviews?user-id=${review.userId}`}>{review.email}</a>
        </div>
        <div class="basis-2/4 gap-4">
            <h5 class="text-2xl font-bold tracking-tight text-gray-900 dark:text-white flex">
                { review.title }
                <div class="inline">
                    <StarRating currentRating={review.rating} active={false} color={'yellow'}></StarRating>

                </div>
            </h5>
            <p class="font-normal text-gray-700 dark:text-gray-400 leading-tight">
                { review.body }
            </p>
        </div>
        <div class="basis-1/4 gap-4">
            <GradientButton type="submit" class="w-full1" on:click={() => deleteReview(review)}>Delete review</GradientButton>
            <GradientButton type="submit" class="w-full1" on:click={() => editReview(review)}>Edit review</GradientButton>
        </div>
    </div>
</Card>