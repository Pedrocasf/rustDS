all: clean arm9 arm7 rsdstool

arm9:
	$(MAKE) -C arm9/arm9app/
	arm-none-eabi-objcopy arm9/arm9app.bin rsdstool/arm9.bin

arm7:
	$(MAKE) -C arm7/arm7app/
	arm-none-eabi-objcopy arm7/arm7app.bin rsdstool/arm7.bin

rsdstool:
	$(MAKE) -C rsdstool

clean:
	$(MAKE) clean -C arm7/arm7app/ 
	$(MAKE) clean -C arm9/arm9app/ 
	$(MAKE) clean -C rsdstool
	rm -rf *.nds