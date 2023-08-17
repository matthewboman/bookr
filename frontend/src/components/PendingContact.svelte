<script>
    import { createEventDispatcher } from 'svelte'
    import { ButtonGroup, Card, GradientButton} from 'flowbite-svelte'

    export let contact

    const dispatch = createEventDispatcher()

    function approveContact() {
        dispatch('approve', {
            contact: contact
        })
    }

    function deleteContact() {
        dispatch('delete', {
            contact: contact
        })
    }
</script>

<Card size="xl">
    <div class="pending-contact">
        <div class="pending-name">
            { contact.displayName }
        </div>
        <div class="pending-address">
            { contact.address }
        </div>
        <div class="pending-city">
            { contact.city }
        </div>
        <div class="pending-state">
            { contact.state }
        </div>
        <div class="pending-zip">
            { contact.zipCode }
        </div>
        <div class="pending-buttons">
            <ButtonGroup>
                <GradientButton shadow="green" color="green" on:click={approveContact}>Approve</GradientButton>
                <GradientButton shadow="red" color="red" on:click={deleteContact}>Delete</GradientButton>
            </ButtonGroup>
        </div>
    </div>
</Card>


<style>
    .pending-contact {
        display: flex;
        justify-content: space-between;
    }

    .pending-name {

    }

    .pending-address {

    }

    .pending-city {

    }

    .pending-state {

    }

    .pending-zip {

    }

    .pending-buttons {

    }
</style>