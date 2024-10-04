import type { MessageExtended } from "$lib/types";
import type { QueryResult } from "@tauri-apps/plugin-sql";
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
  console.log("globalPubsubHandler", message);
  if (message.type === "signed" && !blacklist.includes(String(message.from))) {
    let json = new TextDecoder("utf-8").decode(message.data);
    let parsed = JSON.parse(json);
    message.inReplyTo = parsed["inReplyTo"] || "";
    message.body = parsed["body"] || "";
    message.files = parsed["files"] || [];

    if (message.inReplyTo != null) {
      pubsubStore.set(message.topic, message.inReplyTo, message);
    }
  }
}

export async function getTopicsFromDB() {
  const rows: object[] = await select("SELECT * FROM topics");
  return rows.map((e) => e["topic"]);
}

export async function insertTopicIntoDB(topic: string): Promise<QueryResult> {
  return await execute("INSERT INTO topics (topic) VALUES ($1)", [topic]);
}

export async function deleteTopicFromDB(topic: string): Promise<QueryResult> {
  return await execute("DELETE FROM topics WHERE topic = ?", [topic]);
}