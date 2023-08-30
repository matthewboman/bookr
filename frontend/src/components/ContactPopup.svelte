<script lang="ts">
    import ContactAddress  from './contact/ContactAddress.svelte'
    import ContactCapacity from './contact/ContactCapacity.svelte';
    import ContactLinks    from './contact/ContactLinks.svelte';
    import ContactName     from './contact/ContactName.svelte'

    export let contact: any
</script>

<div class="contact">
    <ContactName
        className={'flex justify-between'}
        name={contact.displayName} 
        rating={contact.averageRating} 
        starColor={'blue'}
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
    <ContactLinks
        className={''}
        link={contact.contactForm}
        email={contact.email}
    />
</div>