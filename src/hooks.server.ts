import type { Handle } from "@sveltejs/kit";
import renderCache from "$lib/renderCache";
export const handle: Handle = async ({ event, resolve }) => {
  const resolver = async () => {
    const response = await resolve(event);
    return response;
  };
  if (event.request.method == "GET") {
    return await renderCache.getOrResolve(event.request.url, resolver);
  }
  const response = await resolve(event);
  return response;
};
