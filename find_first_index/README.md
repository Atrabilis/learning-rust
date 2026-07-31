# Find First Index

Implementa:

```rust
fn find_first_index(numbers: &[i32], target: i32) -> Option<usize>
```

La función debe devolver el índice de la primera aparición de `target`.

## Restricciones

- Recorrer el slice manualmente con `while`.
- No usar `position()`.
- Devolver `None` si el valor no existe.

## Ejemplos

```rust
find_first_index(&[4, 8, 2, 8], 8)  // Some(1)
find_first_index(&[4, 8, 2, 8], 5)  // None
```

## Complejidad esperada

- Tiempo: `O(n)`
- Espacio: `O(1)`