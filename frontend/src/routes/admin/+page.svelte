<script lang="ts">
    import { onMount } from 'svelte'
    import { Alert }   from 'flowbite-svelte'
    import { goto }    from '$app/navigation'

    import Menu                     from '../../components/Menu.svelte'
    import PendingContact           from '../../components/PendingContact.svelte'
    import { get, post }            from '../../api'
    import { clearExpiredToken, isAdmin, isAuthenticated } from '../../functions'
    import { admin, authenticated } from "../../store"
    import type { Contact }         from '../../types'

    const PENDING_CONTACTS_URL = "/admin/pending-contacts"
    const APPROVE_CONTACT_URL  = "/admin/approve-pending-contact"
    const DELETE_CONTACT_URL   = "/admin/delete-pending-contact"

    let pendingContacts: Contact[] = []
    let errorMessage: String

    function authenticateAndAuthorize() {
        if (isAuthenticated()) {
            authenticated.update(() => true)
        } else {
            clearExpiredToken()
            authenticated.update(() => false)
            goto("/")
        }

        if (isAdmin()) {
            admin.update(() => true)
            return true
        } else {
            admin.update(() => false)
            goto("/")
        }
    }

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
        if (authenticateAndAuthorize()) {
            getContacts()
        }
    })

</script>

<Menu/>
<div>
    {#if errorMessage}
        <Alert border color="red">
            <svg slot="icon" aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>
            <span class="font-medium">Error</span> { errorMessage }
        </Alert>
    {/if}
    {#each pendingContacts as contact}
        <PendingContact contact={contact} on:approve={approveContact} on:delete={deleteContact}/>
    {/each}
</div>
