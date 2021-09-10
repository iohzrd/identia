export type Identity = {
  av: string;
  dn: string;
  following: string[];
  meta: object;
  posts: string[];
  publisher: string;
  ts: number;
};

export type Post = {
  body: string;
  files: string[];
  meta: object;
  publisher: string;
  ts: number;
};

export type PostRequest = {
  body: string;
  files: string[];
  meta: object;
};

export type PostResponse = {
  post: Post;
  cid: string;
};
