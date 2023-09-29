<script lang="ts">
    import { Label, Input, Helper } from 'flowbite-svelte'

    import ErrorMessage   from './ui/ErrorMessage.svelte'
    import SuccessMessage from './ui/SuccessMessage.svelte'

    export let newPassword:      string
    export let newPasswordCheck: string

    let disabled        = true
    let passowordsMatch = true
    let errorMessage:   string
    let successMessage: string

    function comparePasswords() {
        if (newPassword && newPasswordCheck) {
            passowordsMatch = newPassword === newPasswordCheck
            disabled = newPassword !== newPasswordCheck
        } else if (!newPassword || !newPasswordCheck) {
            disabled = true
        } else if (!newPassword && !newPasswordCheck) {
            passowordsMatch = true
            disabled = true
        }
    }
</script>

{#if errorMessage}
    <ErrorMessage message={errorMessage} />
{/if}
{#if successMessage}
    <SuccessMessage message={successMessage} />
{/if}
<slot name="change" />
<Label class="space-y-2">
    <span>New password</span>
    <Input type="password" name="password" placeholder="•••••" bind:value={newPassword} on:input={comparePasswords} required/>
</Label>
<Label class="space-y-2">
    <span>Confirm new password</span>
    <Input type="password" name="password" placeholder="•••••" bind:value={newPasswordCheck} on:input={comparePasswords} required/>
</Label>
{#if !passowordsMatch}
    <Helper class='mt-2' color='red'><span class="font-medium">Passwords do not match</span></Helper>
{/if}
<slot name="button" {disabled} />