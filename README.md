## Using VS Code to debug rust

https://www.brycevandyk.com/debug-rust-on-windows-with-visual-studio-code-and-the-msvc-debugger/


For me, I had to copy the visualizers
from here

```
C:\Users\Bilal\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\etc
```

to here

```
C:\Users\Bilal\.vscode\extensions\ms-vscode.cpptools-0.26.2\debugAdapters\vsdbg\bin\Visualizers
```

See the `minigrep` folder for VSCode settings

## To view documentation locally

`rustup docs`

## Create a new rust project

```
cargo new my-app --vcs none
```

## Failing CI
