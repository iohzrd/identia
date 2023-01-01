import type { PageLoad } from "./$types";

export const load = (({ params }) => {
  return {
    b64_publisher: params.b64_publisher,
  };
}) satisfies PageLoad;
