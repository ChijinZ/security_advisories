# A vulnerability in tinyexr
There is a vulnerability in tinyexr (git repository: https://github.com/syoyo/tinyexr, Latest commit 7953aea).

git log 

    commit 7953aeaa372573fa4c33bb371ff9e9e224cc4e63
    Merge: 9ccd039 917257c
    Author: Syoyo Fujita <syoyo@lighttransport.com>
    Date:   Sun May 27 01:01:10 2018 +0900

I build tinyexr with clang and address sanitizer. When testcase (see: ) is input into test_tinyexr (command: ./test_tinyexr testcase), a heap-buffer-overflow has triggered.

Address sanitizer provided information as below: 

    ==28461==ERROR: AddressSanitizer: heap-buffer-overflow on address 0x603000000027 at pc 0x00000055e7d0 bp 0x7ffc4ae2a370 sp 0x7ffc4ae2a368
    READ of size 4 at 0x603000000027 thread T0
        #0 0x55e7cf in tinyexr::ReadChannelInfo(std::vector<tinyexr::ChannelInfo, std::allocator<tinyexr::ChannelInfo> >&, std::vector<unsigned char, std::allocator<unsigned char> > const&) /path/to/tinyexr/./tinyexr.h:7320:5
        #1 0x53bb5a in tinyexr::ParseEXRHeader(tinyexr::HeaderInfo*, bool*, _EXRVersion const*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >*, unsigned char const*, unsigned long) /path/to/tinyexr/./tinyexr.h:10304:12
        #2 0x53a27f in ParseEXRHeaderFromMemory /path/to/tinyexr/./tinyexr.h:11088:13
        #3 0x538f4f in ParseEXRHeaderFromFile /path/to/tinyexr/./tinyexr.h:12369:10
        #4 0x534c17 in LoadEXR /path/to/tinyexr/./tinyexr.h:10921:15
        #5 0x5680e5 in main /path/to/tinyexr/test_tinyexr.cc:130:13
        #6 0x7f8fa177d82f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291
        #7 0x41b288 in _start (/path/to/tinyexr/test_tinyexr+0x41b288)

    0x603000000027 is located 0 bytes to the right of 23-byte region [0x603000000010,0x603000000027)
    allocated by thread T0 here:
        #0 0x516f48 in operator new(unsigned long) /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_new_delete.cc:92
        #1 0x55d30a in __gnu_cxx::new_allocator<unsigned char>::allocate(unsigned long, void const*) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/ext/new_allocator.h:104:27
        #2 0x55d30a in std::allocator_traits<std::allocator<unsigned char> >::allocate(std::allocator<unsigned char>&, unsigned long) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/alloc_traits.h:491
        #3 0x55d30a in std::_Vector_base<unsigned char, std::allocator<unsigned char> >::_M_allocate(unsigned long) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/stl_vector.h:170
        #4 0x55d30a in std::vector<unsigned char, std::allocator<unsigned char> >::_M_default_append(unsigned long) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/vector.tcc:557
        #5 0x55d30a in std::vector<unsigned char, std::allocator<unsigned char> >::resize(unsigned long) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/stl_vector.h:676
        #6 0x55d30a in tinyexr::ReadAttribute(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >*, std::vector<unsigned char, std::allocator<unsigned char> >*, unsigned long*, char const*, unsigned long) /path/to/tinyexr/./tinyexr.h:7215
        #7 0x53b24f in tinyexr::ParseEXRHeader(tinyexr::HeaderInfo*, bool*, _EXRVersion const*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >*, unsigned char const*, unsigned long) /path/to/tinyexr/./tinyexr.h:10234:10
        #8 0x53a27f in ParseEXRHeaderFromMemory /path/to/tinyexr/./tinyexr.h:11088:13
        #9 0x538f4f in ParseEXRHeaderFromFile /path/to/tinyexr/./tinyexr.h:12369:10

    SUMMARY: AddressSanitizer: heap-buffer-overflow /path/to/tinyexr/./tinyexr.h:7320:5 in tinyexr::ReadChannelInfo(std::vector<tinyexr::ChannelInfo, std::allocator<tinyexr::ChannelInfo> >&, std::vector<unsigned char, std::allocator<unsigned char> > const&)
    Shadow bytes around the buggy address:
    0x0c067fff7fb0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c067fff7fc0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c067fff7fd0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c067fff7fe0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0x0c067fff7ff0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    =>0x0c067fff8000: fa fa 00 00[07]fa fa fa fa fa fa fa fa fa fa fa
    0x0c067fff8010: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c067fff8020: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c067fff8030: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c067fff8040: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
    0x0c067fff8050: fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa fa
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
    ==28461==ABORTING
    =================================================================