# Append Suffix by Mutable Reference

Implementa:

```rust
fn append_suffix(text: &mut String, suffix: &str)
```

La función debe agregar `suffix` al final de `text`, modificando el `String` original sin tomar ownership.

## Restricciones

- Usar `push_str()`.
- No usar `clone()`.
- No retornar ningún valor.
- `text` debe seguir siendo utilizable después de llamar a la función.

## Ejemplo

```rust
let mut message = String::from("hello");

append_suffix(&mut message, " world");

println!("{message}");
```

Resultado esperado:

```text
hello world
```

## Objetivo

Distinguir entre:

```rust
String
&String
&mut String
&str
```

## Complejidad esperada

- Tiempo: `O(m)`, donde `m` es la longitud del sufijo.
- Espacio adicional: `O(m)` amortizado por el crecimiento del `String`.