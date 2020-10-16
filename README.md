# AiliceOS

### Simplified Chinese(简体中文)

## 注意⚠️
> - 项目仍在进行，该项目正在进行大规模重构,包括 多CPU架构支持,UEFI支持、完整文件系统、内存等

----------

#### 进度报告
> - 基础的内存和堆分配已经完成，加入了中断功能，但是并未实现更详细的中断
> - 加了QEMU图形化，同时PC实体机也能够显示
> - 优化代码，精简无用功能和代码
> - 下一步：完整的中断支持和X86_64位CPU特性支持(多核支持以及FPU运算支持)

----------

#### 未来：
> - 进程管理
> - 基础驱动
> - IPC通信
> - 模块化
> - ....

希望可以当做一个正常的Unix使用

----------

#### 关于系统
> 这是AiliceOS，基于Rust开发的操作系统
> 系统名称：`AiliceOS`

----------

#### 多平台
> 架构：`[amd64]x64`(mips,aarch64下一个版本将会支持)。

----------

#### 编译
> 平台：`Windows10 MacOS Linux* Unix*`
> 需要的工具：`Rust` `QEMU`

----------

#### 配置编译环境
> - 安装Rust
```
$  curl https://sh.rustup.rs -sSf | sh
```

> - 安装每夜版Rust和额外工具
```
$  make install-depends
```

> - 安装QEMU
```
# 如果为MacOS，则使用brew安装QMEU
$  /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
$  brew install qemu

# 如果为Ubuntu，则使用apt安装QMEU
$  sudo apt update && sudo apt install qemu-system # or sudo apt install qemu

# 如果为Windows，推荐使用Windows10的WSL(Linux子系统)
```

> - 运行：

```
$  make all
```

致谢：rCore、osdev