# Invalid Address Accsss in lldb.rs

There is a vulnerability in lldb.rs (git repository: https://github.com/mity/md4c, commit 28c3e86 on July 9, 2018).

When I use main.rs (see: ) as driver to call lldb.rs libaray to checkout coredump file, it raises a *SIGSEGV* signal.

lldb provide information as bellow: 

    (lldb) bt
    * thread #1, name = 'crash_analyse', stop reason = signal SIGSEGV: invalid address (fault address: 0x0)
    * frame #0: 0x00007ffff60ec661 libc.so.6`__strlen_avx2 + 17
        frame #1: 0x000055555556b859 crash_analyse`std::ffi::c_str::CStr::from_ptr::h7d6778659191997d at c_str.rs:904
        frame #2: 0x000055555555ed45 crash_analyse`lldb::frame::SBFrame::function_name::h6e8217ef78893c79(self=0x00007fffffffe848) at frame.rs:153

I pulled request to the repository and the author have already merge the request to the master brach.( see: https://github.com/endoli/lldb.rs/commit/10873dcab6e8d2478f2164f47b552cae3c796cdf).
