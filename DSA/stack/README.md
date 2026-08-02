# Stack

Implementar una estructura `Stack` de enteros usando internamente un `Vec<i32>`.

## Estructura

```rust
pub struct Stack {
    // almacenamiento interno
}
```

## Métodos requeridos

```rust
pub fn new() -> Stack
pub fn push(&mut self, value: i32)
pub fn pop(&mut self) -> Option<i32>
pub fn peek(&self) -> Option<&i32>
pub fn len(&self) -> usize
pub fn is_empty(&self) -> bool
```

## Requisitos

* Comportamiento LIFO: el último elemento insertado es el primero en salir.
* `pop()` debe eliminar y devolver el elemento superior.
* `peek()` debe devolver una referencia al elemento superior sin eliminarlo.
* `pop()` y `peek()` deben devolver `None` cuando la pila esté vacía.
* No usar `VecDeque`.

## Pseudocódigo

```text
push(value):
    agregar value al final del almacenamiento

pop():
    si la pila está vacía:
        retornar no_encontrado
    eliminar y retornar el último elemento

peek():
    si la pila está vacía:
        retornar no_encontrado
    retornar referencia al último elemento
```

## Complejidad esperada

* `push`: O(1) amortizado
* `pop`: O(1)
* `peek`: O(1)
* `len`: O(1)
* `is_empty`: O(1)
