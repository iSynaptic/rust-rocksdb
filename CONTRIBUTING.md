# Contributing to rust-rocksdb
Thank you for taking an interest in the project, and contributing to it - it's appreciated! 

## Design Guidelines

When submitting pull requests or recommending API changes as part of issues, please keep the following in mind:

- The `librocksdb-sys` crate is intended to be a mechanical 1-to-1 mapping with the official RocksDB C API.  Even this API mirrors the design of the C++ API, following the naming convention `rocksdb_{class}_{method}`.

- The main crate, `rocksdb`, is intended to be a 1-to-1 mapping with the official RocksDB C++ object model. Classes that exist in the C++ API should generally have an equivalently named `struct` in the Rust library; the same should be said for methods on those classes. Generally, identically named methods should be found on the `struct`s, either through direct `impl`s or through `trait`s implemented on the `struct`s.  Calling a Rust method named identically to the underlying C++ API should invoke that C++ method.  Additional `trait`s and methods can be implemented to provide better ergonomics and support Rust idioms, but as a general rule the "shape" of a `struct` exposing the functionality of a C++ class should be the same as the C++ class.

## How to Contribute

There are several ways you can contribute:
- [Bug Reports](#bug-reports)
- [Feature Requests](#feature-requests)
- [Documentation](#documentation)
- [Discussion](#discussion)
- [Pull Requests](#pull-requests)

**Please note all contributors must adhere to the [code of conduct](code-of-conduct.md).**

## Bug Reports
[bug-reports]: #bug-reports
- **Ensure the bug has not already been reported** - this can be done with a quick search of the [existing open issues](https://github.com/rust-rocksdb/rust-rocksdb/issues?q=is%3Aissue+is%3Aopen+).
- **Ensure the bug applies to the Rust wrapper, and not the underlying library** - bugs in the RocksDB library should be [reported upstream](https://github.com/facebook/rocksdb/issues).
- When [creating an issue](https://github.com/rust-rocksdb/rust-rocksdb/issues/new) please try to:
    - **Use a clear and descriptive title** to identify the issue
    - **Provide enough context** to acurately summarize the issue. Not every issue will need detailed steps to recreate, example code, stack traces, etc. - use your own judgment on what information would be helpful to anyone working on the issue. It's easier for someone to skim over too much context, than stop and wait for a response when context is missing.

## Feature Requests
[feature-requests]: #feature-requests
Feature requests will primarily come in the form of ergonomics involving the Rust language, or in bringing the wrapper into parity with the library's API. Please create an issue with any relevant information.

## Documentation
[documentation]: #documentation
Much of the documentation should mirror or reference the library's [documentation](https://github.com/facebook/rocksdb/wiki). If the wrapper or its exposed functions are missing documentation or contain inaccurate information please submit a pull request.

## Discussion
[discussion]: #discussion
Discussion around design and development of the wrapper primarily occurs within issues and pull requests. Don't be afraid to participate if you have questions, concerns, insight, or advice.

## Pull Requests
[pull-requests]: #pull-requests
Pull requests are welcome, and when contributing code, the author agrees to do so under the project's [licensing](https://github.com/rust-rocksdb/rust-rocksdb/blob/master/LICENSE) - Apache 2.0 as of the time of this writing. The maintainers greatly appreciate PRs that follow open-source contribution best practices:
1. Fork this repository to your personal GitHub account.
1. Create a branch that includes your changes, **keep changes isolated and granular**.
1. Include any relevant documentation and/or tests. Write [documentation tests](https://doc.rust-lang.org/rustdoc/documentation-tests.html) when relevant.
1. Apply `cargo fmt` to ensure consistent formatting.
1. [Create a pull request](https://help.github.com/en/articles/about-pull-requests) against this repository.

For pull requests that would benefit from discussion and review earlier in the development process, use a [Draft Pull Request](https://help.github.com/en/articles/about-pull-requests#draft-pull-requests).

## Additional Resources
Some useful information for working with RocksDB in Rust:
- [RocksDB library primary site](https://rocksdb.org)
- [RocksDB library GitHub repository](https://github.com/facebook/rocksdb)
- [RocksDB library documentation](https://github.com/facebook/rocksdb/wiki)
- [Rust's Foreign Function Interface (ffi)](https://doc.rust-lang.org/nomicon/ffi.html)

