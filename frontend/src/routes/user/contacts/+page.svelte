<script lang="ts">
    import { onMount } from "svelte"

    import UserContact from "../../../components/contact/UserContact.svelte"
    import { get }     from '../../../api'
    import { handleResponse  } from "../../../functions"
    import { authenticated }   from "../../../store"

    const CONTACTS_URL = "/user/contacts"

    $: contacts = []
    let errorMessage: string | null

    async function getContacts() {
        let response = await get(CONTACTS_URL)

        errorMessage = handleResponse(
            response,
            "Unable to get contacts.",
            logout
        )

        if (response.status === 200) {
            contacts = await response.json()
        }
    }

    function deleteContact(e) {
        contacts = contacts.filter(c => c.contactId !== e.detail.contact.contactId)
    }

    function logout() {
        authenticated.update(() => false)
    }

    onMount(async () => {
        getContacts()
    })
</script>

<div class="">
    {#each contacts as contact}
        <UserContact contact={contact} on:delete={deleteContact}/>
    {/each}
</div>