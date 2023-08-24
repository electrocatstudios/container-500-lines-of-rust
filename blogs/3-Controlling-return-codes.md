## Controlling the Return Code 

### Overview
Applications return a value to indicate to the controlling operating system
whether they exited as expected (by returning a 0) or that they encountered
an error and were either forced to exit, or otherwise halt the process. Any
non-zero value indicates an error, we will create a function to handle this
which we can call from anywhere in the application.

### Error Codes
First we create a new file `errors.rs` and define a new enum `Errcode`, and
we are populating the first type, which is an `InvalidArgument`.

```rust
#[derive(Debug)]
pub enum Errcode{
    ArgumentInvalid(&'static str),
}
```

Note: The `Debug` derive command allows the Enum to be printed nicely
using `println!("{:?}", err)`. 

### Return Codes
Also inside `errors.rs` we will create a function to exit the program passing
the return value (non-zero for an error) back to the Operating System (OS). This
[code](https://github.com/litchipi/crabcan/blob/step3/src/errors.rs) is taken 
directly from the code by [LitchiPi](https://github.com/litchipi/) 
from the [CrabCan](https://github.com/litchipi/crabcan) project.

```rust
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        // If it's a success, return 0
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0);
        },

        // If there's an error, print an error message and return the retcode
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}
```

The are a number of implmentations that we need to enact in order to allow
our `ErrCode` enum to get the error code and to format the display.

### Applying Error Codes
We will add a check in the `main()` entrypoint to check the result from the
argument parsing code, to verify the arguments passed are valid. Including
making sure the mount point exists. This also involves returning a `Result`
from 

```rust
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            errors::exit_with_retcode(Ok(()))
        },
        Err(e) => {
            log::debug!("Error while parsing args:\n\t{}", e);
            errors::exit_with_retcode(Err(e))
        }
    };
```

### Testing the Code
We can test our new code by running the following:

```bash
cargo run -- --mount ./ --uid 0 --command "bash" --debug
```

And we can check the error is returned for the invalid arument:

```bash
cargo run -- --mount ./does_not_exist --uid 0 --command "bash" --debug
```