<script lang="ts">
	import type { PageData } from './$types';
	let { data }: { data: PageData } = $props();
</script>

<div class="terminal">
	<div class="block">
		<div class="line">
			<span class="prompt">❯</span>
			<span class="cmd"
				>gh repo list <span class="flag">--limit</span> <span class="arg">10</span></span
			>
		</div>
	</div>

	{#if data.error}
		<div class="output error">error: could not connect to backend</div>
	{:else if data.projects.length === 0}
		<div class="output muted">// no repositories found</div>
	{:else}
		{#each data.projects as project (project.id)}
			<div class="project">
				<div class="project-head">
					<span class="project-name">{project.name}</span>
					<span class="project-visibility">public</span>
				</div>
				<div class="project-desc">{project.description}</div>
				<div class="project-tags">
					{#each project.technologies as tech (tech)}
						<span class="tag">#{tech}</span>
					{/each}
				</div>
			</div>
		{/each}
	{/if}

	<div class="block last">
		<div class="line">
			<span class="prompt">❯</span>
			<span class="cursor"></span>
		</div>
	</div>
</div>

<style>
	.terminal {
		font-size: 0.85rem;
	}

	.block {
		margin-bottom: 1rem;
	}

	.last {
		margin-top: 1.5rem;
	}

	.line {
		display: flex;
		align-items: center;
		gap: 0.6rem;
	}

	.prompt {
		color: var(--accent);
		flex-shrink: 0;
	}

	.cmd {
		color: var(--text);
	}

	.flag {
		color: var(--blue);
	}

	.arg {
		color: var(--yellow);
	}

	.output {
		padding-left: 1.2rem;
		line-height: 1.8;
	}

	.output.muted {
		color: var(--text-muted);
	}

	.output.error {
		color: var(--red);
	}

	.project {
		border: 1px solid var(--border);
		background: var(--bg-subtle);
		padding: 0.9rem 1.1rem;
		margin-bottom: 0.5rem;
		transition: border-color 0.15s;
	}

	.project:hover {
		border-color: var(--accent);
	}

	.project-head {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 0.4rem;
	}

	.project-name {
		color: var(--accent);
		font-weight: 600;
	}

	.project-visibility {
		font-size: 0.65rem;
		color: var(--text-dim);
		border: 1px solid var(--border);
		padding: 0.1rem 0.4rem;
	}

	.project-desc {
		color: var(--text-muted);
		font-size: 0.78rem;
		line-height: 1.6;
		margin-bottom: 0.5rem;
	}

	.project-tags {
		display: flex;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.tag {
		font-size: 0.68rem;
		color: var(--blue);
	}

	.cursor {
		display: inline-block;
		width: 7px;
		height: 14px;
		background: var(--accent);
		animation: blink 1s step-end infinite;
		vertical-align: middle;
	}

	@keyframes blink {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0;
		}
	}
</style>
