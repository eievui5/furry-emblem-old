PNGS := $(shell find src/ -name '*.png')
TARGETS := \
	$(patsubst src/%.png, $(OUT)/%.4bpp, $(PNGS))

resources: $(TARGETS)
.PHONY: resources

clean:
	rm -f $(TARGETS)
.PHONY: clean

$(OUT)/%.4bpp: src/%.png
	@mkdir -p $(@D)
	grit $< -gTFF00FF -ftb -fa -gB4 "-o$(OUT)/$*"
	mv $(OUT)/$*.img.bin $(OUT)/$*.4bpp
