toward least-authority lint for rust: test lint works

 1. to build `liblint_plugin_test.so`:
    - `rustc lint_plugin_test.rs`
 2. to use the lint:
    -  `rustc -L . code.rs`
