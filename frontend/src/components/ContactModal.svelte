<script lang="ts">
    import { Button, Label, Input, Checkbox, Select } from 'flowbite-svelte'
    // import { NewContact } from '../types'

    export let apiUrl

    // TODO: ability to edit contacts
    let currentAction = 'add'
    let state
    let ageRange

    // move to store or library of constants 
    let states = [
        { name: "North Carolina", value: "NC" }
    ]
    let ageRanges = [
        { name: "All Ages", value: "allAges" },
        { name: "18+",      value: "eighteenPlus" },
        { name: "21+",      value: "twentyonePlus" },
        { name: "Unknown",  value: "unknown" }
    ]

    const submit = () => {
        console.log(`${apiUrl}/new-contact`)
    }
</script>

<form class="flex flex-col space-y-6" action="#">
    <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">Add new contact</h3>
    <Label class="space-y-2">
        <span>Display Name</span>
        <Input type="text" name="displayName" placeholder="" />
    </Label>
    <Label class="space-y-2">
        <span>Address</span>
        <Input type="text" name="address" placeholder="666 aska punk ave" />
    </Label>
    <Label class="space-y-2">
        <span>City</span>
        <Input type="text" name="city" placeholder="" required />
    </Label>
    <Label>State/Province
        <Select class="mt-2" items={states} bind:value={state} required/>
    </Label>
    <Label class="space-y-2">
        <span>Zip code</span>
        <Input type="number" name="zipCode" placeholder="" required />
    </Label>
    <Label>Age range
        <Select class="mt-2" items={ageRanges} bind:value={ageRange} required/>
    </Label>
    <Label class="space-y-2">
        <span>Capacity</span>
        <Input type="number" name="capacity" placeholder="" />
    </Label>
    <Label class="space-y-2">
        <span>Email</span>
        <Input type="email" name="email" placeholder="name@company.com" />
    </Label>
    <Label class="space-y-2">
        <span>Contact link</span>
        <Input type="text" name="contactLink" placeholder="" />
    </Label>

    <Button type="submit" on:click={submit} class="w-full1">Add contact</Button>
</form>