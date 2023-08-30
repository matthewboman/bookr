<script lang="ts">
    import { onMount } from 'svelte'
    import { 
        Alert, 
        Card, 
        GradientButton, 
        Label, 
        Input, 
        Textarea 
    } from 'flowbite-svelte'

    import { post } from '../api'
    import { handleResponse, isAdmin, isAuthenticated } from '../functions'
    import { 
        admin, 
        authenticated, 
        contactReviews, 
        selectedContact, 
        userId 
    } from '../store'
    import type { NewReview, Review } from '../types'
    import StarRating                 from './StarRating.svelte'

    const ADD_REVIEW_URL = "/user/review-contact"

    export let reviews: Review[]

    let allowAddReview    = false
    let allowDeleteReview = false
    let editingReview: Review
    let errorMessage:  string | null
    
    let body:   string
    let rating: number | null
    let title:  string

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
            allowAddReview  = false
            $contactReviews = [ review, ...$contactReviews ]
        }
    }

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
            $contactReviews = $contactReviews.filter((r: Review) => r.reviewId !== review.reviewId)
        }
    }

    async function submitEdit() {
        const url = $admin ? "/admin/edit-review" : "/user/edit-review"
        const review: Review = { 
            reviewId: editingReview.reviewId,
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
            allowAddReview  = false
            $contactReviews = [ review, ...$contactReviews ]
        }
    }

    function editReview(review: Review) {
        allowAddReview = true
        title          = review.title
        body           = review.body
        rating         = review.rating
        editingReview  = review

        console.log('rating', rating)
        // TODO: buggy. Need better way of handling
        $contactReviews = $contactReviews.filter((r: Review) => r.reviewId !== review.reviewId)
    }

    // TODO: buggy. Need better way of handling
    function cancelEdit() {
        allowAddReview  = false
        $contactReviews = [ editingReview, ...$contactReviews ]
    }

    function updateRating(e) {
        rating = e.detail.rating
    }

    function logout() {
        authenticated.update(() => false)
    }

    onMount(() => {
        let hasReviewed = $contactReviews
                .map((r: Review) => r.userId)
                .includes($userId)

        if (isAdmin() && !hasReviewed) { // actually use this
        // if (isAdmin()) { // testing by adding multiple reviews
            admin.update(() => true)
            allowDeleteReview = true
            allowAddReview    = true
        } else {
            admin.update(() => false)
        }

        if (isAuthenticated() && !hasReviewed) {
            allowAddReview = true
        }

        allowAddReview = true
    })
</script>

<div class="flex">
    {#if allowAddReview && $selectedContact}
        <Card class="text-center m-4" size="xl" padding="lg">
            {#if errorMessage}
            <Alert border color="red">
                <svg slot="icon" aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>
                <span class="font-medium">Error</span> { errorMessage }
            </Alert>
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
                    {#if editingReview }
                        <GradientButton type="submit" class="w-full1" on:click={submitEdit}>Edit review</GradientButton>
                        <GradientButton class="w-full1" on:click={cancelEdit}>Cancel</GradientButton>
                    {:else}
                        <GradientButton type="submit" class="w-full1" on:click={submitReview}>Submit review</GradientButton>
                    {/if}
                </div>
            </form>
        </Card>
    {/if}
    {#each reviews as review}
        <Card class="text-center m-4" size="xl" padding="lg">
            <h5 class="mb-2text-2xl font-bold tracking-tight text-gray-900 dark:text-white flex justify-between">
                { review.title }
                <StarRating currentRating={review.rating} active={false} color={'yellow'}></StarRating>
            </h5>
            <p class="mb-3 font-normal text-gray-700 dark:text-gray-400 leading-tight">
                { review.body }
            </p>
            <div>
                {#if $admin || review.userId ===  $userId}
                    <GradientButton type="submit" class="w-full1" on:click={() => deleteReview(review)}>Delete review</GradientButton>
                {/if}
                {#if $admin || review.userId ===  $userId}
                    <GradientButton type="submit" class="w-full1" on:click={() => editReview(review)}>Edit review</GradientButton>
                {/if}
            </div>
        </Card>
    {/each}
</div>