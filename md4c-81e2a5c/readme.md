# Multiple vulnerabilities in md4c
There are multiple vulnerabilities in md4c (git repository: https://github.com/mity/md4c, Latest commit 81e2a5c  on Apr 12, 2018).

git log 

    commit 81e2a5cac2c8c2b1f8fe63b7bce3fe7e516e2891
    Author: Martin Mitas <mity@morous.org>
    Date:   Thu Apr 12 17:03:37 2018 +0200

## Heap buffer overflow in md_split_simple_pairing_mark() (**CVE-2018-11536**)

command: ./md2html testfile

testcase: https://github.com/ChijinZ/security_advisories/blob/master/md4c-81e2a5c/Heap_buffer_overflow_in_md_split_simple_pairing_mark

It seems like that an overflow happened in memcpy() in md4c.c:3499:

*memcpy(dummy, mark, sizeof(MD_MARK));*

AddressSanitizer provided information as below:

    ==27938==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x61a000000684 at pc 0x0000004dd7f5 bp 0x7ffedcfedc30 sp 0x7ffedcfed3e0
    WRITE of size 20 at 0x61a000000684 thread T0
        #0 0x4dd7f4 in __asan_memcpy /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cc:23
        #1 0x546dd3 in md_split_simple_pairing_mark /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:3499:5
        #2 0x546dd3 in md_analyze_simple_pairing_mark /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:3553
        #3 0x540584 in md_analyze_marks /home/ubuntu/fuzz/test/md4c/md4c/md4c.c
        #4 0x53c9c8 in md_analyze_link_contents /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:3813:5
        #5 0x53c9c8 in md_analyze_inlines /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:3802
        #6 0x550b95 in md_process_normal_block_contents /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4283:5
        #7 0x52e7f7 in md_process_leaf_block /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4454:13
        #8 0x52e7f7 in md_process_all_blocks /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4529
        #9 0x52e7f7 in md_process_doc /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5856
        #10 0x5202cb in md_parse /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5917:11
        #11 0x51a7a8 in md_render_html /home/ubuntu/fuzz/test/md4c/md2html/render_html.c:488:12
        #12 0x5195cc in process_file /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:139:11
        #13 0x5195cc in main /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:343
        #14 0x7f17c6fc582f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291
        #15 0x41a668 in _start (/home/ubuntu/fuzz/test/md4c/build/md2html/md2html+0x41a668)

    Address 0x61a000000684 is a wild pointer.
    SUMMARY: AddressSanitizer: heap-buffer-overflow /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cc:23 in __asan_memcpy
    Shadow bytes around the buggy address:
    0x0c347fff8080: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c347fff8090: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c347fff80a0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c347fff80b0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c347fff80c0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    =>0x0c347fff80d0:[fa]fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c347fff80e0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c347fff80f0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c347fff8100: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c347fff8110: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c347fff8120: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    Shadow byte legend (one shadow byte represents 8 application bytes):
    Addressable:           00
    Partially addressable: 01 02 03 04 05 06 07 
    Heap left redzone:       fa
    Freed heap region:       fd
    Stack left redzone:      f1
    Stack mid redzone:       f2
    Stack right redzone:     f3
    Stack after return:      f5
    Stack use after scope:   f8
    Global redzone:          f9
    Global init order:       f6
    Poisoned by user:        f7
    Container overflow:      fc
    Array cookie:            ac
    Intra object redzone:    bb
    ASan internal:           fe
    Left alloca redzone:     ca
    Right alloca redzone:    cb
    ==27938==ABORTING

## Heap buffer overflow in md_process_inlines()

command: ./md2html testfile

testcase: https://github.com/ChijinZ/security_advisories/blob/master/md4c-81e2a5c/Heap_buffer_overflow_in_md_process_inlines

It seems like that *mark* variable access a restricted area of memory in md4c.c:4004:

*while(!(mark->flags & MD_MARK_RESOLVED)  ||  mark->beg < off)*

AddressSanitizer provided information as below:

    ==29037==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x61e000000a91 at pc 0x000000553328 bp 0x7fffe3bdfa70 sp 0x7fffe3bdfa68
    READ of size 1 at 0x61e000000a91 thread T0
        #0 0x553327 in md_process_inlines /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4004:27
        #1 0x553327 in md_process_normal_block_contents /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4284
        #2 0x52e7f7 in md_process_leaf_block /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4454:13
        #3 0x52e7f7 in md_process_all_blocks /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4529
        #4 0x52e7f7 in md_process_doc /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5856
        #5 0x5202cb in md_parse /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5917:11
        #6 0x51a7a8 in md_render_html /home/ubuntu/fuzz/test/md4c/md2html/render_html.c:488:12
        #7 0x5195cc in process_file /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:139:11
        #8 0x5195cc in main /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:343
        #9 0x7f49ca83682f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291
        #10 0x41a668 in _start (/home/ubuntu/fuzz/test/md4c/build/md2html/md2html+0x41a668)

    0x61e000000a91 is located 17 bytes to the right of 2560-byte region [0x61e000000080,0x61e000000a80)
    allocated by thread T0 here:
        #0 0x4ded00 in realloc /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_malloc_linux.cc:107
        #1 0x5369af in md_push_mark /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:2496:21
        #2 0x5369af in md_collect_marks /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:2897
        #3 0x5369af in md_analyze_inlines /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:3774
        #4 0x550b95 in md_process_normal_block_contents /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4283:5

    SUMMARY: AddressSanitizer: heap-buffer-overflow /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4004:27 in md_process_inlines
    Shadow bytes around the buggy address:
    0x0c3c7fff8100: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c3c7fff8110: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c3c7fff8120: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c3c7fff8130: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c3c7fff8140: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    =>0x0c3c7fff8150: fa fa[fa]fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c3c7fff8160: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c3c7fff8170: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c3c7fff8180: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c3c7fff8190: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c3c7fff81a0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    Shadow byte legend (one shadow byte represents 8 application bytes):
    Addressable:           00
    Partially addressable: 01 02 03 04 05 06 07 
    Heap left redzone:       fa
    Freed heap region:       fd
    Stack left redzone:      f1
    Stack mid redzone:       f2
    Stack right redzone:     f3
    Stack after return:      f5
    Stack use after scope:   f8
    Global redzone:          f9
    Global init order:       f6
    Poisoned by user:        f7
    Container overflow:      fc
    Array cookie:            ac
    Intra object redzone:    bb
    ASan internal:           fe
    Left alloca redzone:     ca
    Right alloca redzone:    cb
    ==29037==ABORTING
