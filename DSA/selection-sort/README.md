# Selection Sort

Implementar una función que ordene un slice mutable de enteros de menor a mayor usando selection sort.

## Firma

```rust
pub fn selection_sort(numbers: &mut [i32])
```

## Requisitos

* Ordenar el slice directamente.
* No crear otro `Vec`.
* No usar `sort()` ni `sort_unstable()`.
* En cada iteración, buscar el menor elemento de la parte no ordenada y moverlo a la posición actual.
* Manejar slices vacíos y de un elemento.

## Pseudocódigo

```text
for current desde 0 hasta length(numbers) - 1:
    smallest = current

    for candidate desde current + 1 hasta length(numbers) - 1:
        if numbers[candidate] < numbers[smallest]:
            smallest = candidate

    if smallest != current:
        intercambiar numbers[current] y numbers[smallest]
```

## Complejidad esperada

* Mejor, promedio y peor caso: O(n²)
* Memoria adicional: O(1)
