<script>
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger } from 'flowbite-svelte'
    import { Button } from 'flowbite-svelte'

    import { post }          from '../api'
    import { authenticated } from '../store'

    export let authModal    = false
    export let contactModal = false

    let userExists

    authenticated.subscribe((val) => {userExists = val})

    async function signOut() {
        const res = await post("/user/logout", {})

        if (res.status === 200) {
            sessionStorage.removeItem('byotoken')
            authenticated.update(() => false)
        }
    }
</script>
  
<Navbar let:hidden let:toggle>
<NavBrand href="/">
    <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
        Book your own tour
    </span>
</NavBrand>
<NavHamburger on:click={toggle} />
    <NavUl {hidden}>
        {#if userExists}
            <Button on:click={() => contactModal = true}>Add contact</Button>
            <Button on:click={signOut}>Sign Out</Button>
        {:else}           
            <Button on:click={() => authModal = true}>Sign in</Button>
        {/if}
    </NavUl>
</Navbar>