# Find Second Largest

Implementa:

```rust
fn find_second_largest(numbers: &[i32]) -> i32
```

La función debe devolver el segundo valor más grande **distinto** del máximo.

## Restricciones

- Recorrer el slice manualmente.
- No usar `sort()`, `sort_unstable()` ni `max()`.
- No inicializar los acumuladores en `0`.
- Asumir que existen al menos dos valores distintos.

## Ejemplo

```rust
let numbers = [-1, 0, 10, 100, -500];
```

Resultado esperado:

```text
10
```

Con duplicados:

```rust
[10, 100, 100, 5]
```

Resultado esperado:

```text
10
```

## Complejidad esperada

- Tiempo: `O(n)`
- Espacio: `O(1)`