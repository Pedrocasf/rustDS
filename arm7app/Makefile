TARGET = arm7app
ARCH = armv4t-none-eabi
MODE = release
all: copy

xbuild: 
	cargo build --$(MODE)
copy: xbuild
	/opt/devkitpro/devkitARM/bin/arm-none-eabi-objcopy ../target/$(ARCH)/$(MODE)/$(TARGET) ../$(TARGET).bin
clean:
	cargo clean