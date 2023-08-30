<script lang="ts">
    export let className: string
    export let capacity:  number | null
</script>

<div class={className}>
    Capacity: 
    {#if capacity}
        { capacity }
    {:else}
        <span class="text-gray-500">unknown</span>
    {/if}
</div>