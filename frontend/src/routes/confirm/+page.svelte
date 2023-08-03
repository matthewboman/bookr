<script lang="ts">
    import { browser } from '$app/environment'
    import { goto }    from '$app/navigation'
    import { onMount } from 'svelte'
	import { Modal }   from 'flowbite-svelte'

    import AuthModal   from "../../components/AuthModal.svelte"
    import Menu        from '../../components/Menu.svelte'
    import { get }     from "../../api"

    let authModal: any
    let message: string = "verifying"

    function closeAuthModal() {
        authModal = false
        goto("/")
    }

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
            message = "Invalid token. Please check your email and verify the link is correct."
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

<Menu bind:authModal={authModal} />

<Modal bind:open={authModal} size="xs" outsideclose class="w-full">
    <AuthModal on:close={closeAuthModal} />
</Modal>

<div>
    { message }
</div>