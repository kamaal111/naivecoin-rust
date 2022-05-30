import type {Result} from '../types';

export function jsonToObject<T>(data: string): Result<T> {
  let object: T;
  try {
    object = JSON.parse(data);
  } catch (error) {
    return {ok: false, error: error as Error};
  }

  return {ok: true, value: object};
}
