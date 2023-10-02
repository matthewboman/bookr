<script lang="ts">
    import { createEventDispatcher, onMount } from 'svelte'
    import { Card, GradientButton, Input, Label, Textarea } from "flowbite-svelte"

    import StarRating         from "../ui/StarRating.svelte"
    import ErrorMessage       from '../ui/ErrorMessage.svelte'
    import { post }           from "../../api"
    import { handleResponse } from '../../functions'
    import { 
        admin,
        authenticated,
        contactReviews,
        selectedContact,
        selectedReview,
        userId
    } from '../../store'
    import type { NewReview, Review } from "../../types"

    const ADD_REVIEW_URL = "/user/review-contact"
    const dispatch       = createEventDispatcher()

    let body:         string
    let errorMessage: string | null
    let rating      = 0
    let title:        string

    function cancelEdit() {
        dispatch('disallowEditReview')
        $contactReviews = [ $selectedReview, ...$contactReviews ]
    }

    function logout() {
        authenticated.update(() => false)
    }

    async function submitEdit() {
        const url = $admin ? "/admin/edit-review" : "/user/edit-review"
        const review: Review = { 
            reviewId: $selectedReview.reviewId,
            userId: $userId,
            contactId: $selectedContact.contactId,
            title,
            body,
            rating
        }
        let response = await post(url, review)
        errorMessage = handleResponse(
            response,
            "There was an error editing the review. Please try again.",
            logout
        )

        if (response.status === 200) {
            dispatch('disallowEditReview')
            $contactReviews = [ review, ...$contactReviews ]
        }
    }

    async function submitReview() {
        errorMessage = ''

        if (!rating) {
            errorMessage = "The review must have a rating"
            return
        }
        const review: NewReview = {
            userId: $userId, 
            contactId: $selectedContact.contactId,
            title,
            body,
            rating
        }

        let response = await post(ADD_REVIEW_URL, review)
        errorMessage = handleResponse(
            response,
            "There was an error adding the review. Please try again.",
            logout
        )

        if (response.status === 200) {
            dispatch('disallowEditReview')
            $contactReviews = [ review, ...$contactReviews ]
        }
    }

    function updateRating(e) {
        rating = e.detail.rating
    }

    onMount(() => {
        if ($selectedReview) {
            title  = $selectedReview.title
            rating = $selectedReview.rating // BUG this doesn't update unless component is hovered
            body   = $selectedReview.body
        }
    })
</script>

<Card class="text-center m-4" size="xl" padding="lg">
    {#if errorMessage}
        <ErrorMessage message={errorMessage} />
    {/if}
    <form class="flex flex-col space-y-6">
        <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white flex justify-between">
            <Label for="title" class="space-y-2">
                <span>Review title</span>
                <Input type="text" name="title" placeholder="" bind:value={title} required />
            </Label>
            <StarRating currentRating={rating} active={true} on:updateRating={updateRating} color={'yellow'}></StarRating>
        </h5>
        <p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
            <Label for="body" class="mb-2">Review</Label>
            <Textarea id="body" placeholder="Your review" rows="4" name="body" bind:value={body}/>
        </p>
        <div>
            {#if $selectedReview }
                <GradientButton type="submit" class="w-full1" on:click={submitEdit}>Edit review</GradientButton>
                <GradientButton class="w-full1" on:click={cancelEdit}>Cancel</GradientButton>
            {:else}
                <GradientButton type="submit" class="w-full1" on:click={submitReview}>Submit review</GradientButton>
            {/if}
        </div>
    </form>
</Card>