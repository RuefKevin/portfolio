<script lang="ts">
	const skills = {
		languages: ['Rust', 'Python', 'Java', 'C++', 'SQL'],
		backend: ['Axum', 'SQLx', 'REST API', 'Migrations'],
		frontend: ['SvelteKit', 'Vite', 'Vitest'],
		devops: ['Docker', 'GitHub Actions', 'CI/CD', 'Dependabot'],
		security: ['OWASP', 'Security Headers', 'CSP', 'CORS', 'cargo-audit', 'Gitleaks']
	};
</script>

<div class="terminal">
	<div class="block">
		<div class="line">
			<span class="prompt">❯</span>
			<span class="cmd">ls <span class="flag">-la</span> skills/</span>
		</div>
	</div>

	{#each Object.entries(skills) as [category, items]}
		<div class="category">
			<div class="category-head">
				<span class="folder">drwxr-xr-x</span>
				<span class="folder-name">{category}/</span>
			</div>
			<div class="items">
				{#each items as skill}
					<span class="skill">{skill}</span>
				{/each}
			</div>
		</div>
	{/each}

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

	.category {
		margin-bottom: 1.25rem;
	}

	.category-head {
		display: flex;
		align-items: baseline;
		gap: 1rem;
		margin-bottom: 0.4rem;
	}

	.folder {
		color: var(--text-dim);
		font-size: 0.8rem;
	}

	.folder-name {
		color: var(--yellow);
		font-weight: 600;
	}

	.items {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.skill {
		font-size: 0.75rem;
		color: var(--text-muted);
		border: 1px solid var(--border);
		padding: 0.2rem 0.6rem;
		transition: all 0.15s;
	}

	.skill:hover {
		color: var(--accent);
		border-color: var(--accent);
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
