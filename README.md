# How to Build Sources

```sh
make build
```

## Generate MLIR Headers

```sh
make mlir-headers
```

## Generate MLIR Docs

```sh
make mlir-docs
```

## Run Tests

```sh
make test
```

# Some useful configuration for VSCode

By default, Vscode won’t recognize the paths from `#include <file-path>.h`. To allow this, we need to create a file `.vscode/c_cpp_properties.json`, at the root of the project, and add the following config:

```json
{
    "configurations": [
        {
            "name": "Mac",
            "includePath": [
                "${workspaceFolder}/**",
                "<library-include-path>/"
            ],
            "defines": [],
            "compilerPath": "/usr/bin/clang",
            "cStandard": "c<standard-version>",
            "cppStandard": "c++<standard-version>",
            "intelliSenseMode": "macos-clang-arm64"
        }
    ],
    "version": 4
}
```

In particular, for llvm we need to add the following line after `"${workspaceFolder}/**"`:

`"/opt/homebrew/opt/llvm@19/include”` (would be better to use a command or env var instead of the full path, but this works for now)

For TableGen development, we have an [extension](https://github.com/arata-nvm/tablegen-lsp) which gives us an TableGen lsp. For it to be able to recognize llvm’s include paths, we need to specify the following config inside a `.vscode/settings.json` file at the root of the project:

```json
{
    "tablegen-lsp.includePath": [
        "/opt/homebrew/opt/llvm@19/include",
    ]
}
```
