import { fetch, ResponseType } from "@tauri-apps/api/http";

export async function getYoutubeChannelID(url: string) {
  let resp = await fetch(url, {
    method: "GET",
    timeout: 30,
    responseType: ResponseType.Text,
  });
  let i0 = '"rssUrl":"';
  let i1 = String(resp.data).indexOf(i0);
  let i2 = String(resp.data).indexOf('"', i1 + i0.length);
  let rssUrl = String(resp.data).slice(i1 + i0.length, i2);
  return rssUrl;
}
