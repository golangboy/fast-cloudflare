FROM rust:alpine3.18 as builder
WORKDIR /app
RUN apk add --no-cache musl-dev
RUN apk add --no-cache pkgconfig
RUN apk add --no-cache libressl-dev
COPY . .
RUN ["cargo","build","--release"]

FROM alpine
WORKDIR /app
COPY --from=builder /app/target/release/fast-cf /app
COPY --from=builder /app/docker-entrypoint.sh /app
COPY --from=builder /app/cloudflare_ipv4.txt /app
RUN ["chmod", "+x" ,"/app/docker-entrypoint.sh"]
CMD [ "/app/docker-entrypoint.sh" ]