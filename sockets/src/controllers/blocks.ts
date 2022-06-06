import type {NextFunction, Response} from 'express';

import CoreClient from '../clients/core';
import sendError from '../utils/sendError';
import type {AppRequest} from '../types';

type MineBlockPayload = {data?: unknown} | undefined;

class BlocksController {
  constructor() {}

  public mineBlock = async (
    request: AppRequest<undefined, undefined, MineBlockPayload>,
    response: Response,
    next: NextFunction
  ) => {
    const data = request.body?.data;
    if (typeof data !== 'string') {
      sendError(response, next)(400);
      return;
    }

    const blocksClient = new CoreClient().blocks;

    const mineBlockResult = await blocksClient.mineBlock(data);
    if ('error' in mineBlockResult) {
      console.log('error while mining block; block:', mineBlockResult.error);
      sendError(response, next)(500);
      return;
    }

    request.context!.peers.broadcastLatestBlocks([mineBlockResult.value]);

    response.status(204).send();
  };
}

export default BlocksController;
