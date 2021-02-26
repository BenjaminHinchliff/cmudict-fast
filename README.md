# CMUSphinx pronunciation dictionary

This is a fork of the [original][1] rust library for getting pronunciations from the [CMUSphinx][2]
pronunciation dictionary.

The major changes are:
- switched from the depreciated failure crate to thiserror based errors
- switched to rust 2018 in the cargo config
- removed the utility function to download
- removed a lot of dependencies that were either no longer needed or depreciated
- fixed hung lookup bug (due to the new internals (I still don't know what caused it))
- changed the internals so file i/o wasn't needed for every lookup (and as a result simplified them)

The last of those changes is where the name comes from - as a result of the entire
dictionary being loaded into volitile memory, each individual lookup is *much* faster
than the original crate (O(k) where k is the maximum length of all words in
the dictionary). Of course, there's the downside that it takes longer to create
the object and uses more memory, but for the application I initially created this for
the lookup time was essential. To adress the obvious, yes, this is slower than a hashmap
since it uses a [Radix tree][3] internally (like
the original crate). Still, it's much faster than the range-based file lookup or whatever
you'd call it old crate used, while also being less complicated.

## Installation

To use in your `rust` project, add the following to your `Cargo.toml`:

```toml,ignore
[dependencies]
cmudict-fast = "0.4"
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
[3]: https://en.wikipedia.org/wiki/Radix_tree
