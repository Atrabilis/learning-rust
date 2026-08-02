# Linear Search

Implementar una función que busque un valor en un slice de enteros y devuelva el índice de su primera aparición.

## Firma

```rust
pub fn linear_search(numbers: &[i32], target: i32) -> Option<usize>
```

## Requisitos

* Retornar `Some(index)` si el valor existe.
* Retornar el índice de la primera aparición.
* Retornar `None` si el valor no existe o el slice está vacío.
* No modificar el slice.
* No usar `position()`, `find()` ni `contains()`.

## Casos a probar

* Valor al inicio, medio y final.
* Valor repetido.
* Valor inexistente.
* Slice vacío.
* Números negativos.
* `i32::MIN` e `i32::MAX`.

## Complejidad esperada

* Tiempo: `O(n)`
* Memoria adicional: `O(1)`
