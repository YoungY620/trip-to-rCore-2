# chap 2 批处理系统

## 内容

- 特权机制
  - risc-v 五级特权：U/S/H/M，H 标准仍在制定
  - 特权是 **针对指令** 的：某些 （汇编）指令 是仅在某种特权级下才允许执行
  - 是利用 硬件 (CPU)实现的
  - 特权级之间的切换接口：XBI（ABI、SBI）、API
    - 用来跳转的 **机器指令** : `ecal` `eret`
    - 运行在 M 模式上的软件被称为 监督模式执行环境 (SEE, Supervisor Execution Environment)
    - 监督模式二进制接口 (Supervisor Binary Interface, SBI): M 和 S 之间
    - 应用程序二进制接口 (Application Binary Interface, ABI): M 和 U 之间. 又叫 **系统调用**
  - interrupts, exceptions 和 trap
    - [参考](https://five-embeddev.com/riscv-isa-manual/latest/intro.html#sec:trap-defn)
    - interrupts: 外部的异步事件 (external asynchronous event)
    - exception: 对于当前 硬件线程 (hart) 正在执行的指令 (instruction) 发生的意外情况
    - trap: 由 interrupts 或 exceptions 引发的控制权向 trap handler 的转移. 描述从软件 **上层到底层** 的过程
    - hart: hardware threads 的缩写. [reference](https://stackoverflow.com/questions/42676827/risc-v-spec-references-the-word-hart-what-does-hart-mean)
  - 相关寄存器
    - `x0 ~ x31` 与特权级无关, 任何特权级都可以使用
    - 状态与控制寄存器(CSR)
      - [包括](https://five-embeddev.com/quickref/csrs.html). 不同状态下允许访问不同的 CSR
      - 用于保存 S 级中断信息的寄存器, 属于CSR.
        ![S 级中断信息的寄存器](images/20220622154424.png)
        其中 `SPP` 是 `sstatus` 的一个字段, 表示中断时的特权级, 参考 [risc-v manual: status](https://five-embeddev.com/riscv-isa-manual/latest/supervisor.html#sstatus)

      > RISC-V 中包含 **M/H/S/U 四级中断信息寄存器**
      > 例如 `mstatus` `sstatus` `ustatus` `vstatus`
      > 其中 U 级(User-Level) 中断 在一般系统中不常见, 常用于只存在 M/U 状态的系统中 (完全禁用 M 级指令主要为了安全考虑?) 中断处理过程在 U 状态实现
      >
      > 在理解 U 级中断时, 要理解 指令集 (如 RISC-V) 与操作系统 (如 UNIX) 之间的上下层关系是分开的

      > 概念解释 **PSW**:
      > 参考: [牛津百科](https://www.encyclopedia.com/computing/dictionaries-thesauruses-pictures-and-press-releases/program-status-word) [维基百科](https://en.wikipedia.org/wiki/Program_status_word)
      > 当前程序基本信息的集合. 通常存储在 Program State Register 中, 有时也可以存储在内存中
      > 通常包括:
      > - Program Counter (PC),
      > - ALU 计算信息 (如 进位, 溢出等),
      > - 特权级标志信息
      > - ......
      >
      > (这样说的话, PSW 应该包含 CSR?)
