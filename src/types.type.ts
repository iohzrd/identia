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
export type MediaObj = {
  blobUrl: string;
  element: Element;
  filename: string;
  thumbnailFor: string;
  mime: string;
};

export type MimeRequest = {
  data: any[];
};

export type Post = {
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
};

export type PostResponse = {
  cid: string;
  display_name: string;
  post: Post;
};
