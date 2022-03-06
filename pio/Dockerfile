FROM espressif/idf-rust

ENV PLATFORMIO_VERSION="5.1.0"

# Install PlatformIO CLI
#RUN python3 -c "$(curl -fsSL https://raw.githubusercontent.com/platformio/platformio/master/scripts/get-platformio.py)"

RUN pip install -U platformio==${PLATFORMIO_VERSION} && \
  mkdir -p /.platformio && \
  chmod a+rwx /.platformio
  
WORKDIR /project 
RUN bash -c "rustup override set esp"
