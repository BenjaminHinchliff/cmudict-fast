# CMUSphinx pronunciation dictionary

This is a fork of the [original][1] rust library for getting pronunciations from the [CMUSphinx][2]
pronunciation dictionary.

> :warning: 0.6.0 onward is not api compatible with the original crate.
> 0.5.0 or before is mostly api compatible but notably doesn't have the
> download utility function.

The major changes are:
- switched from the depreciated `failure` crate to `thiserror` based errors
- switched to rust 2018 in the cargo config
- removed the utility function to download
- removed a lot of dependencies that were either no longer needed or depreciated
- fixed hung lookup bug (due to the new internals)
- changed the internals so file i/o wasn't needed for every lookup (and as a result simplified them)
- merged `cmudict_core` and `cmudict` into one crate
- as of 0.6.0 the get function return a slice of all potential pronunciations

The last of those changes is where the name comes from - as a result of the entire
dictionary being loaded into volitile memory, each individual lookup is *much* faster
than the original crate (should be O(1) on average with worst case O(n)). Of course,
there's the downside that it takes longer to createthe object and uses more memory, but
for the application I initially created this for the lookup time was essential.

## Installation

To use in your `rust` project, add the following to your `Cargo.toml`:

```toml,ignore
[dependencies]
cmudict-fast = "0.5"
```

## Usage

To use the dictionary, you have to get an instance of the `Cmudict`
struct:

```rust,ignore
use cmudict_fast::Cmudict;

fn main() {
  let dict = Cmudict::new("./path/to/a/cmudict/file").expect("Couldn't make Cmudict");
}
```

You can retrieve the pronunciation for a word like this:

```rust
use cmudict_fast::Cmudict;

fn main() {
  let dict = Cmudict::new("path/to/cmudict").expect("Couldn't get/make Cmudict");
  let word = dict.get("apple").unwrap().pronunciation();
  println!("{:?}", word); // &[Symbol::AE(Stress::Primary), Symbol::P, Symbol::AH(Stress::None), Symbol::L]
}
```

[1]: https://gitlab.com/pwoolcoc/cmudict
[2]: https://github.com/cmusphinx/cmudict
