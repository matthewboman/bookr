<script lang="ts">
    import { Button, Label, Input } from 'flowbite-svelte'

    export let apiUrl

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

    const submit = () => {
        console.log(`${apiUrl}${actions[currentAction].endpoint}`)
    }

</script>


<form class="flex flex-col space-y-6" action="#">
    <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">{ actions[currentAction].text }</h3>
    <Label class="space-y-2">
        <span>Email</span>
        <Input type="email" name="email" placeholder="name@company.com" required />
    </Label>
    {#if actions[currentAction].action != "reset"}
        <Label class="space-y-2">
            <span>Your password</span>
            <Input type="password" name="password" placeholder="•••••" required />
        </Label>

        {#if actions[currentAction].action == "signUp"}
            <Label class="space-y-2">
                <span>Verify password</span>
                <Input type="password" name="verifyPassword" placeholder="•••••" required />
            </Label>
        {/if}
        
        <div class="flex items-start">
            <a on:click={() => currentAction = "reset"} class="ml-auto text-sm text-primary-700 hover:underline dark:text-primary-500">Lost password?</a>
        </div>
    {/if }
    <Button type="submit" on:click={submit} class="w-full1">{ actions[currentAction].buttonText }</Button>

    <div class="text-sm font-medium text-gray-500 dark:text-gray-300">
        Not registered? <a on:click={() => currentAction = "signUp"} class="text-primary-700 hover:underline dark:text-primary-500">Create account</a>
    </div>
</form>