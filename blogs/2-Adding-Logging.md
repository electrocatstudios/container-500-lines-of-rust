## Adding Logging

### Overview
We need a way of logging output from the container. Rust has a standard way of logging that
any logger implementation needs to implement. The original guide used the `env_logger` crate,
but we will be using `simple_logger`. `simple_logger` is not the most recently updated logger, 
but it is in version `4.2.0` which demonstrates this is quite mature, and the last update was 
2 months ago (at time of writing) so this feels like a safe bet to prevent breakage of our
project by an update ot the language or this crate.

### Simple Logger

We can add the `simple_logger` crate and the `log` crate (which we need for items such as 
the logging level filter), by running the following command:

```bash
cargo add log
cargo add simple_logger
```

### Updates to cli.rs

We need to set the debug level - which we do through a function in `cli.rs`:

```rust 
pub fn setup_log(level: log::LevelFilter) {
    SimpleLogger::new()
        .with_level(level)
        .init()
        .unwrap();
}
```

And then we call this function from the Arg parsing function:

```rust 
    if args.debug {
        setup_log(log::LevelFilter::Debug);
    } else {
        setup_log(log::LevelFilter::Info);
    }
```

### Log instead of println in main.rs

Finally we can update `main.rs` to log the argument list rather than print it:

```rust 
    log::info!("{:?}", args);
```

### Verifying the new code

We can see the output by running the same command as before - but we should 
see a different output - with a log format including the date time:

```bash
cargo run -- --mount ./ --uid 0 --command "bash" --debug
```
