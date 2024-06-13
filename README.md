# City Distance Calculator

## How to Use

- Select a Mode to get the city Coordinates (API, CSV, Mock)
- With API mode, you can insert multiple words to specify more. Example: Barranco Lima Peru
- With CSV mode, insert only the city name in english. Example: Lima

## Github Brances info:

- Main Branch: Contains the three-city closest pair version
- Legacy: Contains the original two city distance calculator
- UI: The original implementation of the graphical user interface and the three-city closest pair

## Tests

#### How to test:

```bash
cargo test
```

#### Test list:

| Test Case        | Precondition | Test Steps                                                                                                                                    | Test Data                                                                                                                                    | Expected Results             |
|------------------|--------------|----------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|------------------------------|
| `test_1`         | La función es válida      | 1. Crear coordenadas `a` (0.0, 0.0) y `b` (1.0, 1.0). <br> 2. Llamar a `distance_km(&a, &b)` y redondear el resultado.                        | `a: Coordinates { latitude: 0.0, longitude: 0.0 }` <br> `b: Coordinates { latitude: 1.0, longitude: 1.0 }`                                    | Resultado redondeado es 157 km. |
| `test_2`         | La función es válida      | 1. Crear coordenadas `a` (34.0522, -118.2437) y `b` (40.7128, -74.0060). <br> 2. Llamar a `distance_km(&a, &b)` y redondear el resultado.      | `a: Coordinates { latitude: 34.0522, longitude: -118.2437 }` <br> `b: Coordinates { latitude: 40.7128, longitude: -74.0060 }`                | Resultado redondeado es 3936 km.|
| `test_3`         | La función es válida      | 1. Crear coordenadas `a` (-33.8688, 151.2093) y `b` (35.6895, 139.6917). <br> 2. Llamar a `distance_km(&a, &b)` y redondear el resultado.      | `a: Coordinates { latitude: -33.8688, longitude: 151.2093 }` <br> `b: Coordinates { latitude: 35.6895, longitude: 139.6917 }`                | Resultado redondeado es 7827 km.|
| `test_4`         | La función es válida      | 1. Crear coordenadas `a` (-1.0, -1.0) y `b` (2.0, 2.0). <br> 2. Llamar a `distance_km(&a, &b)` y redondear el resultado.                      | `a: Coordinates { latitude: -1.0, longitude: -1.0 }` <br> `b: Coordinates { latitude: 2.0, longitude: 2.0 }`                                  | Resultado redondeado es 472 km. |
| `test_5`         | La función es válida      | 1. Crear coordenadas `a` (51.5074, -0.1278) y `b` (48.8566, 2.3522). <br> 2. Llamar a `distance_km(&a, &b)` y redondear el resultado.          | `a: Coordinates { latitude: 51.5074, longitude: -0.1278 }` <br> `b: Coordinates { latitude: 48.8566, longitude: 2.3522 }`                    | Resultado redondeado es 344 km. |
| `test_6`         | La función es válida      | 1. Crear coordenadas `a` (-37.8136, 144.9631) y `b` (-33.8688, 151.2093). <br> 2. Llamar a `distance_km(&a, &b)` y redondear el resultado.      | `a: Coordinates { latitude: -37.8136, longitude: 144.9631 }` <br> `b: Coordinates { latitude: -33.8688, longitude: 151.2093 }`                | Resultado redondeado es 713 km. |
| `test_7`         | La función es válida      | 1. Crear coordenadas `a` (35.6895, 139.6917) y `b` (55.7558, 37.6173). <br> 2. Llamar a `distance_km(&a, &b)` y redondear el resultado.        | `a: Coordinates { latitude: 35.6895, longitude: 139.6917 }` <br> `b: Coordinates { latitude: 55.7558, longitude: 37.6173 }`                  | Resultado redondeado es 7478 km.|
| `test_invalid_city`  | La función es válida y CSV tiene datos correctos      | 1. Crear un objeto `CityInterface` para "INVALID CITY" desde un archivo CSV. <br> 2. Obtener las coordenadas del objeto. <br> 3. Validar que retorna un error. | `city: CityInterface::CSV(CityCSV::new("INVALID CITY", CSV_FILE))`    | Resultado de coordenadas es Error. |
| `test_dup_city`  | La función es válida y CSV tiene datos correctos      | 1. Crear tres objetos `CityInterface` para "Sydney" desde un archivo CSV. <br> 2. Obtener las coordenadas para cada objeto. <br> 3. Llamar a `three_point_distance` con los objetos y coordenadas. | `city1, city2, city3: CityInterface::CSV(CityCSV::new("Sydney", CSV_FILE))` <br> `coords1, coords2, coords3: get_coordinates(&city)`         | Resultado de distancia es 0 km. |
| `test_api`       | La función es válida y el servicio API está activo      | 1. Crear tres objetos `CityInterface` para "Lima", "Huamanga" y "Iquitos" usando API. <br> 2. Obtener las coordenadas para cada objeto. <br> 3. Llamar a `three_point_distance` con los objetos y coordenadas. | `city1: CityInterface::API(CityAPI::new("Lima", API_KEY))` <br> `city2: CityInterface::API(CityAPI::new("Huamanga", API_KEY))` <br> `city3: CityInterface::API(CityAPI::new("Iquitos", API_KEY))` | Resultado redondeado es 329 km. |
| `test_csv`       | La función es válida y CSV tiene datos correctos      | 1. Crear tres objetos `CityInterface` para "Lima", "Ayacucho" e "Iquitos" desde un archivo CSV. <br> 2. Obtener las coordenadas para cada objeto. <br> 3. Llamar a `three_point_distance` con los objetos y coordenadas. | `city1: CityInterface::CSV(CityCSV::new("Lima", CSV_FILE))` <br> `city2: CityInterface::CSV(CityCSV::new("Ayacucho", CSV_FILE))` <br> `city3: CityInterface::CSV(CityCSV::new("Iquitos", CSV_FILE))` | Resultado redondeado es 329 km. |
