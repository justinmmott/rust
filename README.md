# Rust Analyzer

[![Build Status](https://travis-ci.org/rust-analyzer/rust-analyzer.svg?branch=master)](https://travis-ci.org/rust-analyzer/rust-analyzer)

Rust Analyzer is an **experimental** modular compiler frontend for the Rust
language. It is a part of a larger rls-2.0 effort to create excellent IDE
support for Rust. If you want to get involved, check the rls-2.0 working group
in the compiler-team repository:

https://github.com/rust-lang/compiler-team/tree/master/working-groups/rls-2.0

Work on the Rust Analyzer is sponsored by

[![Ferrous Systems](https://ferrous-systems.com/images/ferrous-logo-text.svg)](https://ferrous-systems.com/)

## Language Server Quick Start

Rust Analyzer is a work-in-progress, so you'll have to build it from source, and
you might encounter critical bugs. That said, it is complete enough to provide a
useful IDE experience and some people use it as a daily driver.

To build rust-analyzer, you need:

* latest stable rust for language server itself
* latest stable npm and VS Code for VS Code extension (`code` should be in path)

For setup for other editors, see [./docs/user](./docs/user).

```
# clone the repo
$ git clone https://github.com/rust-analyzer/rust-analyzer && cd rust-analyzer

# install both the language server and VS Code extension
$ cargo install-code

# alternatively, install only the server. Binary name is `ra_lsp_server`.
$ cargo install-lsp
```
## Documentation

If you want to **contribute** to rust-analyzer or just curious about how things work
under the hood, check the [./docs/dev](./docs/dev) folder.

If you want to **use** rust-analyzer's language server with your editor of
choice, check [./docs/user](./docs/user) folder. It also contains some tips & tricks to help
you be more productive when using rust-analyzer.

## Getting in touch

We are on the rust-lang Zulip!

https://rust-lang.zulipchat.com/#narrow/stream/185405-t-compiler.2Frls-2.2E0

## License

Rust analyzer is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
