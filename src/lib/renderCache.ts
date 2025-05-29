const PAGE_TTL = 1000 * 60 * 5;
let store: Map<string, { expires: number; data: Response }> = new Map();
export default {
  async getOrResolve(url: string, resolver: () => Promise<Response>) {
    let res = store.get(url);

    if (res && res.expires > Date.now()) {
      return res.data.clone();
    }
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
