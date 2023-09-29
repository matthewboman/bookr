<script lang="ts">
    import { createEventDispatcher } from "svelte"
    import { GradientButton }        from "flowbite-svelte"

    import { post }           from "../../api"
    import { handleResponse } from '../../functions'
    import { 
        admin, 
        authenticated, 
        contactReviews, 
        selectedReview,
    } from '../../store'
    import type { Review } from '../../types'
    import ReviewDispaly   from "./Review.svelte"

    export let review: Review
    let errorMessage:  string | null

    const dispatch = createEventDispatcher()

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

<ReviewDispaly review={review}>
    <div class="basis-1/4 gap-4" slot="actions">
        <GradientButton type="submit" class="w-full1" on:click={() => deleteReview(review)}>Delete review</GradientButton>
        <GradientButton type="submit" class="w-full1" on:click={() => editReview(review)}>Edit review</GradientButton>
    </div>
</ReviewDispaly>