import { BACKEND_URL, handleResponse as handleResponse, type BackendResult } from './common';
import type { User } from './generated-types';

export async function getUserMe(customFetch: typeof fetch): Promise<BackendResult<User>> {
  const resp = await customFetch(`${BACKEND_URL}/users/me`);
  return await handleResponse(resp);
}

export async function getUser(customFetch: typeof fetch, id: string): Promise<BackendResult<User>> {
  const resp = await customFetch(`${BACKEND_URL}/users/${id}`);
  return await handleResponse(resp);
}

export async function getUserFromHandle(customFetch: typeof fetch, handle: string): Promise<BackendResult<User>> {
  const idResult = await resolveHandle(customFetch, handle);
  if (!idResult.ok) {
    return idResult; // Return the error if id fetch failed
  }
  return await getUser(customFetch, idResult.data);
}

// Helper to fetch ID from handle
async function resolveHandle(customFetch: typeof fetch, handle: string): Promise<BackendResult<string>> {
  const resp = await customFetch(`${BACKEND_URL}/users/resolve/${handle}`);
  return await handleResponse(resp);
}
