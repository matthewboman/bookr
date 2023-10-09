<script lang=ts>
    import { onMount } from 'svelte'

    export let color   = 'yellow'
    export let filled  = false
    export let partial = false
    export let percent: number
    export let size    = 20
    export let starId: number
    
    let percentFilled: number

    onMount(() => {
        if (partial && percent) {
            percentFilled = Math.floor(percent * 50)
        }
    })
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-mouse-events-have-key-events -->
<svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 50 40"
    class="star"
    on:mouseover on:mouseleave on:click data-starid={starId}
>
    <defs>
        {#if percentFilled}
            <clipPath id="half-clip">
                <rect x="0" y="0" width={percentFilled} height={`${2 * size}`} />
            </clipPath>
        {/if}
    </defs>
    <path
        class:filled={filled}
        fill={filled ? color : 'none'}
        class='star-path'
        d="m25,1 6,17h18l-14,11 5,17-15-10-15,10 5-17-14-11h18z"
    />
    {#if partial}
        <path
            class:filled={filled}
            class="star-path"
            fill={partial ? color : 'none'}
            d="m25,1 6,17h18l-14,11 5,17-15-10-15,10 5-17-14-11h18z"
            clip-path="url(#half-clip)"
        />
    {/if}
</svg>

<style>
    .star {
        margin-right: 4px;
    }
    .star-path {
        stroke: #999;
    }
</style>