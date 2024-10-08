workspace(name = "rust_macos_errno")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "f9b59be3bc20d157212000da1ede8be4399ad869fe439f3d9f3fcc29c4e2fa5a",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.45.1/rules_rust-v0.45.1.tar.gz"],
)

http_archive(
    name = "rust_posix_errno",
    sha256 = "0c86c849ff673372fe6415d4004a233565b57b2884ea49d3b725dd1296cc2529",
    strip_prefix = "posix-errno-1.0.1",
    urls = ["https://github.com/jmillikin/rust-posix-errno/releases/download/v1.0.1/posix-errno-1.0.1.tar.xz"],
)

load(
    "@rules_rust//rust:repositories.bzl",
    "rules_rust_dependencies",
    "rust_register_toolchains",
)

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2018",
    versions = ["1.63.0"],
)
