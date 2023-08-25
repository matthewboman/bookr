<script lang=ts>
    export let filled     = false
    export let halfFilled = false
    export let starId: number
</script>

<svg
    xmlns="http://www.w3.org/2000/svg"
    width="20"
    height="20"
    viewBox="0 0 51 48"
    class="star"
    on:mouseover on:mouseleave on:click data-starid={starId}
    let:class
>
    <defs>
        <clipPath id="half-clip">
            <rect x="0" y="0" width="25" height="48" />
        </clipPath>
    </defs>
    <path
        class:filled={filled}
        class="star-path"
        d="m25,1 6,17h18l-14,11 5,17-15-10-15,10 5-17-14-11h18z"
    />
    {#if halfFilled}
        <path
            class="half-filled star-path"
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
        fill: none;
        stroke: #999;
    }
    .filled {
        fill: yellow;
    }
    .half-filled {
        fill: yellow;
    }
</style>