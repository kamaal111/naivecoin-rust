import type {NextFunction, Response} from 'express';

import type {AppRequest, Context} from '../types';

function contextMiddleware(context: Context) {
  return function (
    request: AppRequest,
    _response: Response,
    next: NextFunction
  ) {
    request.context = context;
    next();
  };
}

export default contextMiddleware;
