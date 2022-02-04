# WASMs I create crash Chrome unless devtools are open

I'm new to Rust & Bevy, and this is the first time I'm trying to target wasm. Unfortunately, I've across a strange situation:

The issue is that when I try to access the wasms I create in Chrome (98.0.4758.80 on Ubuntu 21.10), unless I have the developer tools open (by clicking Inspect), the browser tab crashes  (or sometimes Chrome itself just flat out crashes). If I have the dev tools open, everything works fine. 

How I am trying to build the wasm assets:

```
cargo build --example rect_eg --target wasm32-unknown-unknown

wasm-bindgen --out-dir examples/wasm/target --target web target/wasm32-unknown-unknown/debug/examples/rect_eg.wasm

```

The code is simply the rectangle example from here: https://bevyengine.org/examples/

Note that I can run all the examples at the link above in Chrome without a problem. The issue is happening only with wasms that I create on my machine.


The following is the output from Chrome when it crashes:

```
irf@unicron:~/htdocs/wasm_try$ chrome
[44298:44326:0204/171246.483872:ERROR:chrome_browser_main_extra_parts_metrics.cc(227)] START: ReportBluetoothAvailability(). If you don't see the END: message, this is crbug.com/1216328.
[44298:44326:0204/171246.483889:ERROR:chrome_browser_main_extra_parts_metrics.cc(230)] END: ReportBluetoothAvailability()
[0204/171306.778301:ERROR:directory_reader_posix.cc(42)] opendir /home/irf/.config/google-chrome/Crash Reports/attachments/7b1af2df-7313-4960-a99a-fe42853b30ab: No such file or directory (2)
[0204/171306.968001:ERROR:ptracer.cc(422)] ptrace: No such process (3)
[0204/171306.968014:ERROR:ptracer.cc(446)] Unexpected registers size 0 != 216
[0204/171306.968017:WARNING:process_reader_linux.cc(379)] Couldn't initialize main thread.
[0204/171307.138819:ERROR:scoped_ptrace_attach.cc(37)] process not stopped
[0204/171307.138845:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138855:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138860:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138864:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138868:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138873:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138877:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138881:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138886:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138890:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138894:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138899:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138903:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138907:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138912:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138916:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.138920:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.367401:ERROR:scoped_ptrace_attach.cc(37)] process not stopped
[0204/171307.367423:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.367432:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.367435:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.367438:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/171307.367457:ERROR:file_io_posix.cc(144)] open /proc/44480/auxv: Permission denied (13)
[0204/171307.367479:ERROR:process_memory.cc(41)] short read
[0204/171307.367482:ERROR:process_snapshot_linux.cc(78)] Couldn't read exception info
[0204/171307.368028:ERROR:scoped_ptrace_attach.cc(45)] ptrace: No such process (3)
```

If I have the developer tools open, everything works as expected. I do get the error below in the console log, but I believe it is expected.

```
rect_eg.js:1603 Uncaught (in promise) Error: Using exceptions for control flow, don't mind me. This isn't actually an error!
    at imports.wbg.__wbindgen_throw (rect_eg.js:1603:15)
    at wasm_bindgen::throw_str::h8e401d0f67754eb8 (rect_eg_bg.wasm:0x22d651d)
    at winit::platform_impl::platform::backend::throw::h531b3cf71a47d874 (rect_eg_bg.wasm:0x23a6362)
    at winit::platform_impl::platform::event_loop::EventLoop<T>::run::h559a9df9dbd65dac (rect_eg_bg.wasm:0x19f5278)
    at winit::event_loop::EventLoop<T>::run::haac6a08581092ff4 (rect_eg_bg.wasm:0x2089458)
    at bevy_winit::run::hb888c6e926570c1a (rect_eg_bg.wasm:0x20892e6)
    at bevy_winit::winit_runner_with::h44e91f337cbf6d9e (rect_eg_bg.wasm:0x9b5469)
    at bevy_winit::winit_runner::h18d6d36966529599 (rect_eg_bg.wasm:0x21e1617)
    at core::ops::function::Fn::call::hc945f559e333846d (rect_eg_bg.wasm:0x1f2bf83)
    at <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hf9a55069464dcd49 (rect_eg_bg.wasm:0x1b9c3e9)
```