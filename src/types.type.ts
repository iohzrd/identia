export type AddObj = {
  path: string;
  content: string;
};

export type Comment = {
  body: string;
  cid: string;
  from: string;
  kind: string;
  in_response_to: string;
  topic: string;
  timestamp: number;
};

export type Feed = {
  feed: Post[];
};

export type Identity = {
  avatar: string;
  description: string;
  display_name: string;
  following: string[];
  meta: object;
  posts: string[];
  publisher: string;
  timestamp: number;
};

export type IdentityRequest = {
  publisher: string;
};

export type IdentityResponse = {
  cid: string;
  identity: Identity;
};

export type MediaResponse = {
  data: any[];
  ext: string;
  mime: string;
};

export type Media = {
  blobUrl: string;
  element: Element;
  filename: string;
  thumbnailFor: string;
  mime: string;
};

export type Post = {
  cid: string;
  display_name: string;
  body: string;
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
