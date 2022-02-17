## HTTP Server in Rust

Go to [rust-lang.org](https://rust-lang.org) and click on the `Get Started` button and follow the instructions to install Rust for your operating system.

You should get somewhat similar output if you run commands like the ones below (newer versions are okay).  If you 
already have an old version of Rust installed, then run `rustup update` to install a newer version.

```shell
$ rustc --version
rustc 1.58.1 (db9d1b20b 2022-01-20)
$ cargo --version
cargo 1.58.0 (f01b232bc 2022-01-19)
```

Cargo is a Rust's build system and package manger. 
Cargo helps in downloading the libraries on which code depends on, and building those libraries.

To create new project using cargo run `cargo new my_project --bin or --lib` 
this will generate a "Hello, world!" project for us! <br />
We can run this program by moving into the new directory that we made and running this in our terminal: `cargo run`

```shell
$ cargo run
   Compiling my_project v0.1.0 (C:\Users\gavin\Documents\Rust\my_project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.49s
     Running `target\debug\my_project.exe`
Hello, world!
```

### :handshake: Contributing
Contributions, issues and feature requests are welcome!<br />Feel free to check [issues page](https://github.com/gavincapriola/server/issues).

### Show your support
Give a :star: if this project helped you!
