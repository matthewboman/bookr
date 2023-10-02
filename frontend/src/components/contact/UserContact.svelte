<script lang="ts">
    import { createEventDispatcher } from "svelte"
    import { Button, Modal } from "flowbite-svelte"

    import Reviews        from "../review/Reviews.svelte"
    import ContactDetails from './ContactDetails.svelte'
    import ContactModal   from "./AddContactForm.svelte"
    import { get, post }  from '../../api'
    import { handleResponse } from '../../functions'
    import { selectedContact, userId } from "../../store"
    import type { Contact } from '../../types'
   
    const DELETE_CONTACT_URL = "/user/delete-contact"
    const REVIEWS_URL        = "/reviews?contactId="
    const dispatch = createEventDispatcher()

    export let contact: Contact

    let editingContact = false
    let showingReviews = false
    $: reviewButton = showingReviews ? "Hide reviews" : "Show all reviews"
    $: reviews      = []

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

    async function showReviews() {
        showingReviews = !showingReviews

        if (reviews.length < 1) {
            let res = await get(`${REVIEWS_URL}${contact.contactId}`)
                .then(r => r.json())

            reviews = res.reviews
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

    <Button on:click={showReviews}>{ reviewButton }</Button>
    <Button on:click={editContact}>Edit</Button>
    <Button on:click={deleteContact}>Delete</Button>

    <Modal bind:open={editingContact} size="xs" class="w-full">
        <ContactModal on:update={closeContactModal} action={'edit'}/>
    </Modal>
</div>

{#if showingReviews}
    <Reviews reviews={reviews}/>
{/if}