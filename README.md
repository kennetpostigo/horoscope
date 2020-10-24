<div align="center" style="display: flex; flex: 1; align-items: center; justify-content: center;">
<img src="./assets/logo.png" align="center" height="300px">
</div>
<h1 align="center">horoscope</h1>
<div align="center">
 <strong>
   Advanced Rust Scheduler
 </strong>
</div>

<br />

<div align="center">
   <!-- CI status -->
  <a href="https://github.com/kennetpostigo/horoscope/actions">
    <img src="https://github.com/kennetpostigo/horoscope/workflows/CI/badge.svg?style=flat" alt="CI Badge"/>
  </a>
  <!-- Coverage % -->
  <a href="https://codecov.io/gh/kennetpostigo/horoscope">
    <img src="https://codecov.io/gh/kennetpostigo/horoscope/branch/main/graph/badge.svg?token=XW3IOH1XIG&style=flat"/>
  </a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/horoscope">
    <img src="https://img.shields.io/crates/v/horoscope.svg?style=flat"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/horoscope">
    <img src="https://img.shields.io/crates/d/horoscope.svg?style=flat"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/horoscope">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/horoscope">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/hyperfuse/horoscope/releases">
      Releases
    </a>
  </h3>
</div>

<br/>

This crate provides a general purpose scheduler. The horoscope scheduler
provides functionality out the box and is easily extensible.

## Features

- **Fast:** horoscope runs magnitudes faster than schedulers built in other
  languages that are mainstream. stream, horoscope runs magnitudes faster
- **Concurrent:** executors run jobs in their own tasks in order to get as much
  work done as fast as possible.
- **Intuitive:** You get a lot of powerful functionality out of the box to run
  CRON, Network tasks, retrieve jobs from memory, pg, and redis. If they are not
  what you are looking for, Implement your own Job, Trigger, Store, or Executor.
- **Easy to learn:** [Detailed documentation][docs]

[docs]: https://docs.rs/horoscope

## Examples

```rust
use horoscope::executor::Executor;
use horoscope::job::network::{Job, NetType};
use horoscope::scheduler::{Schedule, blocking, daemon, Msg};
use horoscope::store::memory::Store;

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

[`examples`]: https://github.com/hyperfuse/horoscope/tree/master/examples
[documentation]: https://docs.rs/horoscope#examples

## Philosophy

We believe it's helpful to have general implementations of common use-cases, but
will always allow you to take over and implement to your liking.

## Installation

With [cargo add][cargo-add] installed run:

```sh
$ cargo add horoscope
```

[cargo-add]: https://github.com/killercup/cargo-edit

## License

Licensed under <a href="http://www.apache.org/licenses/LICENSE-2.0">Apache License, Version
2.0</a>
