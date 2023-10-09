<script lang="ts">
    import ContactAddress  from '../contact/ContactAddress.svelte'
    import ContactCapacity from '../contact/ContactCapacity.svelte';
    import ContactLinks    from '../contact/ContactLinks.svelte';
    import ContactName     from '../contact/ContactName.svelte'

    export let contact: any
</script>

<div class="contact">
    <ContactName
        className={'flex justify-between'}
        name={contact.displayName} 
        rating={contact.averageRating} 
        starColor={'yellow'}
        starSize={15}
    />
    <ContactAddress
        className={'mb-2'}
        contact={contact}
    />
    <ContactCapacity
        className={'mb-2'}
        capacity={contact.capacity}
    />
    <div class="mb-2">
        <a href={`/contact?contactId=${contact.contactId}`} class="text-orange-400 leaflet-link-override">Contact details</a>
    </div>
    <ContactLinks
        className={''}
        link={contact.contactForm}
        email={contact.email}
    />
</div>

<style>
    /* override .leaflect-container */
    .contact {
        font-family: Comfortaa;
    }

    .leaflet-link-override {
        color: rgb(255 138 76);
    }
</style>
