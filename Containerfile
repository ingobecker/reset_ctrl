FROM rust

RUN apt-get update \
  && apt-get install -y pkg-config \
			libudev-dev \
  && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home dev
USER dev

WORKDIR /usr/src/app
COPY Cargo.* rust-toolchain.toml .
COPY .cargo/config.toml .cargo/config.toml
# HACK: only here so `cargo fetch` doesn't complain
COPY src/lib.rs src/lib.rs
COPY src/bin src/bin

RUN cargo fetch \
  && cargo install probe-rs --locked --features cli

LABEL SHELL="podman run \
	-it \
	--rm \
	--name reset-ctrl-rust \
	--userns keep-id \
	--group-add keep-groups \
	-v .:/usr/src/app:Z \
	IMAGE"

LABEL SHELLEM="podman run \
	-it \
	--rm \
	--name reset-ctrl-rust \
	--userns keep-id \
	--group-add keep-groups \
	--security-opt label=type:embedded_container.process \
        --device /dev/bus/usb \
	-v .:/usr/src/app:Z \
	IMAGE"
#trigger container rebuild1
