import { redirect } from '@sveltejs/kit';

export async function load({ locals }) {
    if (!locals.user) {
        throw redirect(302, '/login');
    }
    
    const handle = locals.user.handle || 'user';
    throw redirect(302, `/u/${handle}`);
}