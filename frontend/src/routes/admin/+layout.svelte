<script lang="ts">
    import { onMount } from 'svelte'
    import { goto }    from '$app/navigation'

    import AdminSidebar from '../../components/navigation/AdminSidebar.svelte'
    import { isAdmin, isAuthenticated, clearExpiredToken } from '../../functions'
    import { admin, authenticated } from "../../store"

    function authenticateAndAuthorize() {
        if (isAuthenticated()) {
            authenticated.update(() => true)
        } else {
            clearExpiredToken()
            authenticated.update(() => false)
            goto("/")
        }

        if (isAdmin()) {
            admin.update(() => true)
            return true
        } else {
            admin.update(() => false)
            goto("/")
        }
    }

    onMount(async () => {
        authenticateAndAuthorize()
    })
</script>

<div class="flex">
    <div class="min-h-screen">
        <AdminSidebar/>
    </div>
    <div class="flex-col mx-auto container">
        <slot/>
    </div>
</div>