<script lang="ts">
    import type { Genre } from "../../types"
    export let className: string
    export let genres:    Genre[]
</script>

<div class={className}>
    {#each genres as genre}
        <div>
            { genre.genreName }
        </div>
    {/each}
</div>