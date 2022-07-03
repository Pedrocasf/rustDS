TARGET = arm9app
ARCH = armv5-none-dsi
MODE = release
all: xbuild copy

xbuild:
	cargo xbuild --target $(ARCH).json --$(MODE)
copy:
	arm-none-eabi-objcopy ../target/$(ARCH)/$(MODE)/$(TARGET) ../$(TARGET).bin
clean:
	cargo clean