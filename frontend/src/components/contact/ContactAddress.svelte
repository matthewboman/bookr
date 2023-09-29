<script lang="ts">
    import type { Contact } from '../../types'

    export let className: string
    export let contact:   Contact
</script>

<div class={className}>
    {#if contact.address }
        { contact.address }
        <br/>   
    {/if}
    { contact.city }, { contact.state } 
    {#if contact.zipCode }
        { contact.zipCode }
    {/if}
</div>