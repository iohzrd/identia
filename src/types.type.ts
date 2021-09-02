export type Identity = {
  aux: object[];
  av: string;
  dn: string;
  following: string[];
  meta: string[];
  posts: string[];
  publisher: string;
  ts: number;
};

export type Post = {
  aux: object[];
  body: string;
  files: string[];
  meta: string[];
  publisher: string;
  ts: number;
};

export type PostResponse = {
  post: Post;
  cid: string;
};
