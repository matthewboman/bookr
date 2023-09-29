<script lang="ts">
    import { createEventDispatcher, onMount } from 'svelte'
    import { Button, Label, Input, Checkbox, Select, MultiSelect } from 'flowbite-svelte'

    import { get, post }  from '../api'
    import { ageRanges, states, contactTypes } from '../constants'
    import { isAdmin, handleResponse }  from '../functions'
    import { authenticated, genres, selectedContact } from '../store'
    import type { Contact, Genre, NewContact } from '../types'
    import ErrorMessage   from './ui/ErrorMessage.svelte'
    import SuccessMessage from './ui/SuccessMessage.svelte'

    const ADD_CONTACT_URL = "/user/add-contact"
    const GENRES_URL      = "/genres"
    const dispatch        = createEventDispatcher()

    export let action:  string = 'add'

    let errorMessage:   string | null
    let successMessage: string | null
    let displayName:    string
    let address:        string | null
    let city:           string
    let state:          string
    let zipCode:        string | null
    let ageRange:       string = "unknown"
    let contactType:    string = "venue"
    let capacity:       number | null
    let email:          string | null
    let contactForm:    string | null
    let isPrivate:      boolean = false 

    let selectedGenres  = $selectedContact ? $selectedContact.genres.map(g => g.genreId) : null
    let formattedGenres = $genres.map(g => ({ value: g.genreId, name: g.genreName }))
    let title  = action === 'add' ? 'Add new contact' : 'Edit selected contact'
    let button = action === 'add' ? 'Add contact' : 'Update contact'

    async function getGenres() {
        $genres = await get(GENRES_URL).then(r => r.json())
        formattedGenres = $genres.map((g: Genre) => ({ value: g.genreId, name: g.genreName }))
    }

    async function submit() {
        if (action === 'add') {
            await addNewContact()
        } else if (action === 'edit') {
            await editContact()
        }
    }

    async function addNewContact() {
        errorMessage = ''

        const contact: NewContact = {
            displayName,
            address,
            city,
            state,
            zipCode,
            ageRange,
            contactType,
            capacity,
            email,
            contactForm,
            isPrivate,
            genres: selectedGenres
        }

        let response = await post(ADD_CONTACT_URL, contact)
        errorMessage = handleResponse(
            response,
            "There was an error adding the contact. Please try again.",
            logout
        )

        if (response.status === 200) {
            successMessage = "Contact has successfully been added and will appear once approved."

            setTimeout(() => {
                dispatch('close')
            }, 1500)
        }
    }

    async function editContact() {
        errorMessage = ''
        let url = isAdmin() ? "/admin/edit-contact" : "/user/edit-contact"

        const edited: Contact = {
            contactId: $selectedContact.contactId,
            userId:    $selectedContact.userId,
            displayName,
            address,
            city,
            state,
            zipCode,
            ageRange,
            contactType,
            capacity: Number(capacity),
            email,
            contactForm,
            isPrivate,
            genres: selectedGenres
        }

        let response = await post(url, edited)
        errorMessage = handleResponse(
            response,
            "There was an error editing the contact.",
            logout
        )

        if (response.status === 200) {
            successMessage = "Contact has been updated."

            // TODO: update list of contacts

            setTimeout(() => {
                dispatch('update')
            }, 1000)
        }
    }

    function logout() {
        authenticated.update(() => false)
    }

    function populateFields(contact: Contact) {
        displayName    = contact.displayName
        address        = contact.address
        city           = contact.city
        state          = contact.state
        zipCode        = contact.zipCode
        ageRange       = contact.ageRange
        contactType    = contact.contactType
        capacity       = contact.capacity
        email          = contact.email
        contactForm    = contact.contactForm
        isPrivate      = contact.isPrivate
        // selectedGenres = contact.genres.map(g => g.genreId) // doesn't work.
    }

    onMount(() => {
        if ($genres.length === 0) {
            getGenres()
        }
        if (action === 'edit' && $selectedContact !== null) {
            populateFields($selectedContact)
        }     
    })
</script>

<h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">{ title }</h3>
{#if errorMessage}
    <ErrorMessage message={errorMessage} />
{/if}
{#if successMessage}
    <SuccessMessage message={successMessage} />
{/if}
<Label class="space-y-2">
    <span>Display Name</span>
    <Input type="text" name="displayName" placeholder="" bind:value={displayName} required />
</Label>
<Label class="space-y-2">
    <span>Address</span>
    <Input type="text" name="address" placeholder="666 aska punk ave" bind:value={address}/>
</Label>
<Label class="space-y-2">
    <span>City</span>
    <Input type="text" name="city" placeholder="" bind:value={city} required/>
</Label>
<Label>State/Province
    <Select class="mt-2" items={states} bind:value={state} required/>
</Label>
<Label class="space-y-2">
    <span>Zip code</span>
    <Input type="text" name="zipCode" placeholder="" bind:value={zipCode} />
</Label>
<Label class="space-y-2">
    <span>Genres</span>
    <!-- BUG: <MultiSelect> doesn't work within <Label> -->
</Label>
<MultiSelect items={formattedGenres} bind:value={selectedGenres} size="lg" />
<Label>Age range
    <Select class="mt-2" items={ageRanges} bind:value={ageRange} required/>
</Label>
<Label>Type of contact
    <Select class="mt-2" items={contactTypes} bind:value={contactType} required/>
</Label>
<Label class="space-y-2">
    <span>Capacity</span>
    <Input type="number" name="capacity" placeholder="" bind:value={capacity} />
</Label>
<Label class="space-y-2">
    <span>Email</span>
    <Input type="email" name="email" placeholder="" bind:value={email} />
</Label>
<Label class="space-y-2">
    <span>Contact link</span>
    <Input type="text" name="contactForm" placeholder="" bind:value={contactForm} />
</Label>
<Checkbox checked={isPrivate}>Contact is private</Checkbox>

<Button class="w-full1" on:click={submit}>{ button }</Button>