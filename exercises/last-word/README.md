# Last Word

Implementa:

```rust
fn last_word(text: &str) -> &str
```

La función debe devolver la última palabra del texto sin crear un `String` nuevo.

## Reglas

- Las palabras están separadas por espacios.
- Si no hay espacios, devolver todo el texto.
- No usar `split()`, `rsplit()` ni `split_whitespace()`.
- No usar `clone()` ni `to_string()`.
- Recorrer el texto manualmente y devolver un slice.

## Ejemplos

```rust
last_word("rust ownership") // "ownership"
last_word("hello")          // "hello"
last_word("one two three")  // "three"
```

## Objetivo

Practicar:

```rust
&str
bytes()
enumerate()
rangos
string slices
```