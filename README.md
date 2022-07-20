# WasmWeb

A Webassembly prodcedure call demo

## Description

This monorepo contains three codebases.
One for a guest, the web assembly binary the rest of the demo uses
Another for the host, the program that actually runs the binary,
and a third, server, serves web assembly binaries for the host to run


## How to use

### Requirements

* Rust (^1.58)

### Building the guest 

`cd` into the guest folder and run `make`.
This will produce a binary in the `build/` direciory.
To get ahead you should copy this into the servers `bin/` directory.

### Starting the server 

`cd` the server directory and run `cargo run`.
This should start the server on `localhost:3000` serving all the binaries in the `bin` folder.

### Using the host

``` sh
USAGE:
    host [OPTIONS] --bin <BIN> --arg <ARG>

For more information try --help
```

Once in the host folder you can use `cargo run` to run the host.
Without any arguments you'll get this useage info.
So to run the example you would use the command `cargo run -- -b gues -a name`
