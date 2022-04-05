export type Comment = {
  cid: string;
  body: string;
  from: string;
  in_response_to: string;
  kind: string;
  timestamp: number;
  topic: string;
};

export type Feed = {
  feed: Post[];
};

export type Identity = {
  cid: string;
  avatar: string;
  description: string;
  display_name: string;
  following: string[];
  meta: object;
  posts: string[];
  publisher: string;
  timestamp: number;
};

export type Media = {
  element: Element;
  filename: string;
  mime: string;
  thumbnailFor: string;
  url: string;
};

export type Post = {
  cid: string;
  body: string;
  display_name: string;
  files: string[];
  meta: object;
  publisher: string;
  timestamp: number;
};

export type PostRequest = {
  body: string;
  files: string[];
  meta: object;
  timestamp: number;
};

export type PostResponse = {
  cid: string;
  files: string[];
};
