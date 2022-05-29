import * as express from 'express';
import * as logger from 'morgan';

const app = express();
app.use(logger('dev'));
app.use(express.json());

app.get('/', (_request, response) => {
  response.json({hello: 'world'});
});

app.use((_request, response) => {
  response.status(404).json({details: 'Not Found'});
});

const serverPort = process.env.SERVER_PORT ?? '3001';

app.listen(serverPort, () => {
  console.log(`listening on port ${serverPort}`);
});
