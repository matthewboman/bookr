<script lang="ts">
    import Reviews         from "./Reviews.svelte"
    import ContactAddress  from "./contact/ContactAddress.svelte"
    import ContactCapacity from './contact/ContactCapacity.svelte'
    import ContactLinks    from './contact/ContactLinks.svelte'
    import ContactName     from "./contact/ContactName.svelte"
    import { contactReviews, selectedContact } from "../store"
</script>

<div>
    <ContactName 
        className={'flex'}
        name={$selectedContact.displayName} 
        rating={$selectedContact.averageRating} 
        starColor={'red'}
        starSize={25}
    />
    <ContactAddress
        className={'mb-2'}
        contact={$selectedContact}
    />
    <ContactCapacity
        className={''}
        capacity={$selectedContact.capacity}
    />
    <ContactLinks
        className={''}
        link={$selectedContact.contactForm}
        email={$selectedContact.email}
    />
</div>

<Reviews reviews={$contactReviews}/>