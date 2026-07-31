# Count Occurrences

Implementa:

```rust
fn count_occurrences(numbers: &[i32], target: i32) -> usize
```

La función debe contar cuántas veces aparece `target` en el slice.

## Restricciones

- Recorrer el slice manualmente.
- No usar `filter()`, `count()` ni colecciones auxiliares.

## Ejemplos

```rust
count_occurrences(&[4, 8, 2, 8, 8], 8) // 3
count_occurrences(&[4, 8, 2, 8], 5)    // 0
```

## Complejidad esperada

- Tiempo: `O(n)`
- Espacio: `O(1)`