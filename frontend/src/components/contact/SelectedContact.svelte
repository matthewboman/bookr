<script lang="ts">
    import { onMount } from "svelte"
    import { Button, Modal } from "flowbite-svelte"

    import Reviews        from "../review/Reviews.svelte"
    import ContactDetails from './ContactDetails.svelte'
    import AddContactForm from "./AddContactForm.svelte"
    import { post }       from '../../api'
    import { handleResponse, isAdmin, isAuthenticated } from '../../functions'
    import { contactReviews, selectedContact, userId }  from "../../store"
   
    const DELETE_CONTACT_URL = "/user/delete-contact"

    let canEditContact = false
    let editingContact = false

    function checkIfUserCanEdit(): boolean {
        if (isAdmin()) return true

        if (isAuthenticated() && $userId === $selectedContact.userId) return true

        return false
    }

    async function deleteContact() {
        let req = {
            userId:    $userId,
            contactId: $selectedContact.contactId
        }
        let res = await post(DELETE_CONTACT_URL, req)

        errorMessage = handleResponse(
            res,
            "There was an error deleting the contact",
            Function
        )

        if (res.status === 200) {
            $selectedContact = null
        }
    }

    function editContact() {
        editingContact = true
    }

    function closeContactModal() {
        editingContact = false
    }

    onMount(() => {
        canEditContact = checkIfUserCanEdit()
    })
</script>

<div>
    <ContactDetails contact={$selectedContact} className={'mb-4'} />

    {#if canEditContact}
        <Button on:click={editContact}>Edit</Button>
        <Button on:click={deleteContact}>Delete</Button>
    {/if}

    <Modal bind:open={editingContact} size="xs" class="w-full">
        <AddContactForm on:update={closeContactModal} action={'edit'}/>
    </Modal>
</div>

{#if $contactReviews}
    <Reviews reviews={$contactReviews} />
{/if}