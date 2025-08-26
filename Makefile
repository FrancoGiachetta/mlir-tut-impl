BUILD_DIR=build-dir

.PHONY: all
all: 
	cmake -S . -B ${BUILD_DIR}
	cmake --build ${BUILD_DIR} 

.PHONY: mlir-headers
mlir-headers: 
	cmake -S . -B ${BUILD_DIR}
	cmake --build ${BUILD_DIR} --target mlir-headers

.PHONY: mlir-docs
mlir-docs: 
	cmake -S . -B ${BUILD_DIR}
	cmake --build ${BUILD_DIR} --target mlir-docs

.PHONY: test
test:
	cmake -S . -B ${BUILD_DIR}
	cmake --build ${BUILD_DIR} --target check-mlir-tutorial 

.PHONY: clean
clean:
	rm -rf ${BUILD_DIR}
