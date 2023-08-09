<script lang="ts">
    import { browser } from '$app/environment'
    import { onMount } from 'svelte'

    import Menu    from '../../components/Menu.svelte'
    import { get } from "../../api"

    export let onAuthClose = 'redirect'
    let message: string = "verifying"

    async function verifyEmail() {
        const params = new URLSearchParams(window.location.search)
        const token  = params.get("token")

        if (!token) {
            message = "No token provided. Please check your email and verify the link is correct."
            return
        }

        const response = await get(`/confirm?token=${token}`)

        if (response.status === 200) {
            message = "Email has successfully been verified."
        }

        if (response.status === 401) {
            message = "Invalid token. Please check your email and verify the link is correct and token not expired."
        }

        if (response.status === 500) {
            message = "There was an error validating your token."
        }

    }

    onMount(async () => {
        if (browser) {
            verifyEmail()
        }      
    })

</script>

<Menu bind:onAuthClose={onAuthClose}/>
<div>
    { message }
</div>