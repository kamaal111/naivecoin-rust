import * as WebSocket from 'ws';

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

  public connectToPeer(newPeer: string) {}
}

export default Peers;
