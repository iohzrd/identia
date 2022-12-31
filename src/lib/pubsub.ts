import { flatbuffers } from "flatbuffers/js/flatbuffers";
import { ipfs } from "./core";
import { peerIdFromPeerId } from "@libp2p/peer-id";
import { select, execute } from "./db";
import {
  Message,
  PubsubMessage,
  TopicPost,
} from "./flatbuffers/messages_generated";

export async function globalPubsubHandler(message) {
  console.log("globalPubsubHandler", message);
  //
}

export function createTopicPost(body: string): Uint8Array {
  let builder = new flatbuffers.Builder();
  let messageOffset = TopicPost.createTopicPost(
    builder,
    builder.createString(body)
  );
  builder.finish(messageOffset);
  let pubsubOffset = PubsubMessage.createPubsubMessage(
    builder,
    Message.TopicPost,
    messageOffset,
    BigInt(new Date().getTime())
  );
  builder.finish(pubsubOffset);
  return builder.asUint8Array();
}

export function parsePubsubMessage(message) {
  let parsed = undefined;
  let buff = new flatbuffers.ByteBuffer(message.data);
  let pubsubMessage = PubsubMessage.getRootAsPubsubMessage(buff, null);
  if (pubsubMessage.messageType() != undefined) {
    // let timestamp = pubsubMessage.timestamp();
    // let id = peerIdFromPeerId(message.from);
    // console.log(id);
    switch (pubsubMessage.messageType()) {
      case Message.NONE:
        break;
      case Message.Comment:
        break;
      case Message.TopicPost:
        let msg = pubsubMessage.message(TopicPost.getRootAsTopicPost(buff));
        parsed = msg.body();
        break;
    }
  }
  return parsed;
}
