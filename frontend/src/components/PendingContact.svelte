<script lang="ts">
    import { createEventDispatcher } from 'svelte'
    import { ButtonGroup, Card, GradientButton} from 'flowbite-svelte'
    import type { Contact } from '../types'

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
        <div class="">
            { contact.displayName }
        </div>
        <div class="">
            { contact.address }
        </div>
        <div class="">
            { contact.city }
        </div>
        <div class="">
            { contact.state }
        </div>
        <div class="">
            { contact.zipCode }
        </div>
        <div class="">
            <ButtonGroup>
                <GradientButton shadow={true} color="green" on:click={approveContact}>Approve</GradientButton>
                <GradientButton shadow={true} color="red" on:click={deleteContact}>Delete</GradientButton>
            </ButtonGroup>
        </div>
    </div>
</Card>