<script lang="ts">
	import { onMount } from 'svelte'
    import { goto }    from '$app/navigation'

    import PasswordReset     from "../../../components/PasswordReset.svelte"
    import Menu              from '../../../components/Menu.svelte'
    import DisplayReview     from '../../../components/review/UserDisplayReview.svelte'
    import EditReview        from '../../../components/review/EditReview.svelte'
    import { get }           from '../../../api'
    import { isAuthenticated, clearExpiredToken, handleResponse } from '../../../functions'
    import { authenticated } from "../../../store"

    const REVIEW_URL = "/user/my-reviews"

    $: reviews = []
    let errorMessage: string | null
    let allowEditReview = false

    function setAuthenticated() {
        if (isAuthenticated()) {
            authenticated.update(() => true)
        } else {
            clearExpiredToken()
            goto("/")
        }
    }

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
        setAuthenticated()
        getReviews()
    })
</script>

<Menu/>
<PasswordReset></PasswordReset>
{#if allowEditReview}
    <EditReview on:disallowEditReview={() => allowEditReview = false}/>
{/if}
{#each reviews as review}
    <DisplayReview review={review} on:allowEdit={() => allowEditReview = true}/>
{/each}