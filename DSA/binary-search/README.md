# Binary Search

Implementar una función que busque un valor en un slice de enteros ordenado de forma ascendente usando búsqueda binaria.

## Firma

```rust
pub fn binary_search(numbers: &[i32], target: i32) -> Option<usize>
```

## Requisitos

* Retornar `Some(index)` si el valor existe.
* Retornar `None` si no existe o el slice está vacío.
* Reducir el rango de búsqueda a la mitad en cada iteración.
* No usar `binary_search()` de la biblioteca estándar.
* No modificar ni copiar el slice.
* Asumir que el slice está ordenado ascendentemente.

## Casos a considerar

* Valor al inicio, medio y final.
* Valor inexistente.
* Slice vacío.
* Slice con un elemento.
* Números negativos.
* Cantidad par e impar de elementos.

## Complejidad esperada

* Tiempo: `O(log n)`
* Memoria adicional: `O(1)`
