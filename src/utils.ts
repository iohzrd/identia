import { fetch, ResponseType } from "@tauri-apps/api/http";

export function getLongestString(a: string[]) {
  return a.reduce(
    (savedText, text) =>
      typeof text == "string" && text.length > savedText.length
        ? text
        : savedText,
    ""
  );
}

export async function getYoutubeChannelID(url: string) {
  let resp = await fetch(url, {
    method: "GET",
    timeout: 30,
    responseType: ResponseType.Text,
  });
  let m = '"rssUrl":"';
  let i1 = String(resp.data).indexOf(m);
  let i2 = String(resp.data).indexOf('"', i1 + m.length);
  let rssUrl = String(resp.data).slice(i1 + m.length, i2);
  return rssUrl;
}
