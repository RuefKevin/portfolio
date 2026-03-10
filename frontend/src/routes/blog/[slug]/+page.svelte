<script lang="ts">
	import type { PageData } from './$types';
	let { data }: { data: PageData } = $props();
</script>

<div class="terminal">

	<div class="block">
		<div class="line">
			<span class="prompt">❯</span>
			<span class="cmd">cat <span class="arg">{data.post.slug}.md</span></span>
		</div>
	</div>

	<div class="post">
		<div class="post-meta">
			<span class="meta-key">date</span>
			<span class="meta-sep">:</span>
			<span class="meta-value">{data.post.published_at}</span>
		</div>

		<div class="post-title"># {data.post.title}</div>

		<div class="post-content">{data.post.content}</div>
	</div>

	<div class="block last">
		<div class="line">
			<span class="prompt">❯</span>
			<span class="cmd">cd <a href="/blog">..</a></span>
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

	.post {
		padding-left: 1.2rem;
		margin-bottom: 1.5rem;
		border-left: 2px solid var(--border);
		padding-top: 0.5rem;
		padding-bottom: 0.5rem;
	}

	.post-meta {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1rem;
		font-size: 0.78rem;
	}

	.meta-key {
		color: var(--blue);
	}

	.meta-sep {
		color: var(--text-muted);
	}

	.meta-value {
		color: var(--text-muted);
	}

	.post-title {
		color: var(--text);
		font-weight: 700;
		font-size: 1rem;
		margin-bottom: 1rem;
	}

	.post-content {
		color: var(--text-muted);
		line-height: 1.8;
		white-space: pre-wrap;
	}
</style>