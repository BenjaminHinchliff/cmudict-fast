# CMUSphinx pronunciation dictionary

This is a fork of the rust library for getting pronunciations from the [CMUSphinx][1]
pronunciation dictionary.

The major changes are:
- switched from the depreciated failure crate to thiserror based errors
- switched to rust 2018 in the cargo config
- removed the utility function to download
- removed a lot of dependencies that were either no longer needed
- fixed hung lookup bug (due to the new internals (I still don't know what caused it))
- changed the internals so file i/o wasn't needed for every lookup (and as a result simplified them)

The last of those changes is where the name comes from - as a result of the entire
dictionary being loaded into volitile memory, each individual lookup is *much* faster
than the original crate (O(k) where k is the maximum length of all words in
the dictionary). Of course, there's the downside that it takes longer to create
the object and uses more memory, but for the application I initially created this for
the lookup time was essential. To adress the obvious, yes, this is slower than a hashmap
since it uses a [Radix tree][2] internally (like
the original crate). Still, it's much faster than the range-based file lookup or whatever
you'd call it old crate used, while also being lass complicated.

## Installation

To use in your `rust` project, add the following to your `Cargo.toml`:

```toml,ignore
[dependencies]
cmudict = "0.3"
```

then in your crate root:

```rust,ignore
extern crate cmudict;
```

## Usage

To use the dictionary, you have to get an instance of the `Cmudict`
struct:

```rust,ignore
extern crate cmudict;

use cmudict::Cmudict;

fn main() {
  let dict = Cmudict::new("./path/to/a/cmudict/file").expect("Couldn't make Cmudict");
}
```

If you don't want to pass your own `cmudict` file, you can call
`Cmudict::download` instead, and the library will download a copy of the
dictionary from https://raw.githubusercontent.com/cmusphinx/cmudict/master/cmudict.dict :

```rust,ignore
extern crate cmudict;

use cmudict::Cmudict;

fn main() {
  let dict = Cmudict::download().expect("Couldn't get/make Cmudict");
}
```

You can retrieve the pronunciation for a word like this:

```rust,ignore
extern crate cmudict;

use cmudict::Cmudict;

fn main() {
  let dict = Cmudict::download().expect("Couldn't get/make Cmudict");
  let word = dict.get("apple").unwrap().pronunciation();
  println!("{:?}", word); // &[Symbol::AE(Stress::Primary), Symbol::P, Symbol::AH(Stress::None), Symbol::L]
}
```

[1]: https://github.com/cmusphinx/cmudict
[2]: https://en.wikipedia.org/wiki/Radix_tree
