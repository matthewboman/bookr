<script lang="ts">
    import { createEventDispatcher } from 'svelte'
    import { ButtonGroup, Card, GradientButton} from 'flowbite-svelte'

    import type { Contact } from '../../types'
    import ContactAddress   from './ContactAddress.svelte'

    export let contact: Contact

    const dispatch = createEventDispatcher()

    function approveContact() {
        dispatch('approve', { contact })
    }

    function deleteContact() {
        dispatch('delete', { contact })
    }
</script>

<Card size="xl">
    <div class="flex justify-between">
        <div>
            { contact.displayName }
        </div>
        <ContactAddress contact={contact} className={''}/>
        <div class="">
            <ButtonGroup>
                <GradientButton shadow={true} color="green" on:click={approveContact}>Approve</GradientButton>
                <GradientButton shadow={true} color="red" on:click={deleteContact}>Delete</GradientButton>
            </ButtonGroup>
        </div>
    </div>
</Card>