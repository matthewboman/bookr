<script lang="ts">
    import StarRating from './StarRating.svelte'

    export let contact: any
</script>

<div class="contact">
    <div class="contact-name">
        {contact.displayName}
        {#if contact.averageRating}
            <StarRating currentRating={contact.averageRating} active={false} color='blue'/>
        {/if}
    </div>

    <div class="contact-address">
        {#if contact.address }
            {(contact.address)}
            <br />
        {/if}     
        {contact.city}, {contact.state} {#if contact.zipCode }{contact.zipCode}{/if}
    </div>

    <div class="venue-capacity">
        Capacity: 
        {#if contact.capacity}
            {contact.capacity}
        {:else}
            <span class="unknown">unknown</span>
        {/if}
    </div>
 
    {#if contact.contactForm}
    <div class="contact-info">
        <a target="_blank" href="{contact.contactForm}">
            {contact.contactForm}
        </a>
    </div>
    {/if}
</div>

<style>
    .contact {

    }

    .contact-name {
        font-size: 1.2em;
        margin-bottom: 6px;
    }

    .contact-address {
        margin-bottom: 6px;
    }

    .venue-capacity {
        margin-bottom: 6px;
    }

    .unknown {
        color: grey;
    }

    .contact-info {

    }
</style>