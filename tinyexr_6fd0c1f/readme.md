# Multiple vulnerabilities in tinyexr
There are multiple vulnerabilities in tinyexr (git repository: https://github.com/syoyo/tinyexr, Latest commit 6fd0c1f, on Jun 7, 2018).

git log

    commit 6fd0c1f7575b9119f287fbe5577b2eff41c71bd5
    Author: Syoyo Fujita <syoyo@lighttransport.com>
    Date:   Thu Jun 7 13:53:40 2018 +0900

## Heap Buffer overflow

I build tinyexr with clang and address sanitizer. When testcase (see: https://github.com/ChijinZ/security_advisories/blob/master/tinyexr_6fd0c1f/heap-buffer-overflow) is input into test_tinyexr (command: ./test_tinyexr testcase), a heap-buffer-overflow has triggered.

Address sanitizer provided information as below: 

    ==25897==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x62a00000b35d at pc 0x00000058117b bp 0x7ffe8c41dd10 sp 0x7ffe8c41dd08
    READ of size 1 at 0x62a00000b35d thread T0
        #0 0x58117a in tinyexr::DecodePixelData(unsigned char**, int const*, unsigned char const*, unsigned long, int, int, int, int, int, int, int, int, unsigned long, unsigned long, _EXRAttribute const*, unsigned long, _EXRChannelInfo const*, std::vector<unsigned long, std::allocator<unsigned long> > const&) /home/ubuntu/fuzz/tinyexr/tinyexr_6fd0c1f/./tinyexr.h:8848:7
        #1 0x564765 in tinyexr::DecodeChunk(_EXRImage*, _EXRHeader const*, std::vector<unsigned long, std::allocator<unsigned long> > const&, unsigned char const*, unsigned long) /home/ubuntu/fuzz/tinyexr/tinyexr_6fd0c1f/./tinyexr.h:10699:20
        #2 0x544fd4 in LoadEXRImageFromMemory /home/ubuntu/fuzz/tinyexr/tinyexr_6fd0c1f/./tinyexr.h:10891:10
        #3 0x539a8a in LoadEXRImageFromFile /home/ubuntu/fuzz/tinyexr/tinyexr_6fd0c1f/./tinyexr.h:11260:10
        #4 0x535134 in LoadEXR /home/ubuntu/fuzz/tinyexr/tinyexr_6fd0c1f/./tinyexr.h:10941:15
        #5 0x5684f5 in main /home/ubuntu/fuzz/tinyexr/tinyexr_6fd0c1f/test_tinyexr.cc:130:13
        #6 0x7f2ff6d6982f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291
        #7 0x41b288 in _start (/home/ubuntu/fuzz/tinyexr/tinyexr_6fd0c1f/test_tinyexr+0x41b288)

    0x62a00000b35d is located 0 bytes to the right of 20829-byte region [0x62a000006200,0x62a00000b35d)
    allocated by thread T0 here:
        #0 0x516f48 in operator new(unsigned long) /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_new_delete.cc:92
        #1 0x5399b3 in __gnu_cxx::new_allocator<unsigned char>::allocate(unsigned long, void const*) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/ext/new_allocator.h:104:27
        #2 0x5399b3 in std::allocator_traits<std::allocator<unsigned char> >::allocate(std::allocator<unsigned char>&, unsigned long) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/alloc_traits.h:491
        #3 0x5399b3 in std::_Vector_base<unsigned char, std::allocator<unsigned char> >::_M_allocate(unsigned long) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/stl_vector.h:170
        #4 0x5399b3 in std::_Vector_base<unsigned char, std::allocator<unsigned char> >::_M_create_storage(unsigned long) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/stl_vector.h:185
        #5 0x5399b3 in std::_Vector_base<unsigned char, std::allocator<unsigned char> >::_Vector_base(unsigned long, std::allocator<unsigned char> const&) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/stl_vector.h:136
        #6 0x5399b3 in std::vector<unsigned char, std::allocator<unsigned char> >::vector(unsigned long, std::allocator<unsigned char> const&) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/stl_vector.h:278
        #7 0x5399b3 in LoadEXRImageFromFile /home/ubuntu/fuzz/tinyexr/tinyexr_6fd0c1f/./tinyexr.h:11251

    SUMMARY: AddressSanitizer: heap-buffer-overflow /home/ubuntu/fuzz/tinyexr/tinyexr_6fd0c1f/./tinyexr.h:8848:7 in tinyexr::DecodePixelData(unsigned char**, int const*, unsigned char const*, unsigned long, int, int, int, int, int, int, int, int, unsigned long, unsigned long, _EXRAttribute const*, unsigned long, _EXRChannelInfo const*, std::vector<unsigned long, std::allocator<unsigned long> > const&)
    Shadow bytes around the buggy address:
    0x0c547fff9610: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c547fff9620: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c547fff9630: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c547fff9640: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c547fff9650: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    =>0x0c547fff9660: 00 00 00 00 00 00 00 00 00 00 00[05]fa fa fa fa
    0x0c547fff9670: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c547fff9680: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c547fff9690: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c547fff96a0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c547fff96b0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
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
    ==25897==ABORTING
    Load EXR err: Failed to parse channel info.(code -4)

## Memory Leak

I build tinyexr with clang and leak sanitizer. When testcase (see: https://github.com/ChijinZ/security_advisories/blob/master/tinyexr_6fd0c1f/memory_leak) is input into test_tinyexr (command: ./test_tinyexr testcase), sanitizer detected memory leaks in *ParseEXRHeaderFromMemory*.

Leak sanitizer provided information as below:

    ==25951==ERROR: LeakSanitizer: detected memory leaks

    Direct leak of 30 byte(s) in 1 object(s) allocated from:
        #0 0x441968 in strdup /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_interceptors.cc:407
        #1 0x53a6a7 in ParseEXRHeaderFromMemory /home/ubuntu/fuzz/tinyexr/tinyexr_6fd0c1f/./tinyexr.h:11101:16
        #2 0x5392cd in ParseEXRHeaderFromFile /home/ubuntu/fuzz/tinyexr/tinyexr_6fd0c1f/./tinyexr.h:12375:10

    SUMMARY: AddressSanitizer: 30 byte(s) leaked in 1 allocation(s)