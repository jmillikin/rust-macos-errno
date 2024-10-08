load("@rules_rust//rust:defs.bzl", "rust_doc", "rust_library")

rust_library(
    name = "macos-errno",
    srcs = ["macos-errno.rs"],
    crate_features = ["posix-traits"],
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = ["@rust_posix_errno//posix-errno"],
)

rust_doc(
    name = "macos-errno_doc",
    crate = ":macos-errno",
)
