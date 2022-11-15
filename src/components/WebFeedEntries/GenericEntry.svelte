<script lang="ts">
  import linkifyHtml from "linkify-html";
  import type { WebFeedEntry } from "../../types";
  import { onMount, onDestroy } from "svelte";
  import { stripHtml } from "string-strip-html";

  export let entry: WebFeedEntry;

  let deleting: boolean = false;

  let stripOpts = {
    onlyStripTags: ["script", "style", "xml", "sandbox"],
    stripTogetherWithTheirContents: ["script", "style", "xml", "sandbox"],
  };
  let linkifyOpts = {
    target: "_blank",
  };
  let possibleBodies = [entry.content, entry.description, entry.summary];
  let bodyHTML: string = possibleBodies.reduce(
    (savedText, text) => (text.length > savedText.length ? text : savedText),
    ""
  );
  bodyHTML = bodyHTML.replace(/\n/g, "<br>");
  bodyHTML = stripHtml(bodyHTML, stripOpts).result;
  bodyHTML = linkifyHtml(bodyHTML, linkifyOpts);

  onMount(async () => {});

  onDestroy(() => {});
</script>

<div>
  <br />
  {@html bodyHTML}
</div>
