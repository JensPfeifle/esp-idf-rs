FROM espressif/idf-rust

# Install addiitonal dependencies
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC
RUN apt-get update && apt-get install -y \
  pkg-config \
  && rm -rf /var/lib/apt/lists/*

# Install PlatformIO CLI
ENV PLATFORMIO_VERSION="5.1.0"

RUN pip install -U platformio==${PLATFORMIO_VERSION} && \
  mkdir -p /.platformio && \
  chmod a+rwx /.platformio

WORKDIR /project 
RUN bash -c "rustup override set esp"
