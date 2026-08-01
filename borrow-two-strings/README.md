# Borrow Two Strings

Implementa:

```rust
fn total_length(first: &str, second: &str) -> usize
```

La función debe devolver la suma de las longitudes de ambos textos.

## Restricciones

- No tomar ownership.
- No usar `clone()`.
- Usar `len()`.
- Ambos valores deben seguir siendo utilizables después de llamar a la función.

## Ejemplo

```rust
let first = String::from("rust");
let second = String::from("lang");

let total = total_length(&first, &second);

println!("{total}");
println!("{first}");
println!("{second}");
```

Resultado esperado:

```text
8
rust
lang
```

## Objetivo

Practicar préstamos inmutables múltiples.
