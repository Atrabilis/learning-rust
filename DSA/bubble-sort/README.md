# Bubble Sort

Implementar una función que ordene un slice mutable de enteros de menor a mayor usando bubble sort.

## Firma

```rust
pub fn bubble_sort(numbers: &mut [i32])
```

## Requisitos

* Ordenar el slice directamente.
* No crear otro `Vec`.
* No usar `sort()` ni `sort_unstable()`.
* Comparar elementos adyacentes e intercambiarlos cuando estén desordenados.
* Detener el algoritmo anticipadamente si una pasada no realiza intercambios.

## Pseudocódigo

```text
repeat:
    swapped = false

    for current desde 0 hasta length(numbers) - 2:
        if numbers[current] > numbers[current + 1]:
            intercambiar numbers[current] y numbers[current + 1]
            swapped = true

until swapped sea false
```

## Complejidad esperada

* Mejor caso: O(n)
* Promedio y peor caso: O(n²)
* Memoria adicional: O(1)
