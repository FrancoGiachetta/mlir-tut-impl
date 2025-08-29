BUILD_DIR=build

.PHONY: build
build: 
	cmake -S . -B ${BUILD_DIR}
	cmake --build ${BUILD_DIR} 

.PHONY: mlir-headers
mlir-headers: build
	cmake --build ${BUILD_DIR} --target mlir-headers

.PHONY: mlir-docs
mlir-docs: build
	cmake --build ${BUILD_DIR} --target mlir-doc

.PHONY: test
test: build
	cmake --build ${BUILD_DIR} --target check-mlir-tutorial 

.PHONY: fmt
fmt:
	find . -iname '*.h' -o -iname '*.cpp' -o -iname '*.hpp' | clang-format --style=file -i --files=/dev/stdin

.PHONY: clean
clean:
	rm -rf ${BUILD_DIR}
	cd dialect-intregration && cargo clean
