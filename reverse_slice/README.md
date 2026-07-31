# Reverse Slice

Implementa:

```rust
fn reverse(numbers: &mut [i32])
```

La función debe invertir los elementos del slice **sin crear otro arreglo**.

## Restricciones

- Usar un bucle `while`.
- No usar `reverse()`.
- Intercambiar los valores manualmente.
- La función no debe retornar nada.

## Ejemplo

```rust
let mut numbers = [1, 2, 3, 4, 5];

reverse(&mut numbers);

println!("{numbers:?}");
```

Resultado esperado:

```text
[5, 4, 3, 2, 1]
```

## Complejidad esperada

- Tiempo: `O(n)`
- Espacio: `O(1)`