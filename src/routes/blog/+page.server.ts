import { BlogCache } from "$lib/blog";
export const load = async () => {
  await BlogCache.load();

  const blogPosts = BlogCache.getCachedData();
  blogPosts.sort((p1, p2) => {
    const date1 = p1.data.metadata.date as string | undefined;
    const date2 = p2.data.metadata.date as string | undefined;
    const ts1 = date1 ? new Date(date1).getTime() : Date.now();
    const ts2 = date2 ? new Date(date2).getTime() : Date.now();
    return ts2 - ts1;
  });
  return {
    blogPosts,
  };
};
