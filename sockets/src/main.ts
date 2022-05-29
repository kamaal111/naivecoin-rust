import App from './app';
import config from './config';

import PeersRouter from './routers/peers';

import type {AppRouter} from './types';

const routers: [AppRouter] = [new PeersRouter()];

const app = new App({routers});

app.listen({serverPort: config.SERVER_PORT, socketsPort: config.SOCKETS_PORT});
