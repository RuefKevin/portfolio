<script lang="ts">
	import type { PageData } from './$types';
	let { data }: { data: PageData } = $props();
</script>

<h1>Projects</h1>

{#if data.error}
    <p>Projects could not be loaded. Please try again later.</p>
{:else if data.projects.length === 0}
    <p>No projects yet.</p>
{:else}
    {#each data.projects as project (project.id)}
        <div>
            <h2>{project.name}</h2>
            <p>{project.description}</p>
            <p>{project.technologies.join(', ')}</p>
        </div>
    {/each}
{/if}
