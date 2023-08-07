<script lang="ts">
	import { onMount } from 'svelte'
	import { Modal }   from 'flowbite-svelte'
    import { goto }    from '$app/navigation'


    import PasswordReset     from "../../../components/PasswordReset.svelte"
    import ContactModal      from "../../../components/ContactModal.svelte"
    import Menu              from '../../../components/Menu.svelte'
    import { authenticated } from "../../../store"


    let contactModal: any

    function closeContactModal() {
        contactModal = false
    }

    function setAuthenticated() {
        let token = sessionStorage.getItem('byotoken')

        if (token != null) {
            authenticated.update(() => true)
        } else {
            goto("/")
        }
    }

    onMount(async () => {
        setAuthenticated()
    })
</script>

<Menu bind:contactModal={contactModal} />

<Modal bind:open={contactModal} size="xs" outsideclose class="w-full">
    <ContactModal on:close={closeContactModal} />
</Modal>

<PasswordReset></PasswordReset>