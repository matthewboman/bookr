<script lang="ts">
    import { goto } from '$app/navigation'
    import { Alert, Button, Label, Input, Helper, P } from 'flowbite-svelte'

    import { post }          from '../api'
    import { authenticated } from '../store'
    import { onMount } from 'svelte';

    let loggedIn       = false
    let disabled       = true
    let passowordsMatch = true
    let currentPassword: string
    let newPassword: string
    let newPasswordCheck: string
    let errorMessage: string
    let successMessage: string

    authenticated.subscribe((val) => {loggedIn = val})

    async function submit() {
        errorMessage = ''

        const params      = new URLSearchParams(window.location.search)
        const resetToken  = params.get("reset_token")

        const body = loggedIn 
            ? {
                currentPassword,
                newPassword,
                newPasswordCheck
            } 
            : {
                resetToken,
                newPassword,
                newPasswordCheck
            }
        
        const url = loggedIn ? "/user/change-password" : "/reset-password"
        const res = await post(url, body)

        if (res.status === 200) {
            successMessage = "Your password has successfully been updated."
            
            setTimeout(() => {
                goto("/")
            }, 2000)
        }

        if (res.status === 400) {
            errorMessage = "Please re-check the link and make sure there is a token."
        }

        if (res.status === 401 && !loggedIn) {
            errorMessage = "Invalid token in password reset link."
        }

        if (res.status === 401 && loggedIn) {
            errorMessage = "The current password you entered was incorrect."
        }

        if (res.status === 500) {
            errorMessage = "There was an error updating your password."
        }
    }

    function setAuthenticated() {
        let token = sessionStorage.getItem('byotoken')

        if (token != null) {
            authenticated.update(() => true)
        }
    }

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

    onMount(() => {
        setAuthenticated()
    })

  </script>
<form class="flex flex-col space-y-6" action="" on:submit|preventDefault={submit}>
    {#if errorMessage}
        <Alert border color="red">
            <svg slot="icon" aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>
            <span class="font-medium">Error</span> { errorMessage }
        </Alert>
    {/if}
    {#if successMessage}
        <Alert border color="blue">
            <svg slot="icon" aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>
            <span class="font-medium">Success</span> { successMessage }
        </Alert>
    {/if}
    {#if loggedIn}
        <Label class="space-y-2">
            <span>Current password</span>
            <Input type="password" name="password" placeholder="•••••" bind:value={currentPassword} required/>
        </Label>
    {/if}
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
    <Button type="submit" class="w-full1" disabled={disabled}>Reset password</Button>
</form>