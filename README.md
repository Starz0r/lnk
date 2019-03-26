# mklink
A way to create symbolic, junctions, and hard links without using the CMD Shell.

![](https://img.shields.io/github/license/Starz0r/mklink.svg?style=flat-square) ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/starz0r/mklink.svg?style=flat-square)

---

## Commands

##### Soft Link (files only)
```
--soft <Destination> <Source>
```

##### Hard Link (files only)
```
--hard <Destination> <Source>
```

##### Junction (soft linking for directories)
```
--junction <Destination> <Source>
```

---

## Building

##### Requirements
```
Microsoft Windows Vista or higher
Rust 1.26.0 or higher
Microsoft Visual Compiler or GNU Compatible Compiler
Xargo or Cargo
```

Once you have all the requirements, run `xargo +nightly build --release` and the completed binary will be in `target/release/`.
