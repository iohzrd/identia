import type { PageLoad } from "./$types";

export const load = (({ params }) => {
  return {
    topic: params.topic,
  };
}) satisfies PageLoad;
