import type {Request, Router} from 'express';

import type Peers from './models/peers';

export type AppRouter = {
  path: string;
  router: Router;
};

export type Context = {
  peers: Peers;
};

export type AppRequest<
  Params = Record<string, unknown>,
  ResponseBody = Record<string, unknown>,
  RequestBody = Record<string, unknown>,
  RequestQuery = qs.ParsedQs
> = Request<Params, ResponseBody, RequestBody, RequestQuery> & {
  context?: Context;
};
