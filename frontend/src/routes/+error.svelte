<script lang="ts">
	import { page } from '$app/state';
</script>

<div class="terminal">
	<div class="block">
		<div class="line">
			<span class="prompt">❯</span>
			<span class="cmd">cat <span class="arg">error.log</span></span>
		</div>
	</div>

	<div class="error-block">
		<div class="error-line">
			<span class="key">status</span>
			<span class="colon">:</span>
			<span class="status">{page.status}</span>
		</div>
		<div class="error-line">
			<span class="key">message</span>
			<span class="colon">:</span>
			<span class="message">"{page.error?.message ?? 'unknown error'}"</span>
		</div>
	</div>

	<div class="block">
		<div class="line">
			<span class="prompt">❯</span>
			<span class="cmd">cd <a href="/">~</a></span>
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

	.arg {
		color: var(--yellow);
	}

	.error-block {
		padding-left: 1.2rem;
		margin-bottom: 1.5rem;
		border-left: 2px solid var(--red);
		padding-top: 0.25rem;
		padding-bottom: 0.25rem;
	}

	.error-line {
		display: flex;
		gap: 0.75rem;
		line-height: 1.9;
	}

	.key {
		color: var(--blue);
	}

	.colon {
		color: var(--text-muted);
	}

	.status {
		color: var(--red);
		font-weight: 600;
	}

	.message {
		color: var(--text-muted);
	}
</style>
