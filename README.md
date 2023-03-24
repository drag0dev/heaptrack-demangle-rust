# heaptrack-demangle-rust
quick and dirty script to demangle rust symbols in a heaptrack capture

## build
```
cargo install --path ./
```
binary installed under name '**rdemangle-heaptrack**'

## usage

```
rdemangle-heaptrack FILE [--output|-o] FILE [--level|-l] COMPRESSION-LEVEL
```
output filename defaults to 'demangled_*inputfilename*'  
compression level defaults to 3  
any symbol that can't be demangled will be skipped