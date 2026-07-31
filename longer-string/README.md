# Return the Longer String

Implementa:

```rust
fn longer(first: String, second: String) -> String
```

La función debe tomar ownership de dos `String` y devolver el más largo.

## Reglas

- No usar `clone()`.
- No crear un tercer `String`.
- Si ambos tienen la misma longitud, devolver `first`.
- Comparar usando `len()`.

## Ejemplo

```rust
let first = String::from("rust");
let second = String::from("ownership");

let result = longer(first, second);

println!("{result}");
```

Resultado esperado:

```text
ownership
```

Luego intenta usar `first` y `second` después de llamar a la función y revisa el error del compilador.