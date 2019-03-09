# Plox OS
An operating system written *(poorly)* in Rust. The two main goals of this project are to learn Rust and learn how to write an operating system.

I'm following [Writing an OS in Rust](https://os.phil-opp.com/) for this project.

## Setting up the development environment
* Install and run [Rustup](https://www.rust-lang.org/tools/install) to install Rust.
* [Visual Studio Code](https://code.visualstudio.com/) is used as my IDE.
  * [Rust (rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) plugin for syntax highlighting and autocomplete.
  * [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) for Cargo.toml editing.
* Clone this project: https://github.com/toverbay/plox-os.git
* Install xbuild to automatically cross-compile the Rust core and other built-in libraries:
  * ```cargo install cargo-xbuild```
* Add the Rust source files for xbuild to use:
  * ```rustup component add rust-src```
* This project uses experimental features of Rust from the *nightly* channel. If you would like to use a specific version of the toolchain, edit the 'rust-toolchain' file or use ```rustup override add {toolchain}```

## Building Plox

> ### Development *(unoptimized)*:
> ```cargo xbuild --target x86_64-plox_os.json```

> ### Release:
> ```cargo xbuild --target x86_64-plox_os.json --release```

I work in Windows, so I created a batch file called build.cmd that does all this for me.

```build``` for development

```build release``` for release

We could use a config file for these flags, but I'm not sure how to do it in a cross-platform way.

> ### Boot Image:
* First, install the bootimage crate: ```cargo install bootimage --version "^0.5.0"```

* Add a dependency to bootloader in Cargo.toml:
```
# in Cago.toml

[dependencies]
bootloader = "0.3.14"
```

* Build using bootimage, which wraps the cargo xbuild and combines the bootloader with the build output.
```bootimage build```

If you're developing on a Windows machine, you can use the build.cmd:
```build boot``` for development
```build boot release``` for release

git remote add origin https://github.com/toverbay/plox-os.git
git push -u origin master
