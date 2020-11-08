# rust-bazel-sample

## Execute a binary and a library with only lib.rs

Look at `hello_world` and `hello_lib`.

```sh
bazel run //hello_world
```

## Build a library with lib.rs and other files

Look at `hello_lib2`.

```sh
bazel build //hello_lib2
```

## Unit-test a library with lib.rs

Look at `hello_lib3`.

```sh
bazel test //hello_lib3:hello_lib3_test
```

## Integration-test a library with lib.rs and tests

Look at `hello_lib3`.

```sh
bazel test //hello_lib3:greet_test
```

