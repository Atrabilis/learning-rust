# First Word

Implementa:

```rust
fn first_word(text: &str) -> &str
```

La función debe devolver la primera palabra del texto sin crear un `String` nuevo.

## Reglas

- La primera palabra termina en el primer espacio.
- Si no hay espacios, devolver todo el texto.
- No usar `clone()`.
- No usar `to_string()`.
- Trabajar con slices de string.

## Ejemplos

```rust
first_word("rust ownership") // "rust"
first_word("hello")          // "hello"
```

## Objetivo

Practicar:

```rust
&str
string slices
rangos
préstamos
```