import * as jspb from 'google-protobuf'



export class Message extends jspb.Message {
  getName(): string;
  setName(value: string): Message;

  getMsg(): string;
  setMsg(value: string): Message;

  getCreateAt(): number;
  setCreateAt(value: number): Message;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Message.AsObject;
  static toObject(includeInstance: boolean, msg: Message): Message.AsObject;
  static serializeBinaryToWriter(message: Message, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Message;
  static deserializeBinaryFromReader(message: Message, reader: jspb.BinaryReader): Message;
}

export namespace Message {
  export type AsObject = {
    name: string,
    msg: string,
    createAt: number,
  }
}

export class JoinReq extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JoinReq.AsObject;
  static toObject(includeInstance: boolean, msg: JoinReq): JoinReq.AsObject;
  static serializeBinaryToWriter(message: JoinReq, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JoinReq;
  static deserializeBinaryFromReader(message: JoinReq, reader: jspb.BinaryReader): JoinReq;
}

export namespace JoinReq {
  export type AsObject = {
  }
}

export class JoinRes extends jspb.Message {
  getMsgsList(): Array<Message>;
  setMsgsList(value: Array<Message>): JoinRes;
  clearMsgsList(): JoinRes;
  addMsgs(value?: Message, index?: number): Message;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): JoinRes.AsObject;
  static toObject(includeInstance: boolean, msg: JoinRes): JoinRes.AsObject;
  static serializeBinaryToWriter(message: JoinRes, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): JoinRes;
  static deserializeBinaryFromReader(message: JoinRes, reader: jspb.BinaryReader): JoinRes;
}

export namespace JoinRes {
  export type AsObject = {
    msgsList: Array<Message.AsObject>,
  }
}

export class SayReq extends jspb.Message {
  getName(): string;
  setName(value: string): SayReq;

  getMsg(): string;
  setMsg(value: string): SayReq;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SayReq.AsObject;
  static toObject(includeInstance: boolean, msg: SayReq): SayReq.AsObject;
  static serializeBinaryToWriter(message: SayReq, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SayReq;
  static deserializeBinaryFromReader(message: SayReq, reader: jspb.BinaryReader): SayReq;
}

export namespace SayReq {
  export type AsObject = {
    name: string,
    msg: string,
  }
}

export class SayRes extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SayRes.AsObject;
  static toObject(includeInstance: boolean, msg: SayRes): SayRes.AsObject;
  static serializeBinaryToWriter(message: SayRes, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SayRes;
  static deserializeBinaryFromReader(message: SayRes, reader: jspb.BinaryReader): SayRes;
}

export namespace SayRes {
  export type AsObject = {
  }
}

export class NewMsgReq extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): NewMsgReq.AsObject;
  static toObject(includeInstance: boolean, msg: NewMsgReq): NewMsgReq.AsObject;
  static serializeBinaryToWriter(message: NewMsgReq, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): NewMsgReq;
  static deserializeBinaryFromReader(message: NewMsgReq, reader: jspb.BinaryReader): NewMsgReq;
}

export namespace NewMsgReq {
  export type AsObject = {
  }
}

