<script lang="ts">
	import { onMount } from 'svelte'
    import { goto }    from '$app/navigation'

    import PasswordReset       from "../../../components/PasswordReset.svelte"
    import Menu                from '../../../components/Menu.svelte'
    import { isAuthenticated } from '../../../functions'
    import { authenticated }   from "../../../store"

    function setAuthenticated() {
        if (isAuthenticated()) {
            authenticated.update(() => true)
        } else {
            goto("/")
        }
    }

    onMount(() => {
        setAuthenticated()
    })
</script>

<Menu/>
<PasswordReset></PasswordReset>