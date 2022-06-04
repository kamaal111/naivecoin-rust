import {get} from '../utils/request';

import type {BlockType, Result} from '../types';

const BASE_URL = 'http://127.0.0.1:8080';

class BlocksClient {
  public path = '/blocks';

  constructor() {}

  public async getAll(): Promise<Result<BlockType[]>> {
    return get({url: this.makeURL()});
  }

  public async getLatest(): Promise<Result<BlockType[]>> {
    return get({url: this.makeURL('?latest=1')});
  }

  private makeURL(extension = '') {
    return `${BASE_URL}${this.path}${extension}`;
  }
}

class CoreClient {
  public blocks = new BlocksClient();

  constructor() {}
}

export default CoreClient;
