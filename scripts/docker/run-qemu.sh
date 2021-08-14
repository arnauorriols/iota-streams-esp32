docker run -it -v $(pwd):/opt/streams --workdir /opt/streams --rm --name esp-qemu mluis/qemu-esp32 /bin/bash -c '/opt/streams/scripts/run-qemu.sh'
