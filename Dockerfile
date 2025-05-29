FROM oven/bun:slim AS dependency_cache

COPY package.json ./
COPY bun.lock ./
RUN bun install

FROM dependency_cache
COPY . ./
RUN bun run build
ENTRYPOINT ["bun", "build/index.js"]
