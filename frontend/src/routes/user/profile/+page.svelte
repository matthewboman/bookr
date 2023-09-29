<script lang="ts">
    import { Button, Label, Input } from 'flowbite-svelte'

    import { post }      from '../../../api'
    import PasswordReset from '../../../components/PasswordReset.svelte'

    const CHANGE_PASSWORD_URL = "/user/change-password"

    export let disabled:       boolean
    export let errorMessage:   string
    export let successMessage: string

    let currentPassword:  string
    let newPassword:      string
    let newPasswordCheck: string

    async function submit() {
        errorMessage   = ''
        const body     = {
            currentPassword,
            newPassword,
            newPasswordCheck
        }
        const response = await post(CHANGE_PASSWORD_URL, body)

        // Don't use `handleResponse`. Different error codes require different messages.
        if (response.status === 200) {
            successMessage = "Your password has successfully been updated."
        }
        if (response.status === 400) {
            errorMessage = "Please re-check the link and make sure there is a token."
        }
        if (response.status === 401) {
            errorMessage = "The current password you entered was incorrect."
        }
        if (response.status === 500) {
            errorMessage = "There was an error updating your password."
        }
    }
</script>

<PasswordReset bind:newPassword={newPassword} bind:newPasswordCheck={newPasswordCheck}>
    <Label class="space-y-2" slot="change">
        <span>Current password</span>
        <Input type="password" name="password" placeholder="•••••" bind:value={currentPassword} required/>
    </Label>
    <Button slot="button" type="submit" class="w-full1" disabled={disabled} on:click={submit}>Change password</Button>
</PasswordReset>