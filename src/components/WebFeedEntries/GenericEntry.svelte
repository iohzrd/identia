<script lang="ts">
  import GenericMediaComponent from "./GenericMedia.svelte";
  import linkifyHtml from "linkify-html";
  import type { WebFeedEntry } from "../../types";
  import { ExpandableTile } from "carbon-components-svelte";
  import { stripHtml } from "string-strip-html";

  export let entry: WebFeedEntry;
  let deleting: boolean = false;

  let stripOpts = {
    onlyStripTags: ["script", "style", "xml", "sandbox"],
    stripTogetherWithTheirContents: ["script", "style", "xml", "sandbox"],
  };

  // let possibleBodies = [entry.content, entry.description, entry.summary];
  let possibleBodies = [entry.content, entry.summary];
  let longestBody: string = possibleBodies.reduce(
    (savedText, text) => (text.length > savedText.length ? text : savedText),
    ""
  );
  longestBody = stripHtml(longestBody, stripOpts).result;
  longestBody = linkifyHtml(longestBody, { target: "_blank" });
  longestBody = longestBody.replace(/\r\n?/g, "\n");
  longestBody = longestBody.replace(/\n+/g, "<br>");
  let first_br = longestBody.indexOf("<br>");
</script>

<div>
  <br />
  {#if entry.media.length > 0}
    <GenericMediaComponent {entry} />
  {/if}
  <br />
  <ExpandableTile tileExpandedLabel="Show less" tileCollapsedLabel="Show more">
    <div slot="above">
      {@html longestBody.slice(0, first_br)}
    </div>
    <div slot="below">
      {@html longestBody.slice(first_br, longestBody.length)}
    </div>
  </ExpandableTile>
</div>
