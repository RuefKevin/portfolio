<script lang="ts">
    import type { PageData } from './$types';
    export let data: PageData;
</script>

<h1>Projects</h1>

{#each data.projects as project}
    <div>
        <h2>{project.name}</h2>
        <p>{project.description}</p>
        <p>{project.technologies.join(', ')}</p>
    </div>
{/each}
