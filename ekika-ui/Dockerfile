FROM node:21-bookworm-slim as builder
WORKDIR /work
RUN npm install -g pnpm
COPY ekika-ui /work
RUN pnpm install
RUN pnpm run build

FROM nginx:1.25
COPY ekika-ui/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /work/dist /www
