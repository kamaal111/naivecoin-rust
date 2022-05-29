import type {NextFunction, Response} from 'express';

function sendError(response: Response, next: NextFunction) {
  return function (code: number) {
    response.status(code);
    next('router');
  };
}

export default sendError;
