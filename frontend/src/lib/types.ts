export interface Project {
	id: number;
	name: string;
	description: string;
	technologies: string[];
}

export interface BlogPost {
    id: number;
    title: string;
    slug: string;
    content: string;
    published_at: string;
}