import type {NextFunction, Response} from 'express';

import sendError from '../utils/sendError';
import type {AppRequest} from '../types';

type AddPeerPayload = {peer?: unknown} | undefined;

class PeersController {
  constructor() {}

  public getPeers = (request: AppRequest, response: Response) => {
    response.send(request.context!.peers.socketAddresses);
  };

  public minedBlock = (request: AppRequest, response: Response) => {
    // TODO:

    // get latest block from request

    // broadcast latest block

    response.status(204).send();
  };

  public addPeer = (
    request: AppRequest<undefined, undefined, AddPeerPayload>,
    response: Response,
    next: NextFunction
  ) => {
    const peer = request.body?.peer;
    if (typeof peer !== 'string') {
      sendError(response, next)(400);
      return;
    }

    request.context!.peers.connectToPeer(peer);

    response.status(204).send();
  };
}

export default PeersController;
