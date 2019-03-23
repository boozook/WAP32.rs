# Preparation for tests:

## Desktop

1. Create here (in the current dur where this file is) file with name `GRUNTZ_REZ_DIR_PATH` (without .ext);
2. Give content to the `GRUNTZ_REZ_DIR_PATH`:

```text
/* used as including as raw-src part in some integration tests */
"/FULL/PATH/TO/DIRECTORY/WITH/Gruntz/ORIGINAL/AND_PATCHED/RESOURCES/E.G./GRUNTZ.REZ/GRUNTZ.ZZZ/GRUNTZ.VRZ/"
```

Run tests:

* all tests - `cargo test`;
* specified by part of test's name - `cargo test read_all` (here `read_all` is a part of many tests).


- - -


## iOS device / sim

### Install deps & Preparation

* Install `cargo install cargo-lipo`.
* Install [dinghy](https://crates.io/crates/dinghy) - `cargo install dinghy`.

More info about Dinghy: [iOS](https://github.com/snipsco/dinghy/blob/master/docs/ios.md), [files](https://github.com/snipsco/dinghy/blob/master/docs/files.md)


Put file `.dinghy.toml` into the root of project. Its content should be like this:

```toml
[test_data]
GRUNTZ_REZ = "/FULL/PATH/TO/GRUNTZ.REZ"
GRUNTZ_ZZZ = "/FULL/PATH/TO/GRUNTZ.ZZZ"
GRUNTZ_VRZ = "/FULL/PATH/TO/GRUNTZ.VRZ"
```

Run tests:

* devices list - `cargo dinghy devices`;
* all tests - `cargo dinghy --device iphone test` or `cargo dinghy test` - run on the first device in the list;
* not all tests - `cargo dinghy test read_all`.
