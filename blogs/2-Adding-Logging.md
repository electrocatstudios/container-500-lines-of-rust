## Adding Logging

### Overview
We need a way of logging output from the container. Rust has a standard way of logging that
any logger implementation needs to implement. The original guide used the `env_logger` crate,
but we will be using `std_logging` as this has been more recently updated so looks like a
more actively maintained crate. 