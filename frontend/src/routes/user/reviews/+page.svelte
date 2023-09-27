<script lang="ts">
	import { onMount } from 'svelte'

    import UserDisplayReview  from '../../../components/review/UserDisplayReview.svelte'
    import EditReview         from '../../../components/review/EditReview.svelte'
    import { get }            from '../../../api'
    import { handleResponse } from '../../../functions'
    import { authenticated }  from "../../../store"

    const REVIEW_URL = "/user/my-reviews"

    $: reviews = []
    let errorMessage: string | null
    let allowEditReview = false

    async function getReviews() {
        let response = await get(REVIEW_URL)

        errorMessage = handleResponse(
            response,
            "Unable to get reviews.",
            logout
        )

        if (response.status === 200) {
            reviews = await response.json()
        }
    }

    function logout() {
        authenticated.update(() => false)
    }

    onMount(async () => {
        getReviews()
    })
</script>

{#if allowEditReview}
    <EditReview on:disallowEditReview={() => allowEditReview = false}/>
{/if}
{#each reviews as review}
    <UserDisplayReview review={review} on:allowEdit={() => allowEditReview = true}/>
{/each}