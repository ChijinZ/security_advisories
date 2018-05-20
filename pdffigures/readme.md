# a vulnerability in pdffigures

There is a vulnerability in pdffigures (git repository: https://github.com/allenai/pdffigures, Latest commit ffbeba0 on Apr 10, 2018).

git log:

    commit ffbeba09658341304c917d2277b680e6fc643f0d
    Author: chrisc36 <chrisc@allenai.org>
    Date:   Mon Apr 9 09:53:30 2018 -0700

When testcase (see: https://github.com/ChijinZ/security_advisories/blob/master/pdffigures/crash.pdf) was input into **pdffigures** (command: *pdffigures -f testfile* ), a **SEGV** signal was triggered.

It seems that the unordered_map *fontNameCounts* has no element when the input pdf is blank page. And the variable is used to access its first element in TextUtils.cpp:157:

    modeFontName = fontNameCounts.begin()->first

AddressSanitizer provided information as below:

    ==3673==ERROR: AddressSanitizer: SEGV on unknown address 0x000000000010 (pc 0x7ff062c8436e bp 0x7fffdc79de90 sp 0x7fffdc79d5e0 T0)
    ==3673==The signal is caused by a READ memory access.
    ==3673==Hint: address points to the zero page.
        #0 0x7ff062c8436d in std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::_M_assign(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&) (/usr/lib/x86_64-linux-gnu/libstdc++.so.6+0x11f36d)
        #1 0x530ab6 in std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::assign(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/basic_string.h:1095:8
        #2 0x530ab6 in std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::operator=(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&) /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/basic_string.h:551
        #3 0x530ab6 in DocumentStatistics::DocumentStatistics(std::vector<TextPage*, std::allocator<TextPage*> >&, PDFDoc*, bool) /path/to/pdffigures/TextUtils.cpp:157
        #4 0x5de267 in main /path/to/pdffigures/pdffigures.cpp:164:33
        #5 0x7ff061c7382f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291
        #6 0x421e28 in _start (/path/to/pdffigures/pdffigures+0x421e28)

    AddressSanitizer can not provide additional info.
    SUMMARY: AddressSanitizer: SEGV (/usr/lib/x86_64-linux-gnu/libstdc++.so.6+0x11f36d) in std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::_M_assign(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&)

GDB provided information as below:

    (gdb) r
    Starting program: /path/to/pdffigures/pdffigures -f crash.pdf
    [Thread debugging using libthread_db enabled]
    Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

    Program received signal SIGSEGV, Segmentation fault.
    0x00007ffff72b336e in std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::_M_assign(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&) ()
    from /usr/lib/x86_64-linux-gnu/libstdc++.so.6
    (gdb) bt
    #0  0x00007ffff72b336e in std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::_M_assign(std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&) ()
    from /usr/lib/x86_64-linux-gnu/libstdc++.so.6
    Python Exception <class 'gdb.error'> There is no member named _M_dataplus.: 
    #1  0x0000000000530ab7 in std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::assign (this=0x7fffffffcdb0, __str=)
        at /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/basic_string.h:1095
    Python Exception <class 'gdb.error'> There is no member named _M_dataplus.: 
    #2  std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> >::operator= (this=0x7fffffffcdb0, __str=)
        at /usr/lib/gcc/x86_64-linux-gnu/5.4.0/../../../../include/c++/5.4.0/bits/basic_string.h:551
    #3  DocumentStatistics::DocumentStatistics (this=<optimized out>, 
        textPages=..., doc=<optimized out>, verbose=false) at TextUtils.cpp:157
    #4  0x00000000005de268 in main (argc=3, argv=0x7fffffffdd68)
        at pdffigures.cpp:164
