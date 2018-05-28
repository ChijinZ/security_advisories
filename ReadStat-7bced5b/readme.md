# Multiple vulnerabilities in ReadStat-7bced5b
We discovered multiple vulnerabilities in ReadStat (git repository:https://github.com/WizardMac/ReadStat, Latest commit 7bced5b on May 8, 2018).

git log

    commit 7bced5b279486b92f362d97aa671241e787a809a
    Author: Evan Miller <emmiller@gmail.com>
    Date:   Tue May 8 07:23:27 2018 -0400

I modify the src/fuzz/Makefile (see: https://github.com/ChijinZ/security_advisories/blob/master/ReadStat-7bced5b/Makefile) so that I can generate libfuzzer-style entries to src/spss/readstat_sav_read.c and src/sas/readstat_sas7bcat_read.c

## Memory Leaks

When the testcase (see: https://github.com/ChijinZ/security_advisories/blob/master/ReadStat-7bced5b/crash-sav-read) was input, a crash was triggered in ReadStat/src/spss/readstat_sav_read.c:894:

    iconv_t converter = iconv_open(dst_charset, src_charset);

LeakSanitizer provided imformation as below:

    ==9990==ERROR: LeakSanitizer: detected memory leaks

    Direct leak of 112 byte(s) in 1 object(s) allocated from:
        #0 0x4e2ef8 in __interceptor_malloc /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_malloc_linux.cc:88
        #1 0x7f3db68be3e0 in __gconv_open /build/glibc-Cl5G7W/glibc-2.23/iconv/gconv_open.c:114
        #2 0x7f3db68bde29 in iconv_open /build/glibc-Cl5G7W/glibc-2.23/iconv/iconv_open.c:71
        #3 0x7f3db7ba9e51 in sav_parse_machine_integer_info_record path/to/ReadStat/src/spss/readstat_sav_read.c:894:29
        #4 0x7f3db7ba4866 in sav_parse_records_pass1 path/to/ReadStat/src/spss/readstat_sav_read.c:1170:30
        #5 0x7f3db7ba31f6 in readstat_parse_sav path/to/ReadStat/src/spss/readstat_sav_read.c:1443:19
        #6 0x51b1a8 in LLVMFuzzerTestOneInput (path/to/ReadStat/src/fuzz/fuzz_format_sav+0x51b1a8)
        #7 0x5258d4 in fuzzer::Fuzzer::ExecuteCallback(unsigned char const*, unsigned long) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerLoop.cpp:451:13
        #8 0x525b01 in fuzzer::Fuzzer::RunOne(unsigned char const*, unsigned long) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerLoop.cpp:408:3
        #9 0x51b4e1 in fuzzer::RunOneTest(fuzzer::Fuzzer*, char const*, unsigned long) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerDriver.cpp:268:6
        #10 0x51e2b8 in fuzzer::FuzzerDriver(int*, char***, int (*)(unsigned char const*, unsigned long)) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerDriver.cpp:585:9
        #11 0x51b260 in main path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerMain.cpp:20:10
        #12 0x7f3db68bd82f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291

    Indirect leak of 32640 byte(s) in 1 object(s) allocated from:
        #0 0x4e2ef8 in __interceptor_malloc /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_malloc_linux.cc:88
        #1 0x7f3db68be44b in __gconv_open /build/glibc-Cl5G7W/glibc-2.23/iconv/gconv_open.c:164
        #2 0x7f3db68bde29 in iconv_open /build/glibc-Cl5G7W/glibc-2.23/iconv/iconv_open.c:71
        #3 0x7f3db7ba9e51 in sav_parse_machine_integer_info_record path/to/ReadStat/src/spss/readstat_sav_read.c:894:29
        #4 0x7f3db7ba4866 in sav_parse_records_pass1 path/to/ReadStat/src/spss/readstat_sav_read.c:1170:30
        #5 0x7f3db7ba31f6 in readstat_parse_sav path/to/ReadStat/src/spss/readstat_sav_read.c:1443:19
        #6 0x51b1a8 in LLVMFuzzerTestOneInput (path/to/ReadStat/src/fuzz/fuzz_format_sav+0x51b1a8)
        #7 0x5258d4 in fuzzer::Fuzzer::ExecuteCallback(unsigned char const*, unsigned long) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerLoop.cpp:451:13
        #8 0x525b01 in fuzzer::Fuzzer::RunOne(unsigned char const*, unsigned long) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerLoop.cpp:408:3
        #9 0x51b4e1 in fuzzer::RunOneTest(fuzzer::Fuzzer*, char const*, unsigned long) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerDriver.cpp:268:6
        #10 0x51e2b8 in fuzzer::FuzzerDriver(int*, char***, int (*)(unsigned char const*, unsigned long)) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerDriver.cpp:585:9
        #11 0x51b260 in main path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerMain.cpp:20:10
        #12 0x7f3db68bd82f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291

    Indirect leak of 208 byte(s) in 1 object(s) allocated from:
        #0 0x4e2ef8 in __interceptor_malloc /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_malloc_linux.cc:88
        #1 0x7f3db68c7314 in __gconv_lookup_cache /build/glibc-Cl5G7W/glibc-2.23/iconv/gconv_cache.c:372

    SUMMARY: AddressSanitizer: 32960 byte(s) leaked in 3 allocation(s).

    INFO: a leak has been found in the initial corpus.

    INFO: to ignore leaks on libFuzzer side use -detect_leaks=0.

##  Infinite Loop

The testcase (see: https://github.com/ChijinZ/security_advisories/blob/master/ReadStat-7bced5b/crash-sas7bcat) caused an infinite loop in src/sas/readstat_sas7bcat_read.c: 288 to 309:

    while (next_page > 0 && next_page_pos > 0) {
        if (io->seek(ctx->header_size+(next_page-1)*ctx->page_size+next_page_pos, READSTAT_SEEK_SET, io->io_ctx) == -1) {
            retval = READSTAT_ERROR_SEEK;
            goto cleanup;
        }
        if (io->read(chain_link, sizeof(chain_link), io->io_ctx) < sizeof(chain_link)) {
            retval = READSTAT_ERROR_READ;
            goto cleanup;
        }
        next_page = sas_read4(&chain_link[0], ctx->bswap);
        next_page_pos = sas_read2(&chain_link[4], ctx->bswap);
        chain_link_len = sas_read2(&chain_link[6], ctx->bswap);
        if (buffer_offset + chain_link_len > buffer_len) {
            retval = READSTAT_ERROR_PARSE;
            goto cleanup;
        }
        if (io->read(buffer + buffer_offset, chain_link_len, io->io_ctx) < chain_link_len) {
            retval = READSTAT_ERROR_READ;
            goto cleanup;
        }
        buffer_offset += chain_link_len;
    }

libFuzzer provided information as bellow:

    ==29158== ERROR: libFuzzer: timeout after 121 seconds
        #0 0x4eee23 in __sanitizer_print_stack_trace /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_stack.cc:38
        #1 0x524978 in fuzzer::Fuzzer::AlarmCallback() path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerLoop.cpp:234:7
        #2 0x7f44ed6ca38f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1138f)
        #3 0x4e19a8 in __asan_memcpy /home/ubuntu/llvm/llvm-6.0.0.src/projects/compiler-rt/lib/asan/asan_interceptors_memintrinsics.cc:23
        #4 0x538e4c in rt_read_handler path/to/ReadStat/src/test/test_buffer_io.c:43:9
        #5 0x7f44edc82cb6 in sas7bcat_read_block path/to/ReadStat/src/sas/readstat_sas7bcat_read.c:304:13
        #6 0x7f44edc8109a in readstat_parse_sas7bcat path/to/ReadStat/src/sas/readstat_sas7bcat_read.c:442:23
        #7 0x51b1a8 in LLVMFuzzerTestOneInput (path/to/ReadStat/src/fuzz/fuzz_format_sas7bcat+0x51b1a8)
        #8 0x5258d4 in fuzzer::Fuzzer::ExecuteCallback(unsigned char const*, unsigned long) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerLoop.cpp:451:13
        #9 0x525b01 in fuzzer::Fuzzer::RunOne(unsigned char const*, unsigned long) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerLoop.cpp:408:3
        #10 0x51b4e1 in fuzzer::RunOneTest(fuzzer::Fuzzer*, char const*, unsigned long) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerDriver.cpp:268:6
        #11 0x51e2b8 in fuzzer::FuzzerDriver(int*, char***, int (*)(unsigned char const*, unsigned long)) path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerDriver.cpp:585:9
        #12 0x51b260 in main path/to/libfuzzer/libFuzzer/Fuzzer/./FuzzerMain.cpp:20:10
        #13 0x7f44ec9e482f in __libc_start_main /build/glibc-Cl5G7W/glibc-2.23/csu/../csu/libc-start.c:291
        #14 0x41ecc8 in _start (path/to/ReadStat/src/fuzz/fuzz_format_sas7bcat+0x41ecc8)

    SUMMARY: libFuzzer: timeout