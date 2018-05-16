# multiple vulnerabilities in libjpeg-version 9a

There are multiple vulnerabilities in libjpeg-version 9a (git repository:https://github.com/LuaDist/libjpeg, Latest commit 6c0fcb8  on Nov 2, 2015).

# Floating point exception (jmemmgr.c:407)
When testcase of  **[libjpeg-v9a/Floating point exception/](https://github.com/ChijinZ/security_advisories/tree/master/libjpeg-v9a/Floating%20point%20exception)jmemmgr.c:407-***  was input into **cjpeg** (command: *cjpeg testfile* ), a crash was triggered due to Floating point exception.

The problem seems like that an incorrect operation to float happened in line 407 of **jmemmgr.c** :

*ltemp = (MAX_ALLOC_CHUNK-SIZEOF(large_pool_hdr)) / ((long) samplesperrow * SIZEOF(JSAMPLE));*  

AddressSanitizer provided information as below:

    ==27271==ERROR: AddressSanitizer: FPE on unknown address 0x7fd5b49f2fd8 (pc 0x7fd5b49f2fd8 bp 0x7ffc9e688bd0 sp 0x7ffc9e688280 T0)
        #0 0x7fd5b49f2fd7  (/usr/local/lib/libjpeg.so.9+0xf4fd7)
        #1 0x502fdf  (/home/ubuntu/fuzz/libjpeg/.libs/cjpeg+0x502fdf)
        #2 0x4eaf08  (/home/ubuntu/fuzz/libjpeg/.libs/cjpeg+0x4eaf08)
        #3 0x7fd5b3a0c82f  (/lib/x86_64-linux-gnu/libc.so.6+0x2082f)
        #4 0x418ce8  (/home/ubuntu/fuzz/libjpeg/.libs/cjpeg+0x418ce8)

    AddressSanitizer can not provide additional info.
    SUMMARY: AddressSanitizer: FPE (/usr/local/lib/libjpeg.so.9+0xf4fd7)

GDB provided information as below:

    (gdb) r
    Starting program: /home/ubuntu/fuzz/libjpeg/.libs/cjpeg output4cjpeg/crashes/id:000000,sig:08,src:000031,op:int16,pos:12,val:+0
    [Thread debugging using libthread_db enabled]
    Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

    Program received signal SIGFPE, Arithmetic exception.
    alloc_sarray (cinfo=0x7fffffffdd60, pool_id=1, samplesperrow=0, numrows=1)
        at jmemmgr.c:407
    407	  ltemp = (MAX_ALLOC_CHUNK-SIZEOF(large_pool_hdr)) /
    (gdb) bt
    #0  alloc_sarray (cinfo=0x7fffffffdd60, pool_id=1, samplesperrow=0, numrows=1)
        at jmemmgr.c:407
    #1  0x0000000000502fe0 in start_input_tga (cinfo=0x7fffffffdd60, 
        sinfo=0x629000005218) at rdtarga.c:437
    #2  0x00000000004eaf09 in main (argc=2, argv=0x7fffffffe428) at cjpeg.c:626

# Segmentation fault (rdppm.c:153,171,172,173)
When testcase of **[libjpeg-v9a/Segmentation fault/](https://github.com/ChijinZ/security_advisories/tree/master/libjpeg-v9a/Segmentation%20fault)rdppm.c:153-*** was input into **cjpeg** (command: *cjpeg testfile* ), a crash was triggered due to segmentation fault.

The problem seems like that ***ptr** attempted to access a restricted area of memory in line 153 of **rdppm.c** :

**ptr++ = rescale[read_pbm_integer(cinfo, infile)];*

AddressSanitizer provided information as below:

    ==20420==ERROR: AddressSanitizer: SEGV on unknown address 0x62800008fb93 (pc 0x0000004f7178 bp 0x7fff8f0ae5b0 sp 0x7fff8f0ae500 T0)
        #0 0x4f7177  (/home/ubuntu/fuzz/libjpeg/.libs/cjpeg+0x4f7177)
        #1 0x4eb1bc  (/home/ubuntu/fuzz/libjpeg/.libs/cjpeg+0x4eb1bc)
        #2 0x7fca4237e82f  (/lib/x86_64-linux-gnu/libc.so.6+0x2082f)
        #3 0x418ce8  (/home/ubuntu/fuzz/libjpeg/.libs/cjpeg+0x418ce8)

    AddressSanitizer can not provide additional info.
    SUMMARY: AddressSanitizer: SEGV (/home/ubuntu/fuzz/libjpeg/.libs/cjpeg+0x4f7177) 

GDB provided information as below:

    (gdb) r
    Starting program: /home/ubuntu/fuzz/libjpeg/.libs/cjpeg output4cjpeg/crashes/id:000004,sig:11,src:000081,op:havoc,rep:2
    [Thread debugging using libthread_db enabled]
    Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

    Program received signal SIGSEGV, Segmentation fault.
    0x00000000004f7178 in get_text_gray_row (cinfo=0x7fffffffdd80, 
        sinfo=0x628000008118) at rdppm.c:153
    153	    *ptr++ = rescale[read_pbm_integer(cinfo, infile)];
    (gdb) bt
    #0  0x00000000004f7178 in get_text_gray_row (cinfo=0x7fffffffdd80, 
        sinfo=0x628000008118) at rdppm.c:153
    #1  0x00000000004eb1bd in main (argc=2, argv=0x7fffffffe438) at cjpeg.c:642

Same problems happened in 

rdppm.c:171. (testcase: **[libjpeg-v9a/Segmentation fault/](https://github.com/ChijinZ/security_advisories/tree/master/libjpeg-v9a/Segmentation%20fault)rdppm.c:171-*** ), 

rdppm.c:172. (testcase:**[libjpeg-v9a/Segmentation fault/](https://github.com/ChijinZ/security_advisories/tree/master/libjpeg-v9a/Segmentation%20fault)rdppm.c:172-*** ), 

rdppm.c:173. (testcase:**[libjpeg-v9a/Segmentation fault/](https://github.com/ChijinZ/security_advisories/tree/master/libjpeg-v9a/Segmentation%20fault)rdppm.c:173-*** ).
