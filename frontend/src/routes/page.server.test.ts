import { describe, it, expect, vi } from 'vitest';
import { load } from './+page.server';
import { BACKEND_URL } from '$env/static/private';
import type { ServerLoadEvent } from '@sveltejs/kit';

vi.mock('$env/static/private', () => 
({
    BACKEND_URL: 'http://mocked-backend:3000'
}));

describe('Portfolio Page Server Load', () =>
{
    it('fetches projects from backend and returns them (Happy Path)', async () =>
    {
        const mockProjects =
        [{
            id: 1, name: 'Mocked Sec-Portfolio', description: 'Test', technologies: ['Svelte']
        }];

        const mockFetch = vi.fn().mockResolvedValue
        ({
            status: 200,
            json: async () =>
                mockProjects
        });

        const mockEvent = {fetch: mockFetch} as unknown as Parameters<typeof load>[0];

        const result = await load(mockEvent);

        if (!result) throw new Error('Expected load to return data');

        expect(mockFetch).toHaveBeenCalledWith('http://mocked-backend:3000/api/projects');

        expect(result.projects).toEqual(mockProjects);
    });

    it('returns empty array on fetch error (Resilience / Fail-Safe)', async () =>
    {
        const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

        const mockFetch = vi.fn().mockRejectedValue(new Error('Backend offline'));

        const mockEvent = {fetch: mockFetch} as unknown as Parameters<typeof load>[0];

        const result = await load(mockEvent);

        if (!result) throw new Error('Expected load to return a fallback object');
        
        expect(result.projects).toEqual([]);

        expect(consoleSpy).toHaveBeenCalled();

        consoleSpy.mockRestore();
    });
})
