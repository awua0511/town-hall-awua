import { getUser } from '$lib/backend/user.js';
import { error, redirect } from '@sveltejs/kit';

export const load = async ({ params, fetch }) => {
  const result = await getUser(fetch, params.id);
  if (!result.ok) error(result.status, result.body);
  const targetUser = result.data;
  if (targetUser.handle !== params.handle) {
    redirect(308, `/user/${targetUser.user_id}/${targetUser.handle}`);
  }
  return { targetUser };
};
