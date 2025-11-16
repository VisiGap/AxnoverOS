.PHONY: all kernel userspace boot boot-bios boot-uefi clean run iso

all: kernel userspace boot

kernel:
	cd kernel && cargo build --release

userspace:
	$(MAKE) -C userspace

boot: boot-bios

boot-bios:
	$(MAKE) -C boot

boot-uefi:
	$(MAKE) -C boot/uefi

clean:
	cd kernel && cargo clean
	$(MAKE) -C userspace clean
	$(MAKE) -C boot clean
	rm -rf build/

run: all
	qemu-system-x86_64 -drive format=raw,file=build/fractureos.img

run-uefi: all boot-uefi
	qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd \
	                   -drive format=raw,file=build/fractureos-uefi.img

iso: all
	./tools/create-iso.sh

image: all
	./tools/create-image.sh

.DEFAULT_GOAL := all
