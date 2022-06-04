import * as superagent from 'superagent';

import type {Result} from '../types';

export async function get<T>({url}: {url: string}): Promise<Result<T>> {
  let response: Awaited<ReturnType<typeof superagent.get>>;
  try {
    response = await superagent.get(url);
  } catch (error) {
    return {error: error as Error, ok: false};
  }

  return {value: response.body, ok: true};
}

export async function post<T>({
  url,
  payload,
}: {
  url: string;
  payload?: string | object;
}): Promise<Result<T>> {
  let response: Awaited<ReturnType<typeof superagent.post>>;
  try {
    response = await superagent.post(url).send(payload);
  } catch (error) {
    return {error: error as Error, ok: false};
  }

  return {value: response.body, ok: true};
}
