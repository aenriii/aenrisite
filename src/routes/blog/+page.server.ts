import { BlogCache } from "$lib/blog";
export const load = async (opts) => {
  await BlogCache.load();

  let blogPosts = BlogCache.getCachedData();
  blogPosts.sort((p1, p2) => {
    let date1 = p1.data.metadata.date as string | undefined;
    let date2 = p2.data.metadata.date as string | undefined;
    let ts1 = date1 ? new Date(date1).getTime() : Date.now();
    let ts2 = date2 ? new Date(date2).getTime() : Date.now();
    return ts2 - ts1;
  });
  return {
    blogPosts,
  };
};
