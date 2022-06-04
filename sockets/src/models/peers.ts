import * as WebSocket from 'ws';

import Block from './block';

import {jsonToObject} from '../utils/json';
import CoreClient from '../clients/core';
import isEmptyArray from '../utils/isEmptyArray';

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
  private blocksClient = new CoreClient().blocks;

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
    socket.on('message', async data => {
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
          this.handleQueryAllMessage(socket);
          break;
        case SocketMessageType.QUERY_LATEST:
          this.handleQueryLatestMessage(socket);
          break;
        case SocketMessageType.RESPONSE_BLOCKCHAIN:
          this.handleBlockchainResponseMessage({
            socket,
            message: message as {
              type: SocketMessageType;
              data: string;
            },
          });
          break;
        case SocketMessageType.ERROR:
          return;
        default:
          break;
      }
    });
  }

  private async handleQueryAllMessage(socket: WebSocket) {
    const allBlocksResult = await this.blocksClient.getAll();
    if ('error' in allBlocksResult) {
      this.sendError({
        socket,
        message: 'Okey we messed up, please help!',
      });
      console.log(
        'something went wrong while getting all blocks; error:',
        allBlocksResult.error
      );
      return;
    }

    this.send({
      socket,
      message: {
        type: SocketMessageType.RESPONSE_BLOCKCHAIN,
        data: JSON.stringify(allBlocksResult.value),
      },
    });
  }

  private async handleQueryLatestMessage(socket: WebSocket) {
    const latestBlockResult = await this.blocksClient.getLatest();
    if ('error' in latestBlockResult) {
      this.sendError({
        socket,
        message: 'Okey we messed up, please help!',
      });
      console.log(
        'something went wrong while getting the latest block; error:',
        latestBlockResult.error
      );
      return;
    }

    this.broadcast({
      message: {
        type: SocketMessageType.RESPONSE_BLOCKCHAIN,
        data: JSON.stringify(latestBlockResult.value),
      },
    });
  }

  private async handleBlockchainResponseMessage({
    socket,
    message,
  }: {
    socket: WebSocket;
    message: {
      type: SocketMessageType;
      data: string;
    };
  }) {
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
    if (isEmptyArray(receivedBlocks)) {
      this.sendError({socket, message: 'Invalid message sent'});
      return;
    }

    const latestBlockReceived = new Block((receivedBlocks.at(-1) ?? {}) as any);
    if (!latestBlockReceived.isValidBlockStructure) {
      this.sendError({socket, message: 'Invalid message sent'});
      return;
    }

    const latestBlockResult = await this.blocksClient.getLatest();
    if ('error' in latestBlockResult) {
      this.sendError({
        socket,
        message: 'Okey we messed up, please help!',
      });
      console.log(
        'something went wrong while getting the latest block; error:',
        latestBlockResult.error
      );
      return;
    }

    if (isEmptyArray(latestBlockResult.value)) {
      this.sendError({
        socket,
        message: 'Okey we messed up, please help!',
      });
      return;
    }

    const [latestBlockHeld] = latestBlockResult.value;
    if (latestBlockReceived.index <= latestBlockHeld.index) {
      // do nothing everything is well
      return;
    }

    if (latestBlockHeld.hash === latestBlockReceived.parent_hash) {
      const addToChainResult = await this.blocksClient.addToChain(
        latestBlockReceived
      );
      if ('error' in addToChainResult) {
        this.sendError({
          socket,
          message: 'Okey we messed up, please help!',
        });
        console.log(
          'something went wrong while adding block to chain; error:',
          addToChainResult.error
        );
        return;
      }

      this.broadcast({
        message: {
          type: SocketMessageType.RESPONSE_BLOCKCHAIN,
          data: JSON.stringify([latestBlockReceived]),
        },
      });
      return;
    }

    // TODO:

    // if receivedBlocks.length === 1
    // broadcast to get all blocks
    // return

    // TODO:

    // replace whole chain call
    // Broadcast latest block
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

export default Peers;
