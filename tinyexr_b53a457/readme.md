# Multiple vulnerabilities in tinyexr
There are multiple vulnerabilities in tinyexr (git repository: https://github.com/syoyo/tinyexr, Latest commit b53a457, on Jun 13, 2018).

git log 

    commit b53a457fd2ba519f06a8b2207794e19bcebe6af7
    Merge: f79bbf9 7349d67
    Author: Syoyo Fujita <syoyo@lighttransport.com>
    Date:   Wed Jun 13 19:12:22 2018 +0900

## Heap Buffer overflow in LoadEXRImageFromMemory

I build tinyexr with clang and address sanitizer. When testcase (see: https://github.com/ChijinZ/security_advisories/blob/master/tinyexr_b53a457/heap-buffer-overflow) is input into test_tinyexr (command: ./test_tinyexr testcase), a heap-buffer-overflow has triggered.

    ==17583==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x6140000003e0 at pc 0x000000548c65 bp 0x7ffe577d0190 sp 0x7ffe577d0188
    READ of size 8 at 0x6140000003e0 thread T0
        #0 0x548c64 in LoadEXRImageFromMemory /path/to/tinyexr/./tinyexr.h:10925:5
        #1 0x53b481 in LoadEXRImageFromFile /path/to/tinyexr/./tinyexr.h:11303:10
        #2 0x53567f in LoadEXR /path/to/tinyexr/./tinyexr.h:11003:15
        #3 0x572465 in main /path/to/tinyexr/test_tinyexr.cc:130:13
        #4 0x7f5e43e9e82f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291
        #5 0x41b3e8 in _start (/path/to/tinyexr/test_tinyexr+0x41b3e8)

    0x6140000003e0 is located 0 bytes to the right of 416-byte region [0x614000000240,0x6140000003e0)
    allocated by thread T0 here:
        #0 0x5170a8 in operator new(unsigned long) /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_new_delete.cc:92
        #1 0x53b3f4 in __gnu_cxx::new_allocator<unsigned char>::allocate(unsigned long, void const*) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/ext/new_allocator.h:104:27
        #2 0x53b3f4 in std::allocator_traits<std::allocator<unsigned char> >::allocate(std::allocator<unsigned char>&, unsigned long) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/alloc_traits.h:491
        #3 0x53b3f4 in std::_Vector_base<unsigned char, std::allocator<unsigned char> >::_M_allocate(unsigned long) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/stl_vector.h:170
        #4 0x53b3f4 in std::_Vector_base<unsigned char, std::allocator<unsigned char> >::_M_create_storage(unsigned long) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/stl_vector.h:185
        #5 0x53b3f4 in std::_Vector_base<unsigned char, std::allocator<unsigned char> >::_Vector_base(unsigned long, std::allocator<unsigned char> const&) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/stl_vector.h:136
        #6 0x53b3f4 in std::vector<unsigned char, std::allocator<unsigned char> >::vector(unsigned long, std::allocator<unsigned char> const&) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/stl_vector.h:278
        #7 0x53b3f4 in LoadEXRImageFromFile /path/to/tinyexr/./tinyexr.h:11294
        #8 0x53567f in LoadEXR /path/to/tinyexr/./tinyexr.h:11003:15
        #9 0x572465 in main /path/to/tinyexr/test_tinyexr.cc:130:13
        #10 0x7f5e43e9e82f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291

    SUMMARY: AddressSanitizer: heap-buffer-overflow /path/to/tinyexr/./tinyexr.h:10925:5 in LoadEXRImageFromMemory
    Shadow bytes around the buggy address:
    0x0c287fff8020: fd fd fd fd fd fd fd fd fd fd fd fd fd fd fd fd
    0x0c287fff8030: fd fd fd fd fd fd fd fd fd fd fd fd fa fa fa fa
    0x0c287fff8040: fa fa fa fa fa fa fa fa 00 00 00 00 00 00 00 00
    0x0c287fff8050: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c287fff8060: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    =>0x0c287fff8070: 00 00 00 00 00 00 00 00 00 00 00 00[fa]fa fa fa
    0x0c287fff8080: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c287fff8090: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c287fff80a0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c287fff80b0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c287fff80c0: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
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
    ==17583==ABORTING

## Assert failure

When testcase (see: https://github.com/ChijinZ/security_advisories/blob/master/tinyexr_b53a457/assert_10195) is input into test_tinyexr (command: ./test_tinyexr testcase), a assert-failure problem is triggered in tinyexr.h:10195.

    test_tinyexr: ./tinyexr.h:10195: void tinyexr::ComputeChannelLayout(std::vector<size_t> *, int *, size_t *, int, const EXRChannelInfo *): Assertion `0' failed.
    Aborted