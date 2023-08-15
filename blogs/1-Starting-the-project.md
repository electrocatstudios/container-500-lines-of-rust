## Starting the project

### Introduction
In this part we are setting up the application to accept arguments, meaning we 
can specify switches to customize the operation of the application. The 
[original blog post](https://github.com/litchipi/litchipi.github.io/blob/main/_posts/tutorials/containers_in_rust/2021-09-30-container-in-rust-part2.md)
uses the `structopt` crate, however the page for this crate [https://docs.rs/structopt/latest/structopt/](https://docs.rs/structopt/latest/structopt/) suggests that the `clap v3` crate is a better alternative, as `structopt` is now in maintenance mode. However 
`clap v4` is the most recent version, so we will use this.

### Clap v4 - Parsing Arguments
The [clap library](https://docs.rs/clap/latest/clap/) is an easy way to parse input arguments, following the tutorial we need to parse
the following arguments:

```rust
use clap::Parser;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Activate debug mode
    // short and long flags, (-d, --debug) derived from the field name
    #[arg(short, long)]
    debug: bool,

    /// The command to execute
    #[arg(short, long)]
    pub command: String,
    
    /// The user ID to use in the container
    #[arg(short, long)]
    pub uid: u32,

    /// The directory to be mounted as root for the container
    #[arg(short=char::from_str("m").unwrap(), long="mount")]
    pub mount_dir: PathBuf,
}
```

We can add the library by using the following command:

```bash
cargo add clap --features derive
```

And we will parse the aruments by using:

```rust
let args = cli::parse_args();
println!("{:?}", args);
```

This will be put into `cli.rs` to keep it separate in it's own module. 

### Testing the application

We can build the code (available in the branch `1-starting-the-project`) by 
running:

```bash 
cargo run
```

It should display some errors because the flags are command line args are missing. 
If you add those arguments in you should see them printed. 

```bash
cargo run -- --mount ./ --uid 0 --command "bash" --debug
```

Or using the short forms

```bash
cargo run -- -m ./ -u 0 -c "bash" -d
```