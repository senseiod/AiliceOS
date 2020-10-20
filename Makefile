.PHONY: all amd64 crate_esp run clean install-depends

all: amd64 run

# 编译bootloader和kernel
amd64:
	@cd bootloader && cargo build --release
	@cd kernel && cargo build  --release
	@make crate_esp

# x86_64编译出的文件目录
crate_esp:
	@mkdir -p build/pc/esp/EFI/kernel build/pc/esp/EFI/Boot
	@cp target/x86_64-unknown-uefi/release/bootloader.efi build/pc/esp/EFI/Boot/BootX64.efi
	@cp target/amd64/release/kernel build/pc/esp/EFI/kernel/kernel.elf

# QEMU运行x86_64
run:
	@qemu-system-x86_64 \
    -bios bootloader/OVMF.fd \
    -drive format=raw,file=fat:rw:build/pc/esp \
    -m 4096 \
    -smp 2 \
    -serial mon:stdio \
   # -nographic \

# 清理编译出来的文件
clean:
	@cargo clean
	@rm -rf build

# 依赖安装
install-depends:
	rustup install nightly
	rustup default nightly
	rustup component add rust-src
	rustup component add llvm-tools-preview
