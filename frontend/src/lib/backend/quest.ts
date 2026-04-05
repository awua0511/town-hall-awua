import { fetchBackend, handleResponse, type BackendResult } from './common';
import type { Quest, UpdateQuestRequest } from './generated-types';

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

export async function getQuests(): Promise<BackendResult<Quest[]>> {
  const resp = await fetchBackend(fetch, '/quests');
  return await handleResponse(resp);
}

export async function updateQuest(id: string, data: UpdateQuestRequest): Promise<BackendResult<void>> {
  const resp = await fetchBackend(fetch, `/quests/${id}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data)
  });
  return await handleResponse(resp);
}

export async function markQuestOngoing(id: string): Promise<BackendResult<void>> {
  const resp = await fetchBackend(fetch, `/quests/${id}/ongoing`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' }
  });
  return await handleResponse(resp);
}

export async function markQuestSolved(id: string): Promise<BackendResult<void>> {
  const resp = await fetchBackend(fetch, `/quests/${id}/solved`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' }
  });
  return await handleResponse(resp);
}
