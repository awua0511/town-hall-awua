import { getUserMe } from '$lib/backend/user.js';

export const load = async ({ fetch }) => {
  const me = await getUserMe(fetch);
  if (!me.ok) {
    return { user: null };
  }
  return { me: me.data };
};
