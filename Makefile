PREFIX ?= $(HOME)/.local
BINDIR ?= $(PREFIX)/bin
APPDIR ?= $(PREFIX)/share/applications
ICONDIR ?= $(PREFIX)/share/icons/hicolor/scalable/apps

.PHONY: all build install uninstall clean

all: build

build:
	cargo build --release

install: build
	@echo "Installing to $(PREFIX)..."
	@mkdir -p $(BINDIR)
	@mkdir -p $(APPDIR)
	@mkdir -p $(ICONDIR)
	install -m 755 target/release/ask $(BINDIR)/ask
	install -m 755 target/release/aura $(BINDIR)/aura
	
	@# Generate desktop file with absolute paths
	@sed -e "s|Exec=aura|Exec=$(BINDIR)/aura|" \
	     -e "s|Icon=aura|Icon=$(ICONDIR)/aura.svg|" \
	     assets/aura.desktop > target/aura.desktop
	
	install -m 644 target/aura.desktop $(APPDIR)/aura.desktop
	install -m 644 assets/aura.svg $(ICONDIR)/aura.svg
	
	@# Try to copy to Desktop if it exists
	@if [ -d "$(HOME)/Desktop" ]; then \
		echo "Creating Desktop shortcut..."; \
		install -m 755 target/aura.desktop $(HOME)/Desktop/aura.desktop; \
	fi

	@echo "Installation complete."
	@echo "Binaries installed to: $(BINDIR)"
	@echo "Desktop entry installed to: $(APPDIR)"
	@echo "Icon installed to: $(ICONDIR)"
	@echo "Please ensure $(BINDIR) is in your PATH."
	@echo "Installation complete."
	@echo "Binaries installed to: $(BINDIR)"
	@echo "Desktop entry installed to: $(APPDIR)"
	@echo "Icon installed to: $(ICONDIR)"
	@echo "Please ensure $(BINDIR) is in your PATH."

uninstall:
	rm -f $(BINDIR)/ask
	rm -f $(BINDIR)/aura
	rm -f $(APPDIR)/aura.desktop
	rm -f $(ICONDIR)/aura.svg
	@echo "Uninstallation complete."

clean:
	cargo clean
