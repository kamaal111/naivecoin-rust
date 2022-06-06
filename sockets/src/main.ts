import App from './app';
import config from './config';
import BlocksRouter from './routers/blocks';
import PeersRouter from './routers/peers';

import type {AppRouter} from './types';

const routers: AppRouter[] = [new PeersRouter(), new BlocksRouter()];

const app = new App({routers});

app.listen({serverPort: config.SERVER_PORT, socketsPort: config.SOCKETS_PORT});
