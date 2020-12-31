# mklink
This command line tool gives you the ability to create links or junctions in your filesystem, without the need to open a Command Prompt shell to invoke it. Windows only.

![](https://img.shields.io/github/license/Starz0r/mklink.svg?style=flat-square) ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/starz0r/mklink.svg?style=flat-square)

---

## Usage

```
USAGE:  
mklink.exe \[FLAGS\] --link <dst> --target <src>  
  
FLAGS:  
-h, --hard  
--help Prints help information  
-j, --junction  
-d, --directory  
-V, --version Prints version information  
  
OPTIONS:  
-t, --link <dst>  
-o, --target <src>
```

## Example

Link -> Target.

##### File
```
$> bat > a.txt
$> mklink --hard --link .\b.txt --target .\a.txt
```

##### Directory
```
$> mkdir "foo"
$> mklink --junction --link .\bar\ --target .\foo\
```

---

## Building

##### Requirements
```
Microsoft Windows Vista or higher
Rust 1.45.0 or higher
Microsoft Visual Compiler or GNU Compatible Compiler
Xargo or Cargo
```

Once you have all the requirements, run `xargo +nightly build --release` and the completed binary will be in `target/release/`.
