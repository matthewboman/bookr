<script lang="ts">
    import { onMount } from "svelte"
    import { Button, Modal } from "flowbite-svelte"

    import Reviews         from "./Reviews.svelte"
    import ContactModal    from "./ContactModal.svelte"
    import ContactAddress  from "./contact/ContactAddress.svelte"
    import ContactCapacity from './contact/ContactCapacity.svelte'
    import ContactLinks    from './contact/ContactLinks.svelte'
    import ContactName     from "./contact/ContactName.svelte"
    import { isAdmin, isAuthenticated } from '../functions'
    import { contactReviews, selectedContact, userId } from "../store"
   
    let canEditContact = false
    let editingContact = false

    function checkIfUserCanEdit(): boolean {
        if (isAdmin()) {
            return true
        }

        if (isAuthenticated() && $userId === $selectedContact.userId) {
            return true
        }

        return false
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
    <ContactName 
        className={'flex'}
        name={$selectedContact.displayName} 
        rating={$selectedContact.averageRating} 
        starColor={'red'}
        starSize={25}
    />
    <ContactAddress
        className={'mb-2'}
        contact={$selectedContact}
    />
    <ContactCapacity
        className={''}
        capacity={$selectedContact.capacity}
    />
    <ContactLinks
        className={''}
        link={$selectedContact.contactForm}
        email={$selectedContact.email}
    />
    {#if canEditContact}
        <Button on:click={editContact}>Edit</Button>
    {/if}

    <Modal bind:open={editingContact} size="xs" class="w-full">
        <ContactModal on:update={closeContactModal} contact={$selectedContact} action={'edit'}/>
    </Modal>
</div>

<Reviews reviews={$contactReviews}/>