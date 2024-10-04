import { fetch } from "@tauri-apps/plugin-http";

export async function getYoutubeChannelID(url: string) {
  let resp = await fetch(url, {
    method: "GET",
  });
  let i0 = '"rssUrl":"';
  let i1 = String(resp.body).indexOf(i0);
  let i2 = String(resp.body).indexOf('"', i1 + i0.length);
  let rssUrl = String(resp.body).slice(i1 + i0.length, i2);
  return rssUrl;
}
