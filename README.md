# Hello

在 c 代码中调用 rust 代码示例软件包

## 如何使用

1. 在 **bsp/qemu-a9** 工程目录下打开 env
2. 输入 `menuconfig` 命令，在 `RT-Thread Online Package/miscellaneous packages` 选择 **rust_hello** 
3. 输入 `pkgs --update`，拉取 rust 软件包
4. 输入 `scons` 编译工程
5. 输入 `qemu.bat` 启动 qemu
6. 输入 `rust_hello_example` 调用 rust 函数
    ```shell
    $ qemu.bat
    WARNING: Image format was not specified for 'sd.bin' and probing guessed raw.
    Automatically detecting the format is dangerous for raw images, write operations on block 0 will be restricted.
    Specify the 'raw' format explicitly to remove the restrictions.
    
    \ | /
    - RT -     Thread Operating System
    / | \     4.0.4 build Aug 24 2021
    2006 - 2021 Copyright by rt-thread team
    lwIP-2.1.2 initialized!
    [I/sal.skt] Socket Abstraction Layer initialize success.
    [I/SDIO] SD card capacity 65536 KB.
    [I/SDIO] switching card to high speed failed!
    hello, rt-thread
    msh />rus
    rust_hello_example
    msh />rust_hello_example
    this is from rust
    hello, world
    hello, rt-thread
    msh />
    ```

## 目录结构

- example: rust 生成的静态库文件
- src: rust 库文件
- Cargo.toml: rust 项目信息
- rust_hello.c: c 代码调用 rust 示例

## 生成静态库文件

1. 安装 target: `rustup target add armv7a-none-eabi`
2. 生成静态库: `cargo build --target=armv7a-none-eabi --release --verbose`

