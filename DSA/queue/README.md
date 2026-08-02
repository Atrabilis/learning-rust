# Queue

Implementar una estructura `Queue` de enteros usando internamente `VecDeque<i32>`.

## Estructura

```rust
pub struct Queue {
    // almacenamiento interno
}
```

## Métodos requeridos

```rust
pub fn new() -> Queue
pub fn enqueue(&mut self, value: i32)
pub fn dequeue(&mut self) -> Option<i32>
pub fn front(&self) -> Option<&i32>
pub fn len(&self) -> usize
pub fn is_empty(&self) -> bool
```

## Requisitos

* Comportamiento FIFO: el primer elemento insertado es el primero en salir.
* `enqueue()` agrega al final.
* `dequeue()` elimina desde el frente.
* `front()` observa el primer elemento sin eliminarlo.
* `dequeue()` y `front()` retornan `None` cuando la cola está vacía.

## Pseudocódigo

```text
enqueue(value):
    agregar value al final

dequeue():
    si la cola está vacía:
        retornar no_encontrado
    eliminar y retornar el primer elemento

front():
    si la cola está vacía:
        retornar no_encontrado
    retornar referencia al primer elemento
```

## Complejidad esperada

* `enqueue`: O(1) amortizado
* `dequeue`: O(1)
* `front`: O(1)
* `len`: O(1)
* `is_empty`: O(1)
