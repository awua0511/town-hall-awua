import { fetchBackend, handleResponse, type BackendResult } from './common';
import type { Solution, CreateSolutionRequest } from './generated-types';

export async function createSolution(data: CreateSolutionRequest): Promise<BackendResult<number>> {
  const resp = await fetchBackend(fetch, `/quests/${data.quest_id}/solutions`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data)
  });
  return await handleResponse(resp);
}

export async function getSolution(customFetch: typeof fetch, questId: string, solutionId: string): Promise<BackendResult<Solution>> {
  const resp = await fetchBackend(customFetch, `/quests/${questId}/solutions/${solutionId}`);
  return await handleResponse(resp);
}

export async function getSolutionsByQuest(questId: string): Promise<BackendResult<Solution[]>> {
  const resp = await fetchBackend(fetch, `/quests/${questId}/solutions`);
  return await handleResponse(resp);
}

export async function approveSolution(questId: string, solutionId: string): Promise<BackendResult<void>> {
  const resp = await fetchBackend(fetch, `/quests/${questId}/solutions/${solutionId}/approve`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' }
  });
  return await handleResponse(resp);
}

export async function rejectSolution(questId: string, solutionId: string): Promise<BackendResult<void>> {
  const resp = await fetchBackend(fetch, `/quests/${questId}/solutions/${solutionId}/reject`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' }
  });
  return await handleResponse(resp);
}