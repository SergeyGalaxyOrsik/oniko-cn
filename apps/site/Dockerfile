# The oniko-cn website, served at oniko.orsik.site/ui. Build from the repository root:
#   docker build -f apps/site/Dockerfile -t oniko-site .
FROM rust:1-bookworm AS build
RUN rustup target add wasm32-unknown-unknown \
    && cargo install dioxus-cli --version 0.7.10 --locked
WORKDIR /src
COPY . .
# dx gives up if `cargo metadata` has to download the dependencies first.
RUN cargo fetch --locked
RUN cd apps/site && dx bundle --web --release --out-dir /out

FROM nginx:1.29-alpine
COPY apps/site/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /out/public /usr/share/nginx/html/ui
COPY public/r /usr/share/nginx/html/ui/r
