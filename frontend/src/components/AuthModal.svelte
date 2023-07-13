<script lang="ts">
    import { createEventDispatcher } from 'svelte'
    import { Button, Label, Input } from 'flowbite-svelte'

    import { type User } from '../types' // TODO: verify w types
    import { post }          from '../api'
    import { authenticated } from '../store'

    const dispatch = createEventDispatcher()

    let currentAction = "signIn"
    let actions       = {
        signIn: {
            action:     "signIn",
            text:       "Sign in to view your contacts",
            buttonText: "Login to your account",
            endpoint:   "/login"
        },
        signUp: {
            action:     "signUp",
            text:       "Register to add your own private contacts",
            buttonText: "Sign up",
            endpoint:   "/signup"
        },
        reset: {
            action:     "reset",
            text:       "Enter your email to recieve a password reset link",
            buttonText: "Reset password",
            endpoint:   "/reset-password"
        }
    }

    let email          = ''
    let password       = ''
    let verifyPassword = ''
    let errorText      = null
    let disabled       = false // TODO: disable button if passwords don't match

    async function submit() {
        if (currentAction === 'signIn') {
            await login()
        } else if (currentAction === 'signUp') {
            await register()
        } else if (currentAction === 'reset') {
            await resetPassword()
        } else {
            // TODO: this shouldn't happen
        }  
    }

    async function login() {
        errorText = ''

        const res = await post("/login", { email, password})
        
        if (res.status === 401) {
            errorText = "Incorrect email or password"
        }

        if (res.status === 200) {
            const json = await res.json()

            sessionStorage.setItem('byotoken', json.token)

            authenticated.update(() => true)

            dispatch('close')
        }

        // TODO: handle 400 & 500 errors
    }

    async function register() {
        errorText = ''

        // TODO: make reactive to user input
        if (password.length && verifyPassword.length && password != verifyPassword) {
            errorText = "Passwords do not match"
        }

        const res = await post("/signup", { email, password, verifyPassword})

        if (res.status === 200) {
            // TODO: open success modal w/ instructions to check email

            dispatch('close')
        }

        // TODO: handle errors where email is taken

        // TODO: handle other server errors
    }

    async function resetPassword() {
        errorText = ''

        const res = await post("/reset-password-link", { email })
        
        if (res.status === 200) {
            // TODO: open success modal w/ instructions to check email
            
            dispatch('close')
        }

        // TODO: handle errors
    }
 

</script>


<form class="flex flex-col space-y-6" action="" on:submit|preventDefault={submit}>
    <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">{ actions[currentAction].text }</h3>
    {#if errorText}
    <p>
        {errorText}
    </p>
    {/if}
    <Label class="space-y-2">
        <span>Email</span>
        <Input type="email" name="email" placeholder="name@company.com" bind:value={email} required/>
    </Label>
    {#if actions[currentAction].action != "reset"}
        <Label class="space-y-2">
            <span>Your password</span>
            <Input type="password" name="password" placeholder="•••••" bind:value={password} required/>
        </Label>

        {#if actions[currentAction].action == "signUp"}
            <Label class="space-y-2">
                <span>Verify password</span>
                <Input type="password" name="verifyPassword" placeholder="•••••" bind:value={verifyPassword} required/>
            </Label>
        {/if}
        
        <div class="flex items-start">
            <a on:click={() => currentAction = "reset"} class="ml-auto text-sm text-primary-700 hover:underline dark:text-primary-500">Lost password?</a>
        </div>
    {/if }
    <Button type="submit" class="w-full1">{ actions[currentAction].buttonText }</Button>

    <div class="text-sm font-medium text-gray-500 dark:text-gray-300">
        Not registered? <a on:click={() => currentAction = "signUp"} class="text-primary-700 hover:underline dark:text-primary-500">Create account</a>
    </div>
</form>