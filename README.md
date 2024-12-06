# **Concurrent Static File Server in Rust**

## **Descripción del Proyecto**
Este proyecto implementa un servidor web concurrente en Rust que sirve archivos estáticos desde un directorio especificado. Es una demostración práctica de las capacidades de **concurrencia**, **manejo de archivos** y **redes** en Rust.

El servidor permite:
- Manejar múltiples solicitudes HTTP concurrentes usando hilos (`std::thread`).
- Servir archivos estáticos como HTML, imágenes y otros recursos.
- Generar respuestas HTTP válidas con encabezados apropiados, incluyendo tipos MIME detectados dinámicamente.
- Manejar errores comunes, como archivos no encontrados o solicitudes malformadas.

## **Características**
1. **Concurrencia:**
   - Manejo de conexiones entrantes en hilos separados, permitiendo múltiples solicitudes simultáneas.
2. **HTTP Básico:**
   - Soporte para solicitudes `GET`.
   - Generación de encabezados HTTP (`200 OK`, `404 Not Found`, etc.).
3. **Manejo de Archivos:**
   - Servir cualquier archivo desde el directorio `static`.
   - Detección automática de tipos MIME usando `mime_guess`.
4. **Manejo de Errores:**
   - Respuesta `404 Not Found` para archivos inexistentes.
   - Respuesta `500 Internal Server Error` para errores inesperados.
   - Respuesta `400 Bad Request` para solicitudes malformadas.

## **Requisitos**
- **Rust** (descargar en https://www.rust-lang.org/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Python 3.x** (para ejecutar `test.py`).

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

El servidor estará disponible en `http://127.0.0.1:5005`.

### **4. Probar el Servidor Manualmente**
- Abre `http://127.0.0.1:5005/index.html` en tu navegador.
- Realiza solicitudes HTTP válidas e inválidas para probar su funcionalidad.

### **5. Realizar Pruebas con `test.py`**
El script `test.py` automatiza pruebas comunes para verificar el comportamiento del servidor. 

#### Ejecución:
Asegúrate de que el servidor esté corriendo y ejecuta:
```bash
python test.py
```

#### Funcionalidades probadas:
- Solicitudes válidas (`/index.html` y otros archivos existentes).
- Solicitudes inválidas (archivos inexistentes o rutas incorrectas).
- Manejo de múltiples solicitudes rápidas.

## **Ejemplo de Uso**

1. **Archivo Existente (`index.html`)**
   - Solicitud:
     ```
     GET /index.html HTTP/1.1
     Host: 127.0.0.1:5005
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
     Host: 127.0.0.1:5005
     ```
   - Respuesta:
     ```
     HTTP/1.1 404 Not Found
     Content-Type: text/plain
     Content-Length: 13

     File not found
     ```