<script lang="ts">
    import { goto }    from '$app/navigation'
	import { Modal }   from 'flowbite-svelte'

    import PasswordReset from "../../components/PasswordReset.svelte"
    import AuthModal     from "../../components/AuthModal.svelte"
    import Menu          from '../../components/Menu.svelte'

    let authModal: any

    function closeAuthModal() {
        authModal = false
        goto("/")
    }
</script>

<Menu bind:authModal={authModal} />

<Modal bind:open={authModal} size="xs" outsideclose class="w-full">
    <AuthModal on:close={closeAuthModal} />
</Modal>

<PasswordReset></PasswordReset>