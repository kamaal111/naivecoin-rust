import {get} from 'superagent';

import type {BlockType, Result} from '../types';

const BASE_URL = 'http://127.0.0.1:8080';

class BlocksClient {
  constructor() {}

  public async getAll(): Promise<Result<BlockType>> {
    let response: Awaited<ReturnType<typeof get>>;
    try {
      response = await get(`${BASE_URL}/blocks`);
    } catch (error) {
      return {error: error as Error, ok: false};
    }

    return {value: response.body, ok: true};
  }
}

class CoreClient {
  public blocks = new BlocksClient();

  constructor() {}
}

export default CoreClient;
