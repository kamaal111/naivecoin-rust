import * as express from 'express';
import * as logger from 'morgan';

import Peers from './models/peers';

import contextMiddleware from './middleware/contextMiddleware';
import type {AppRequest, AppRouter, Context} from './types';

const INTERNAL_SERVER_ERROR_MESSAGE = 'Okey we messed up, please help!';
const STATUS_CODE_TO_MESSAGE: {[code: number]: string} = {
  400: 'Bad Request',
  404: 'Not Found',
  500: INTERNAL_SERVER_ERROR_MESSAGE,
};

const peers = new Peers();
const context: Context = {
  peers,
};

class App {
  private app = express();

  constructor({routers}: {routers: AppRouter[]}) {
    this.initializeMiddleware();
    this.initializeRoutes(routers);
  }

  public listen({
    serverPort,
    socketsPort,
  }: {
    serverPort: string;
    socketsPort: number;
  }) {
    this.app.listen(serverPort, () => {
      console.log(`server listening on port ${serverPort}`);
    });

    peers.listen(socketsPort);
  }

  private initializeMiddleware() {
    this.app.use(logger('dev'));
    this.app.use(express.json());
    this.app.use(contextMiddleware(context));
  }

  private initializeRoutes(routers: AppRouter[]) {
    this.app.get('/', (_request, response) => {
      response.json({hello: 'world'});
    });

    for (const router of routers) {
      this.app.use(router.path, router.router);
    }

    this.app.use(this.errorHandler);
    this.app.use((_request, response) => {
      const statusCode = 404;
      response
        .status(statusCode)
        .json({details: STATUS_CODE_TO_MESSAGE[statusCode]});
    });
  }

  private errorHandler(
    _request: AppRequest,
    response: express.Response,
    next: express.NextFunction
  ) {
    const statusCode = response.statusCode;
    if (statusCode === 404 || statusCode === 200) {
      next();
      return;
    }

    const message =
      STATUS_CODE_TO_MESSAGE[statusCode] ?? INTERNAL_SERVER_ERROR_MESSAGE;
    response.status(statusCode).json({
      details: message,
    });
  }
}

export default App;
