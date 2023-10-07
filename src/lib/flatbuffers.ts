// import type { Message } from "kubo-rpc-client/pubsub/subscribe";
import { Builder, ByteBuffer } from "flatbuffers";
import {
  Json,
  MessageType,
  PubsubMessage,
  Topical,
} from "./flatbuffers/messages";

export function createJson(data: string | object): Uint8Array {
  let str: string;
  if (typeof data === "object") {
    str = JSON.stringify(data);
  } else {
    str = data;
  }
  let builder = new Builder();
  let dataOffset = Json.createJson(builder, builder.createString(str));
  builder.finish(dataOffset);
  let pubsubOffset = PubsubMessage.createPubsubMessage(
    builder,
    MessageType.Json,
    dataOffset,
    BigInt(new Date().getTime())
  );
  builder.finish(pubsubOffset);
  return builder.asUint8Array();
}

export function createTopical(
  topic: string,
  body: string,
  files: string[]
): Uint8Array {
  let builder = new Builder();
  let topicOffset = builder.createString(topic);
  let bodyOffset = builder.createString(body);
  let filesOffset = Topical.createFilesVector(
    builder,
    builder.createObjectOffsetList(files)
  );
  let messageOffset = Topical.createTopical(
    builder,
    topicOffset,
    bodyOffset,
    filesOffset
  );
  builder.finish(messageOffset);
  let pubsubOffset = PubsubMessage.createPubsubMessage(
    builder,
    MessageType.Topical,
    messageOffset,
    BigInt(new Date().getTime())
  );
  builder.finish(pubsubOffset);
  return builder.asUint8Array();
}

export function parsePubsubMessage(message: any) {
  let buff = new ByteBuffer(message.data);
  let pubsubMessage = PubsubMessage.getRootAsPubsubMessage(buff);
  if (pubsubMessage.messageType() != undefined) {
    switch (pubsubMessage.messageType()) {
      case MessageType.Json:
        let j: Json = pubsubMessage.message(Json.getRootAsJson(buff));
        return JSON.parse(j.data() || "");
      case MessageType.Topical:
        let t: Topical = pubsubMessage.message(Topical.getRootAsTopical(buff));
        return {
          inReplyTo: t.inReplyTo(),
          body: t.body(),
          files: t.files(t.filesLength()),
        };
    }
  }
}
