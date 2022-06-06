import {Router} from 'express';

import BlocksController from '../controllers/blocks';

import type {AppRouter} from '../types';

class BlocksRouter implements AppRouter {
  public path = '/blocks';
  public router = Router();

  private controller: BlocksController;

  constructor() {
    this.controller = new BlocksController();
    this.initializeMiddleware();
    this.initializeRoutes();
  }

  private initializeMiddleware() {}

  private initializeRoutes() {
    this.router.post('/', this.controller.mineBlock);
  }
}

export default BlocksRouter;
