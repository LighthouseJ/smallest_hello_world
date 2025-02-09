# smallest_hello_world

## Build and test ##

uild the release version to ensure it's optimized:

```sh
./cargo-release-strip.sh
```

Then, run the test:

```sh
cargo test --release -- --nocapture
```

--release ensures that the test runs against the stripped binary.
--nocapture allows the test to show stdout output if it fails.