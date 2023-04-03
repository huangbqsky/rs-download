# rs-download 下载文件工具

根据 HTTP range requests 实现并发下载文件。

# Usage

```
❯ cargo build --quiet && target/debug/rs-download --help
rs-download 0.1.0
USAGE:
    rs-download <size> <uri> <file-path>

ARGS:
    <size>         并发任务数量
    <uri>          资源 URI
    <file-path>    保存文件路径

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```




# Example

```js
❯ cargo build --quiet && target/debug/rs-download 5 https://cdn.thenewstack.io/media/2021/05/bd292b24-rust-logo.jpg /Users/XXXX/Rust/rust_down.jpg
[##################################################] [任务 1 下载完成] [18.25 KiB/18.25 KiB] (0s)
[##################################################] [任务 2 下载完成] [18.25 KiB/18.25 KiB] (0s)
[##################################################] [任务 3 下载完成] [18.25 KiB/18.25 KiB] (0s)
[##################################################] [任务 4 下载完成] [18.25 KiB/18.25 KiB] (0s)
[##################################################] [任务 5 下载完成] [18.25 KiB/18.25 KiB] (0s)                                                   [##################################################] [合并文件完成] (0s)
耗时：2.590953625s
```
