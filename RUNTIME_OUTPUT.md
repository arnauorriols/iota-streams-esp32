```
I (27) boot: ESP-IDF v4.4-dev-3235-g3e370c4296 2nd stage bootloader
I (27) boot: compile time 00:17:31
I (27) boot: chip revision: 0
I (32) boot.esp32: SPI Speed      : 40MHz
I (36) boot.esp32: SPI Mode       : DIO
I (41) boot.esp32: SPI Flash Size : 4MB
I (45) boot: Enabling RNG early entropy source...
I (51) boot: Partition Table:
I (54) boot: ## Label            Usage          Type ST Offset   Length
I (62) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (69) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (77) boot:  2 factory          factory app      00 00 00010000 00100000
I (84) boot: End of partition table
I (88) esp_image: segment 0: paddr=00010020 vaddr=3f400020 size=2592ch (153900) map
I (152) esp_image: segment 1: paddr=00035954 vaddr=3ffb0000 size=034b4h ( 13492) load
I (158) esp_image: segment 2: paddr=00038e10 vaddr=40080000 size=07208h ( 29192) load
I (170) esp_image: segment 3: paddr=00040020 vaddr=400d0020 size=52804h (337924) map
I (293) esp_image: segment 4: paddr=0009282c vaddr=40087208 size=034d4h ( 13524) load
I (299) esp_image: segment 5: paddr=00095d08 vaddr=50000000 size=00010h (    16) load
I (305) boot: Loaded app from partition at offset 0x10000
I (305) boot: Disabling RNG early entropy source...
I (321) cpu_start: Pro cpu up.
I (322) cpu_start: Starting app cpu, entry point is 0x4008112c
0x4008112c: call_start_cpu1 at /opt/esp/idf/components/esp_system/port/cpu_start.c:155

I (0) cpu_start: App cpu up.
I (336) cpu_start: Pro cpu start user code
I (336) cpu_start: cpu freq: 240000000
I (336) cpu_start: Application information:
I (340) cpu_start: Project name:     iota-streams-esp32
I (346) cpu_start: App version:      4d359f2-dirty
I (352) cpu_start: Compile time:     Oct 14 2021 00:18:07
I (358) cpu_start: ELF file SHA256:  83030ec8560e37eb...
I (364) cpu_start: ESP-IDF:          v4.4-dev-3235-g3e370c4296
I (371) heap_init: Initializing. RAM available for dynamic allocation:
I (378) heap_init: At 3FFAE6E0 len 00001920 (6 KiB): DRAM
I (384) heap_init: At 3FFB4140 len 0002BEC0 (175 KiB): DRAM
I (390) heap_init: At 3FFE0440 len 00003AE0 (14 KiB): D/IRAM
I (396) heap_init: At 3FFE4350 len 0001BCB0 (111 KiB): D/IRAM
I (403) heap_init: At 4008A6DC len 00015924 (86 KiB): IRAM
I (410) spi_flash: detected chip: winbond
I (414) spi_flash: flash io: dio
I (418) esp_core_dump_uart: Init core dump to UART
I (423) cpu_start: Starting scheduler on PRO CPU.
I (0) cpu_start: Starting scheduler on APP CPU.
This is esp32 chip with 2 CPU cores, WiFi/BT/BLE, silicon revision 0, 4MB external flash
Free heap: 261352
STARTING STREAMS IN ESP-32 QEMU!
NEW BUCKET TRANSPORT INSTANTIATED
Free heap: 252452
GENERATED SEED 'd3976a749c4201830a4913535032a1e3b3a269f41198989fa13ffdc6a5df0edc' for author
Free heap: 252388
NEW AUTHOR INSTANTIATED: <7c8fb7ea1aa59df8861aba7dceec2dcc250c0e42f087dbf89243c160fd1a9870>
    <7c8fb7ea1aa59df8861aba7dceec2dcc250c0e42f087dbf89243c160fd1a9870> => <f9ffcd31325e70463f6f144b,0:2>

Free heap: 251344
GENERATED SEED 'a793bd21c1c8f380ea9762ce495415f1e042a07c2cb19c0eecf3a738e7019afe' for subscriber
Free heap: 251280
NEW SUBSCRIBER INSTANTIATED: <77216eb677c7558978731816fd1370c17bf37bbc1ec3e04bbb58508cecde65df>

Free heap: 251268
ANNOUNCEMENT MESSAGE SENT: TangleAddress { appinst: AppInst { id: NBytes([124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112, 0, 0, 0, 0, 0, 0, 0, 0]) }, msgid: MsgId { id: NBytes([249, 255, 205, 49, 50, 94, 112, 70, 63, 111, 20, 75]) } }
Free heap: 250548
SUBSCRIBER RECEIVED ANNOUNCEMENT LINK OOB AND RETRIEVED ANNOUNCEMENT MESSAGE
Free heap: 249328
SUBSCRIBER SUBSCRIPTION MESSAGE SENT: TangleAddress { appinst: AppInst { id: NBytes([124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112, 0, 0, 0, 0, 0, 0, 0, 0]) }, msgid: MsgId { id: NBytes([105, 156, 145, 152, 74, 176, 35, 49, 167, 139, 254, 200]) } }
Free heap: 248928
AUTHOR RECEIVED SUBSCRIPTION MESSAGE
AUTHOR SYNC'ED STATE
Free heap: 248928
AUTHOR SENT KEYLOAD TO EVERYONE: TangleAddress { appinst: AppInst { id: NBytes([124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112, 0, 0, 0, 0, 0, 0, 0, 0]) }, msgid: MsgId { id: NBytes([94, 63, 199, 144, 20, 36, 209, 35, 165, 53, 54, 37]) } }
Free heap: 247684
SIGNED PACKET SENT: TangleAddress { appinst: AppInst { id: NBytes([124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112, 0, 0, 0, 0, 0, 0, 0, 0]) }, msgid: MsgId { id: NBytes([172, 65, 83, 159, 130, 23, 15, 221, 65, 113, 238, 114]) } }
Free heap: 247028
SIGNED PACKET SENT: TangleAddress { appinst: AppInst { id: NBytes([124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112, 0, 0, 0, 0, 0, 0, 0, 0]) }, msgid: MsgId { id: NBytes([133, 13, 61, 129, 123, 61, 81, 227, 129, 82, 140, 52]) } }
Free heap: 245480
MESSAGES RECEIVED BY SUBSCRIBER:
[
    @TangleAddress { appinst: AppInst { id: NBytes([124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112, 0, 0, 0, 0, 0, 0, 0, 0]) }, msgid: MsgId { id: NBytes([94, 63, 199, 144, 20, 36, 209, 35, 165, 53, 54, 37]) } }[Keyload]->TangleAddress { appinst: AppInst { id: NBytes([124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112, 0, 0, 0, 0, 0, 0, 0, 0]) }, msgid: MsgId { id: NBytes([249, 255, 205, 49, 50, 94, 112, 70, 63, 111, 20, 75]) } },
    @TangleAddress { appinst: AppInst { id: NBytes([124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112, 0, 0, 0, 0, 0, 0, 0, 0]) }, msgid: MsgId { id: NBytes([172, 65, 83, 159, 130, 23, 15, 221, 65, 113, 238, 114]) } }[SignedPacket { pk: PublicKey(CompressedEdwardsY: [124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112]), EdwardsPoint{
    	X: FieldElement51([24733988183662, 1883476240945311, 501857644971857, 1662686669458509, 1508848196790034]),
    	Y: FieldElement51([1588909909184380, 509860992048915, 61781346138035, 503011231397921, 1980777442446356]),
    	Z: FieldElement51([1, 0, 0, 0, 0]),
    	T: FieldElement51([1251962173536019, 1413515129951389, 1655286767461770, 1418172370839492, 2225848386083331])
    }), public_payload: Bytes([112, 117, 98, 108, 105, 99, 32, 112, 97, 121, 108, 111, 97, 100]), masked_payload: Bytes([101, 110, 99, 114, 121, 116, 101, 100, 32, 112, 97, 121, 108, 111, 97, 100]) }]->TangleAddress { appinst: AppInst { id: NBytes([124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112, 0, 0, 0, 0, 0, 0, 0, 0]) }, msgid: MsgId { id: NBytes([94, 63, 199, 144, 20, 36, 209, 35, 165, 53, 54, 37]) } },
    @TangleAddress { appinst: AppInst { id: NBytes([124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112, 0, 0, 0, 0, 0, 0, 0, 0]) }, msgid: MsgId { id: NBytes([133, 13, 61, 129, 123, 61, 81, 227, 129, 82, 140, 52]) } }[SignedPacket { pk: PublicKey(CompressedEdwardsY: [124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112]), EdwardsPoint{
    	X: FieldElement51([24733988183662, 1883476240945311, 501857644971857, 1662686669458509, 1508848196790034]),
    	Y: FieldElement51([1588909909184380, 509860992048915, 61781346138035, 503011231397921, 1980777442446356]),
    	Z: FieldElement51([1, 0, 0, 0, 0]),
    	T: FieldElement51([1251962173536019, 1413515129951389, 1655286767461770, 1418172370839492, 2225848386083331])
    }), public_payload: Bytes([112, 117, 98, 108, 105, 99, 32, 112, 97, 121, 108, 111, 97, 100, 32, 50]), masked_payload: Bytes([101, 110, 99, 114, 121, 116, 101, 100, 32, 112, 97, 121, 108, 111, 97, 100, 32, 50]) }]->TangleAddress { appinst: AppInst { id: NBytes([124, 143, 183, 234, 26, 165, 157, 248, 134, 26, 186, 125, 206, 236, 45, 204, 37, 12, 14, 66, 240, 135, 219, 248, 146, 67, 193, 96, 253, 26, 152, 112, 0, 0, 0, 0, 0, 0, 0, 0]) }, msgid: MsgId { id: NBytes([172, 65, 83, 159, 130, 23, 15, 221, 65, 113, 238, 114]) } },
]
Free heap: 243528

```
