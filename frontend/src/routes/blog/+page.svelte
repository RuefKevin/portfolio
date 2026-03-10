<script lang="ts">
	import type { PageData } from './$types';
	let { data }: { data: PageData } = $props();
</script>

<div class="terminal">
	<div class="block">
		<div class="line">
			<span class="prompt">❯</span>
			<span class="cmd">ls <span class="flag">-la</span> posts/</span>
		</div>
	</div>

	{#if data.error}
		<div class="output error">error: could not connect to backend</div>
	{:else if data.posts.length === 0}
		<div class="output muted">// no posts found</div>
	{:else}
		{#each data.posts as post (post.id)}
			<div class="post-line">
				<span class="permissions">-rw-r--r--</span>
				<span class="date">{post.published_at}</span>
				<a class="post-title" href="/blog/{post.slug}">{post.slug}.md</a>
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

	.post-line {
		display: flex;
		align-items: baseline;
		gap: 1.5rem;
		padding: 0.3rem 0;
	}

	.permissions {
		color: var(--text-dim);
		font-size: 0.75rem;
		flex-shrink: 0;
	}

	.date {
		color: var(--text-muted);
		font-size: 0.78rem;
		flex-shrink: 0;
	}

	.post-title {
		color: var(--accent);
		text-decoration: none;
	}

	.post-title:hover {
		text-decoration: underline;
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
