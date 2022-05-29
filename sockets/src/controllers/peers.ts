import type {Request, Response} from 'express';

import type {AppController} from '../types';

class PeersController implements AppController {
  constructor() {}

  public getPeers(request: Request, response: Response) {
    response.send({});
  }

  public addPeer(request: Request, response: Response) {
    response.send({});
  }
}

export default PeersController;
