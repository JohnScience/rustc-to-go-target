# Design document

## Motivation

The motivation for creation of `rustc-to-go-target` was to provide a way to embed Go binaries (which are in the context of embedding external binaries are called "sidecars") into Tauri applications.

At the moment of writing, building a Tauri application for arbitrary [target triple](https://github.com/cross-lang-and-cross-platform/compile-targets) requires [presence of directory with binaries for that target triple and configuration of embedding external binaries in `tauri.conf.json`](https://tauri.app/v1/guides/building/sidecar).

## Approaches

The simplest approach is to have a huge hardcoded match statement.

The problem with such approach is that it is not that easy to maintain and it is not that easy to unify the code for other compile target sets, such as those of `vcpkg` and LLVM. In addition, it is a bit more difficult to have
a single source of truth for (1) the lists of supported targets for each compile target set and (2) this huge match statement. This is possible to do with declarative macro on the big match statement, but it is not that easy to maintain as well.

Eventually each compile target set should be parsed into a unified format, which then should used to generate the code. However, this can introduce a lot of complexity. Ideally the info about the assumptions about various compile targets and their sets should be easily accessible, for example in the form of a GitHub repository (e.g. <https://github.com/cross-lang-and-cross-platform/compile-targets>) with Markdown files.
