all: arm9 arm7 dstool

arm9:
	$(MAKE) -C arm9app

arm7:
	$(MAKE) -C arm7app

dstool:
	ndstool	-c rusds.nds -7 arm7app.bin -9 arm9app.bin -i

clean:
	$(MAKE) clean -C arm7app
	$(MAKE) clean -C arm9app
	$(MAKE) clean -C rsdstool