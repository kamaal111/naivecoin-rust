import {Router} from 'express';

import PeersController from '../controllers/peers';

import type {AppRouter} from '../types';

class PeersRouter implements AppRouter {
  public path = '/peers';
  public router = Router();

  private controller = new PeersController();

  constructor() {
    this.initializeMiddleware();
    this.initializeRoutes();
  }

  private initializeMiddleware() {}

  private initializeRoutes() {
    this.router.get('/', this.controller.getPeers);
    this.router.post('/', this.controller.addPeer);
  }
}

export default PeersRouter;
