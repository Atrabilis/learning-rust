# Replace Negatives

Implementa:

```rust
fn replace_negatives(numbers: &mut [i32])
```

La función debe reemplazar cada número negativo por `0`, modificando el slice original.

## Restricciones

- No crear otro array o vector.
- Recorrer el slice manualmente.
- No retornar ningún valor.

## Ejemplo

```rust
let mut numbers = [-4, 8, -2, 0, 5];

replace_negatives(&mut numbers);

println!("{numbers:?}");
```

Resultado esperado:

```text
[0, 8, 0, 0, 5]
```

## Complejidad esperada

- Tiempo: `O(n)`
- Espacio: `O(1)`