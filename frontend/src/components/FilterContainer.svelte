<script lang="ts">
	import { onMount } from 'svelte'
    import { Button, Checkbox, Input, Label, MultiSelect, Search } from 'flowbite-svelte'

    import { post }           from '../api'
    import { handleResponse } from '../functions'
    import { contactList, genres, mapOptions } from '../store'
    import type { Contact }   from '../types'
    import ErrorMessage       from './ui/ErrorMessage.svelte'
    
    export let filteredContacts: Contact[]

    const CITY_SEARCH_URL = "/find-coordinates-for-city"
    const CITY_ZOOM       = 12

    let errorMessage: string | null

    // Capacity
    let minCapacity         = 0
    let maxCapacity         = 1000
    let allowNullCapacity   = true
    // Age range
    let allAges             = true
    let eighteenPlus        = true
    let twentyonePlus       = true
    let allowNullAgeRange   = true
    // Contact type
    let venue               = true
    let promoter            = true
    let diy                 = true
    let band                = true
    // Genre
    let formattedGenres     = $genres.map(g => ({ value: g.genreId, name: g.genreName }))
    let selectedGenres      = formattedGenres.map(g => g.value)
    // City search
    let city = ''

    const capacityFilter = (contact: Contact): boolean => {
        if (allowNullCapacity && contact.capacity == null) return true
        if (!allowNullCapacity && contact.capacity == null) return false
        if (contact.capacity >= minCapacity && contact.capacity <= maxCapacity) return true
        
        return false
    }

    const ageRangeFilter = (contact: Contact): boolean => {
        if (allAges && contact.ageRange === 'all') return true
        if (eighteenPlus && contact.ageRange === '18+') return true
        if (twentyonePlus && contact.ageRange === '21+') return true
        if (allowNullAgeRange && contact.ageRange == null) return true

        return false
    }

    const genreFilter = (contact: Contact): boolean => {
        const contactGenres = contact.genres.map(g => g.genreId)
        return contactGenres.filter(g => selectedGenres.includes(g)).length > 0
    }

    const contactTypeFilter = (contact: Contact): boolean => {
        if (venue && contact.contactType === 'venue') return true
        if (promoter && contact.contactType === 'promoter') return true
        if (diy && contact.contactType === 'diy') return true
        if (band && contact.contactType === 'band') return true

        return false
    }

    async function searchCity() {
        if (city.length < 1) return

        let response = await post(CITY_SEARCH_URL, { city })
        errorMessage = handleResponse(
            response,
            `Unable to find ${city} on map`,
            Function() // `handleResponse` requires function as 3rd argument
        )

        if (response.status == 200) {
            let geo = await response.json()
            mapOptions.set({
                center: [ geo.lat, geo.lng],
                zoom: CITY_ZOOM
            })
        }
    }

    function update() {
        filteredContacts = $contactList
            .filter(c => capacityFilter(c))
            .filter(c => ageRangeFilter(c))
            .filter(c => genreFilter(c))
            .filter(c => contactTypeFilter(c))
    }

    // Workaround to reload when data is fetched
    onMount(() => {
        update()
    })
</script>

<div class="filter-block mb-5">
    <h3 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">Filter venues by capacity</h3>
    <div class="flex w-full gap-2">
        <Label class="space-y-2 mb-2 flex-grow">
            <span>Min</span>
            <Input type="number" name="displayName" placeholder="" bind:value={minCapacity} on:change={update}/>
        </Label>
        <Label class="space-y-2 mb-2 flex-grow">
            <span>Max</span>
            <Input type="number" name="displayName" placeholder="" bind:value={maxCapacity} on:change={update}/>
        </Label>
    </div>
    <Checkbox bind:checked={allowNullCapacity} on:change={update}>Allow for venues with unknown capacity</Checkbox>
</div>

<div class="filter-block mb-5">
    <h3 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">Filter contacts by age range</h3>
    <Checkbox bind:checked={allAges} on:change={update}>All ages</Checkbox>
    <Checkbox bind:checked={eighteenPlus} on:change={update}>18+</Checkbox>
    <Checkbox bind:checked={twentyonePlus} on:change={update}>21+</Checkbox>
    <Checkbox bind:checked={allowNullAgeRange} on:change={update}>Allow for venues with unknown age range</Checkbox>
</div>

<div class="filter-block mb-5">
    <h3 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">Filter contacts by type</h3>
    <Checkbox bind:checked={venue} on:change={update}>Venues</Checkbox>
    <Checkbox bind:checked={promoter} on:change={update}>Promoters</Checkbox>
    <Checkbox bind:checked={diy} on:change={update}>DIY spaces, houses, etc.</Checkbox>
    <Checkbox bind:checked={band} on:change={update}>Bands</Checkbox>
</div>

<div class="filter-block mb-5">
    <h3 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">Filter contacts by genre</h3>
    <MultiSelect items={formattedGenres} bind:value={selectedGenres} on:change={update} size="lg" />
    This button is a <a href="workaround https://github.com/themesberg/flowbite-svelte/pull/1081" target="_blank">workaround </a>
    <Button on:click={update}>update</Button>
</div>

<div class="filter-block mb-5">
    {#if errorMessage}
        <ErrorMessage message={errorMessage} />
    {/if}
    <h3 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">Find venues in city</h3>
    <Search bind:value={city}>
        <Button on:click={searchCity}>Search</Button>
    </Search>
</div>