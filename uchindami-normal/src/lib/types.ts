/**
 * Shared TypeScript types for the portfolio
 */

export interface SocialLink {
    name: string;
    url: string;
    ariaLabel: string;
}

export interface NavItem {
    name: string;
    link: string;
    icon?: React.ComponentType<{ className?: string }>;
}

export interface Project {
    id: number;
    title: string;
    description: string;
    image: ImageMetadata;
    tags: string[];
    links: {
        github: string;
        live: string;
    };
    className?: string;
    featured?: boolean;
}

export interface ImageMetadata {
    src: string;
    width: number;
    height: number;
    format: string;
}

export interface Stat {
    value: string;
    label: string;
}

export interface ContactFormData {
    name: string;
    email: string;
    project: string;
    message: string;
}

export interface BlogPost {
    frontmatter: BlogFrontmatter;
    url: string;
    file: string;
}

export interface BlogFrontmatter {
    title: string;
    pubDate: string | Date;
    description: string;
    author: string;
    tags: string[];
    image?: {
        url: string;
        alt: string;
    };
}

export interface ServiceItem {
    icon: React.ComponentType<{ className?: string }>;
    title: string;
    skills: SkillGroup[];
}

export interface SkillGroup {
    title: string;
    subtitle: string;
    skills: string[];
}
