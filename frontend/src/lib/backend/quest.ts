import { fetchBackend, handleResponse, type BackendResult } from './common';
import type { Quest } from './generated-types';

export async function createQuest(title: string): Promise<BackendResult<number>> {
  const resp = await fetchBackend(fetch, '/quests', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(title)
  });
  return await handleResponse(resp);
}

export async function getQuest(customFetch: typeof fetch, id: string): Promise<BackendResult<Quest>> {
  const resp = await fetchBackend(customFetch, `/quests/${id}`);
  return await handleResponse(resp);
}
