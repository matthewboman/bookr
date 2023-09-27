<script lang="ts">
    import ContactAddress   from "./ContactAddress.svelte"
    import ContactCapacity  from './ContactCapacity.svelte'
    import ContactGenres    from "./ContactGenres.svelte"
    import ContactLinks     from './ContactLinks.svelte'
    import ContactName      from "./ContactName.svelte"
    import type { Contact } from '../../types'

    export let contact:   Contact
    export let className: string
</script>

<div class={className}>
    <ContactName 
        className={'flex'}
        name={contact.displayName} 
        rating={contact.averageRating} 
        starColor={'red'}
        starSize={25}
    />
    <ContactAddress
        className={'mb-2'}
        contact={contact}
    />
    <ContactCapacity
        className={''}
        capacity={contact.capacity}
    />
    <ContactLinks
        className={''}
        link={contact.contactForm}
        email={contact.email}
    />
    <ContactGenres
        className={''}
        genres={contact.genres}
    />
</div>