FROM espressif/idf:release-v4.4 as idf

ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo
ENV PATH=/opt/cargo/bin:/opt/rustup/bin:/opt/esp/tools/xtensa-esp32-elf-clang/esp-14.0.0-20220415-x86_64-unknown-linux-gnu/bin/:$PATH

WORKDIR /opt

ADD https://raw.githubusercontent.com/esp-rs/rust-build/main/install-rust-toolchain.sh ./install-rust-toolchain.sh
RUN chmod a+x ./install-rust-toolchain.sh
RUN ./install-rust-toolchain.sh



FROM idf

# Install addiitonal dependencies
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC
RUN apt-get update && apt-get install -y \
  libssl-dev \
  pkg-config \
  && rm -rf /var/lib/apt/lists/*

# Install PlatformIO CLI
#ENV PLATFORMIO_VERSION="6.0.1"
#
#RUN pip install -U platformio==${PLATFORMIO_VERSION} && \
#  mkdir -p /.platformio && \
#  chmod a+rwx /.platformio

COPY root_bashrc /root/
RUN  cat /root/root_bashrs >> /root/.bashrc

WORKDIR /project
RUN bash -c "rustup override set esp"
RUN bash -c "cargo install cargo-pio"
