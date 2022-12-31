import type { PageLoad } from "./$types";

export const load = (({ params }) => {
  return {
    publisher: params.publisher,
  };
}) satisfies PageLoad;
