<script lang="ts">
	import { onMount } from 'svelte';

	let typed1 = $state('');
	let typed2 = $state('');
	let showWhoami = $state(false);
	let showReadme = $state(false);
	let showCursor = $state(true);

	const cmd1 = 'whoami';
	const cmd2 = 'cat README.md';

	function typeCommand(cmd: string, onDone: () => void): () => void {
		let i = 0;
		const interval = setInterval(() => {
			if (cmd === cmd1) typed1 = cmd.slice(0, i + 1);
			else typed2 = cmd.slice(0, i + 1);
			i++;
			if (i === cmd.length) {
				clearInterval(interval);
				setTimeout(onDone, 300);
			}
		}, 80);
		return () => clearInterval(interval);
	}

	onMount(() => {
		const stop1 = typeCommand(cmd1, () => {
			showWhoami = true;
			setTimeout(() => {
				typeCommand(cmd2, () => {
					showReadme = true;
				});
			}, 400);
		});
		return stop1;
	});
</script>

<div class="terminal">
	<div class="block">
		<div class="line">
			<span class="prompt">❯</span>
			<span class="cmd">{typed1}</span>
			{#if typed1.length < cmd1.length}
				<span class="cursor"></span>
			{/if}
		</div>
		{#if showWhoami}
			<div class="output hi">Kevin Ruef</div>
		{/if}
	</div>

	{#if showWhoami}
		<div class="block">
			<div class="line">
				<span class="prompt">❯</span>
				<span class="cmd">{typed2}</span>
				{#if typed2.length > 0 && typed2.length < cmd2.length}
					<span class="cursor"></span>
				{/if}
			</div>

			{#if showReadme}
				<div class="readme">
					<div class="readme-section">
						<span class="md-h2">## </span>
						<span class="md-heading">Studium</span>
					</div>
					<div class="readme-body">
						<span class="md-bullet">-</span> Informatik B.Sc. · Technische Hochschule Augsburg
					</div>
					<div class="readme-body">
						<span class="md-bullet">-</span> 4. Semester · Schwerpunkt Softwareentwicklung
					</div>

					<div class="readme-section">
						<span class="md-h2">## </span>
						<span class="md-heading">Interessen</span>
					</div>
					<div class="readme-body">
						<span class="md-bullet">-</span> Backend-Entwicklung & Systemarchitektur
					</div>
					<div class="readme-body">
						<span class="md-bullet">-</span> Web Security & sichere Softwareentwicklung
					</div>
					<div class="readme-body">
						<span class="md-bullet">-</span> Open Source & Developer Tooling
					</div>

					<div class="readme-section">
						<span class="md-h2">## </span>
						<span class="md-heading">Aktuell</span>
					</div>
					<div class="readme-body">
						<span class="md-bullet">-</span> Lerne Rust & vertiefe Kenntnisse in DevOps
					</div>
					<div class="readme-body">
						<span class="md-bullet">-</span> Baue dieses Portfolio
					</div>
				</div>
			{/if}
		</div>
	{/if}

	{#if showReadme}
		<div class="block last">
			<div class="line">
				<span class="prompt">❯</span>
				{#if showCursor}
					<span class="cursor"></span>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
	.terminal {
		font-size: 0.85rem;
	}

	.block {
		margin-bottom: 1rem;
	}

	.last {
		margin-top: 0.5rem;
	}

	.line {
		display: flex;
		align-items: center;
		gap: 0.6rem;
		margin-bottom: 0.3rem;
	}

	.prompt {
		color: var(--accent);
		flex-shrink: 0;
	}

	.cmd {
		color: var(--text);
	}

	.output {
		padding-left: 1.2rem;
	}

	.output.hi {
		color: var(--accent);
		font-weight: 600;
		font-size: 1rem;
	}

	.cursor {
		display: inline-flex;
		width: 7px;
		height: 14px;
		background: var(--accent);
		animation: blink 1s step-end infinite;
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

	.readme {
		padding-left: 1.2rem;
		margin-top: 0.75rem;
		border-left: 2px solid var(--border);
		padding-top: 0.25rem;
		padding-bottom: 0.25rem;
	}

	.readme-section {
		margin-top: 1rem;
		margin-bottom: 0.3rem;
		line-height: 1.8;
	}

	.readme-body {
		line-height: 1.8;
		padding-left: 0.5rem;
		color: var(--text-muted);
	}

	.md-h2 {
		color: var(--blue);
		font-weight: 600;
	}

	.md-heading {
		color: var(--text);
		font-weight: 600;
	}

	.md-bullet {
		color: var(--text-dim);
		margin-right: 0.5rem;
	}
</style>
