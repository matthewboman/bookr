<script lang="ts">
	import { onMount } from 'svelte'
    
    export let filteredContacts: any[]
    export let contactList: any[]

    // Filters
    let minCapacity         = 0
    let maxCapacity         = 1000
    let allowNullCapacity   = true
    let allAges             = true
    let eighteenPlus        = true
    let twentyonePlus       = true
    let allowNullAgeRange   = true


    const capacityFilter = (contact: any) => {
        if (allowNullCapacity && contact.capacity == null) return true
        if (!allowNullCapacity && contact.capacity == null) return false

        if (contact.capacity >= minCapacity && contact.capacity <= maxCapacity) return true
        
        return false
    }

    const ageRangeFilter = (contact: any) => {
        if (allAges && contact.ageRange == 'all') return true
        if (eighteenPlus && contact.ageRange == '18+') return true
        if (twentyonePlus && contact.ageRange == '21+') return true

        if (allowNullAgeRange && contact.ageRange == null) return true

        return false
    }

    function update() {
        filteredContacts = contactList
            .filter(c => capacityFilter(c))
            .filter(c => ageRangeFilter(c))
    }

    // Workaround to reload when data is fetched
    onMount(() => {
        setTimeout(()=> { update() }, 1) 
    })
</script>

<style>
    .filter-container {
        display: flex;
        flex-direction: row;
		font-family: sans-serif;

        /* display: block; */
        margin-left: auto;
        margin-right: auto;

        width: 90vw;
    }
    .filter {
        padding: 10px;
        flex-grow: 1;
    }
    .filter-title {
        margin-bottom: 10px;
    }
    .filter-input {
        margin-bottom: 0.5em;
    }

    /* Checkboxes */
    .checkbox-container {
        line-height: 1.1;
        display: grid;
        grid-template-columns: 1em auto;
        gap: 0.5em;
    }
    .checkbox-container:hover {
        cursor: pointer;
    }
    input[type="checkbox"] {
        -webkit-appearance: none;
        appearance: none;
        background-color: #fff;
        margin: 0;
        font: inherit;
        color: currentColor;
        width: 1.15em;
        height: 1.15em;
        border: 0.1em solid currentColor;
        border-radius: 0.15em;
        transform: translateY(-0.075em);
        display: grid;
        place-content: center;
    }
    input[type="checkbox"]::before {
        content: "";
        width: 0.65em;
        height: 0.65em;
        transform: scale(0);
        transition: 120ms transform ease-in-out;
        box-shadow: inset 1em 1em rgb(49, 46, 232);
        background-color: CanvasText;
    }
    input[type="checkbox"]:checked::before {
        transform: scale(1);
    }
    input:focus {
        outline: max(2px, 0.15em) solid grey;
        outline-offset: max(2px, 0.15em);
    }
</style>

<div class="filter-container">
    <div class="filter">
        <div class="filter-title">
            Filter venues by capacity
        </div>
        <div class="filter-input">
            <label for="min">min</label>
            <input id="min" type="number" bind:value={minCapacity} on:change={update}/>
        </div>
        <div class="filter-input">
            <label for="max">max:</label>
            <input id="max" type="numer" bind:value={maxCapacity} on:change={update}/>       
        </div>
        <div class="filter-input">
            <label for="null-capacity" class="checkbox-container">
                <input id="null-capacity" type="checkbox" bind:checked={allowNullCapacity} on:change={update}/>                     
                allow for venues with unknown capacity
            </label>
        </div>
    </div>
    <div class="filter">
        <div class="filter-title">
            Filter venues by age range
        </div>
        <div class="filter-input">
            <label class="checkbox-container">
                <input type=checkbox class="checkbox" bind:checked={allAges} on:change={update}>
                All Ages
            </label>
        </div>
        <div class="filter-input">
            <label class="checkbox-container">
                <input type=checkbox class="checkbox" bind:checked={eighteenPlus} on:change={update}>
                18+
            </label>
        </div>
        <div class="filter-input">
            <label class="checkbox-container">
                <input type=checkbox class="checkbox" bind:checked={twentyonePlus} on:change={update}>
                21+
            </label>
        </div>
        <div class="filter-input">
            <label class="checkbox-container">
                <input type=checkbox class="checkbox" bind:checked={allowNullAgeRange} on:change={update}>
                allow for venues with unknown age range
            </label>
        </div>
    </div>
</div>