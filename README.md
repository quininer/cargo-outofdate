# cargo-outofdate

Check outdated cargo dependencies, Inspired by [cargo-outdated](https://github.com/kbknapp/cargo-outdated).

## install

```
$ cargo install cargo-outofdate
```

## usage

```
$ cargo-outofdate
Name                   Now     Compat  Latest
chrono:                0.2.25  --      0.4.0
serde_json:            0.9.10  --      1.0.2
semver:                0.6.0   --      0.7.0
unicode-xid:           0.0.4   --      0.1.0
serde:                 0.9.15  --      1.0.8
unicode-segmentation:  1.1.0   --      1.2.0
unreachable:           0.1.1   --      1.0.0
serde_derive:          0.9.15  --      1.0.8
docopt:                0.7.0   --      0.8.1
serde_ignored:         0.0.2   --      0.0.4
toml:                  0.3.2   --      0.4.1
```

## license

`cargo-outofdate` is released under the terms of the MIT. See the LICENSE file for the details.
