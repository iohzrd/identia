<script lang="ts">
  import GenericMediaComponent from "./GenericMedia.svelte";
  import TimeagoComponent from "$lib/Timeago.svelte";
  import linkifyHtml from "linkify-html";
  import type { WebFeedEntry } from "$lib/types";
  import { ExpandableTile, Link } from "carbon-components-svelte";
  import { getLongestString } from "$lib/utils";
  import { stripHtml } from "string-strip-html";

  export let entry: WebFeedEntry;

  let stripOpts = {
    onlyStripTags: ["script", "style", "xml", "sandbox"],
    stripTogetherWithTheirContents: ["script", "style", "xml", "sandbox"],
  };

  let body: string = getLongestString([entry.content, entry.summary]);
  body = stripHtml(body, stripOpts).result;
  body = linkifyHtml(body, { target: "_blank" });
  body = body.replace(/\r\n?/g, "\n");
  body = body.replace(/\n+/g, "<br>");
  body = body.replaceAll("<br />", "<br>");
  let first_br = body.indexOf("<br>");
  let link = entry.links.length > 0 ? entry.links[0] : entry.publisher;
</script>

<TimeagoComponent timestamp={entry.timestamp} />
<br />
<Link size="lg" target="_blank" href={link}>
  {entry.title}
</Link>
<br />
{#if entry.media.length > 0}
  <GenericMediaComponent {entry} />
{/if}
<ExpandableTile tileExpandedLabel="Show less" tileCollapsedLabel="Show more">
  <div slot="above">
    {@html body.slice(0, first_br)}
  </div>
  <div slot="below">
    {@html body.slice(first_br, body.length)}
  </div>
</ExpandableTile>
