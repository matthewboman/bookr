<script lang="ts">
    import { onMount } from 'svelte'
    import { goto }    from '$app/navigation'

    import UserSidebar from '../../components/navigation/UserSidebar.svelte'
    import { isAuthenticated, clearExpiredToken } from '../../functions'
    import { authenticated } from "../../store"

    function setAuthenticated() {
        if (isAuthenticated()) {
            authenticated.update(() => true)
        } else {
            clearExpiredToken()
            goto("/")
        }
    }

    onMount(async () => {
        setAuthenticated()
    })
</script>

<div class="flex">
    <div class="min-h-screen">
        <UserSidebar/>
    </div>
    <div class="flex-col mx-auto container">
        <slot/>
    </div>
</div>