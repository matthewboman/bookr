<script lang="ts">
    import { onMount } from 'svelte'

    import ErrorMessage     from '../../../components/ui/ErrorMessage.svelte'
    import PendingContact   from '../../../components/contact/PendingContact.svelte'
    import { get, post }    from '../../../api'
    import type { Contact } from '../../../types'

    const PENDING_CONTACTS_URL = "/admin/pending-contacts"
    const APPROVE_CONTACT_URL  = "/admin/approve-pending-contact"
    const DELETE_CONTACT_URL   = "/admin/delete-pending-contact"

    let pendingContacts: Contact[] = []
    let errorMessage:    String

    async function approveContact(e) {
        let contact = formatContact(e.detail.contact)
        const res   = await post(APPROVE_CONTACT_URL, contact)

        handleResponse(res, contact.contactId)
    }

    async function deleteContact(e) {
        let contact = formatContact(e.detail.contact)
        const res   = await post(DELETE_CONTACT_URL, contact)

        handleResponse(res, contact.contactId)
    }

    async function getContacts() {
        pendingContacts = await get(PENDING_CONTACTS_URL)
            .then(r => r.json())
    }

    function formatContact(contact: Contact) {
        return {
            contactId: contact.contactId,
            address:   contact.address,
            city:      contact.city,
            state:     contact.state,
            zipCode:   contact.zipCode
        }
    }

    // TODO
    function handleResponse(res: any, contactId: number) {
        if (res.status === 200) {
            removeContactFromList(contactId)
        }

        if (res.status === 400 || res.status === 500) {
            errorMessage = "There was an error"
        }

        if (res.status === 401) {
            errorMessage = "Unauthorized action"
        }
    }

    function removeContactFromList(contactId: number) {
        pendingContacts = pendingContacts.filter(c => c.contactId !== contactId)
    }

    onMount(async() => {
        getContacts()
    })

</script>

<div>
    <h2>
        Pending contacts
    </h2>
    {#if errorMessage}
        <ErrorMessage message={errorMessage} />
    {/if}
    {#each pendingContacts as contact}
        <PendingContact contact={contact} on:approve={approveContact} on:delete={deleteContact}/>
    {/each}
</div>
