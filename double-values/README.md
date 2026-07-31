# Double Values In Place

Implementa:

```rust
fn double_values(numbers: &mut [i32])
```

La función debe multiplicar por `2` cada elemento del slice, modificándolo directamente.

## Restricciones

- Usar `iter_mut()`.
- No crear otro array o vector.
- No retornar ningún valor.

## Ejemplo

```rust
let mut numbers = [1, -2, 3, 0];

double_values(&mut numbers);

println!("{numbers:?}");
```

Resultado esperado:

```text
[2, -4, 6, 0]
```

## Complejidad esperada

- Tiempo: `O(n)`
- Espacio: `O(1)`