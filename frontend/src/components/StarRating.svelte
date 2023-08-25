<script lang="ts">
    import { onMount, createEventDispatcher } from 'svelte'

    import Star from './Star.svelte'

    export let currentRating: number
    export let active: boolean

    const dispatch = createEventDispatcher()

    let rating: number
    let hoverRating: number
    const stars = [
        { id: 1, title: 'One star' },
        { id: 2, title: 'Two stars' },
        { id: 3, title: 'Three stars' },
        { id: 4, title: 'Four stars' },
        { id: 5, title: 'Five stars' },
    ]

    const handleHover = (id: number, half: boolean) => () => {
        if (active) {
            hoverRating = half ? (id + 0.5) : id
        } 
    }

    const handleRate = (id: number) => (event: Event) => {
        if (active) {
            rating = id
            dispatch('updateRating', {
                rating
            })
        }
    }

    onMount(() => {
        rating = currentRating
    })
</script>

<style>
    .star-container {
		display: flex;
	}
</style>

<div class="star-container">
    {#each stars as star (star.id)}
        <Star
            filled={hoverRating ? (hoverRating >= star.id) : (rating >= star.id)}
            halfFilled={hoverRating ? (hoverRating === star.id - 0.5) : (rating === star.id - 0.5)}
            starId={star.id}
            on:mouseover={handleHover(star.id, false)}
            on:mouseleave={() => hoverRating = 0}
            on:click={handleRate(star.id)}
        />
    {/each}
</div>