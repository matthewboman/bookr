<script lang="ts">
    import { afterUpdate, createEventDispatcher } from 'svelte'

    import Star from './Star.svelte'

    export let active: boolean
    export let color: string = 'yellow'
    export let currentRating: number
    export let size: number = 20

    const dispatch = createEventDispatcher()

    let rating:      number
    let hoverRating: number
    let percent:     number
    const stars = [
        { id: 1, title: 'One star' },
        { id: 2, title: 'Two stars' },
        { id: 3, title: 'Three stars' },
        { id: 4, title: 'Four stars' },
        { id: 5, title: 'Five stars' },
    ]

    const handleHover = (id: number) => () => {
        if (active) {
            hoverRating = id
        } 
    }

    const handleRate = (id: number) => (event: Event) => {
        if (active) {
            rating = id
            dispatch('updateRating', { rating })
        }
    }

    function isPartial(id: number, percent: number): boolean {
        if (percent === 0 ) {
            return false
        }
        if (Math.ceil(currentRating) > id ) {
            return false
        }
        return Math.floor(currentRating) === Math.floor(id - percent)
    }

    afterUpdate(() => {
        if (active) {
            rating = currentRating
        }
    })

    $: {
        if (!active) {
            rating  = currentRating
            percent = Math.floor((currentRating - Math.floor(currentRating)) * 100) / 100
        }
    }
</script>

<div class="flex">
    {#each stars as star (star.id)}
        <Star
            filled={hoverRating ? (hoverRating >= star.id) : (rating >= star.id)}
            partial={isPartial(star.id, percent)}
            percent={percent}
            starId={star.id}
            color={color}
            size={size}
            on:mouseover={handleHover(star.id)}
            on:mouseleave={() => hoverRating = 0}
            on:click={handleRate(star.id)}
        />
    {/each}
</div>