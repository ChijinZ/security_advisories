# Multiple vulnerabilities in md4c
There are multiple vulnerabilities in md4c (git repository: https://github.com/mity/md4c, Latest commit 387bd02 on May 28, 2018).

git log:

    commit 387bd020b6811014df8f7d28f1a0e2589d1f75ee
    Author: Martin Mitas <mity@morous.org>
    Date:   Mon May 28 23:09:09 2018 +0200

## Heap buffer overflow in md_is_link_reference_definition_helper()

command: ./md2html testfile

testcase: https://github.com/ChijinZ/security_advisories/blob/master/md4c-387bd02/crash_md_is_link_reference_definition_helper

AddressSanitizer provided information as below:

    =================================================================
    ==7016==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x615000000280 at pc 0x00000054e1e4 bp 0x7ffdf438ab70 sp 0x7ffdf438ab68
    READ of size 4 at 0x615000000280 thread T0
        #0 0x54e1e3 in md_is_link_reference_definition_helper /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:1931:33
        #1 0x5320d5 in md_is_link_reference_definition /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:2213:11
        #2 0x5320d5 in md_consume_link_reference_definitions /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4648
        #3 0x5320d5 in md_end_current_block /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4694
        #4 0x52c7f7 in md_process_doc /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5850:5
        #5 0x5202cb in md_parse /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5917:11
        #6 0x51a7a8 in md_render_html /home/ubuntu/fuzz/test/md4c/md2html/render_html.c:488:12
        #7 0x5195cc in process_file /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:139:11
        #8 0x5195cc in main /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:343
        #9 0x7f20771c082f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291
        #10 0x41a668 in _start (/home/ubuntu/fuzz/test/md4c/build/md2html/md2html+0x41a668)

    0x615000000280 is located 0 bytes to the right of 512-byte region [0x615000000080,0x615000000280)
    allocated by thread T0 here:
        #0 0x4ded00 in realloc /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_malloc_linux.cc:107
        #1 0x527b65 in md_push_block_bytes /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4560:27
        #2 0x527b65 in md_start_new_block /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4587
        #3 0x527b65 in md_process_line /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5820
        #4 0x527b65 in md_process_doc /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5847
        #5 0x5202cb in md_parse /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5917:11
        #6 0x51a7a8 in md_render_html /home/ubuntu/fuzz/test/md4c/md2html/render_html.c:488:12
        #7 0x5195cc in process_file /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:139:11
        #8 0x5195cc in main /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:343
        #9 0x7f20771c082f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291

    SUMMARY: AddressSanitizer: heap-buffer-overflow /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:1931:33 in md_is_link_reference_definition_helper
    Shadow bytes around the buggy address:
    0x0c2a7fff8000: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c2a7fff8010: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c2a7fff8020: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c2a7fff8030: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c2a7fff8040: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    =>0x0c2a7fff8050:[fa]fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c2a7fff8060: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c2a7fff8070: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c2a7fff8080: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c2a7fff8090: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c2a7fff80a0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
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
    ==7016==ABORTING

## Heap buffer overflow in md_is_named_entity_contents()

command: ./md2html testfile

testcase: https://github.com/ChijinZ/security_advisories/blob/master/md4c-387bd02/crash_md_is_named_entity_contents

AddressSanitizer provided information as below:

    ==16545==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x60200000001f at pc 0x0000005464c6 bp 0x7ffe90e1b080 sp 0x7ffe90e1b078
    READ of size 1 at 0x60200000001f thread T0
        #0 0x5464c5 in md_is_named_entity_contents /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:1311:28
        #1 0x5464c5 in md_is_entity_str /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:1341
        #2 0x553b62 in md_build_attribute /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:1473:20
        #3 0x5562f9 in md_enter_leave_span_a /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:3838:5
        #4 0x5510d2 in md_process_inlines /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:3947:21
        #5 0x5510d2 in md_process_normal_block_contents /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4284
        #6 0x52e7f7 in md_process_leaf_block /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4454:13
        #7 0x52e7f7 in md_process_all_blocks /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4529
        #8 0x52e7f7 in md_process_doc /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5856
        #9 0x5202cb in md_parse /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5917:11
        #10 0x51a7a8 in md_render_html /home/ubuntu/fuzz/test/md4c/md2html/render_html.c:488:12
        #11 0x5195cc in process_file /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:139:11
        #12 0x5195cc in main /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:343
        #13 0x7fd6be7ec82f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291
        #14 0x41a668 in _start (/home/ubuntu/fuzz/test/md4c/build/md2html/md2html+0x41a668)

    0x60200000001f is located 0 bytes to the right of 15-byte region [0x602000000010,0x60200000001f)
    allocated by thread T0 here:
        #0 0x4de898 in __interceptor_malloc /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_malloc_linux.cc:88
        #1 0x54bedb in md_merge_lines_alloc /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:904:22
        #2 0x54bedb in md_is_inline_link_spec_helper /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:2352
        #3 0x53b9bf in md_is_inline_link_spec /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:2370:12
        #4 0x53b9bf in md_resolve_links /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:3367
        #5 0x53b9bf in md_analyze_inlines /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:3786
        #6 0x550b75 in md_process_normal_block_contents /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4283:5

    SUMMARY: AddressSanitizer: heap-buffer-overflow /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:1311:28 in md_is_named_entity_contents
    Shadow bytes around the buggy address:
    0x0c047fff7fb0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c047fff7fc0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c047fff7fd0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c047fff7fe0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c047fff7ff0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    =>0x0c047fff8000: fa fa 00[07]fa fa 00 07 fa fa fa fa fa fa fa fa
    0x0c047fff8010: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c047fff8020: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c047fff8030: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c047fff8040: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c047fff8050: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
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
    ==16545==ABORTING

## Heap buffer overflow in md_merge_lines()

command: ./md2html testfile

testcase: https://github.com/ChijinZ/security_advisories/blob/master/md4c-387bd02/crash_md_merge_lines

AddressSanitizer provided information as below:

    =================================================================
    ==21464==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x6040000000b1 at pc 0x00000054ff84 bp 0x7fff500be8d0 sp 0x7fff500be8c8
    WRITE of size 1 at 0x6040000000b1 thread T0
        #0 0x54ff83 in md_merge_lines /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:878:18
        #1 0x54ff83 in md_merge_lines_alloc /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:910
        #2 0x54ff83 in md_is_link_reference_definition_helper /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:2154
        #3 0x532108 in md_is_link_reference_definition /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:2215:15
        #4 0x532108 in md_consume_link_reference_definitions /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4648
        #5 0x532108 in md_end_current_block /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4694
        #6 0x52c7f7 in md_process_doc /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5850:5
        #7 0x5202cb in md_parse /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:5917:11
        #8 0x51a7a8 in md_render_html /home/ubuntu/fuzz/test/md4c/md2html/render_html.c:488:12
        #9 0x5195cc in process_file /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:139:11
        #10 0x5195cc in main /home/ubuntu/fuzz/test/md4c/md2html/md2html.c:343
        #11 0x7fda7443a82f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291
        #12 0x41a668 in _start (/home/ubuntu/fuzz/test/md4c/build/md2html/md2html+0x41a668)

    0x6040000000b1 is located 0 bytes to the right of 33-byte region [0x604000000090,0x6040000000b1)
    allocated by thread T0 here:
        #0 0x4de898 in __interceptor_malloc /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_malloc_linux.cc:88
        #1 0x54e91f in md_merge_lines_alloc /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:904:22
        #2 0x54e91f in md_is_link_reference_definition_helper /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:2154
        #3 0x532108 in md_is_link_reference_definition /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:2215:15
        #4 0x532108 in md_consume_link_reference_definitions /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4648
        #5 0x532108 in md_end_current_block /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:4694

    SUMMARY: AddressSanitizer: heap-buffer-overflow /home/ubuntu/fuzz/test/md4c/md4c/md4c.c:878:18 in md_merge_lines
    Shadow bytes around the buggy address:
    0x0c087fff7fc0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c087fff7fd0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c087fff7fe0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c087fff7ff0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c087fff8000: fa fa fd fd fd fd fd fd fa fa 00 00 00 00 00 04
    =>0x0c087fff8010: fa fa 00 00 00 00[01]fa fa fa fa fa fa fa fa fa
    0x0c087fff8020: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c087fff8030: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c087fff8040: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c087fff8050: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c087fff8060: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
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
    ==21464==ABORTING