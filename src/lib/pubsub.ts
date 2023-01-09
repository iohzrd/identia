import type { MessageExtended } from "$lib/types";
import type { QueryResult } from "tauri-plugin-sql-api";
import {
  Json,
  MessageType,
  PubsubMessage,
  Topical,
} from "./flatbuffers/messages_generated";
import { flatbuffers } from "flatbuffers/js/flatbuffers";
import { ipfs } from "./core";
import { peerIdFromPeerId } from "@libp2p/peer-id";
import { select, execute } from "./db";

const blacklist: string[] = [];
const subscriptions = new Map();

function subscriptionStore() {
  const subscribe = (topic: string, subTopic: string, handler: any) => {
    const topicSubs = subscriptions.get(topic) || new Map();
    topicSubs.set(subTopic, handler);
    subscriptions.set(topic, topicSubs);
    return () => {
      const topicSubs = subscriptions.get(topic) || new Map();
      topicSubs.delete(subTopic);
    };
  };
  const set = (topic: string, subTopic: string, message: any) => {
    const topicSubs = subscriptions.get(topic) || new Map();
    if (topicSubs.has(subTopic)) {
      const fn = topicSubs.get(subTopic);
      fn(message);
    }
  };
  return { subscribe, set };
}

export const pubsubStore = subscriptionStore();

export async function globalPubsubHandler(message: MessageExtended) {
  // console.log("globalPubsubHandler");
  if (message.type === "signed" && !blacklist.includes(String(message.from))) {
    let buff = new flatbuffers.ByteBuffer(message.data);
    let pubsubMessage = PubsubMessage.getRootAsPubsubMessage(buff);
    let messageType = pubsubMessage.messageType();
    // let timestamp = pubsubMessage.timestamp();
    switch (messageType) {
      case MessageType.Json:
        // console.log("Json");
        let j: Json = pubsubMessage.message(Json.getRootAsJson(buff));
        let parsed = JSON.parse(j.data() || "");
        message.inReplyTo = parsed["inReplyTo"] || "";
        message.body = parsed["body"] || "";
        message.files = parsed["files"] || [];
        break;
      case MessageType.Topical:
        // console.log("Topical");
        let t: Topical = pubsubMessage.message(Topical.getRootAsTopical(buff));
        message.inReplyTo = t.inReplyTo() || "";
        message.body = t.body() || "";
        // message.files = t.files(t.filesLength()) || [];
        break;
    }
    // console.log("inReplyTo");
    // console.log(message.inReplyTo);
    if (message.inReplyTo != null) {
      pubsubStore.set(message.topic, message.inReplyTo, message);
    }
  }
}

export async function getTopicsFromDB() {
  console.log("getTopicsFromDB");
  const rows: object[] = await select("SELECT * FROM topics");
  return rows.map((e) => e["topic"]);
}

export async function insertTopicIntoDB(topic: string): Promise<QueryResult> {
  console.log("insertTopicIntoDB: ", topic);
  return await execute("INSERT INTO topics (topic) VALUES ($1)", [topic]);
}

export async function deleteTopicFromDB(topic: string): Promise<QueryResult> {
  console.log("deleteTopicFromDB: ", topic);
  return await execute("DELETE FROM topics WHERE topic = ?", [topic]);
}
