<script lang="ts">
  import LiteYouTube from "svelte-lite-youtube-embed";
  import TimeagoComponent from "$lib/Timeago.svelte";
  import getVideoId from "get-video-id";
  import linkifyHtml from "linkify-html";
  import type { WebFeedEntry } from "$lib/types";
  import { ExpandableTile, Link } from "carbon-components-svelte";
  import { stripHtml } from "string-strip-html";

  interface Props {
    entry: WebFeedEntry;
  }

  let { entry }: Props = $props();

  let video_link: string = entry.links.length > 0 ? entry.links[0] : "";
  let video_id_obj = getVideoId(video_link);
  let description: string =
    $state(entry.media.length > 0 && typeof entry.media[0].description == "string"
      ? entry.media[0].description
      : "");
  description = stripHtml(description).result;
  description = linkifyHtml(description, {
    nl2br: true,
    target: "_blank",
  });
  let first_br = description.indexOf("<br />");
</script>

<TimeagoComponent timestamp={entry.timestamp} />
<br />
<Link size="lg" target="_blank" href={video_link}>
  {entry.title}
</Link>
<br />
<div style="width: 400px;">
  <LiteYouTube videoId={video_id_obj.id} videoTitle={entry.title} />
</div>
<ExpandableTile tileExpandedLabel="Show less" tileCollapsedLabel="Show more">
  {#snippet above()}
    <div >
      {@html description.slice(0, first_br)}
    </div>
  {/snippet}
  {#snippet below()}
    <div >
      {@html description.slice(first_br, description.length)}
    </div>
  {/snippet}
</ExpandableTile>
