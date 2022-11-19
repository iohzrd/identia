<script lang="ts">
  import GenericMediaComponent from "./GenericMedia.svelte";
  import TimeagoComponent from "../Timeago.svelte";
  import linkifyHtml from "linkify-html";
  import type { WebFeedEntry } from "../../types";
  import { ExpandableTile, Link } from "carbon-components-svelte";
  import { stripHtml } from "string-strip-html";

  export let entry: WebFeedEntry;

  let stripOpts = {
    onlyStripTags: ["p"],
    stripTogetherWithTheirContents: ["p"],
  };

  let possibleBodies = [entry.content, entry.summary];
  // use the longer string as the body...
  let body: string = possibleBodies.reduce(
    (savedText, text) => (text.length > savedText.length ? text : savedText),
    ""
  );
  // this strips the thumbnail from the body
  body = stripHtml(body, stripOpts).result;
  body = linkifyHtml(body, { target: "_blank" });
  body = body.replace(/\r\n?/g, "\n");
  body = body.replace(/\n+/g, "<br>");
  body = body.replaceAll("<br />", "<br>");
  let first_br = body.indexOf("<br>");
</script>

<Link size="lg" target="_blank" href={entry.publisher}>
  {entry.display_name}
</Link> - <TimeagoComponent timestamp={entry.timestamp} />
<br />
<Link size="lg" target="_blank" href={entry.id}>
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
