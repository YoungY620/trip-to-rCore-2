# chap 1 单进程系统

## 主要内容

正常 rust 程序建立在操作系统和 rust 标准库之上（标准库是基于操作系统的系统调用的）

因此包含以下工作

- 交叉编译: 将 编译目标 改为 RISC-V 裸机
  - 平台目标三元组: 主要规定了 CPU 架构, CPU 厂商, 操作系统, 运行时库
- 编译时, 会在 std 检查 当前编译目标. 修改 编译目标 (target) 后, 可能找不到
  - `#![no_std]` 无需检查 std
- 编译时, 需要首先检查 `_start` 入口标签，一般由 `_start` 之后代码调用 main 函数。
  - 手动实现这一过程：需要规定内存空间，其中栈空间保证函数调用（简陋方法）。
  - `#[no_mangle]` 外部宏确保编译时 label name 不变

## 问题

- **链接脚本** 
  - `. = ALIGN(4K)` 当前地址 对齐到 4kb(?): [解释](https://stackoverflow.com/questions/8458084/align-in-linker-scripts)
  - 每段的前后 分别由变量 表明起始与结束. 如 `.text` 段 起始 `stext` 与结束 `etext`

  ```
    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
  ```

- **error[E0463]: can't find crate for core**
  - 当 更改 编译目标 (target) 之后, 显示的是 找不到 `std`:

    ```
    error[E0463]: can't find crate for `std`
    ```

  - 当 添加 `#![no_std]` 之后, 显示的是 找不到 `core`
  - [reference](https://os.phil-opp.com/cross-compile-libcore/#:~:text=If%20you%20get%20an%20error%3A%20can%27t%20find%20crate,problem.%20For%20more%20details%2C%20see%20the%20rust-cross%20project.)
  - `core` 是 特定于 某个架构的, 所以 在交叉编译时 找不到 `core` 是指 找不到符合编译目标架构的 `core` 使用 `rustup target add <target-triple>` 即可 
  - rustup 是 rust 安装器, 可用于 rust 语言版本, 编译目标等的切换. [rustup cross-compilation](https://rust-lang.github.io/rustup/cross-compilation.html#cross-compilation) 

- **函数调用的系统实现**
  - *被调用者保存(Callee-Saved) 寄存器* 和 *调用者保存(Caller-Saved) 寄存器*
  - C语言规范中, 函数参数保存在 通用寄存器 `a0~a7` 中. 当参数个数超过 8 个时, 压入栈中. 因此不推荐超过 8 个参数. [参考](https://www.cnblogs.com/northeast-coder/p/15851692.html)

- **系统调用 rustsbi 实现原理**
  - 对于 操作系统内核 之下的服务层 [参见评论](https://rcore-os.github.io/rCore-Tutorial-Book-v3/appendix-c/index.html)

