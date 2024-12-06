# **Concurrent Static File Server in Rust**

## **Descripción del Proyecto**
Este proyecto implementa un servidor web concurrente en Rust que sirve archivos estáticos desde un directorio especificado. El objetivo principal es demostrar el uso de las capacidades de **concurrencia**, **manejo de archivos** y **redes** en Rust. 

El servidor está diseñado para:
- Manejar múltiples solicitudes HTTP concurrentes usando hilos (`std::thread`).
- Servir archivos estáticos como HTML, imágenes, y otros recursos.
- Generar respuestas HTTP válidas con encabezados apropiados, incluyendo tipos MIME detectados dinámicamente.
- Manejar errores comunes, como archivos no encontrados o solicitudes malformadas.

## **Características**
1. **Concurrencia:**
   - Las conexiones entrantes se manejan en hilos separados, permitiendo múltiples solicitudes simultáneas.
2. **HTTP Básico:**
   - Soporte para solicitudes `GET`.
   - Generación de encabezados HTTP con estado (`200 OK`, `404 Not Found`, etc.).
3. **Manejo de Archivos:**
   - Servir cualquier archivo desde el directorio `static` del proyecto.
   - Detección automática de tipos MIME usando la biblioteca `mime_guess`.
4. **Manejo de Errores:**
   - Respuesta `404 Not Found` para archivos inexistentes.
   - Respuesta `500 Internal Server Error` para errores inesperados.
   - Respuesta `400 Bad Request` para solicitudes malformadas.
5. **Ejemplo de HTML con Imágenes:**
   - Una página de prueba (`index.html`) que incluye imágenes servidas directamente desde el servidor.

## **Estructura del Proyecto**
```
concurrent_server/
├── Cargo.toml          # Archivo de configuración del proyecto
├── static/             # Directorio para archivos estáticos
│   ├── index.html      # Página principal de prueba
│   ├── img_1.jpg       # Imagen 1
│   ├── img_2.jpg       # Imagen 2
│   └── img_3.jpg       # Imagen 3
└── src/
    └── main.rs         # Código fuente del servidor
```

## **Requisitos**
- **Rust** (Instalado desde https://www.rust-lang.org/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Navegador web para probar el servidor (Chrome, Firefox, etc.).

## **Cómo Usar**

### **1. Clonar el Repositorio**
```bash
git clone https://github.com/usuario/concurrent_server.git
cd concurrent_server
```

### **2. Compilar el Proyecto**
```bash
cargo build
```

### **3. Ejecutar el Servidor**
```bash
cargo run
```

El servidor estará disponible en `http://127.0.0.1:8080`.

### **4. Probar el Servidor**
- Abre `http://127.0.0.1:8080/index.html` en tu navegador.
- La página mostrará:
  - Descripción del sistema.
  - Imágenes servidas dinámicamente desde el servidor.

## **Ejemplo de Uso**

1. **Archivo Existente (`index.html`)**
   - Solicitud:
     ```
     GET /index.html HTTP/1.1
     Host: 127.0.0.1:8080
     ```
   - Respuesta:
     ```
     HTTP/1.1 200 OK
     Content-Type: text/html
     Content-Length: 1234

     (Contenido del archivo index.html)
     ```

2. **Archivo Inexistente (`missing.html`)**
   - Solicitud:
     ```
     GET /missing.html HTTP/1.1
     Host: 127.0.0.1:8080
     ```
   - Respuesta:
     ```
     HTTP/1.1 404 Not Found
     Content-Type: text/plain
     Content-Length: 13

     File not found
     ```

## **Detalles Técnicos**

### **Concurrencia**
El servidor utiliza `std::thread` para manejar cada conexión en un hilo separado, permitiendo solicitudes simultáneas sin bloquear el servidor principal.

### **Manejo de Archivos**
- Los archivos solicitados son leídos desde el directorio `static` usando la biblioteca estándar de Rust (`std::fs`).
- Se utiliza `mime_guess` para detectar el tipo MIME según la extensión del archivo.

### **Errores y Respuestas**
- El servidor responde con:
  - `200 OK`: Archivo servido correctamente.
  - `404 Not Found`: Archivo no encontrado.
  - `500 Internal Server Error`: Error inesperado al procesar la solicitud.
  - `400 Bad Request`: Solicitud HTTP malformada.

## **Capturas de Pantalla**

### **Página Principal**
![Index Page](http://127.0.0.1:8080/img_1.jpg)

### **Imágenes Servidas**
![Image 2](http://127.0.0.1:8080/img_2.jpg)
![Image 3](http://127.0.0.1:8080/img_3.jpg)

## **Próximos Pasos**
1. **Optimización:**
   - Utilizar un pool de hilos para limitar el uso de recursos en lugar de crear un hilo por conexión.
2. **Soporte Avanzado para HTTP:**
   - Añadir soporte para otros métodos como `POST` o `HEAD`.
3. **Integración con HTTPS:**
   - Usar certificados SSL para permitir conexiones seguras.
4. **Pruebas Automatizadas:**
   - Implementar pruebas de integración para verificar la funcionalidad completa del servidor.

## **Contribuciones**
- Si deseas contribuir, por favor realiza un fork del repositorio, crea una rama, realiza los cambios y envía un pull request.

## **Licencia**
Este proyecto está bajo la licencia MIT. Consulta el archivo `LICENSE` para más detalles.