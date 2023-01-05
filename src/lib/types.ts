// import type { Message } from "ipfs-http-client/pubsub/subscribe";
// import type { Message } from '@libp2p/interface-pubsub'
import type { PeerId } from "@libp2p/interface-peer-id";

export interface MessageType {
  // pubsub
  data: Uint8Array;
  from: PeerId;
  key: Uint8Array;
  sequenceNumber: bigint;
  signature: Uint8Array;
  topic: string;
  type: "signed";
  // custom
  inReplyTo: string; // cid or seq
  timestamp: number;
}

export interface Feed {
  feed: (Post | WebFeedEntry)[];
}

export interface Identity {
  cid?: string;
  avatar: string;
  description: string;
  display_name: string;
  following: string[];
  meta: object;
  posts: string[];
  publisher: string;
  timestamp: number;
}

export interface Media {
  element: Element;
  filename: string;
  content_type: string;
  thumbnail_for: string;
  url: string;
}

export interface Post {
  cid: string;
  body: string;
  display_name: string;
  files: string[];
  meta: object;
  publisher: string;
  timestamp: number;
}

export interface PostRequest {
  body: string;
  files: string[];
  meta: object;
  timestamp: number;
}

export interface PostResponse {
  cid: string;
  files: string[];
}

export interface WebFeedLink {
  href: string;
  rel: string | null;
  media_type: string | null;
  href_lang: string | null;
  title: string | null;
  length: number | null;
}

export interface WebFeedImage {
  uri: string;
  title: string | null;
  link: WebFeedLink | null;
  width: number | null;
  height: number | null;
  description: string | null;
}

export interface WebFeed {
  id: string;
  title: string | null;
  updated: string | null;
  description: string | null;
  links: string[];
  publisher: string;
  published: string | null;
  logo: WebFeedImage | null;
  timestamp: number;
  entries: WebFeedEntry[];
}
export interface WebFeedEntry {
  // custom
  cid: string;
  display_name: string;
  publisher: string;
  publisher_links: string[];
  timestamp: number;
  // from feed-rs
  authors: WebFeedPerson[];
  categories: string[];
  content: string;
  contributors: WebFeedPerson[];
  id: string;
  links: string[];
  media: WebFeedMediaObject[];
  published: string | null;
  rights: string | null;
  source: string | null;
  summary: string;
  title: string | null;
  updated: string | null;
}
export interface WebFeedMediaCommunity {
  stars_avg: number | null;
  stars_count: number | null;
  stars_max: number | null;
  stars_min: number | null;
  stats_favorites: number | null;
  stats_views: number | null;
}

export interface WebFeedMediaCredit {
  entity: string;
}

export interface WebFeedMediaContent {
  url: string | null;
  content_type: string | null;
  height: number | null;
  width: number | null;
  duration: number | null;
  size: number | null;
  rating: string | null;
}

export interface WebFeedMediaObject {
  title: string | null;
  content: WebFeedMediaContent[];
  duration: number | null;
  thumbnails: WebFeedMediaThumbnail[];
  texts: string[];
  description: string | null;
  community: WebFeedMediaCommunity | null;
  credits: WebFeedMediaCredit[];
}

export interface WebFeedMediaThumbnail {
  time: number | null;
}

export interface WebFeedPerson {
  name: string;
  uri: string | null;
  email: string | null;
}
