<script lang="ts">
	import { onMount } from 'svelte'
    import { Button, Label, Input, Checkbox, MultiSelect } from 'flowbite-svelte'

    import { genres }       from '../store'
    import type { Contact } from '../types'
    
    export let filteredContacts: Contact[]
    export let contactList: Contact[]

    // Filters
    let minCapacity         = 0
    let maxCapacity         = 1000
    let allowNullCapacity   = true
    let allAges             = true
    let eighteenPlus        = true
    let twentyonePlus       = true
    let allowNullAgeRange   = true
    let formattedGenres     = $genres.map(g => ({ value: g.genreId, name: g.genreName }))
    let selectedGenres      = formattedGenres.map(g => g.value)

    const capacityFilter = (contact: Contact) => {
        if (allowNullCapacity && contact.capacity == null) return true
        if (!allowNullCapacity && contact.capacity == null) return false

        if (contact.capacity >= minCapacity && contact.capacity <= maxCapacity) return true
        
        return false
    }

    const ageRangeFilter = (contact: Contact) => {
        if (allAges && contact.ageRange == 'all') return true
        if (eighteenPlus && contact.ageRange == '18+') return true
        if (twentyonePlus && contact.ageRange == '21+') return true

        if (allowNullAgeRange && contact.ageRange == null) return true

        return false
    }

    const genreFilter = (contact: Contact) => {
        const contactGenres = contact.genres.map(g => g.genreId)
        return contactGenres.filter(g => selectedGenres.includes(g)).length > 0
    }

    function update() {
        filteredContacts = contactList
            .filter(c => capacityFilter(c))
            .filter(c => ageRangeFilter(c))
            .filter(c => genreFilter(c))
    }

    // Workaround to reload when data is fetched
    onMount(() => {
        update()
    })
</script>

<div class="mb-4">
    <h3 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">Filter venues by capacity</h3>
    <Label class="space-y-2 mb-2">
        <span>Min</span>
        <Input type="number" name="displayName" placeholder="" bind:value={minCapacity} on:change={update}/>
    </Label>
    <Label class="space-y-2 mb-2">
        <span>Max</span>
        <Input type="number" name="displayName" placeholder="" bind:value={maxCapacity} on:change={update}/>
    </Label>
    <Checkbox bind:checked={allowNullCapacity} on:change={update}>Allow for venues with unknown capacity</Checkbox>
</div>

<div class="mb-4">
    <h3 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">Filter venues by age range</h3>
    <Checkbox bind:checked={allAges} on:change={update}>All ages</Checkbox>
    <Checkbox bind:checked={eighteenPlus} on:change={update}>18+</Checkbox>
    <Checkbox bind:checked={twentyonePlus} on:change={update}>21+</Checkbox>
    <Checkbox bind:checked={allowNullAgeRange} on:change={update}>Allow for venues with unknown age range</Checkbox>
</div>

<div>
    <h3 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">Filter venues by genre</h3>
    <MultiSelect items={formattedGenres} bind:value={selectedGenres} on:change={update} size="lg" />
    This button is a <a href="workaround https://github.com/themesberg/flowbite-svelte/pull/1081" target="_blank">workaround </a>
    <Button on:click={update}>update</Button>
</div>