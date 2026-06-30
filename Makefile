.PHONY: build dev clean

build:
	cd engine && wasm-pack build --target web

dev: build
	cd web && npm run dev

clean:
	rm -rf engine/pkg
