# Assert failure in ucl_msgpack_insert_object (ucl_msgpack.c:845)

There is a undefined-behavior vulnerability in libucl (git repository: https://github.com/vstakhov/libucl, Latest commit 230f197 on Jul 1, 2020).

When a malicious testcase is input into test driver, the program exits due to assert failure. gdb message as bellow:

    #0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:51
    #1  0x00007ffff6aba801 in __GI_abort () at abort.c:79
    #2  0x00007ffff6aaa39a in __assert_fail_base (fmt=0x7ffff6c317d8 "%s%s%s:%u: %s%sAssertion `%s' failed.\n%n", assertion=assertion@entry=0x54dc9b "container != NULL", 
        file=file@entry=0x54dc8d "ucl_msgpack.c", line=line@entry=845, function=function@entry=0x54e040 <__PRETTY_FUNCTION__.7030> "ucl_msgpack_insert_object") at assert.c:92
    #3  0x00007ffff6aaa412 in __GI___assert_fail (assertion=assertion@entry=0x54dc9b "container != NULL", file=file@entry=0x54dc8d "ucl_msgpack.c", line=line@entry=845, 
        function=function@entry=0x54e040 <__PRETTY_FUNCTION__.7030> "ucl_msgpack_insert_object") at assert.c:101
    #4  0x000000000052f89b in ucl_msgpack_insert_object (parser=parser@entry=0x60f000000040, key=key@entry=0x0, keylen=keylen@entry=0, obj=<optimized out>) at ucl_msgpack.c:845
    #5  0x000000000053060f in ucl_msgpack_consume (parser=0x60f000000040) at ucl_msgpack.c:1251
    #6  ucl_parse_msgpack (parser=parser@entry=0x60f000000040) at ucl_msgpack.c:1346
    #7  0x000000000052321f in ucl_parser_add_chunk_full (parser=0x60f000000040, data=<optimized out>, len=<optimized out>, priority=0, strat=UCL_DUPLICATE_APPEND, parse_type=<optimized out>)
        at ucl_parser.c:2952
    #8  0x000000000051bb37 in LLVMFuzzerTestOneInput (data=0x602000000010 <incomplete sequence \337>, size=4) at ucl_msgpack_fuzzer.c:25
    #9  0x000000000051c3ce in ExecuteFilesOnyByOne (argc=2, argv=0x7fffffffdcc8) at afl/afl_driver.cpp:265
    #10 0x000000000051cd18 in main (argc=2, argv=0x7fffffffdcc8) at afl/afl_driver.cpp:302

[test driver](https://github.com/ChijinZ/security_advisories/blob/master/libucl-230f197/ucl_msgpack_fuzzer.c)

[attack vector](https://github.com/ChijinZ/security_advisories/blob/master/libucl-230f197/assertion-failure)
