# Borrow Without Taking Ownership

Implementa:

```rust
fn count_characters(text: &String) -> usize
```

La función debe contar cuántos caracteres tiene el texto sin tomar ownership.

## Restricciones

- No usar `clone()`.
- No modificar el `String`.
- Usar `chars()` y `count()`.
- Después de llamar a la función, `message` debe seguir siendo utilizable.

## Ejemplo

```rust
let message = String::from("rust");

let count = count_characters(&message);

println!("{count}");
println!("{message}");
```

Resultado esperado:

```text
4
rust
```

## Objetivo

Observar la diferencia entre:

```rust
String
```

y:

```rust
&String
```

## Complejidad esperada

- Tiempo: `O(n)`
- Espacio: `O(1)`