<script lang="ts">
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger } from 'flowbite-svelte'
    import { Button, Modal } from 'flowbite-svelte'
    import { goto }          from '$app/navigation'

    import { post }          from '../api'
    import { authenticated } from '../store'
    import AuthModal         from "../components/AuthModal.svelte"
    import ContactModal      from '../components/ContactModal.svelte'

    export let onAuthClose = null
    
    let authModal    = false
    let contactModal = false
    let userExists: boolean

    authenticated.subscribe((val) => {userExists = val})

    async function signOut() {
        const res = await post("/user/logout", {})

        if (res.status === 200) {
            sessionStorage.removeItem('byotoken')
            authenticated.update(() => false)
        }
    }

    function closeAuthModal() {
        authModal = false

        if (onAuthClose === 'redirect') {
            goto("/")
        }
    }

    function closeContactModal() {
        contactModal = false
    }
</script>
  
<Navbar let:hidden let:toggle color="primary">
    <NavBrand href="/">
        <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
            Book your own tour
        </span>
    </NavBrand>
    <NavHamburger on:click={toggle} />
    <NavUl {hidden}>
        <NavLi href="/about">About</NavLi>
        {#if userExists}
            <Button size="sm" on:click={() => contactModal = true}>Add contact</Button>
            <Button size="sm" on:click={signOut}>Sign Out</Button>
        {:else}           
            <Button size="sm" on:click={() => authModal = true}>Sign in</Button>
        {/if}
    </NavUl>
</Navbar>

<Modal bind:open={authModal} size="xs" outsideclose class="w-full">
    <AuthModal on:close={closeAuthModal}/>
</Modal>
<Modal bind:open={contactModal} size="xs" outsideclose class="w-full">
    <ContactModal on:close={closeContactModal} />
</Modal>