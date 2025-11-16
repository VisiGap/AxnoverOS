NASM := nasm
BUILD_DIR := ../build

.PHONY: all clean

all: $(BUILD_DIR)/boot.bin $(BUILD_DIR)/stage2.bin

$(BUILD_DIR)/boot.bin: boot.asm
	@mkdir -p $(BUILD_DIR)
	$(NASM) -f bin $< -o $@
	@echo "Built Stage 1 bootloader"

$(BUILD_DIR)/stage2.bin: stage2.asm
	@mkdir -p $(BUILD_DIR)
	$(NASM) -f bin $< -o $@
	@echo "Built Stage 2 bootloader"

clean:
	rm -f $(BUILD_DIR)/boot.bin $(BUILD_DIR)/stage2.bin

.DEFAULT_GOAL := all
