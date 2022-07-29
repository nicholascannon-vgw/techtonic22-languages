# Techtonic 2022 Workshop: Languages

Learn what makes programming languages unique and how to select the one that’s right for your project.

## Languages Covered

-   **JavaScript**: high level, dynamically typed, interpreted, garbage collected scripting languages
-   **Go**: Natively compiled, statically typed, garbage collected programming language
-   **Rust**: Natively compiled, statically typed, low level, non garbage collected programming language

## Standard library docs

-   https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference
-   https://pkg.go.dev/std
-   https://doc.rust-lang.org/std/

## How to measure container sizes

```sh
➜ docker inspect -f "{{ .Size }}" techtonic-$LANG:latest
```

Replace `$LANG` with `js`, `go` or `rust`.
