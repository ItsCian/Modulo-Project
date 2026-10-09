# syntax=docker/dockerfile:1

# ---------- Stage 1: build ----------
FROM oven/bun:1 AS builder
WORKDIR /app
COPY package.json bun.lock* .npmrc* ./
RUN bun install --frozen-lockfile
COPY . .
RUN bun run build

# ---------- Stage 2: runtime ----------
FROM node:22-bookworm-slim AS runtime
WORKDIR /app
ENV NODE_ENV=production PORT=3000

COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./package.json

USER node
EXPOSE 3000

HEALTHCHECK --interval=10s --timeout=3s --start-period=10s --retries=5 \
  CMD node -e "fetch('http://localhost:3000').then(r=>process.exit(r.ok?0:1)).catch(()=>process.exit(1))"

CMD ["node", "build"]