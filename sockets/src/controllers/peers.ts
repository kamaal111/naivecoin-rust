import type {NextFunction, Request, Response} from 'express';

import Peers from '../models/peers';

import sendError from '../utils/sendError';

type AddPeerPayload = {peer?: unknown} | undefined;

class PeersController {
  private peers: Peers;

  constructor() {
    this.peers = new Peers();
  }

  public getPeers = (_request: Request, response: Response) => {
    response.send(this.peers.socketAddresses);
  };

  public addPeer = (
    request: Request<undefined, undefined, AddPeerPayload>,
    response: Response,
    next: NextFunction
  ) => {
    const peer = request.body?.peer;
    if (typeof peer !== 'string') {
      sendError(response, next)(400);
      return;
    }

    this.peers.connectToPeer(peer);

    response.status(204).send();
  };
}

export default PeersController;
