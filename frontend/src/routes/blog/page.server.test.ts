import { describe, it, expect, vi } from 'vitest';
import { load } from './+page.server';

vi.mock('$env/dynamic/private', () => ({
	env: { BACKEND_URL: 'http://mocked-backend:3000' }
}));

vi.mock('$lib/logger', () => ({
	logRequest: vi.fn(),
	logError: vi.fn()
}));

describe('Blog Page Server Load', () => {
	it('fetches blog posts from backend and returns them (Happy Path)', async () => {
		const mockPosts = [
			{
				id: 1,
				title: 'Test Post',
				slug: 'test-post',
				content: 'Test content',
				published_at: '2026-03-10'
			}
		];

		const mockFetch = vi.fn().mockResolvedValue({
			status: 200,
			ok: true,
			json: async () => mockPosts
		});

		const mockEvent = { fetch: mockFetch } as unknown as Parameters<typeof load>[0];
		const result = await load(mockEvent);

		if (!result) throw new Error('Expected load to return data');

		expect(mockFetch).toHaveBeenCalledWith('http://mocked-backend:3000/api/blog');
		expect(result.posts).toEqual(mockPosts);
		expect(result.error).toBe(false);
	});

	it('returns empty array on fetch error (Resilience / Fail-Safe)', async () => {
		const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

		const mockFetch = vi.fn().mockRejectedValue(new Error('Backend offline'));
		const mockEvent = { fetch: mockFetch } as unknown as Parameters<typeof load>[0];

		const result = await load(mockEvent);

		if (!result) throw new Error('Expected load to return a fallback object');

		expect(result.posts).toEqual([]);
		expect(result.error).toBe(true);

		consoleSpy.mockRestore();
	});

	it('returns error flag on non-ok response', async () => {
		const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

		const mockFetch = vi.fn().mockResolvedValue({
			status: 500,
			ok: false
		});

		const mockEvent = { fetch: mockFetch } as unknown as Parameters<typeof load>[0];
		const result = await load(mockEvent);

		if (!result) throw new Error('Expected load to return a fallback object');

		expect(result.posts).toEqual([]);
		expect(result.error).toBe(true);

		consoleSpy.mockRestore();
	});
});
