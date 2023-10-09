<script lang="ts">
    import { onMount } from "svelte"

    import { get }         from '../../api'
    import { contactReviews, selectedContact } from "../../store"
    import SelectedContact from "../../components/contact/SelectedContact.svelte";

    const CONTACT_URL = "/contact?contactId="
    const REVIEWS_URL = "/reviews?contactId="

    async function getContact(contactId: string) {
        const res = await get(`${CONTACT_URL}${contactId}`)

        if (res.status === 200) {
            $selectedContact = await res.json()
        }
    }

    async function getReviews(contactId: string) {
        const res = await get(`${REVIEWS_URL}${contactId}`)

        if (res.status === 200) {
            let body = await res.json()
            $contactReviews = body.reviews
        }
    }

    onMount(() => {
        if ($selectedContact == null) {
            const params    = new URLSearchParams(window.location.search)
            const contactId = params.get("contactId")

            if (contactId) {
                getContact(contactId)
                getReviews(contactId)

            }
        }
    })
</script>

<div class="container">
    {#if $selectedContact}
        <SelectedContact />
    {:else}
    no contact found
    {/if}
</div>

