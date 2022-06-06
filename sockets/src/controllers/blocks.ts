import type {Response} from 'express';

import type {AppRequest} from '../types';

class BlocksController {
  constructor() {}

  public mineBlock = (request: AppRequest, response: Response) => {
    // TODO:

    // get latest block from request

    // broadcast latest block

    response.status(204).send();
  };
}

export default BlocksController;
