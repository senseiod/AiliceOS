# AiliceOS

### Simplified Chinese(简体中文)

## 注意⚠️
> - 项目仍在进行，由于事务繁忙，正在缓慢更新

----------

#### 进度报告
> - 下一步实现内存完整分配
> - FPU运算实现，下一步实现CPU多核支持
> - 目前正在实现Processer，同时事务繁忙，正在缓慢更新

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

### 致谢：
> 感谢以下开源项目
[rCore][1]
[zCore][2]
[juner_os][3]

> 同时感谢以下文章或博客
[os.phil-opp][4]

 [1]: https://github.com/rcore-os/rCore
 [2]: https://github.com/rcore-os/zCore
 [3]: https://github.com/zzhgithub/juner_os
 [4]: https://os.phil-opp.com/
