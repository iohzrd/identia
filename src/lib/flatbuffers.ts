import type { Message } from "ipfs-http-client/pubsub/subscribe";
import { flatbuffers } from "flatbuffers/js/flatbuffers";
import {
  Message as MessageType,
  PubsubMessage,
  TopicPost,
} from "./flatbuffers/messages_generated";

export function createTopicPost(body: string): Uint8Array {
  let builder = new flatbuffers.Builder();
  let messageOffset = TopicPost.createTopicPost(
    builder,
    builder.createString(body)
  );
  builder.finish(messageOffset);
  let pubsubOffset = PubsubMessage.createPubsubMessage(
    builder,
    MessageType.TopicPost,
    messageOffset,
    BigInt(new Date().getTime())
  );
  builder.finish(pubsubOffset);
  return builder.asUint8Array();
}

export function parsePubsubMessage(message: Message) {
  let parsed = undefined;
  let buff = new flatbuffers.ByteBuffer(message.data);
  let pubsubMessage = PubsubMessage.getRootAsPubsubMessage(buff);
  if (pubsubMessage.messageType() != undefined) {
    // let timestamp = pubsubMessage.timestamp();
    // let id = peerIdFromPeerId(message.from);
    // console.log(id);
    switch (pubsubMessage.messageType()) {
      case MessageType.NONE:
        break;
      case MessageType.Comment:
        break;
      case MessageType.TopicPost:
        let msg = pubsubMessage.message(TopicPost.getRootAsTopicPost(buff));
        parsed = msg.body();
        break;
    }
  }
  return parsed;
}
