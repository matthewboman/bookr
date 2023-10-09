<script lang="ts">
    import type { Genre } from "../../types"
    export let className: string
    export let genres:    Genre[]
</script>

<div class={className}>
    Genres:
    <ul>
        {#each genres as genre}  
            <li>{ genre.genreName }</li>
        {/each}
    </ul>
</div>