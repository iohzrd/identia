import type { PageLoad } from "./$types";

export const load = (({ params }) => {
  return {
    cid: params.cid,
  };
}) satisfies PageLoad;
