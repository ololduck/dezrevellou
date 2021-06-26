RS_SOURCES := $(wildcard src/*.rs)

.PHONY: release clean test clippy docker static

release: dist node_modules dist/dezrevellou.js dist/dezrevellou.min.js \
			dist/dezrevellou dist/dezrevellou.min.css dist/dezrevellou.css\
			dist/demo.html target/.docker

static:  dist node_modules dist/dezrevellou.min.js dist/dezrevellou.min.css dist/demo.html

node_modules: package.json
	npm install
	touch node_modules

dist/%: target/release/%
	cp $< $@

clean:
	rm -rf dist node_modules

dist:
	mkdir $@

target/.docker: dist/dezrevellou Dockerfile
	docker-compose build
	touch $@

dist/%.html: src/%.html
	cp $< $@
dist/dezrevellou.js dist/dezrevellou.js.map: dist node_modules src/dezrevellou.ts
	node_modules/.bin/tsc --sourceMap --outFile dist/dezrevellou.js src/dezrevellou.ts

dist/dezrevellou.min.js dist/dezrevellou.min.js.map: dist node_modules dist/dezrevellou.js
	node_modules/.bin/uglifyjs dist/dezrevellou.js --source-map url=dezrevellou.min.js.map --compress --mangle -o $@

dist/dezrevellou.css dist/dezrevellou.css.map: dist node_modules src/dezrevellou.sass
	node_modules/.bin/sass src/dezrevellou.sass --source-map dist/dezrevellou.css

dist/dezrevellou.min.css dist/dezrevellou.min.css.map: dist node_modules dist/dezrevellou.css
	cd dist && ../node_modules/.bin/cleancss -O2 --source-map --input-source-map dezrevellou.css.map -o dezrevellou.min.css dezrevellou.css

test: target/.last-test-run
clippy: target/.last-clippy-run

target/.last-test-run: $(RS_SOURCES)
	cargo test
	touch $@

target/.last-clippy-run: $(RS_SOURCES)
	RUSTC_WRAPPER="" cargo clippy -- -D warnings
	touch $@

target/release/dezrevellou: $(RS_SOURCES) target/.last-test-run target/.last-clippy-run dist/dezrevellou.min.css dist/dezrevellou.min.js dist/demo.html
	@echo "Rust sources: $(RS_SOURCES)"
	cargo build --release