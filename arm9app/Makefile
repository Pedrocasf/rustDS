TARGET = arm9app
ARCH = armv5te-none-eabi
MODE = release
all: xbuild copy

xbuild:
	cargo build --$(MODE)
copy:
	/opt/devkitpro/devkitARM/bin/arm-none-eabi-objcopy ../target/$(ARCH)/$(MODE)/$(TARGET) ../$(TARGET).bin
clean:
	cargo clean