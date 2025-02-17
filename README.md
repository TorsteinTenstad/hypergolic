# Hypergolic 🚀
A collection of tools that improves developer protuctivity.

All tools are written in rust. Use cargo to build all tools with

`cargo build`

## nin
Wrapper for ninja that jumps VSCode to the source location of the first compilation error
```text
Usage: nin.exe [OPTIONS] [TARGET]

Arguments:
  [TARGET]  Target to build. If not specified, use the last target as determined by the cache

Options:
      --cache_dir <DIR>     Features that require persistent state between executions creates files in <PATH>/.nin/. If not specified, use the ninja build directory
  -b, --build-dir <DIR>     Directory where ninja build files are located [default: build]
  -c, --clean               Clean out the build directory before building
  -w, --warnings-as-errors  Treat warnings as errors
  -h, --help                Print help
```

## cb
Wrapper for cbuild that jumps VSCode to the source location of the first compilation error
```text
Usage: cb.exe [OPTIONS] [CSOLUTION_YML]

Arguments:
  [CSOLUTION_YML]  Path to the project file to pass to cbuild. If not specified, use the last project file as determined by the cache

Options:
      --cache_dir <PATH>    Features that require persistent state between executions creates files in <PATH>/.cb/ [default: .devlocal]
  -c, --clean               Clean out the build directory before building
  -w, --warnings-as-errors  Treat warnings as errors
  -s, --skip <N>            Skip the first N detected issues [default: 0]
  -h, --help                Print help
```
