.PHONY: build dev clean

build:
	cd engine && wasm-pack build --target web --out-dir ../web/src/engine/pkg

dev: build
	cd web && npm run dev

clean:
	rm -rf web/src/engine/pkg
