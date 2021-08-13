```
Total sizes:
 DRAM .data size:    8636 bytes
 DRAM .bss  size:    2304 bytes
Used static DRAM:   10940 bytes ( 169796 available, 6.1% used)
Used static IRAM:   41382 bytes (  89690 available, 31.6% used)
      Flash code:  275411 bytes
    Flash rodata:  139848 bytes
Total image size:~ 465277 bytes (.bin may be padded larger)
Per-archive contributions to ELF file:
            Archive File DRAM .data & .bss & other   IRAM   D/IRAM Flash code & rodata   Total
            librustlib.a         32     48       0      0        0     149937   109381  259398
                  libc.a          4      0       0      0        0      56238     6652   62894
librustlib.a(rustlib-2c1          0      0       0      0        0      37415      280   37695
           libfreertos.a       3104    740       0  12226        0        200     3456   19726
          libspi_flash.a       1558    294       0   6772        0       1386     2108   12118
               libheap.a       2009      8       0   5296        0       1382     2189   10884
         libesp_system.a        136    198       0   3082        0       4062     2988   10466
         libesp_common.a          0      0       0      0        0         48     6975    7023
     libesp_hw_support.a        124     34       0   3406        0       2808      448    6820
                libvfs.a        308     48       0      0        0       4931      380    5667
                libhal.a          0      0       0   3642        0       1315      560    5517
             libdriver.a         72     53       0      0        0       2913      632    3670
             libnewlib.a        190    440       0   1130        0        913      331    3004
          libesp_timer.a         32     24       0    770        0        809      450    2085
            libpthread.a         16     12       0    220        0       1365      455    2068
              libefuse.a         28      4       0      0        0       1111      723    1866
librustlib.a(rustlib-2c1          0      0       0      0        0       1690       83    1773
            libesp_ipc.a         28     88       0    642        0        620      282    1660
 libbootloader_support.a          0      0       0   1319        0        119        0    1438
        libesp_ringbuf.a          0      0       0    729        0          0      691    1420
librustlib.a(rustlib-2c1          0      0       0      0        0       1178      161    1339
librustlib.a(rustlib-2c1          0      0       0      0        0       1133      122    1255
             libxtensa.a       1024      0       0     77        0        114       35    1250
                liblog.a          8    272       0    232        0        413      120    1045
librustlib.a(rustlib-2c1          0      4       0      0        0        981       32    1017
librustlib.a(compiler_bu          0      0       0      0        0        693        0     693
             libxt_hal.a          0      0       0    443        0          0       32     475
         libapp_update.a          1     12       0    162        0        161      124     460
               libmain.a          0      0       0      0        0        130      131     261
            libesp_rom.a          0      0       0     90        0          0        0      90
               libclib.a          0      0       0      0        0         22       15      37
                   (exe)          0      0       0      3        0          3       12      18
             libesp_pm.a          0      0       0      0        0          8        0       8
                libcxx.a          0      0       0      0        0          5        0       5
                libgcc.a          0      0       0      0        0          0        0       0
         libmbedcrypto.a          0      0       0      0        0          0        0       0
                libsoc.a          0      0       0      0        0          0        0       0
```