FROM ubuntu:24.04@sha256:008173c23f95b170204355c12626cb5a965d779a7e1283b09e9cffbb1bf33ca3

ARG RUST_TOOLCHAIN=1.92.0
ARG RUSTUP_VERSION=1.28.2
ARG RUSTUP_X86_64_SHA256=20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da0cdd43c
ARG RUSTUP_AARCH64_SHA256=e3853c5a252fca15252d07cb23a1bdd9377a8c6f3efa01531109281ae47f841c
ARG SCCACHE_VERSION=v0.16.0
ARG SCCACHE_X86_64_SHA256=aec995a83ad3dff3d14b6314e08858b7b73d35ca85a5bcf3d3a9ec07dee35588
ARG SCCACHE_AARCH64_SHA256=f73a5c39f96bb6ebb89cc7915cf182260d4cbf30765322c5e793d0fe8bd80784
ARG CARGO_NEXTEST_VERSION=0.9.140
ARG CARGO_NEXTEST_X86_64_SHA256=4ee9aaa0d0171a985a5d0eb735b87355894c1c455972e9674fb9fdbd1387c9a3
ARG CARGO_NEXTEST_AARCH64_SHA256=8b3f4d4560b6b0f83774fecc6be07e47716dbad0eb0bb6c3890f478f4affe4b6
ARG CARGO_LLVM_COV_VERSION=0.8.7
ARG CARGO_LLVM_COV_X86_64_SHA256=967b5cc996c29d8baa52bbb4595ef1f53af35255af8e2036ddbc6468d7b523c7
ARG CARGO_LLVM_COV_AARCH64_SHA256=b7bb2ad514166f3b19fc06874c577eefde0010e5ac571f9ae33ff45cd19a4785
ARG AWSCLI_VERSION=2.27.41
ARG AWSCLI_X86_64_SHA256=15daae6cc803984064e3d4be9cfd07c4ae8ea703633c0a0b67acc6e321f706a3
ARG AWSCLI_AARCH64_SHA256=2c6ed21cf7cff0a7d77118c69bee867128bf4c588db7b5c044ffba5faeb6ccde
ARG RUBY_VERSION=3.3.6
ARG RUBY_SOURCE_SHA256=540975969d1af42190d26ff629bc93b1c3f4bffff4ab253e245e125085e66266

ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:/usr/local/bin:/usr/bin:/bin
ENV CARGO_INCREMENTAL=0
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_CACHE_SIZE=20G
ENV RUSTFLAGS="-C link-arg=-fuse-ld=lld --remap-path-prefix=/workspace=/workspace --remap-path-prefix=/root=/home"

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends \
      bash \
      build-essential \
      ca-certificates \
      clang \
      curl \
      git \
      gh \
      jq \
      lld \
      openssh-client \
      pkg-config \
      python3 \
      tar \
      time \
      unzip \
      xz-utils \
      zstd \
      libreadline-dev \
      libssl-dev \
      libyaml-dev \
      zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

COPY adl/docker/adl-builder/verify_sha256.sh /usr/local/share/adl-verify-sha256.sh

RUN ruby_archive="ruby-${RUBY_VERSION}.tar.xz" \
    && ruby_url="https://cache.ruby-lang.org/pub/ruby/${RUBY_VERSION%.*}/${ruby_archive}" \
    && curl -fsSL "$ruby_url" -o "/tmp/${ruby_archive}" \
    && printf '%s  %s\n' "$RUBY_SOURCE_SHA256" "/tmp/${ruby_archive}" | sha256sum -c - \
    && mkdir -p /tmp/ruby-source \
    && tar -xJf "/tmp/${ruby_archive}" -C /tmp/ruby-source --strip-components=1 \
    && cd /tmp/ruby-source \
    && ./configure --disable-install-doc \
    && make -j"$(nproc)" \
    && make install \
    && ruby --version \
    && rm -rf /tmp/ruby-source "/tmp/${ruby_archive}"

RUN arch="$(uname -m)" \
    && case "$arch" in \
      x86_64) awscli_arch="x86_64"; awscli_sha256="$AWSCLI_X86_64_SHA256" ;; \
      aarch64|arm64) awscli_arch="aarch64"; awscli_sha256="$AWSCLI_AARCH64_SHA256" ;; \
      *) echo "unsupported awscli arch: $arch" >&2; exit 2 ;; \
    esac \
    && curl -fsSL "https://awscli.amazonaws.com/awscli-exe-linux-${awscli_arch}-${AWSCLI_VERSION}.zip" -o /tmp/awscliv2.zip \
    && bash /usr/local/share/adl-verify-sha256.sh "$awscli_sha256" /tmp/awscliv2.zip \
    && unzip -q /tmp/awscliv2.zip -d /tmp \
    && /tmp/aws/install \
    && rm -rf /tmp/aws /tmp/awscliv2.zip

RUN arch="$(uname -m)" \
    && case "$arch" in \
      x86_64) rustup_arch="x86_64-unknown-linux-gnu"; rustup_sha256="$RUSTUP_X86_64_SHA256" ;; \
      aarch64|arm64) rustup_arch="aarch64-unknown-linux-gnu"; rustup_sha256="$RUSTUP_AARCH64_SHA256" ;; \
      *) echo "unsupported rustup arch: $arch" >&2; exit 2 ;; \
    esac \
    && curl -fsSL "https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/${rustup_arch}/rustup-init" -o /tmp/rustup-init \
    && bash /usr/local/share/adl-verify-sha256.sh "$rustup_sha256" /tmp/rustup-init \
    && chmod 0755 /tmp/rustup-init \
    && /tmp/rustup-init -y --no-modify-path --profile minimal --default-toolchain "${RUST_TOOLCHAIN}" \
    && rm -f /tmp/rustup-init \
    && rustup component add rustfmt clippy llvm-tools-preview

RUN arch="$(uname -m)" \
    && case "$arch" in \
      x86_64) sccache_arch="x86_64-unknown-linux-musl"; sccache_sha256="$SCCACHE_X86_64_SHA256" ;; \
      aarch64|arm64) sccache_arch="aarch64-unknown-linux-musl"; sccache_sha256="$SCCACHE_AARCH64_SHA256" ;; \
      *) echo "unsupported sccache arch: $arch" >&2; exit 2 ;; \
    esac \
    && sccache_archive="sccache-${SCCACHE_VERSION}-${sccache_arch}.tar.gz" \
    && curl -fsSL "https://github.com/mozilla/sccache/releases/download/${SCCACHE_VERSION}/${sccache_archive}" -o "/tmp/${sccache_archive}" \
    && bash /usr/local/share/adl-verify-sha256.sh "$sccache_sha256" "/tmp/${sccache_archive}" \
    && tar -xzf "/tmp/${sccache_archive}" -C /tmp \
    && install -m 0755 "/tmp/sccache-${SCCACHE_VERSION}-${sccache_arch}/sccache" /usr/local/bin/sccache \
    && rm -rf "/tmp/${sccache_archive}" "/tmp/sccache-${SCCACHE_VERSION}-${sccache_arch}"

RUN arch="$(uname -m)" \
    && case "$arch" in \
      x86_64) nextest_arch="x86_64-unknown-linux-gnu"; nextest_sha256="$CARGO_NEXTEST_X86_64_SHA256" ;; \
      aarch64|arm64) nextest_arch="aarch64-unknown-linux-gnu"; nextest_sha256="$CARGO_NEXTEST_AARCH64_SHA256" ;; \
      *) echo "unsupported cargo-nextest arch: $arch" >&2; exit 2 ;; \
    esac \
    && nextest_archive="cargo-nextest-${CARGO_NEXTEST_VERSION}-${nextest_arch}.tar.gz" \
    && nextest_url="https://github.com/nextest-rs/nextest/releases/download/cargo-nextest-${CARGO_NEXTEST_VERSION}/${nextest_archive}" \
    && curl -fsSL "$nextest_url" -o "/tmp/${nextest_archive}" \
    && printf '%s  %s\n' "$nextest_sha256" "/tmp/${nextest_archive}" | sha256sum -c - \
    && mkdir -p /tmp/cargo-nextest \
    && tar -xzf "/tmp/${nextest_archive}" -C /tmp/cargo-nextest \
    && nextest_bin="$(find /tmp/cargo-nextest -type f -name cargo-nextest | head -n 1)" \
    && test -n "$nextest_bin" \
    && install -m 0755 "$nextest_bin" /usr/local/cargo/bin/cargo-nextest \
    && cargo nextest --version \
    && rm -rf /tmp/cargo-nextest "/tmp/${nextest_archive}"


RUN arch="$(uname -m)" \
    && case "$arch" in \
      x86_64) llvm_cov_arch="x86_64-unknown-linux-musl"; llvm_cov_sha256="$CARGO_LLVM_COV_X86_64_SHA256" ;; \
      aarch64|arm64) llvm_cov_arch="aarch64-unknown-linux-musl"; llvm_cov_sha256="$CARGO_LLVM_COV_AARCH64_SHA256" ;; \
      *) echo "unsupported cargo-llvm-cov arch: $arch" >&2; exit 2 ;; \
    esac \
    && llvm_cov_url="https://github.com/taiki-e/cargo-llvm-cov/releases/download/v${CARGO_LLVM_COV_VERSION}/cargo-llvm-cov-${llvm_cov_arch}.tar.gz" \
    && curl -fsSL "$llvm_cov_url" -o /tmp/cargo-llvm-cov.tar.gz \
    && bash /usr/local/share/adl-verify-sha256.sh "$llvm_cov_sha256" /tmp/cargo-llvm-cov.tar.gz \
    && mkdir -p /tmp/cargo-llvm-cov \
    && tar -xzf /tmp/cargo-llvm-cov.tar.gz -C /tmp/cargo-llvm-cov \
    && llvm_cov_bin="$(find /tmp/cargo-llvm-cov -type f -name cargo-llvm-cov | head -n 1)" \
    && test -n "$llvm_cov_bin" \
    && install -m 0755 "$llvm_cov_bin" /usr/local/cargo/bin/cargo-llvm-cov \
    && cargo llvm-cov --version \
    && rm -rf /tmp/cargo-llvm-cov /tmp/cargo-llvm-cov.tar.gz

RUN printf '%s\n' \
      "rustc=$(rustc --version)" \
      "cargo=$(cargo --version)" \
      "cargo-nextest=$(cargo nextest --version)" \
      "cargo-llvm-cov=$(cargo llvm-cov --version)" \
      "sccache=$(sccache --version)" \
      "lld=$(ld.lld --version | head -n 1)" \
      "aws=$(aws --version 2>&1)" \
      "ruby=$(ruby --version)" \
      "base=ubuntu:24.04@sha256:008173c23f95b170204355c12626cb5a965d779a7e1283b09e9cffbb1bf33ca3" \
      "rust-toolchain=${RUST_TOOLCHAIN}" \
      "rustup-sha256-x86_64=${RUSTUP_X86_64_SHA256}" \
      "rustup-sha256-aarch64=${RUSTUP_AARCH64_SHA256}" \
      "sccache-sha256-x86_64=${SCCACHE_X86_64_SHA256}" \
      "sccache-sha256-aarch64=${SCCACHE_AARCH64_SHA256}" \
      "nextest-sha256-x86_64=${CARGO_NEXTEST_X86_64_SHA256}" \
      "nextest-sha256-aarch64=${CARGO_NEXTEST_AARCH64_SHA256}" \
      "llvm-cov-sha256-x86_64=${CARGO_LLVM_COV_X86_64_SHA256}" \
      "llvm-cov-sha256-aarch64=${CARGO_LLVM_COV_AARCH64_SHA256}" \
      "awscli-sha256-x86_64=${AWSCLI_X86_64_SHA256}" \
      "awscli-sha256-aarch64=${AWSCLI_AARCH64_SHA256}" \
      "ruby-sha256=${RUBY_SOURCE_SHA256}" \
      > /usr/local/share/adl-builder-toolchain.txt

RUN gh --version

WORKDIR /workspace

ENTRYPOINT ["/bin/bash", "-lc"]
