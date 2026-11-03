# effects-sink and mock-llm in one image; Compose picks the binary per service.
FROM rust:1.99.1-slim-bookworm@sha256:24707e70a4f1e23d9a7b25202ec43126d28a52294b96aad5c670df1fd1f1d400 AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY crates crates
RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/src/target \
	cargo build --release --locked -p effects-sink -p mock-llm \
	&& cp target/release/effects-sink target/release/mock-llm /usr/local/bin/

FROM debian:bookworm-slim@sha256:99c0b25c46d0214d5a2091092e7908907dcaecd6a962cde0ac700acee222b6f3
COPY --from=build /usr/local/bin/effects-sink /usr/local/bin/mock-llm /usr/local/bin/
