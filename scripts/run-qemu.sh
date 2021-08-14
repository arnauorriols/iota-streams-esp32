# Flash
rm -f flash.bin
dd if=/dev/zero bs=1M count=4 of=./flash.bin
dd if=/opt/streams/build/bootloader/bootloader.bin bs=1 count=$(stat -c%s /opt/streams/build/bootloader/bootloader.bin) seek=$((16#1000)) conv=notrunc of=./flash.bin
dd if=/opt/streams/build/partition_table/partition-table.bin bs=1 count=$(stat -c%s /opt/streams/build/partition_table/partition-table.bin) seek=$((16#8000)) conv=notrunc of=./flash.bin
dd if=/opt/streams/build/iota-streams-esp32.bin bs=1 count=$(stat -c%s /opt/streams/build/iota-streams-esp32.bin) seek=$((16#10000)) conv=notrunc of=./flash.bin

# Run
qemu-system-xtensa -nographic -no-reboot -M esp32 -m 4 -drive file=flash.bin,if=mtd,format=raw -nic user,model=open_eth,hostfwd=tcp::80-:80
