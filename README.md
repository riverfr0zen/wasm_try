# Issue resolved

The <a href="https://github.com/bevyengine/bevy/issues/3867">issue</a> this repo was created to illustrate has since been resolved. Leaving this here for historical reasons.

# WASMs I create crash Chrome unless devtools are open

I'm new to Rust & Bevy, and this is the first time I'm trying to target wasm. Unfortunately, I've come across a strange situation:

When I try to access the wasms I've created in Chrome (98.0.4758.80 on Ubuntu 21.10), unless I have the developer tools open (by clicking Inspect), the browser tab crashes  (or sometimes Chrome itself just flat out crashes). If I have the dev tools open, everything works fine. 

How I am building the wasm assets:

```
cargo build --example rect_eg --target wasm32-unknown-unknown

wasm-bindgen --out-dir examples/wasm/target --target web target/wasm32-unknown-unknown/debug/examples/rect_eg.wasm

```

You can see my failing set up here: https://github.com/riverfr0zen/wasm_try

The code is simply the rectangle example from here: https://bevyengine.org/examples/

Note that I can run all the examples at the link above in Chrome without a problem. The issue is happening only with wasms that I create on my machine.


The following is the output from Chrome when it crashes: 

```
[0204/174803.628690:ERROR:ptracer.cc(422)] ptrace: No such process (3)
[0204/174803.628712:ERROR:ptracer.cc(446)] Unexpected registers size 0 != 216
[0204/174803.628715:WARNING:process_reader_linux.cc(379)] Couldn't initialize main thread.
[0204/174803.675713:ERROR:scoped_ptrace_attach.cc(37)] process not stopped
[0204/174803.890453:ERROR:scoped_ptrace_attach.cc(37)] process not stopped
[0204/174803.890482:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890492:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890500:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890507:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890514:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890522:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890529:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890536:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890543:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890550:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890558:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890566:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890572:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890578:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890584:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890590:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890596:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890602:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890619:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890625:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890631:ERROR:scoped_ptrace_attach.cc(27)] ptrace: No such process (3)
[0204/174803.890661:ERROR:file_io_posix.cc(144)] open /proc/54187/auxv: Permission denied (13)
[0204/174803.890674:ERROR:process_memory.cc(41)] short read
[0204/174803.890699:ERROR:process_snapshot_linux.cc(78)] Couldn't read exception info
[0204/174803.891235:ERROR:scoped_ptrace_attach.cc(45)] ptrace: No such process (3)
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
