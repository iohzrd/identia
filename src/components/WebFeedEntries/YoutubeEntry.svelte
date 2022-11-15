<script lang="ts">
  import LiteYouTube from "svelte-lite-youtube-embed";
  import getVideoId from "get-video-id";
  import linkifyHtml from "linkify-html";
  import type { WebFeedEntry } from "../../types";
  import { ExpandableTile } from "carbon-components-svelte";
  import { stripHtml } from "string-strip-html";

  export let entry: WebFeedEntry;

  let video_id: string = entry.links.length > 0 ? entry.links[0] : "";
  let video_id_obj = getVideoId(video_id);

  let description = stripHtml(entry.description).result;
  description = linkifyHtml(description, {
    nl2br: true,
    target: "_blank",
  });
  let first_br = description.indexOf("<br />");
</script>

<div style="width: 400px;">
  <LiteYouTube videoId={video_id_obj.id} videoTitle={entry.title} />
</div>
<br />
<ExpandableTile tileExpandedLabel="Show less" tileCollapsedLabel="Show more">
  <div slot="above">
    <!-- {@html description.slice(0, first_br)} -->
  </div>
  <div slot="below">
    <!-- {@html description.slice(first_br, description.length)} -->
    {@html description}
  </div>
</ExpandableTile>

<style>
</style>
