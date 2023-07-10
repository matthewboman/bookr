<script>
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger } from 'flowbite-svelte'
    import { Button } from 'flowbite-svelte'

    export let authModal    = false
    export let contactModal = false

    let authenticated = false
</script>
  
<Navbar let:hidden let:toggle>
<NavBrand href="/">
    <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
        Book your own tour
    </span>
</NavBrand>
<NavHamburger on:click={toggle} />
    <NavUl {hidden}>
        {#if authenticated}
        <NavLi href="">Sign out</NavLi>
        {:else}
        <Button on:click={() => contactModal = true}>Add contact</Button>
        <Button on:click={() => authModal = true}>Sign in</Button>
        {/if}
    </NavUl>
</Navbar>