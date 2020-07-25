# Buffer overflow when continuously send SIGHUP to postgres
## Bug Report
See: [link to mails](https://www.postgresql.org/message-id/CAA8ZSMqAHDCgo07hqKoM5XJaoQy6Vv76O7966agez4ffyQktkA@mail.gmail.com)

## Details

REPRODUCTION && ERROR MESSAGE:
1. initialize database with executable binary "initdb";
2. start a server with "postgress" (command: "./postgres -Ddata -p23333");
3. continuously send SIGHUP to the server process like:
```
for (( ; ; ))
do kill -s SIGHUP $PID
done
```
4. and the server process exit with address boundary error.
“./postgres -Ddata -p23333” terminated by signal SIGSEGV (Address boundary
error)



Further, I use AddressSanitizer to re-compile the project, and Asan report
shows the call stack as bellow:



==21548==ERROR: AddressSanitizer: stack-overflow on address 0x7fffb3d5a808 (
pc 0x0000004a44c5 bp 0x7fffb3d5b070 sp 0x7fffb3d5a810 T0)
    #0 0x4a44c4 in __interceptor_strlen (path/to/sql/build/bin/postgres
+0x4a44c4)
    #1 0x1b5692d in pg_tzset path/to/sql/src/timezone/pgtz.c:244:6
    #2 0xd68ec6 in check_log_timezone path/to/sql/src/backend
/commands/variable.c:415:11
    #3 0x1a89ee7 in call_string_check_hook path/to/sql/src/backend/utils
/misc/guc.c:10810:7
    #4 0x1a88913 in parse_and_validate_value path/to/sql/src/backend/utils
/misc/guc.c:6623:10
    #5 0x1a84fa0 in set_config_option path/to/sql/src/backend/utils/misc/guc
.c:7226:11
    #6 0x1a9aff9 in ProcessConfigFileInternal path/to/sql/src/backend/utils
/misc/guc-file.l:440:11
    #7 0x1a7abbf in ProcessConfigFile path/to/sql/src/backend/utils/misc/guc
-file.l:156:9
    #8 0x1267473 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2636:3
    #9 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #10 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #11 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #12 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #13 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #14 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #15 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #16 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #17 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #18 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #19 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #20 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #21 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #22 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #23 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #24 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #25 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #26 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #27 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #28 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #29 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #30 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #31 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #32 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #33 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #34 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #35 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #36 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #37 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #38 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #39 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #40 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #41 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #42 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #43 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #44 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #45 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #46 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #47 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #48 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #49 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #50 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #51 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #52 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #53 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #54 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #55 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #56 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #57 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #58 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #59 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #60 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #61 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #62 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #63 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #64 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #65 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #66 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #67 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #68 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #69 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #70 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #71 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #72 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #73 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #74 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #75 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #76 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #77 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #78 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #79 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #80 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #81 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #82 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #83 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #84 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #85 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #86 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #87 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #88 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #89 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #90 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #91 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #92 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #93 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #94 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #95 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #96 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #97 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #98 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #99 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #100 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #101 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #102 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #103 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #104 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #105 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #106 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #107 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #108 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #109 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #110 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #111 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #112 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #113 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #114 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #115 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #116 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #117 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #118 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #119 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #120 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #121 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #122 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #123 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #124 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #125 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #126 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #127 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #128 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #129 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #130 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #131 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #132 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #133 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #134 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #135 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #136 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #137 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #138 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #139 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #140 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #141 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #142 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #143 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #144 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #145 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #146 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #147 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #148 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #149 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #150 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #151 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #152 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #153 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #154 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #155 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #156 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #157 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #158 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #159 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #160 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #161 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #162 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #163 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #164 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #165 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #166 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #167 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #168 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #169 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #170 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #171 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #172 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #173 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #174 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #175 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #176 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #177 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #178 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #179 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #180 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #181 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #182 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #183 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #184 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #185 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #186 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #187 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #188 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #189 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #190 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #191 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #192 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #193 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #194 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #195 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #196 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #197 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #198 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #199 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #200 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #201 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #202 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #203 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #204 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #205 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #206 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #207 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #208 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #209 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #210 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #211 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #212 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #213 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #214 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #215 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #216 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #217 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #218 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #219 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #220 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #221 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #222 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #223 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #224 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #225 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #226 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #227 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #228 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #229 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #230 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #231 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #232 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #233 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #234 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #235 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #236 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #237 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #238 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #239 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #240 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #241 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #242 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #243 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #244 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2
    #245 0x7f8ebb41488f  (/lib/x86_64-linux-gnu/libpthread.so.0+0x1288f)
    #246 0x7f8ebaa2c14c in sigprocmask /build/glibc-OTsEL5/glibc
-2.27/signal/../sysdeps/unix/sysv/linux/x86_64/sigprocmask.c:36
    #247 0x4dedf3 in __interceptor_sigprocmask (path/to/sql/build/bin/
postgres+0x4dedf3)
    #248 0x12676a3 in SIGHUP_handler path/to/sql/src/backend
/postmaster/postmaster.c:2690:2



SUMMARY: AddressSanitizer: stack-overflow
(path/to/sql/build/bin/postgres+0x4a44c4)
in __interceptor_strlen
==21548==ABORTING