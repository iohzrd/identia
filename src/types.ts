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
  feed: Post[];
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
