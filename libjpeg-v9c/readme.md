# A vulnerability in libjpeg-v9c

When https://github.com/ChijinZ/security_advisories/blob/master/libjpeg-v9c/large_loop is input into cjpeg
(command: .lib/cjpeg large_loop > out), it takes a long time (~6 min) to finish the compression and output a 21979501 bytes output file while the input file is 6142 bytes.

perf report provided information as follow:

    #perf record  .libs/cjpeg testfile > /dev/null
    
    [ perf record: Woken up 229 times to write data ]
    [ perf record: Captured and wrote 57.992 MB perf.data (1519923 samples) ]
    
    #perf report
    
    Samples: 1M of event 'cpu-clock', Event count (approx.): 379980750000
    Overhead  Command  Shared Object      Symbol                                   ◆
    11.81%  cjpeg    [kernel.kallsyms]  [k] entry_SYSCALL_64_after_swapgs        ▒
    7.90%  cjpeg    libc-2.23.so       [.] __GI___libc_read                     ▒
    6.16%  cjpeg    [kernel.kallsyms]  [k] generic_file_read_iter               ▒
    5.63%  cjpeg    cjpeg              [.] read_non_rle_pixel                   ▒
    5.24%  cjpeg    [kernel.kallsyms]  [k] find_get_entry                       ▒
    4.29%  cjpeg    [kernel.kallsyms]  [k] put_page                             ▒
    4.08%  cjpeg    libc-2.23.so       [.] _IO_getc                             ▒
    3.66%  cjpeg    libc-2.23.so       [.] _IO_file_underflow@@GLIBC_2.2.5      ▒
    3.16%  cjpeg    libjpeg.so.9.1.0   [.] encode_mcu_huff                      ▒
    2.99%  cjpeg    [kernel.kallsyms]  [k] __radix_tree_lookup                  ▒
    2.70%  cjpeg    [kernel.kallsyms]  [k] new_sync_read                        ▒
    2.48%  cjpeg    [kernel.kallsyms]  [k] security_file_permission             ▒
    2.28%  cjpeg    [kernel.kallsyms]  [k] common_file_perm                     ▒
    2.28%  cjpeg    [kernel.kallsyms]  [k] fsnotify                             ▒
    2.20%  cjpeg    libjpeg.so.9.1.0   [.] forward_DCT                          ▒
    2.14%  cjpeg    [kernel.kallsyms]  [k] vfs_read                             ▒
    2.13%  cjpeg    [kernel.kallsyms]  [k] sys_read                             ▒
    1.68%  cjpeg    [kernel.kallsyms]  [k] aa_file_perm                         ▒
    1.64%  cjpeg    libc-2.23.so       [.] _IO_switch_to_get_mode               ▒
    1.62%  cjpeg    [kernel.kallsyms]  [k] rw_verify_area                       ▒
    1.61%  cjpeg    [kernel.kallsyms]  [k] pagecache_get_page

I have got in touch with JPEG group. They have verified it is a vulnerability and will fixed in the next version (jpeg-v9d). (see: https://github.com/ChijinZ/security_advisories/blob/master/libjpeg-v9c/mail.pdf)