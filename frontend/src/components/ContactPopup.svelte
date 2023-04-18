<script>
    export let contact
</script>

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

<div class="contact">
    <div class="contact-name">
        {contact.displayName}
    </div>

    <div class="contact-address">
        {contact.address}
        <br />
        {contact.city}, {contact.state} {contact.zipCode}
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