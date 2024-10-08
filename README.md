# macOS error numbers for Rust

This library defines an `Error` struct that represents error numbers
returned from macOS system calls.

To depend on `macos-errno` from a Bazel workspace:

```python
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rust_posix_errno",
    sha256 = "0c86c849ff673372fe6415d4004a233565b57b2884ea49d3b725dd1296cc2529",
    strip_prefix = "posix-errno-1.0.1",
    urls = ["https://github.com/jmillikin/rust-posix-errno/releases/download/v1.0.1/posix-errno-1.0.1.tar.xz"],
)

http_archive(
    name = "rust_macos_errno",
    # Obtain the package checksum from the release page:
    # https://github.com/jmillikin/rust-macos-errno/releases/tag/v1.0.0
    sha256 = "",
    strip_prefix = "macos-errno-1.0.0",
    urls = ["https://github.com/jmillikin/rust-macos-errno/releases/download/v1.0.0/macos-errno-1.0.0.tar.xz"],
)
```

To depend on `macos-errno` from a Cargo workspace:

```
[dependencies]
macos-errno = { version = "1.0.0" }
```
