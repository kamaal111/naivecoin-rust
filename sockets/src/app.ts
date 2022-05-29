import * as express from 'express';
import * as logger from 'morgan';

import Peers from './models/peers';

import type {AppRouter} from './types';

const STATUS_CODE_TO_MESSAGE: {[code: number]: string} = {
  400: 'Bad Request',
  404: 'Not Found',
};

const peers = new Peers();

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
      console.log(`listening on port ${serverPort}`);
    });
  }

  private initializeMiddleware() {
    this.app.use(logger('dev'));
    this.app.use(express.json());
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
    _request: express.Request,
    response: express.Response,
    next: express.NextFunction
  ) {
    const statusCode = response.statusCode;
    if (statusCode === 404) {
      next();
      return;
    }

    const message =
      STATUS_CODE_TO_MESSAGE[statusCode] ?? 'Okey we messed up, please help!';
    response.status(statusCode).json({
      details: message,
    });
  }
}

export default App;
