export type AuxObj = {
  key: string;
  value: string;
};

export type Identity = {
  aux: AuxObj[];
  av: string;
  dn: string;
  following: string[];
  meta: string[];
  posts: string[];
  publisher: string;
  ts: number;
};

export type Post = {
  aux: AuxObj[];
  body: string;
  files: string[];
  meta: string[];
  publisher: string;
  ts: number;
};

export type PostRequest = {
  aux: AuxObj[];
  body: string;
  files: string[];
  meta: string[];
};

export type PostResponse = {
  post: Post;
  cid: string;
};
