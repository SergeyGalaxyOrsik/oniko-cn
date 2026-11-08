# The oniko-cn website, served at oniko.orsik.site/ui. Build from the repository root:
#   docker build -f apps/site/Dockerfile -t oniko-site .
# trixie: the wasm-bindgen binary that dx downloads needs glibc 2.38 or newer.
FROM rust:1-trixie AS build
ENV CARGO_NET_RETRY=10 CARGO_HTTP_TIMEOUT=300
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
