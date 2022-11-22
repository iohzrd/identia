<script lang="ts">
  import LiteYouTube from "svelte-lite-youtube-embed";
  import TimeagoComponent from "../Timeago.svelte";
  import getVideoId from "get-video-id";
  import linkifyHtml from "linkify-html";
  import type { WebFeedEntry } from "../../types";
  import { ExpandableTile, Link } from "carbon-components-svelte";
  import { stripHtml } from "string-strip-html";

  export let entry: WebFeedEntry;

  let video_link: string = entry.links.length > 0 ? entry.links[0] : "";
  let video_id_obj = getVideoId(video_link);
  let description: string =
    entry.media.length > 0 && typeof entry.media[0].description == "string"
      ? entry.media[0].description
      : "";
  description = stripHtml(description).result;
  description = linkifyHtml(description, {
    nl2br: true,
    target: "_blank",
  });
  let first_br = description.indexOf("<br />");
</script>

<Link size="lg" target="_blank" href={entry.publisher}>
  {entry.display_name}
</Link> - <TimeagoComponent timestamp={entry.timestamp} />
<br />
<Link size="lg" target="_blank" href={video_link}>
  {entry.title}
</Link>
<br />
<div style="width: 400px;">
  <LiteYouTube videoId={video_id_obj.id} videoTitle={entry.title} />
</div>
<ExpandableTile tileExpandedLabel="Show less" tileCollapsedLabel="Show more">
  <div slot="above">
    {@html description.slice(0, first_br)}
  </div>
  <div slot="below">
    {@html description.slice(first_br, description.length)}
  </div>
</ExpandableTile>
