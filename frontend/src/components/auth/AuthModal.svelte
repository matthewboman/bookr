<script lang="ts">
    import { createEventDispatcher } from 'svelte'
    import { Button, Label, Input } from 'flowbite-svelte'

    import { type User } from '../../types' // TODO: verify w types
    import { post }          from '../../api'
    import { getUserId }     from '../../functions'
    import { authenticated, userId } from '../../store'
    import ErrorMessage      from '../ui/ErrorMessage.svelte'
    import SuccessMessage    from '../ui/SuccessMessage.svelte'

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

    let email: string
    let password: string
    let verifyPassword: string
    let errorMessage: string
    let successMessage: string
    let disabled = false // TODO: disable button if passwords don't match

    // TODO: split into different modals using slots
    // TODO: handle response

    async function submit() {
        errorMessage   = ''
        successMessage = ''

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
        const res = await post("/login", { email, password})
        
        if (res.status === 401) {
            errorMessage = "Incorrect email or password"
        }

        if (res.status === 200) {
            const json = await res.json()

            sessionStorage.setItem('byotoken', json.token)
            authenticated.update(() => true)
            $userId = getUserId()
            dispatch('close')
        }

        // TODO: handle 400 & 500 errors
    }

    async function register() {
        // TODO: make reactive to user input
        if (password.length && verifyPassword.length && password != verifyPassword) {
            errorMessage = "Passwords do not match"
        }

        const res = await post("/signup", { email, password, verifyPassword})

        if (res.status === 200) {
            successMessage = "Please check your email to verify your account."

            setTimeout(() => {
                dispatch('close')
            }, 2000)
        }

        // TODO: handle errors where email is taken

        // TODO: handle other server errors
    }

    async function resetPassword() {
        const res = await post("/generate-reset-token", { email })
        
        if (res.status === 200) {
            successMessage = "Please check your email for a password reset link."
            
            setTimeout(() => {
                dispatch('close')
            }, 2000)
        }

        // TODO: handle errors
    }
</script>


<form class="flex flex-col space-y-6" action="" on:submit|preventDefault={submit}>
    <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">{ actions[currentAction].text }</h3>
    {#if errorMessage}
        <ErrorMessage message={errorMessage} />
    {/if}
    {#if successMessage}
        <SuccessMessage message={successMessage} />
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

    {#if actions[currentAction].action !== "signUp"}
        <div class="text-sm font-medium text-gray-500 dark:text-gray-300">
            Not registered? <a on:click={() => currentAction = "signUp"} class="text-primary-700 hover:underline dark:text-primary-500">Create account</a>
        </div>
    {/if}
</form>