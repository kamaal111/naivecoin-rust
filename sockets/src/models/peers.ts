import * as WebSocket from 'ws';

import {jsonToObject} from '../utils/json';

enum SocketMessageType {
  QUERY_LATEST = 0,
  QUERY_ALL = 1,
  RESPONSE_BLOCKCHAIN = 2,
  ERROR = 3,
}

type SocketMessage = {
  type: SocketMessageType;
  data: string | null;
};

type WebSocketPrivateAPIs = {
  _socket: {remoteAddress: string; remotePort: string};
};

class Peers {
  private _sockets: WebSocket[] = [];

  constructor() {}

  public get socketAddresses() {
    return (this._sockets as unknown as WebSocketPrivateAPIs[]).map(
      socket => `${socket._socket.remoteAddress}:${socket._socket.remotePort}`
    );
  }

  public connectToPeer(newPeer: string) {
    const socket = new WebSocket(newPeer);

    socket.on('open', () => {
      this.initializeConnection(socket);
    });

    socket.on('error', () => {
      console.log('connection failed');
    });
  }

  public listen(port: number) {
    const server = new WebSocket.Server({port});

    server.on('connection', socket => {
      this.initializeConnection(socket);
    });

    console.log(`sockets listening on port ${port}`);
  }

  private initializeConnection(socket: WebSocket) {
    this.addToSockets(socket);

    this.initializeMessageHandler(socket);
    this.initializeErrorHandler(socket);

    this.send({
      socket,
      message: {type: SocketMessageType.QUERY_LATEST, data: null},
    });
  }

  private initializeMessageHandler(socket: WebSocket) {
    socket.on('message', data => {
      if (typeof data !== 'string') {
        this.sendError({socket, message: 'Invalid message sent'});
        return;
      }

      const objectResult = jsonToObject<SocketMessage>(data);
      if ('error' in objectResult) {
        this.sendError({socket, message: 'Invalid message sent'});
        return;
      }

      const message = objectResult.value;
      if (
        typeof message.data !== 'string' ||
        typeof message.type !== 'number'
      ) {
        this.sendError({socket, message: 'Invalid message sent'});
        return;
      }

      switch (message.type) {
        case SocketMessageType.QUERY_ALL:
          // TODO:

          // Get all blocks

          // Send all blocks

          // this.send({
          //   socket,
          //   message: {
          //     type: SocketMessageType.RESPONSE_BLOCKCHAIN,
          //     data: JSON.stringify(<all blocks>),
          //   },
          // });
          break;
        case SocketMessageType.QUERY_LATEST:
          // TODO:

          // Get latest block

          // Broadcast latest block

          // this.broadcast({
          //   message: {
          //     type: SocketMessageType.RESPONSE_BLOCKCHAIN,
          //     data: JSON.stringify([this.blockChain.getLatestBlock()]),
          //   },
          // });
          break;
        case SocketMessageType.RESPONSE_BLOCKCHAIN:
          const receivedBlocksResult = jsonToObject<
            {
              index: number;
              hash: string;
              parent_hash?: string | null;
              timestamp: number;
              data: string;
            }[]
          >(message.data);
          if ('error' in receivedBlocksResult) {
            this.sendError({
              socket,
              message: 'Okey we messed up, please help!',
            });
            return;
          }

          const receivedBlocks = receivedBlocksResult.value;
          if (!Array.isArray(receivedBlocks) || receivedBlocks.length === 0) {
            this.sendError({socket, message: 'Invalid message sent'});
            return;
          }

          const latestBlockReceived = receivedBlocks.at(-1);
          const parentHashIsAOptionalString =
            latestBlockReceived?.parent_hash == null ||
            typeof latestBlockReceived.parent_hash === 'string';
          if (
            latestBlockReceived == null ||
            !parentHashIsAOptionalString ||
            typeof latestBlockReceived.index !== 'number' ||
            typeof latestBlockReceived.hash !== 'string' ||
            typeof latestBlockReceived.timestamp !== 'number' ||
            typeof latestBlockReceived.data !== 'string'
          ) {
            this.sendError({socket, message: 'Invalid message sent'});
            return;
          }
          // TODO:

          // Get latest block
          //  if latestBlockReceived.index <= latestBlockHeld.index return; // do nothing everything is well

          // TODO:

          // if latestBlockHeld.hash === latestBlockReceived.previousHash
          // save block in to chain
          // Broadcast latest block
          // return

          // TODO:

          // if receivedBlocks.length === 1
          // broadcast to get all blocks
          // return

          // TODO:

          // replace whole chain call
          // Broadcast latest block

          break;
        case SocketMessageType.ERROR:
          return;
        default:
          break;
      }
    });
  }

  private broadcast({message}: {message: SocketMessage}) {
    this._sockets.forEach(async socket => this.send({socket, message}));
  }

  private initializeErrorHandler(socket: WebSocket) {
    socket.on('close', () => this.removeFromSockets(socket));
    socket.on('error', () => this.removeFromSockets(socket));
  }

  private removeFromSockets(socket: WebSocket) {
    const index = this._sockets.findIndex(value => value === socket);
    if (index === -1) return;

    this._sockets.splice(index, 1);
  }

  private addToSockets(socket: WebSocket) {
    this._sockets.push(socket);
  }

  private sendError({socket, message}: {socket: WebSocket; message: string}) {
    this.send({
      socket,
      message: {
        type: SocketMessageType.ERROR,
        data: JSON.stringify({details: message}),
      },
    });
  }

  private send({socket, message}: {socket: WebSocket; message: SocketMessage}) {
    socket.send(JSON.stringify(message));
  }
}

class Block {
  public index: number;
  public hash: string;
  public previousHash?: string | null;
  public timestamp: number;
  public data: string;

  constructor({
    index,
    hash,
    previousHash,
    timestamp,
    data,
  }: {
    index: number;
    hash: string;
    previousHash: string | null;
    timestamp: number;
    data: string;
  }) {
    this.index = index;
    this.previousHash = previousHash;
    this.timestamp = timestamp;
    this.data = data;
    this.hash = hash;

    Object.freeze(this);
  }

  public get isValidBlockStructure() {
    return (
      typeof this.index === 'number' &&
      typeof this.hash === 'string' &&
      (typeof this.previousHash === 'string' || this.previousHash == null) &&
      typeof this.timestamp === 'number' &&
      typeof this.data === 'string'
    );
  }
}

export default Peers;
