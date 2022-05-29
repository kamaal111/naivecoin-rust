import * as express from 'express';
import * as logger from 'morgan';

import PeersRouter from './routers/peers';

import type {AppRouter} from './types';

const app = express();
app.use(logger('dev'));
app.use(express.json());

app.get('/', (_request, response) => {
  response.json({hello: 'world'});
});

const routers: [AppRouter] = [new PeersRouter()];

for (const router of routers) {
  app.use(router.path, router.router);
}

const statusCodeToMessageMap: {[code: number]: string} = {
  400: 'Bad Request',
  404: 'Not Found',
};

app.use(
  (
    _request: express.Request,
    response: express.Response,
    next: express.NextFunction
  ) => {
    const statusCode = response.statusCode;
    if (statusCode === 404) {
      next();
      return;
    }

    response.status(statusCode).json({
      details:
        statusCodeToMessageMap[statusCode] ?? 'Okey we messed up, please help!',
    });
  }
);

app.use((_request, response) => {
  const statusCode = 404;
  response
    .status(statusCode)
    .json({details: statusCodeToMessageMap[statusCode]});
});

const serverPort = process.env.SERVER_PORT ?? '3001';

app.listen(serverPort, () => {
  console.log(`listening on port ${serverPort}`);
});
