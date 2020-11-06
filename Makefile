.PHONY: all amd64 crate_esp run clean install-depends

default: all

all: amd64 run

# 编译bootloader和kernel
amd64:
	@echo Building bootloader...
	@cd bootloader && cargo build --release
	@echo Building kernel...
	@cd kernel && cargo build  --release
	@echo Building user
	@cd user && cargo build --release
	@make crate_esp

# x86_64编译出的文件目录
crate_esp:
	@echo Create build...
	@mkdir -p build/pc/esp/EFI/kernel build/pc/esp/EFI/Boot build/pc/user
	@cp target/x86_64-unknown-uefi/release/bootloader.efi build/pc/esp/EFI/Boot/BootX64.efi
	@cp target/amd64/release/kernel build/pc/esp/EFI/kernel/kernel.elf
	@cp target/amd64-user/release/hello_world users/hello_world
	@make pack

# QEMU运行x86_64
run:
	@qemu-system-x86_64 \
    -bios bootloader/OVMF.fd \
    -drive format=raw,file=fat:rw:build/pc/esp \
	-drive format=qcow2,file=build/pc/user.qcow2,media=disk,cache=writeback,id=sfsimg,if=none \
	-device ahci,id=ahci0 \
	-device ide-hd,drive=sfsimg,bus=ahci0.0 \
    -m 4096 \
    -smp 2 \
    -serial mon:stdio \
    -nographic \

# 清理编译出来的文件
clean:
	@cargo clean
	@rm -rf build
	@echo Clean complete!

# 依赖安装
install-depends:
	uname -a
	rustup install nightly
	rustup default nightly
	rustup component add rust-src
	rustup component add llvm-tools-preview
	cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs --rev 7f5eeac --force
	sudo apt install qemu-system
	@echo Install dependenics complete!

# 打包文件
pack:
	@rcore-fs-fuse build/pc/user.img users zip
	@qemu-img convert -f raw build/pc/user.img -O qcow2 build/pc/user.qcow2
	@qemu-img resize build/pc/user.qcow2 +1G
