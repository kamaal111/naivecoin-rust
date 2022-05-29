import type {Request, Response} from 'express';

import Peers from '../models/peers';

class PeersController {
  private peers: Peers;

  constructor() {
    this.peers = new Peers();
  }

  public getPeers = (_request: Request, response: Response) => {
    response.send(this.peers.socketAddresses);
  };

  public addPeer(request: Request, response: Response) {
    response.send({});
  }
}

export default PeersController;
