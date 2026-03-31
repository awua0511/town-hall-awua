import { getQuest } from '$lib/backend/quest.js';
import { slugify } from '$lib/utils.js';
import { error, redirect } from '@sveltejs/kit';

export const load = async ({ fetch, params }) => {
  const questResult = await getQuest(fetch, params.id);
  if (!questResult.ok) {
    error(questResult.status, questResult.body);
  }
  const quest = questResult.data;
  const slug = slugify(quest.title);
  if (slug !== params.title) {
    redirect(308, `/quest/${quest.quest_id}/${slug}`);
  }
  return { quest };
};
