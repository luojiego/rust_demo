# Cargo 命令

官方推荐使用的命令，虽然在特别小的 demo 或者没有外部依赖的情况下，与 rustc 相比没有多少优势，但是依然推荐，毕竟代码总是不停的在增加。

## 使用方法

### 创建工程

```sh
cargo new hello_cargo 
```

将在执行目录创建 hello_cargo 目录，进入目录会有 Cargo.toml 以及 src 目录。关于 Cargo.toml 中记录的文件内容，后面再研究。

### 构建

```shell
cd hello_cargo
cargo build
```

### 运行

```shell
cargo run
```

### 检测

```shell
cargo check
```

**该命令快速检查代码确保其可以编译**，这个命令可算是非常有创举的一件事情，因为相比 cargo build 其它的编译语言 C/C++  或者 go 等，并不会真的创建可执行程序，所以速度会非常快【官方是这样说，有待确认】。

### 其它

`cargo build --release` 来优化编译项目，可以使用选项来生成开发环境和生产环境的可执行文件。