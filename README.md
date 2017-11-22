# `rust-macro-comma-test`

Comprehensive test suite for trailing comma support in rust macros.

The `//~` tags in the source denote the compiler output on the latest version that this test was run (beta channel, the day before the 1.22 release). They **do not** actually do anything, and they most certainly do not represent the desired compiler output.  (If you want to try seeing what happens if you stick this file in `rust/test/compile-fail`, though, be my guest.)

If you want to run the test:

```sh
git clone https://github.com/ExpHP/rust-macro-comma-test
cd rust-macro-comma-test
cargo build
cargo build --features=nightly
```

(you'll have to eyeball the output against the source to make sense of the results)

This only tests the treatment of an extra `,` at the end of a valid list of arguments.  It does not check that e.g. `mac!(,)` or `mac!(arg,,)` are forbidden.  (Those are worth testing since they're backcompat hazards; but I left them out for now to make the file easier to review, so that it is not a random mixture of "desirable" errors and "undesirable" errors.)

