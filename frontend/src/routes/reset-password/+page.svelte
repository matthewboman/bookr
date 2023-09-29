<script lang="ts">
    import { goto }   from '$app/navigation'
    import { Button } from 'flowbite-svelte'

    import { post }      from '../../api'
    import PasswordReset from '../../components/PasswordReset.svelte'

    const RESET_PASSWORD_URL = "/reset-password"
    const REDIRECT_AFTER_MS  = 1000

    export let disabled:       boolean
    export let errorMessage:   string
    export let successMessage: string

    let newPassword:      string
    let newPasswordCheck: string

    async function submit() {
        errorMessage     = ''
        const params     = new URLSearchParams(window.location.search)
        const resetToken = params.get("reset_token")
        const body       = {
            resetToken,
            newPassword,
            newPasswordCheck
        }  
        const response   = await post(RESET_PASSWORD_URL, body)

        // Don't use `handleResponse`. Different error codes require different messages.
        if (response.status === 200) {
            successMessage = "Your password has successfully been updated."
            
            setTimeout(() => {
                goto("/")
            }, REDIRECT_AFTER_MS)
        }
        if (response.status === 400) {
            errorMessage = "Please re-check the link and make sure there is a token."
        }
        if (response.status === 401) {
            errorMessage = "Invalid token in password reset link."
        }
        if (response.status === 500) {
            errorMessage = "There was an error updating your password."
        }
    }
</script>
<PasswordReset bind:newPassword={newPassword} bind:newPasswordCheck={newPasswordCheck}>
    <Button slot="button" type="submit" class="w-full1" disabled={disabled} on:click={submit}>Change password</Button>
</PasswordReset>