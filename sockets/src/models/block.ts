class Block {
  public index: number;
  public hash: string;
  public parent_hash?: string | null;
  public timestamp: number;
  public data: string;

  constructor({
    index,
    hash,
    parent_hash,
    timestamp,
    data,
  }: {
    index: number;
    hash: string;
    parent_hash: string | null;
    timestamp: number;
    data: string;
  }) {
    this.index = index;
    this.parent_hash = parent_hash;
    this.timestamp = timestamp;
    this.data = data;
    this.hash = hash;

    Object.freeze(this);
  }

  public get isValidBlockStructure() {
    return (
      typeof this.index === 'number' &&
      typeof this.hash === 'string' &&
      (typeof this.parent_hash === 'string' || this.parent_hash == null) &&
      typeof this.timestamp === 'number' &&
      typeof this.data === 'string'
    );
  }
}

export default Block;
