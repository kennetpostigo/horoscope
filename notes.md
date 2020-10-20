# Info

This right here is just a lil brain dump for my future self to quickly reference
in case I forget why/how something works a specific way.

## Scheduler/mod.rs

### Enums

This file contains the `SchedulerState` enum and the `Msg` enum.

- `SchedulerState` describes the state of the scheduler at the current moment
- `Msg` enum contains the types of messages the scheduler can act upon

### Function `daemon`

The `daemon` function will take a scheduler and return a copy of both sides of a
channel connected to the scheduler. `daemon` will keep the scheduler running in
the background to keep the main thread unblocked.

### Trait `Schedule`

The `Schedule` trait describes the minimum implementation a `Scheduler` must
adhere too.

## Scheduler/blocking.rs

### Blocking Scheduler

This file contains an implementation of the `Schedule` trait that is a blocking
`Scheduler`.

The `Scheduler` implementation manages the following data:

- state: The state of the scheduler at the moment
- stores: A HMap of stores that the scheduler reads to determine if there are
  jobs to run
- executors: A HMap of executors that are assigned to run jobs
- logger: Instance of a logger that the scheduler uses to print events

## Store/mod.rs

### Enums

Contains a single enum `JobState`, that is either `Success` or `Failure` State.
However, not certain this should be in the Store, should come from the `Job` folder.

### Trait `Silo`

The `Silo` trait contains an implementation for stores. There are a handful of
methods on the trait that proxy to Jobs(modify, pause, resume, remove). However
there are 3 methods unique to the store `startup` and `teardown` are pretty
self explanatory but `get_due_jobs` is probably the most important methods on
the `Silo` trait.

`get_due_jobs` is in charge of collecting `Vec<Job>` that are
ready to be ran or executed.

### Struct `Store`

The `Store` struct in this file is what you can call a wrapper, bag, container
that holds an actual implementation of Silo trait. I want to get rid of it, but
haven't figured out how to get rid of it yet. The stuct holds a String alias and
store which is the Silo implementation.

## Store/memory.rs

### Stuct `Store`

The `Store` struct defined in here is a struct that actually implements the
`Silo` trait for the in memory `Store`.

## Executor/mod.rs

### Struct `Executor`

This `Executor` struct is made up of an alias field that is used to identify an
instance of the `Executor`. It has 4 methods:

- `new` to create an instance of an executor
- `startup` to initialize an Executor
- `teardown` to destroy the Executor
- `execute` takes a job and runs the `func` method on it to run the job and
  returns the result of the `Job`

## Job/mod.rs

### Enums

This file contains a single enum, `Status` meant to model the status of a `Job`.
The `Status` enum implements a `to_string` method for printing.

- `Waiting` gets set when a job is in the store and is waiting to be executed, 
whether that is before it's been called or is waiting to get called again.
- `Running` called set on startup
- `Paused` can happen at any time and startup/func/teardown/get_due_jobs should check for this
- `Success` gets set in `func`
- `Failure(String)` gets set in `func`

### Trait `Work`

Work is an trait for Jobs to implement, it is made up of a few methods:

- `startup` TODO: Determine is this is needed
- `func` Is the logic that runs when a job is executed by an executor
- `teardown` TODO: Determine if this is needed

### Struct `Job`

The `Job` struct in this file is a wrapper around a type that implements the
`Work` trait. However it has additional data fields around it that make it
essential to use, specifically:

- `state` is the current state of the `Job`
- `alias` can be seen as an identifier for a `Job`
- `executor` is the executor to use to run the `Job`
- `start_time` is when to execute a `Job`
- `end_time` is an optional time we should stop trying to execute a `Job`(?)
- `triggers` is the conditions that need to pass to actually run a `Job`

The methods on a `Job` are mostly getters and setters for the `Job` or
`Trigger`s, the exception is `validate_trigger`:

- `new` used to create an instance of a `Job`
- `validate_trigger` used to determine if all `Trigger`s pass to run the `Job`
- `modify_job` used to alter details of the `Job`
- `pause_job` used to pause a `Job`
- `resume_job` used to resume a `Job`
- `add_trigger` used to add a `Trigger`
- `remove_trigger` used to remove as `Trigger`

## Job/network.rs

This type of Job is only responsible for running network Jobs.

### Enum

This file contains a single `NetType` enum that at the moment supports `Get` or 
`Post` network calls.

### Struct `Job`

The `Job` struct in this file differs from the `Job` Struct in the `mod.rs` file 
because it's a struct that implements the `Work` trait and doesn't care about 
the details of `Trigger`s that determine if it will run and `Executor` that will 
run the `Job`. In this case this `Job` only cares about the Network details and 
keeps the following data:

- `alias` Job alias in case it's passed outside of it's wrapper
- `url` API/SITE URL
- `method` HTTP Method
- `headers` HTTP Headers
- `body` HTTP Body

## Job/sys.rs

This type of Job is only responsible for running system Jobs.

### Struct `Job`

The `Job` struct in this file differs from the `Job` Struct in the `mod.rs` file 
because it's a struct that implements the `Work` trait and doesn't care about 
the details of `Trigger`s that determine if it will run and `Executor` that will 
run the `Job`. In this case this `Job` only cares about the System scripts it 
needs to run, the data it holds is:

- `alias` Job alias in case it's passed outside of it's wrapper.
- `script` Name of the system script/program that will be executed.
- `args` The arguments the script/program requires to be able to run.

## Ledger/mod.rs

The ledger module is responsible for keeping a history of the events that occur 
while the scheduler is running.

### Stuct `Ledger`

The `Ledger` struct in this file is a wrapper around a type that implements the
`History` trait. However it has additional data fields around it that make it
essential to use, specifically:

- `alias` is useful in case the ledger gets full and we need to store it in a 
remote database.

## Checklist

1. Create a loop in ledger that is ran every couple MS to sync the local `DB`
2. Implement `listeners`/`event` system
3. Start building out a testsuite
4. Library website similar to async that has notes/guides and link to generated 
api docs