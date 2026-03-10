<script lang="ts">
	import type { PageData } from './$types';
	let { data } = $props();
</script>

<div class="terminal">
	<div class="block">
		<div class="line">
			<span class="prompt">❯</span>
			<span class="cmd">cat <span class="arg">contact.json</span></span>
		</div>
	</div>

	<div class="json">
		<div class="json-line"><span class="bracket">{`{`}</span></div>
		{#each Object.entries(data.contact) as [label, val], i}
			<div class="json-line indent">
				<span class="json-key">"{label}"</span>
				<span class="colon">: </span>
				{#if label === 'email'}
					"<a class="json-value" href="mailto:{val}">{val}</a>"
				{:else if label === 'github'}
					"<a class="json-value" href="https://{val}" target="_blank" rel="noopener noreferrer"
						>{val}</a
					>"
				{:else if label === 'linkedin'}
					"<a class="json-value" href="https://{val}" target="_blank" rel="noopener noreferrer"
						>{val}</a
					>"
				{:else}
					<span class="json-value">"{val}"</span>
				{/if}
				{#if i < Object.entries(data.contact).length - 1}
					<span class="comma">,</span>
				{/if}
			</div>
		{/each}
		<div class="json-line"><span class="bracket">{`}`}</span></div>
	</div>

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

	.arg {
		color: var(--yellow);
	}

	.json {
		padding-left: 1.2rem;
		margin-bottom: 1rem;
	}

	.json-line {
		line-height: 1.9;
	}

	.indent {
		padding-left: 1.5rem;
	}

	.bracket {
		color: var(--text-muted);
	}

	.json-key {
		color: var(--blue);
	}

	.json-value {
		color: var(--accent);
	}

	a.json-value {
		text-decoration: none;
	}

	a.json-value:hover {
		text-decoration: underline;
	}

	.colon {
		color: var(--text-muted);
	}

	.comma {
		color: var(--text-muted);
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
