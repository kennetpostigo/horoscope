<div style="display: flex; flex: 1; align-items: center; justify-content: center;">
<img src="./assets/logo.png" align="center" height="300px">
</div>
<h1 align="center">arscheduler</h1>
<div align="center">
 <strong>
   Advanced Rust Scheduler
 </strong>
</div>

<br />

<div align="center">
   <!-- CI status -->
  <a href="https://github.com/hyperfuse/arscheduler/actions">
    <img src="https://github.com/hyperfuse-rs/arscheduler/workflows/CI/badge.svg"
      alt="CI Status" />
  </a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/arscheduler">
    <img src="https://img.shields.io/crates/v/arscheduler.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/async-std">
    <img src="https://img.shields.io/crates/d/arscheduler.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/arscheduler">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/arscheduler">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/hyperfuse/arscheduler/releases">
      Releases
    </a>
  </h3>
</div>

<br/>

This crate provides a general purpose scheduler. The arscheduler scheduler
provides functionality out the box and is easily extensible.

## Features

- **Fast:** arscheduler runs magnitudes faster than schedulers built in other
  languages that are mainstream. stream, arscheduler runs magnitudes faster
- **Fast:** Our robust allocator and threadpool designs provide ultra-high
  throughput with predictably low latency.
- **Intuitive:** You get a lot of powerful functionality out of the box to run
  CRON, Network tasks, retrieve jobs from memory, pg, and redis. If they are not
  what you are looking for, Implement your own Job, Store, or Executor.
- **Easy to learn:** [Detailed documentation][docs]

[docs]: https://docs.rs/arscheduler

## Examples

```rust
use arscheduler::executor::Executor;
use arscheduler::job::network::{Job, NetType};
use arscheduler::scheduler::{Schedule, blocking, daemon, Msg};
use arscheduler::store::memory::Store;

fn main() {
    let store = Store::new(String::from("jobStore-test"));
    let exec = Executor::new(String::from("executor-test"));
    let njob = Job::new(
        String::from("job-1"),
        String::from("https://ping.me/"),
        NetType::Get,
        None,
    );

    let mut blk_scheduler = blocking::Scheduler::new();
    blk_scheduler
        .add_store(String::from("jobStore-test"), Box::new(store))
        .unwrap();
    blk_scheduler
        .add_executor(String::from("executor-test"), exec)
        .unwrap();

    let scheduler = daemon(blk_scheduler);

    scheduler
        .send(Msg::AddJob(
            String::from("job-1"),
            String::from("jobStore-test"),
            String::from("executor-test"),
            start_time,
            None,
            Box::new(njob),
        ))
        .await;
}
```

More examples, including networking and file access, can be found in our
[`examples`] directory and in our [documentation].

[`examples`]: https://github.com/hyperfuse/arscheduler/tree/master/examples
[documentation]: https://docs.rs/arscheduler#examples

## Philosophy

We believe it's helpful to have general implementations of common use-cases, but
will always allow you to take over and implement to your liking.

## Installation

With [cargo add][cargo-add] installed run:

```sh
$ cargo add arscheduler
```

[cargo-add]: https://github.com/killercup/cargo-edit

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
