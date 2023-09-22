<script lang="ts">
	import { onMount } from 'svelte'
    import { Alert, Button, Checkbox, Input, Label, MultiSelect, Search } from 'flowbite-svelte'

    import { post }               from '../api'
    import { handleResponse }     from '../functions'
    import { genres, mapOptions } from '../store'
    import type { Contact }       from '../types'
    
    export let filteredContacts: Contact[]
    export let contactList: Contact[]

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
    // Genre
    let formattedGenres     = $genres.map(g => ({ value: g.genreId, name: g.genreName }))
    let selectedGenres      = formattedGenres.map(g => g.value)
    // City search
    let city = ''

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

    async function searchCity() {
        if (city.length < 1) {return}

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

<div>
    {#if errorMessage}
        <Alert border color="red">
            <svg slot="icon" aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>
            <span class="font-medium">Error</span> { errorMessage }
        </Alert>
    {/if}
    <h3 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">Find venues in city</h3>
    <Search bind:value={city}>
        <Button on:click={searchCity}>Search</Button>
    </Search>
</div>