RS_SOURCES := $(wildcard src/*.rs)

.PHONY: release clean test clippy docker

release: dist node_modules dist/dezrevellou.js dist/dezrevellou.min.js \
			dist/dezrevellou dist/dezrevellou.min.css dist/dezrevellou.css\
			dist/demo.html target/.docker

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
dist/%.js dist/%.js.map: src/%.ts
	node_modules/.bin/tsc --sourceMap --outFile $@ $<

dist/%.min.js dist/%.min.js.map: dist/%.js
	node_modules/.bin/uglifyjs $< --source-map url=dezrevellou.min.js.map --compress --mangle -o $@

dist/%.css dist/%.css.map: src/%.sass
	node_modules/.bin/sass --source-map $< $@
dist/%.min.css dist/%.min.css.map: dist/%.css
	node_modules/.bin/cleancss -O2 --source-map -o $@ $<

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