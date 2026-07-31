# Rust Conventions

Este documento define las convenciones de código utilizadas en este repositorio de aprendizaje.

El objetivo es mantener ejemplos consistentes, legibles e idiomáticos, siguiendo las prácticas habituales del ecosistema Rust.

## 1. Idioma

- El código, los identificadores y los nombres de archivos se escriben en inglés.
- Los comentarios pueden escribirse en español cuando su propósito sea pedagógico.
- Los mensajes de error destinados al usuario pueden escribirse en el idioma de la aplicación.

## 2. Convenciones de nombres

### `snake_case`

Usar `snake_case` para:

- Variables.
- Funciones.
- Métodos.
- Módulos.
- Nombres de archivos.
- Campos de structs.
- Macros declarativas.

```rust
let sample_count = 10;

fn calculate_average_power(samples: &[f64]) -> f64 {
    samples.iter().sum::<f64>() / samples.len() as f64
}
```

Archivos y módulos:

```text
src/
├── main.rs
├── modbus_client.rs
└── data_processing.rs
```

### `UpperCamelCase`

Usar `UpperCamelCase` para:

- Structs.
- Enums.
- Variantes de enums.
- Traits.
- Type aliases.
- Tipos genéricos con nombres descriptivos.

```rust
struct ModbusClient {
    device_id: u8,
}

enum OperatingState {
    Standby,
    OnGrid,
    FaultDetected,
}

trait DataSource {
    fn read_measurement(&self) -> f64;
}
```

### `SCREAMING_SNAKE_CASE`

Usar `SCREAMING_SNAKE_CASE` para:

- Constantes.
- Variables `static`.

```rust
const MAX_RETRIES: u32 = 3;
static DEFAULT_TIMEOUT_SECONDS: u64 = 5;
```

### Lifetimes

Usar nombres breves en minúsculas:

```rust
fn first_value<'a>(values: &'a [String]) -> Option<&'a String> {
    values.first()
}
```

Preferir `'a`, `'b` y `'static`. Usar nombres más descriptivos solo cuando mejoren claramente la comprensión.

## 3. Acrónimos

Tratar los acrónimos como palabras normales.

```rust
struct HttpClient;
struct PlcController;
struct CsvReader;

fn parse_csv_file() {}
fn read_plc_status() {}
```

Evitar:

```rust
struct HTTPClient;
struct PLCController;

fn ReadPLCStatus() {}
```

## 4. Nombres descriptivos

Elegir nombres que expresen propósito, unidad o significado.

Preferir:

```rust
let temperature_celsius = 25.0;
let retry_count = 3;
let inverter_status = read_inverter_status();
```

Evitar:

```rust
let temp = 25.0;
let x = 3;
let data = read();
```

Los nombres cortos son aceptables cuando su significado es evidente:

```rust
for item in items {
    println!("{item}");
}
```

## 5. Unidades

Incluir la unidad en el nombre cuando el tipo no la haga evidente.

```rust
let timeout_seconds: u64 = 10;
let voltage_volts: f64 = 230.0;
let current_amperes: f64 = 5.2;
```

Cuando una unidad forme parte importante del dominio, considerar un tipo específico:

```rust
struct Voltage(f64);
struct Current(f64);
```

## 6. Formato

Todo el código debe ser compatible con `rustfmt`.

```bash
cargo fmt
```

Antes de considerar terminado un ejercicio:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

No alinear manualmente expresiones con espacios. Dejar que `rustfmt` determine el formato.

## 7. Imports

Agrupar imports relacionados y evitar imports globales innecesarios.

```rust
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
```

Evitar wildcard imports salvo que exista una razón clara:

```rust
use some_module::*;
```

Preferir importar solo los elementos utilizados.

## 8. Estructura de módulos

Cada módulo debe tener una responsabilidad clara.

```text
src/
├── main.rs
├── config.rs
├── error.rs
├── modbus/
│   ├── mod.rs
│   ├── client.rs
│   └── register.rs
└── processing/
    ├── mod.rs
    └── average.rs
```

Evitar módulos demasiado grandes o con responsabilidades no relacionadas.

No crear abstracciones o capas adicionales antes de que exista una necesidad concreta.

## 9. Funciones

Las funciones deben:

- Tener una responsabilidad principal.
- Usar nombres verbales.
- Recibir solo los parámetros necesarios.
- Retornar valores en lugar de modificar estado global.
- Mantener una complejidad razonable.

```rust
fn parse_temperature(raw_value: i16, scale: f64) -> f64 {
    f64::from(raw_value) * scale
}
```

Preferir retornos tempranos para reducir anidación:

```rust
fn validate_samples(samples: &[f64]) -> Result<(), String> {
    if samples.is_empty() {
        return Err("samples cannot be empty".to_string());
    }

    Ok(())
}
```

## 10. Manejo de errores

Usar `Result` para errores recuperables y `Option` para ausencia de valor.

```rust
fn read_config(path: &Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
```

Evitar `unwrap()` y `expect()` en código de producción.

Son aceptables en:

- Tests.
- Ejemplos donde el fallo sea imposible por construcción.
- Prototipos claramente identificados.

Cuando se use `expect()`, incluir contexto útil:

```rust
let config = std::fs::read_to_string("config.toml")
    .expect("failed to read config.toml");
```

Propagar errores con `?` cuando no sea necesario manejarlos localmente:

```rust
fn load_values(path: &Path) -> Result<Vec<String>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content.lines().map(str::to_owned).collect())
}
```

## 11. Tipos

Preferir tipos explícitos cuando:

- La unidad o el rango sean importantes.
- Exista riesgo de conversión incorrecta.
- El tipo mejore la documentación del código.

```rust
let device_id: u8 = 1;
let sample_count: usize = samples.len();
```

Evitar conversiones con `as` cuando puedan truncar o alterar el valor silenciosamente.

Preferir:

```rust
let value = u16::try_from(raw_value)?;
```

## 12. Mutabilidad

Declarar valores inmutables por defecto.

```rust
let sample_count = samples.len();
```

Usar `mut` solo cuando el valor realmente deba cambiar:

```rust
let mut total = 0.0;

for sample in samples {
    total += sample;
}
```

## 13. Comentarios

Los comentarios deben explicar el motivo, la restricción o una decisión no evidente.

Preferir:

```rust
// The device exposes signed values through an unsigned Modbus register.
let temperature = i16::from_be_bytes(raw_value.to_be_bytes());
```

Evitar comentarios que repitan literalmente el código:

```rust
// Add one to count.
count += 1;
```

Usar comentarios de documentación para elementos públicos:

```rust
/// Calculates the arithmetic mean of a non-empty sample set.
///
/// Returns `None` when `samples` is empty.
pub fn calculate_average(samples: &[f64]) -> Option<f64> {
    if samples.is_empty() {
        return None;
    }

    Some(samples.iter().sum::<f64>() / samples.len() as f64)
}
```

## 14. Tests

Los tests deben:

- Usar nombres descriptivos.
- Probar un comportamiento concreto.
- Seguir, cuando sea útil, la estructura Arrange–Act–Assert.
- Incluir casos normales, límites y errores.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_average_returns_none_for_empty_slice() {
        let samples = [];

        let result = calculate_average(&samples);

        assert_eq!(result, None);
    }

    #[test]
    fn calculate_average_returns_expected_value() {
        let samples = [10.0, 20.0, 30.0];

        let result = calculate_average(&samples);

        assert_eq!(result, Some(20.0));
    }
}
```

## 15. Lints

Atender las advertencias del compilador y de Clippy.

Configuración recomendada para código de aprendizaje:

```rust
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
```

Las excepciones deben ser locales y justificadas:

```rust
#[allow(clippy::too_many_arguments)]
fn build_test_fixture(
    // ...
) {
}
```

No desactivar lints globalmente solo para ocultar advertencias.

## 16. Código inseguro

Evitar `unsafe` mientras exista una alternativa segura razonable.

Cuando sea imprescindible:

- Mantener el bloque lo más pequeño posible.
- Documentar la invariantes requeridas.
- Encapsularlo detrás de una API segura.
- Agregar tests específicos.

```rust
// SAFETY: `pointer` is valid, aligned, and points to an initialized `u32`.
let value = unsafe { *pointer };
```

## 17. Dependencias

Agregar una dependencia solo cuando:

- Resuelva un problema concreto.
- Evite implementar funcionalidad compleja o sensible.
- Sea mantenida y ampliamente utilizada.
- Su costo de compilación y complejidad sean razonables.

Revisar las dependencias no utilizadas:

```bash
cargo machete
```

## 18. Criterio de finalización

Un ejercicio o módulo se considera terminado cuando:

1. Compila sin errores.
2. Está formateado con `rustfmt`.
3. No genera advertencias de Clippy.
4. Sus tests pasan.
5. Los nombres siguen este documento.
6. No contiene `unwrap()` injustificados.
7. La solución evita complejidad innecesaria.

Comandos mínimos:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## 19. Referencias

Estas convenciones se basan principalmente en:

- The Rust Style Guide.
- Rust API Guidelines.
- Las recomendaciones de `rustfmt`.
- Las advertencias y sugerencias de Clippy.
