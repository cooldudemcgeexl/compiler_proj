# crust

Compiler for a theoretical language written in Rust.

- [crust](#crust)
  - [Overview](#overview)
  - [Components Used](#components-used)
  - [How to Install](#how-to-install)
    - [From a Package](#from-a-package)
    - [From Source](#from-source)
  - [Usage](#usage)
  - [Completed Functionality](#completed-functionality)
  - [Incomplete Functionality](#incomplete-functionality)
  - [Unique Features](#unique-features)
    - [Comment Stripping](#comment-stripping)
    - [Scanner DFA Implementation](#scanner-dfa-implementation)
      - [Pattern Matching With State](#pattern-matching-with-state)
      - [Error Reporting](#error-reporting)
    - [Analysis Scope Management](#analysis-scope-management)
  - [Features to be Completed](#features-to-be-completed)
    - [LLVM Code Generation](#llvm-code-generation)
    - [Runtime](#runtime)

## Overview

This project serves as the compiler created for [CS6083](https://eecs.ceas.uc.edu/~wilseypa/classes/eece6083/).

## Components Used

- [Rust](https://www.rust-lang.org/) (nightly)
  - [thiserror](https://docs.rs/thiserror/latest/thiserror/) - Used for simplifying error derivation
  - [rstest](https://docs.rs/rstest/latest/rstest/) - Used for making parameterized unit tests
  - [cargo deb](https://crates.io/crates/cargo-deb) - Used for quickly packaging into `.deb` format

## How to Install

### From a Package

Distributions are available at this project's [releases](https://github.com/cooldudemcgeexl/crust/releases/) page. Download the package and run

```bash
sudo dpkg -i crust_x.x.x_amd64.deb
```

crust will automatically be added to your path.

### From Source

This requires the nightly toolchain, which is available through [rustup](https://rustup.rs/). To install the nightly version of rust, use:

```bash
rustup default nightly
```

Next, clone the repository:

```bash
git clone https://github.com/cooldudemcgeexl/crust.git
```

cd into the repository, and run the following to build in debug target:

```bash
cargo build
```

or for release target:

```bash
cargo build -r
```

By default, binaries will output to `target/<targeted build>/crust`

## Usage

```bash
crust <path_to_input_file>
```

The path can either be an absolute path or a relative path. Relative paths assume the CWD from where `crust` is called.

## Completed Functionality

- Scanning
  - Comments are stripped out as a pre-processing step.
- Parsing
- Semantic Analysis (not bug-free)
  - Currently, there is a bug with arrays of integers being incorrectly analyzed as just integers. As of submission, I have not identified the cause.

## Incomplete Functionality

- Code Gen
- Runtime
- Detailed error messages
  - Error messages are output to `stderr`, but do not currently use the standard error format yet. As a result, line numbers are not provided when the compiler errors out.

## Unique Features

These features are unique features that are provided through either implementation, or features of the Rust language itself.

### Comment Stripping

Comments are stripped as a pre-processing step, before scanning is done. The pre-processor tracks its state in order to enable properly stripping nested block comments.

It should be noted that this state is stored as a `u8` value, so there is a maximum comment nesting depth of 255. The compiler will error out if this limit is exceeded.

Theoretically, the limit could be stored in a `u128` if one desired, giving a maximum comment depth of 340,282,366,920,938,463,463,374,607,431,768,211,456. Why they would do this, I cannot answer.

### Scanner DFA Implementation

Thanks to the features of the Rust language, the DFA implementation done for the scanner is relatively concise.

#### Pattern Matching With State

Due to Rust natively supporting structured pattern matching with tuples, the DFA is implemented using a repeating match statement on a tuple consisting of `(current_char, scanner_state)`. Additionally, due to Rust allowing enum variants to hold data, state can be complex. For example, there is a state:

```rust
enum ScannerState{
    None,
    Identifier(String)
}
```

This allows for building out a string in the state, allowing for transition from `None` `Identifier("f")` -> `Identifer("fo")` -> `Identifier("foo")` -> `None` when encountering the token `foo`.

#### Error Reporting

Utilizing Rust's [Result](https://doc.rust-lang.org/rust-by-example/error/result.html) type, error handling is simplified. For each level of the program, there is a unique enum variant that captures the errors that could arise. The higher level error types then derive these variants, as well as define their own. This allows the entire program to run as a `Result<(), CompilerError>` function.

The crate [thiserror](https://docs.rs/thiserror/latest/thiserror/) is used to shorthand the implementation of the [From](https://doc.rust-lang.org/std/convert/trait.From.html) trait on the nested error. The same functionality could be accomplished, albeit more verbosely, by implementing `From` on each of the error enums manually. (This is actually what `thiserror` does under the hood at compile time!)

### Analysis Scope Management

Scope is managed through a context object, which is continuously passed through to subsequent layers of analysis.

## Features to be Completed

These are features not yet complete due to the deadline being reached.

### LLVM Code Generation

Code generation to LLVM has not been started yet.

### Runtime

Runtime has not been stated yet.
