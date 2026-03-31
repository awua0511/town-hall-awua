export type BackendResult<T> = { ok: true; data: T } | { ok: false; status: number; body: string };

export const BACKEND_URL = 'http://localhost:3000';

export async function handleResponse<T>(resp: Response): Promise<BackendResult<T>> {
  if (!resp.ok) {
    const body = await resp.text();
    return { ok: false, status: resp.status, body };
  } else {
    const data: T = await resp.json();
    return { ok: true, data };
  }
}
