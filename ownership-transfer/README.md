# Ownership Transfer

Implementa:

```rust
fn add_suffix(text: String) -> String
```

La función debe recibir ownership de un `String`, agregarle `"!"` y devolverlo.

## Restricciones

- No usar `clone()`.
- Modificar el mismo `String` recibido.
- Usar `push()`.

## Ejemplo

```rust
let message = String::from("hello");

let updated = add_suffix(message);

println!("{updated}");
```

Resultado esperado:

```text
hello!
```

Después intenta ejecutar:

```rust
println!("{message}");
```

y analiza el error del compilador.

## Complejidad esperada

- Tiempo: `O(1)` amortizado
- Espacio adicional: `O(1)` amortizado