# THIS PROJECT IS DEPRECATED

rust-identifiers is not maintained anymore.

# Identifiers [![Build Status](https://travis-ci.org/altmetric/rust-identifiers.svg?branch=master)](https://travis-ci.org/altmetric/rust-identifiers)

A work-in-progress Rust library for handling and extracting scholarly identifiers from text.

**Current version:** Unreleased  
**Supported Rust versions:** 1.14

# Usage

```rust
use identifiers::Doi;
let dois = Doi::extract("I love 10.1234/foobar and 10.1234/bazquux");
```

# Ruby & PHP versions

We also maintain a [version of this library for Ruby](https://github.com/altmetric/identifiers) and [for PHP](https://github.com/altmetric/php-identifiers).

## License

Copyright © 2017 Altmetric LLP.

Distributed under the MIT License.
