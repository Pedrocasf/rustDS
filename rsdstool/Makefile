TARGET = RSDS
all: rsdstool

rsdstool:
	cargo run $(TARGET) arm9.bin arm7.bin $(TARGET).nds
	arm-none-eabi-objcopy -F binary $(TARGET).nds ../$(TARGET).nds

clean:
	cargo clean
	rm -fv arm9 arm9.bin arm7 arm7.bin $(TARGET).nds