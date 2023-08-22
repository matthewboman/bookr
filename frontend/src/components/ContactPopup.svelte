<script lang="ts">
    export let contact: any
</script>

<style>
    .contact {

        font-family: Comfortaa;
        /* background-color: rgba(31, 41, 55, 0.9); */
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

<div class="contact">
    <div class="contact-name">
        {contact.displayName}
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
