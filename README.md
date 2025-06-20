# Geyser Lagging behind

## Outline:
1. ### Certain heavy subscriptions lag behind the current slot.
2. ### The lag behind increases as time goes on before the connection drops.
3. ### Other lighter concurrent streams are not affected.
4. ### The lag is not caused by the lack of resources on the server (shyft grpc is not lagging behind ).
5. ### Opening multiple of the streams doesn't change the situation
6. ### The bandwith for such a subscription feels low, other geysers average 80-100mbps and the ones on your node average 30-50mbps



# Server Usage:
![Screenshot 2025-06-20 at 14.49.37.png](img/Screenshot%202025-06-20%20at%2014.49.37.png)
# Stream Slots
![Screenshot 2025-06-20 at 14.28.31.png](img/Screenshot%202025-06-20%20at%2014.28.31.png)

# DroppedConnection
``` thread 'main' panicked at src/main.rs:4:35:
called `Result::unwrap()` on an `Err` value: "status: Internal, message: \"lagged to send an update\", details: [], metadata: MetadataMap { headers: {} }"
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/std/src/panicking.rs:697:5
   1: core::panicking::panic_fmt
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/core/src/panicking.rs:75:14
   2: core::result::unwrap_failed
             at /rustc/17067e9ac6d7ecb70e50f92c1944e545188d2359/library/core/src/result.rs:1704:5
   3: core::result::Result<T,E>::unwrap
             at /Users/serreburro/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/result.rs:1109:23
   4: geyser_test::main::{{closure}}
             at ./src/main.rs:4:5
   5: <core::pin::Pin<P> as core::future::future::Future>::poll
             at /Users/serreburro/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/future/future.rs:124:9
   6: tokio::runtime::park::CachedParkThread::block_on::{{closure}}
             at /Users/serreburro/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.45.1/src/runtime/park.rs:284:60
   7: tokio::task::coop::with_budget
             at /Users/serreburro/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.45.1/src/task/coop/mod.rs:167:5
   8: tokio::task::coop::budget
             at /Users/serreburro/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.45.1/src/task/coop/mod.rs:133:5
   9: tokio::runtime::park::CachedParkThread::block_on
             at /Users/serreburro/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.45.1/src/runtime/park.rs:284:31
  10: tokio::runtime::context::blocking::BlockingRegionGuard::block_on
             at /Users/serreburro/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.45.1/src/runtime/context/blocking.rs:66:9
  11: tokio::runtime::scheduler::multi_thread::MultiThread::block_on::{{closure}}
             at /Users/serreburro/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.45.1/src/runtime/scheduler/multi_thread/mod.rs:87:13
  12: tokio::runtime::context::runtime::enter_runtime
             at /Users/serreburro/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.45.1/src/runtime/context/runtime.rs:65:16
  13: tokio::runtime::scheduler::multi_thread::MultiThread::block_on
             at /Users/serreburro/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.45.1/src/runtime/scheduler/multi_thread/mod.rs:86:9
  14: tokio::runtime::runtime::Runtime::block_on_inner
             at /Users/serreburro/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.45.1/src/runtime/runtime.rs:358:45
  15: tokio::runtime::runtime::Runtime::block_on
             at /Users/serreburro/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.45.1/src/runtime/runtime.rs:328:13
  16: geyser_test::main
             at ./src/main.rs:4:5
  17: core::ops::function::FnOnce::call_once
             at /Users/serreburro/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```


# Our Assumptions
## _This feels like a limitation of bandwith for a single stream because upon opening multiple streams the total bandwith increases as expected._