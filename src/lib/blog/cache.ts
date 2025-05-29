import { parse } from "yaml";
import { readdir, readFile } from "fs/promises";
import { pathFromProjectRoot } from "$lib/util";
// we dont expect these to change often, so we can have a long TTL
const TTL = 1000 * 60 * 60;
const THRESHOLD_TTL = 1000 * 60 * 5;
/// sources to load blog posts from
const SOURCES = ["remote/blog"];
let store: {
  path: string;
  visible: boolean;
  data: {
    metadata: Record<string, unknown>;
    context: string;
  };
  expires: number;
}[] = [];
const filesThreshold = {
  threshold: 100,
  expires: 0,
};
export default {
  async load() {
    this.clean();
    await this.setThreshold();

    // we have now confirmed that both the store and threshold
    // are valid
    if (store.length >= filesThreshold.threshold) {
      // console.info(`cache length >= threshold of ${filesThreshold.threshold}`);
      return;
    }
    // console.warn(`Reloading cache, cache.length = ${store.length}`);
    // find all posts and get their data
    const postsSrc = (
      await Promise.all(
        SOURCES.map(async (it) => {
          try {
            const files = await readdir(pathFromProjectRoot(it));
            return files.map((that) => {
              return pathFromProjectRoot(it, that);
            });
          } catch {
            return [];
          }
        }),
      )
    ).flat();
    const posts = (
      await Promise.all(
        postsSrc.map(async (it) => {
          try {
            let fileContents = (await readFile(it)).toString();
            let metadata = {} as Record<string, unknown>;
            if (fileContents.includes("---")) {
              // assume (i control all input to this so its ok)
              const frontmatter = fileContents.split("---")[1];
              // completely honest, this will work. most things
              // wont have more than the initial two markers
              // and the things that do wont have them frequently
              // enough to justify having a higher splice number
              // when the primary use of this is just making previews
              fileContents = fileContents.split("---").splice(2, 5).join("---");
              metadata = parse(frontmatter);
            }
            return {
              path: it,
              visible: !metadata.private,
              data: {
                metadata,
                context: fileContents.slice(0, 150), // first 150 chars should be enough for previews
              },
              expires: Date.now() + TTL,
            };
          } catch (e) {
            console.warn(`Error while reading blog post: ${e}`);
            return null;
          }
        }),
      )
    ).filter((it) => !!it);
    if (posts.length < filesThreshold.threshold) {
      filesThreshold.threshold = posts.length * 0.8;
    }
    store = posts;
  },
  async setThreshold() {
    if (filesThreshold.expires < Date.now()) return;
    filesThreshold.expires = Date.now() + THRESHOLD_TTL;
    console.info("fetching files threshold");
    let thresh = -2;
    for (const source of SOURCES) {
      const dir = await readdir(pathFromProjectRoot(source));
      thresh += dir.filter(
        (it) => it.startsWith("+") && it.endsWith(".svx"),
      ).length;
    }
    filesThreshold.threshold = Math.max(thresh, 1);
  },
  getCachedData() {
    return store;
  },
  clean() {
    store = store.filter((it) => it.expires > Date.now());
  },
  clear() {
    store = [];
  },
};
