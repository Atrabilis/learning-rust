# Insertion Sort

Implementar una función que ordene un slice mutable de enteros de menor a mayor usando insertion sort.

## Firma

```rust
pub fn insertion_sort(numbers: &mut [i32])
```

## Requisitos

* Ordenar el slice directamente.
* No crear otro `Vec`.
* No usar `sort()` ni `sort_unstable()`.
* Manejar slices vacíos y de un elemento.

## Pseudocódigo

```text
for current desde 1 hasta length(numbers) - 1:
    position = current

    while position > 0
          y numbers[position] < numbers[position - 1]:

        intercambiar numbers[position]
                    con numbers[position - 1]

        position = position - 1
```

## Casos a considerar

* Slice desordenado.
* Orden ascendente y descendente.
* Valores duplicados.
* Números negativos.
* Slice vacío.
* Un elemento.
* `i32::MIN` e `i32::MAX`.

## Complejidad esperada

* Mejor caso: `O(n)`
* Promedio y peor caso: `O(n²)`
* Memoria adicional: `O(1)`
