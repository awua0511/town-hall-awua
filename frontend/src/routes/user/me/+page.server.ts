import { error, redirect } from '@sveltejs/kit';

export const load = async ({ parent }) => {
  const { me } = await parent();
  if (!me) {
    error(401, 'Not logged in');
  }
  redirect(307, `/user/${me.user_id}/${me.handle}`);
};
