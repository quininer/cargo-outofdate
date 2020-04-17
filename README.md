# cargo-outofdate

Similar to `cargo-outdated`, but simpler and supports offline.

## install

```
$ cargo install cargo-outofdate
```

## usage

```
$ cargo outofdate
    Updating crates.io index
Name                   Now     Compat  Latest
ansi_term:             0.11.0  --      0.12.1
foreign-types:         0.3.2   --      0.5.0
foreign-types-shared:  0.1.1   --      0.3.0
git2:                  0.11.0  --      0.13.2
git2-curl:             0.12.0  --      0.14.0
hex:                   0.3.2   --      0.4.2
humantime:             1.3.0   --      2.0.0
libgit2-sys:           0.10.0  --      0.12.3+1.0.0
semver-parser:         0.7.0   --      0.9.0
sized-chunks:          0.5.3   --      0.6.1
strsim:                0.8.0   --      0.10.0
utf8parse:             0.1.1   --      0.2.0
vte:                   0.3.3   --      0.7.1
```

## license

`cargo-outofdate` is released under the terms of the MIT. See the LICENSE file for the details.
