# larust: toward least-authority lint for rust

src/main.rs uses the `tag_safe` lint with an `ocap` tag.

`cargo run` should work as usual.

## Earlier work

 1. to build `liblint_plugin_test.so`:
    - `rustc lint_plugin_test.rs`
 2. to use the lint:
    -  `rustc -L . code.rs`
