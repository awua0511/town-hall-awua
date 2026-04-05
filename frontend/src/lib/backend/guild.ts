import { fetchBackend, handleResponse, type BackendResult } from './common';
import type { Guild, CreateGuildRequest } from './generated-types';

export async function createGuild(data: CreateGuildRequest): Promise<BackendResult<number>> {
  const resp = await fetchBackend(fetch, '/guilds', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data)
  });
  return await handleResponse(resp);
}

export async function getGuild(customFetch: typeof fetch, slug: string): Promise<BackendResult<Guild>> {
  const resp = await fetchBackend(customFetch, `/guilds/${slug}`);
  return await handleResponse(resp);
}

export async function getGuilds(): Promise<BackendResult<Guild[]>> {
  const resp = await fetchBackend(fetch, '/guilds');
  return await handleResponse(resp);
}

export async function joinGuild(slug: string): Promise<BackendResult<void>> {
  const resp = await fetchBackend(fetch, `/guilds/${slug}/join`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' }
  });
  return await handleResponse(resp);
}

export async function leaveGuild(slug: string): Promise<BackendResult<void>> {
  const resp = await fetchBackend(fetch, `/guilds/${slug}/leave`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' }
  });
  return await handleResponse(resp);
}