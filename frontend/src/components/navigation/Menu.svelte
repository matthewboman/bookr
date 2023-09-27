<script lang="ts">
    import { onMount } from 'svelte'
    import { Button, Modal, Navbar, NavBrand, NavLi, NavUl, NavHamburger } from 'flowbite-svelte'
    import { goto }    from '$app/navigation'

    import { post }          from '../../api'
    import { isAdmin, isAuthenticated } from '../../functions'
    import { admin, authenticated }     from '../../store'
    import AuthModal         from "../AuthModal.svelte"
    import ContactModal      from '../ContactModal.svelte'

    export let onAuthClose: string = ''

    let authModal    = false
    let contactModal = false

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

    onMount(async() => {
        if (isAuthenticated()) {
            authenticated.update(() => true)
        }

        if (isAdmin()) {
            admin.update(() => true)
        }     
    })
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
        {#if $admin}
            <NavLi href="/admin">Admin</NavLi>
        {/if}
        {#if $authenticated}
            <NavLi href="/user/profile">Profile</NavLi>
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
<Modal bind:open={contactModal} size="xs" class="w-full">
    <ContactModal on:close={closeContactModal} />
</Modal>

<style>
    * {
        font-family: Comfortaa;
    }
</style>