export interface Comment {
  cid: string;
  body: string;
  from: string;
  in_response_to: string;
  kind: string;
  timestamp: number;
  topic: string;
}

export interface Feed {
  feed: (Post | WebFeedEntry)[];
}

export interface Identity {
  cid: string;
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
  mime: string;
  thumbnailFor: string;
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

export interface WebFeed {
  body: string;
  entries: WebFeedEntry[];
  id: string;
  title: string;
  url: string;
  timestamp: number;
}

export interface WebFeedEntry {
  // custom
  cid: string;
  description: string;
  publisher: string;
  publisher_url: string;
  timestamp: number;
  // from feed-rs
  authors: WebFeedPerson[];
  categories: string[];
  content: string;
  contributors: WebFeedPerson[];
  id: string;
  links: string[];
  // media: TODO
  published: string | null;
  rights: string | null;
  source: string | null;
  summary: string;
  title: string | null;
  updated: string | null;
}

export interface WebFeedPerson {
  name: string;
  uri: string | null;
  email: string | null;
}
