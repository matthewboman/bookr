<script lang="ts">
    import { Button, Label, Input, Checkbox, Select } from 'flowbite-svelte'
    import { type NewContact } from '../types'
    import { post } from '../api';

    // TODO: move to store in case modal gets closed while entering info

    // TODO: move to store or library of constants 
    const states = [
        { name: "North Carolina", value: "NC" }
    ]
    const ageRanges = [
        { name: "All Ages", value: "allAges" },
        { name: "18+",      value: "eighteenPlus" },
        { name: "21+",      value: "twentyonePlus" },
        { name: "Unknown",  value: "unknown" }
    ]

    let currentAction = 'add' // TODO: ability to edit contacts
    let errorText     = ''

    let displayName: string
    let address: string | null
    let city: string
    let state: string
    let zipCode: string | null
    let ageRange: string = "unknown"
    let capacity: number | null
    let email: string | null
    let contactForm: string | null
    let isPrivate: boolean = false 

    async function submit() {
        if (currentAction === 'add') {
            await addNewContact()
        } else if (currentAction === 'edit') {
            // TODO
        } else {
            // This shouldn't happen
        }
    }

    async function addNewContact() {
        errorText = ''

        const contact: NewContact = {
            displayName,
            address,
            city,
            state,
            zipCode,
            ageRange,
            capacity: 10,
            email,
            contactForm,
            isPrivate
        }

        const res = await post("/user/add-contact", contact)

        if (res.status === 200) {
            // TODO: success message
            // TODO: add contact to list
            
            // const json = await res.json() // TODO: update server to return contact

            console.log(res)
        }

        if (res.status === 400 || res.status === 401) {
            // TODO: log in
            // shouldn't happen but user could try faking token
        }

        if (res.status === 500) {
            // TODO: log error
        }
    }

</script>

<form class="flex flex-col space-y-6" on:submit|preventDefault={submit}>
    <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Add new contact</h3>
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
        <Input type="text" name="city" placeholder="" required bind:value={city}/>
    </Label>
    <Label>State/Province
        <Select class="mt-2" items={states} bind:value={state} required/>
    </Label>
    <Label class="space-y-2">
        <span>Zip code</span>
        <Input type="text" name="zipCode" placeholder="" bind:value={zipCode} required />
    </Label>
    <Label>Age range
        <Select class="mt-2" items={ageRanges} bind:value={ageRange} required/>
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
    <Checkbox checked={isPrivate}>Default checkbox</Checkbox>

    <Button type="submit" class="w-full1">Add contact</Button>
</form>