.PHONY: build run

build:
	podman build -t esp-idf-rs .

 # The three lines after `run` are for granting access to the serial device.
 # Your user (outside the container) must have access to /dev/ttyACM0.
run:
	 podman run  \
	 --device /dev/ttyACM0 \
	 --annotation io.oci.keep_original_groups=1 \
   --security-opt label=disable \
	 -v /home/jens/git/esp-idf-rs:/project:Z \
	 -it localhost/esp-idf-rs

# can only run within container
objdump:
	 xtensa-esp32-elf-objdump -S -d -l .pio/build/debug/firmware.elf > firmware.elf.dump
