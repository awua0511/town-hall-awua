import { getUser } from '$lib/backend/user';
import { error, redirect } from '@sveltejs/kit';

export const load = async ({ fetch, params }) => {
  const result = await getUser(fetch, params.id);
  if (!result.ok) error(result.status, result.body);
  const targetUser = result.data;
  redirect(308, `/user/${targetUser.user_id}/${targetUser.handle}`);
};
