<script lang="ts">
    import { createEventDispatcher } from "svelte"
    import { Button, Modal } from "flowbite-svelte"

    import Reviews        from "../review/Reviews.svelte"
    import ContactDetails from './ContactDetails.svelte'
    import ContactModal   from "../ContactModal.svelte"
    import { post }       from '../../api'
    import { handleResponse } from '../../functions'
    import { selectedContact, userId } from "../../store"
    import type { Contact } from '../../types'

    // TODO: build endpoint && workflow where user clicks to get all reviews for their contact
   
    const DELETE_CONTACT_URL = "/user/delete-contact"
    const dispatch = createEventDispatcher()

    export let contact: Contact

    let editingContact = false
    let errorMessage: string | null

    async function deleteContact() {
        let req = {
            userId:    $userId,
            contactId: contact.contactId
        }
        let res = await post(DELETE_CONTACT_URL, req)

        errorMessage = handleResponse(
            res,
            "There was an error deleting the contact",
            Function
        )

        if (res.status === 200) {
            dispatch('delete', { contact })
        }
    }

    function editContact() {
        editingContact = true
        $selectedContact = contact
    }

    function closeContactModal() {
        editingContact = false
    }
</script>

<div>
    <ContactDetails contact={contact} className={''} />
    { contact.contactType}

    <Button on:click={editContact}>Edit</Button>
    <Button on:click={deleteContact}>Delete</Button>

    <Modal bind:open={editingContact} size="xs" class="w-full">
        <ContactModal on:update={closeContactModal} action={'edit'}/>
    </Modal>
</div>

<Reviews reviews={[]}/>