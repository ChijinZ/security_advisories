# Undefined behavior in config_setting_set_string (libconfig.c:1178)
There is a undefined-behavior vulnerability in libconfig (git repository: https://github.com/hyperrealm/libconfig, Latest commit f53e5de on Dec 20, 2019).

When a malicious testcase is input into test driver, the program exits by SEGV signal. UndefinedBehaviorSanitizer provided information as below:


    UndefinedBehaviorSanitizer:DEADLYSIGNAL
    ==18512==ERROR: UndefinedBehaviorSanitizer: SEGV on unknown address 0x000000000008 (pc 0x00000043248e bp 0x7fff694ee6f0 sp 0x7fff694ee610 T18512)
    ==18512==The signal is caused by a READ memory access.
    ==18512==Hint: address points to the zero page.
        #0 0x43248d in config_setting_set_string /home/fengyutong/libconfig/lib/libconfig.c:1178
        #1 0x429d7a in main /home/fengyutong/libconfig/examples/c/afl2.c:71:3
        #2 0x7f8445767b96 in __libc_start_main /build/glibc-OTsEL5/glibc-2.27/csu/../csu/libc-start.c:310
        #3 0x4049e8 in _start (/home/jin/Documents/tmp/crash_summary/crash_summary/libconfig_summary/honggfuzzer2+0x4049e8)

    UndefinedBehaviorSanitizer can not provide additional info.
    ==18512==ABORTING


[test driver](https://github.com/ChijinZ/security_advisories/blob/master/libconfig-f53e5de/driver.c)

[attack vector](https://github.com/ChijinZ/security_advisories/blob/master/libconfig-f53e5de/undefined-behavior-libconfig.c:1178)

# Undefined behavior in __config_name_compare (libconfig.c:134)
There is a undefined-behavior vulnerability in libconfig (git repository: https://github.com/hyperrealm/libconfig, Latest commit f53e5de on Dec 20, 2019).

When a malicious testcase is input into test driver, the program exits by SEGV signal. UndefinedBehaviorSanitizer provided information as below:

    UndefinedBehaviorSanitizer:DEADLYSIGNAL
    ==18518==ERROR: UndefinedBehaviorSanitizer: SEGV on unknown address 0x000000000000 (pc 0x00000042a1f8 bp 0x00000b266290 sp 0x7ffc42848f80 T18518)
    ==18518==The signal is caused by a READ memory access.
    ==18518==Hint: address points to the zero page.
        #0 0x42a1f7 in __config_name_compare /home/fengyutong/libconfig/lib/libconfig.c:134
        #1 0x42a1f7 in __config_list_search /home/fengyutong/libconfig/lib/libconfig.c:403
        #2 0x438285 in config_setting_get_member /home/fengyutong/libconfig/lib/libconfig.c:1562
        #3 0x438285 in config_setting_add /home/fengyutong/libconfig/lib/libconfig.c:1630
        #4 0x429d47 in main /home/fengyutong/libconfig/examples/c/afl2.c:68:11
        #5 0x7fc4c1b15b96 in __libc_start_main /build/glibc-OTsEL5/glibc-2.27/csu/../csu/libc-start.c:310
        #6 0x4049e8 in _start (/home/jin/Documents/tmp/crash_summary/crash_summary/libconfig_summary/honggfuzzer2+0x4049e8)

    UndefinedBehaviorSanitizer can not provide additional info.
    ==18518==ABORTING

[test driver](https://github.com/ChijinZ/security_advisories/blob/master/libconfig-f53e5de/driver.c)

[attack vector](https://github.com/ChijinZ/security_advisories/blob/master/libconfig-f53e5de/undefined-behavior-libconfig.c:134)
