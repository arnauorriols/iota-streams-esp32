```
Total sizes:
Used static DRAM:   16700 bytes ( 164036 remain, 9.2% used)
      .data size:   13492 bytes
      .bss  size:    3208 bytes
Used static IRAM:   42715 bytes (  88357 remain, 32.6% used)
      .text size:   41688 bytes
   .vectors size:    1027 bytes
Used stat D/IRAM:   59415 bytes ( 252393 remain, 19.1% used)
      .data size:   13492 bytes
      .bss  size:    3208 bytes
      .text size:   41688 bytes
   .vectors size:    1027 bytes
Used Flash size :  491823 bytes
      .text     :  337923 bytes
      .rodata   :  153644 bytes
Total image size:  604237 bytes (.bin may be padded larger)
Per-archive contributions to ELF file:
            Archive File DRAM .data .rtc.data DRAM .bss IRAM0 .text & 0.vectors ram_st_total Flash .text & .rodata & .appdesc flash_total
            librustlib.a         32         0        48           0           0           80      229800    114356          0      344188
                  libc.a          4         0         0           0           0            4       56102      6472          0       62578
librustlib.a(rustlib-1ce          0         0         0           0           0            0       15896      7506          0       23402
           libfreertos.a       4224         0       740       12396         425        17785         192      3713          0       20950
          libspi_flash.a       1451         0       294        6808           0         8553        1465      2251          0       11975
               libheap.a       2011         0         8        5237           0         7256        1390      2217          0       10855
        libespcoredump.a       3732         0       892           0           0         4624        5888       865          0       10485
         libesp_system.a        136         0       199        3071           0         3406        4069      3044          0       10320
         libesp_common.a          0         0         0           0           0            0          48      7023          0        7071
     libesp_hw_support.a        124        16        34        3465           0         3639        2804       451          0        6860
                libhal.a          0         0         0        3827           0         3827        1323       568          0        5718
                libvfs.a        308         0        48           0           0          356        4975       379          0        5662
             libdriver.a         72         0        53           0           0          125        3046       633          0        3751
             libnewlib.a        231         0       440        1416           0         2087         907       332          0        2886
            libpthread.a         16         0        12         212           0          240        1401       457          0        2086
          libesp_timer.a         32         0        24         769           0          825         809       451          0        2061
              libefuse.a         28         0         4           0           0           32        1127       724          0        1879
 libbootloader_support.a          0         0         0        1545           0         1545          80       147          0        1772
            libesp_ipc.a         28         0        88         647           0          763         641       271          0        1587
        libesp_ringbuf.a          0         0         0         717           0          717           0       670          0        1387
librustlib.a(rustlib-1ce          0         0         0           0           0            0        1194       179          0        1373
librustlib.a(rustlib-1ce          0         0         0           0           0            0        1160       118          0        1278
             libxtensa.a       1024         0         0          77           0         1101         114        35          0        1250
librustlib.a(rustlib-1ce          0         0         4           0           0            4         969         0          0         969
                liblog.a          8         0       272         232           0          512         413       121          0         774
         libapp_update.a          1         0        12         162           0          175         165       125        256         709
             libxt_hal.a          0         0         0         443           0          443           0        32          0         475
               libmain.a          0         0         0           0           0            0         130       131          0         261
            libesp_rom.a          0         0         0          90           0           90           0         0          0          90
               libclib.a          0         0         0           0           0            0          22         0          0          22
                   (exe)          0         0         0           0           3            3           3        12          0          18
             libesp_pm.a          0         0         0           0           0            0           8         0          0           8
                libcxx.a          0         0         0           0           0            0           5         0          0           5
                libgcc.a          0         0         0           0           0            0           0         0          0           0
         libmbedcrypto.a          0         0         0           0           0            0           0         0          0           0
                libsoc.a          0         0         0           0           0            0           0         0          0           0
```
