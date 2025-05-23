import { render } from "svelte/server";
import { error } from "@sveltejs/kit";
export const load = async ({ params, url }) => {
  try {
    const {
      default: post,
      metadata: { title, date },
    } = await import(`$remote/blog/+${params.slug}.svx`);
    return {
      content: render(post),
      title,
      date,
      renderHeader: !url.searchParams.has("noheader"),
    };
  } catch {
    error(404, {
      message: "Not found",
    });
  }
};
