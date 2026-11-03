FROM node:24.22.2-slim@sha256:c86ecead9e55a5210ac1727de38a4ca840bf03dcaf34dc9749addbf383b1920c
RUN npm install --global pnpm@10.18.3
WORKDIR /app
COPY pnpm-lock.yaml pnpm-workspace.yaml ./
COPY workload/package.json workload/
RUN pnpm install --frozen-lockfile --prod --filter rivet-chaos-workload
COPY workload/src workload/src
WORKDIR /app/workload
# node directly, not pnpm, so the process gets SIGTERM and can drain.
CMD ["node", "src/registry.ts"]
