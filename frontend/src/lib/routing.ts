export function userPath(handle: string): string {
    return `/u/${handle}`;
}

export function questPath(id: number | string, slug: string): string {
    return `/q/${id}-${slug}`;
}

export function slugifyQuestTitle(title: string): string {
    return title
        .toLowerCase()
        .replace(/[^a-z0-9\s-]/g, '')
        .trim()
        .replace(/\s+/g, '-')
        .replace(/-+/g, '-');
}

export function parseQuestPathParam(param: string): { id: string; slug: string } {
    const parts = param.split('-');
    if (parts.length < 2) {
        return { id: param, slug: '' };
    }
    
    const id = parts[0];
    const slug = parts.slice(1).join('-');
    
    return { id, slug };
}