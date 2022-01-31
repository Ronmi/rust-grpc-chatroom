/**
 * @fileoverview gRPC-Web generated client stub for chat
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck


import * as grpcWeb from 'grpc-web';

import * as chat_pb from './chat_pb';


export class ChatClient {
  client_: grpcWeb.AbstractClientBase;
  hostname_: string;
  credentials_: null | { [index: string]: string; };
  options_: null | { [index: string]: any; };

  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; }) {
    if (!options) options = {};
    if (!credentials) credentials = {};
    options['format'] = 'text';

    this.client_ = new grpcWeb.GrpcWebClientBase(options);
    this.hostname_ = hostname;
    this.credentials_ = credentials;
    this.options_ = options;
  }

  methodInfoJoin = new grpcWeb.MethodDescriptor(
    '/chat.Chat/Join',
    grpcWeb.MethodType.UNARY,
    chat_pb.JoinReq,
    chat_pb.JoinRes,
    (request: chat_pb.JoinReq) => {
      return request.serializeBinary();
    },
    chat_pb.JoinRes.deserializeBinary
  );

  join(
    request: chat_pb.JoinReq,
    metadata: grpcWeb.Metadata | null): Promise<chat_pb.JoinRes>;

  join(
    request: chat_pb.JoinReq,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: chat_pb.JoinRes) => void): grpcWeb.ClientReadableStream<chat_pb.JoinRes>;

  join(
    request: chat_pb.JoinReq,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: chat_pb.JoinRes) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/chat.Chat/Join',
        request,
        metadata || {},
        this.methodInfoJoin,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/chat.Chat/Join',
    request,
    metadata || {},
    this.methodInfoJoin);
  }

  methodInfoSay = new grpcWeb.MethodDescriptor(
    '/chat.Chat/Say',
    grpcWeb.MethodType.UNARY,
    chat_pb.SayReq,
    chat_pb.SayRes,
    (request: chat_pb.SayReq) => {
      return request.serializeBinary();
    },
    chat_pb.SayRes.deserializeBinary
  );

  say(
    request: chat_pb.SayReq,
    metadata: grpcWeb.Metadata | null): Promise<chat_pb.SayRes>;

  say(
    request: chat_pb.SayReq,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: chat_pb.SayRes) => void): grpcWeb.ClientReadableStream<chat_pb.SayRes>;

  say(
    request: chat_pb.SayReq,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: chat_pb.SayRes) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/chat.Chat/Say',
        request,
        metadata || {},
        this.methodInfoSay,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/chat.Chat/Say',
    request,
    metadata || {},
    this.methodInfoSay);
  }

  methodInfoNewMsg = new grpcWeb.MethodDescriptor(
    '/chat.Chat/NewMsg',
    grpcWeb.MethodType.SERVER_STREAMING,
    chat_pb.NewMsgReq,
    chat_pb.Message,
    (request: chat_pb.NewMsgReq) => {
      return request.serializeBinary();
    },
    chat_pb.Message.deserializeBinary
  );

  newMsg(
    request: chat_pb.NewMsgReq,
    metadata?: grpcWeb.Metadata) {
    return this.client_.serverStreaming(
      this.hostname_ +
        '/chat.Chat/NewMsg',
      request,
      metadata || {},
      this.methodInfoNewMsg);
  }

}

