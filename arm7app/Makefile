TARGET = arm7app
ARCH = armv4-none-dsi
MODE = release
all: copy

xbuild: 
	cargo xbuild --target $(ARCH).json --$(MODE)
copy: xbuild
	arm-none-eabi-objcopy ../target/$(ARCH)/$(MODE)/$(TARGET) ../$(TARGET).bin
clean:
	cargo clean