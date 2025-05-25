const PAGE_TTL = 1000 * 60 * 5;
let streak = 0;
let store: Map<string, { expires: number; data: Response }> = new Map();
export default {
  async getOrResolve(url: string, resolver: () => Promise<Response>) {
    let res = store.get(url);

    if (res && res.expires > Date.now()) {
      streak += 1;
      console.log(`Renderer cache hit! Streak: ${streak}`);
      return res.data.clone();
    }
    console.log(`Renderer cache fail! Streak reset.`);
    streak = 0;
    res = {
      expires: Date.now() + PAGE_TTL,
      data: await resolver(),
    };
    store.set(url, res);
    return res.data.clone();
  },
  clear() {
    store = new Map();
  },
};
