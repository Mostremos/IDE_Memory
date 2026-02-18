# Configuración en Cursor IDE

## Guía Paso a Paso

### 1. Compilar el Servidor

```bash
cd IDE-Memory_MCP-Server
cargo build --release
```

El binario estará en:
- **Windows**: `target/release/IDE_Memory.exe`
- **Linux/Mac**: `target/release/IDE_Memory`

### 2. Ubicar el Binario

Copia el binario a una ubicación permanente:

**Windows:**
```powershell
# Crear directorio si no existe
New-Item -ItemType Directory -Force -Path C:\bin

# Copiar binario
Copy-Item target\release\IDE_Memory.exe C:\bin\
```

**Linux/Mac:**
```bash
# Crear directorio si no existe
mkdir -p ~/bin

# Copiar binario
cp target/release/IDE_Memory ~/bin/

# Hacer ejecutable
chmod +x ~/bin/IDE_Memory
```

### 3. Configurar Cursor IDE

#### Opción A: Archivo de Configuración Manual

1. Abre el archivo de configuración de Cursor:
   - **Windows**: `%APPDATA%\Cursor\User\settings.json` o `~/.config/Cursor/User/settings.json`
   - **Linux**: `~/.config/Cursor/User/settings.json`
   - **Mac**: `~/Library/Application Support/Cursor/User/settings.json`

2. Busca la sección `mcpServers` o créala si no existe:

**Windows:**
```json
{
  "mcpServers": {
    "ide_memory": {
      "command": "C:\\bin\\IDE_Memory.exe",
      "args": [
        "--database",
        "C:\\Users\\TuUsuario\\ide_memory.db"
      ]
    }
  }
}
```

**Linux/Mac:**
```json
{
  "mcpServers": {
    "ide_memory": {
      "command": "/home/tuusuario/bin/IDE_Memory",
      "args": [
        "--database",
        "/home/tuusuario/ide_memory.db"
      ]
    }
  }
}
```

#### Opción B: Interfaz de Cursor (si está disponible)

1. Abre Cursor IDE
2. Ve a `File` → `Preferences` → `Settings` (o `Ctrl+,` / `Cmd+,`)
3. Busca "MCP" o "Model Context Protocol"
4. Haz clic en "Edit in settings.json"
5. Agrega la configuración como se muestra arriba

### 4. Verificar la Configuración

1. Reinicia Cursor IDE completamente
2. Abre cualquier proyecto
3. En el chat de Cursor, deberías poder usar las herramientas MCP:
   - `mem_search`: Buscar conocimiento
   - `mem_save`: Guardar conocimiento
   - `mem_get_detail`: Obtener detalles
   - `mem_timeline`: Ver timeline

### 5. Probar el Servidor

Puedes probar el servidor manualmente ejecutándolo:

```bash
# Windows
IDE_Memory.exe --database memory.db

# Linux/Mac
./IDE_Memory --database memory.db
```

Luego envía un mensaje JSON-RPC por stdin:
```json
{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}
```

Deberías recibir una respuesta con las capacidades del servidor.

## Ubicación de la Base de Datos

La base de datos SQLite se crea automáticamente en la ubicación especificada con `--database`.

**Recomendaciones:**
- Usa una ruta absoluta para evitar problemas
- Considera usar una ubicación centralizada para compartir memoria entre proyectos
- Ejemplo: `C:\Users\TuUsuario\ide_memory.db` o `~/ide_memory.db`

## Troubleshooting

### El servidor no aparece en Cursor

1. **Verifica la ruta del binario:**
   ```bash
   # Windows
   C:\bin\IDE_Memory.exe --help
   
   # Linux/Mac
   ~/bin/IDE_Memory --help
   ```

2. **Verifica los permisos (Linux/Mac):**
   ```bash
   chmod +x ~/bin/IDE_Memory
   ```

3. **Revisa los logs de Cursor:**
   - Abre la consola de desarrollador (`Help` → `Toggle Developer Tools`)
   - Busca errores relacionados con MCP

4. **Verifica el formato JSON:**
   - Asegúrate de que el JSON en `settings.json` sea válido
   - Usa comillas dobles, no simples
   - Escapa las barras invertidas en Windows (`\\`)

### Error: "command not found"

- Verifica que la ruta del binario sea correcta
- En Windows, asegúrate de incluir `.exe`
- Verifica que el binario exista en esa ubicación

### Error: "database locked" o permisos

- Asegúrate de tener permisos de escritura en la ubicación de la base de datos
- Verifica que no haya otro proceso usando la base de datos
- Intenta usar una ubicación diferente

### El servidor no responde

- Verifica que estés usando el modo `stdio` (default)
- Revisa que el servidor esté recibiendo mensajes (puedes agregar logs)
- Reinicia Cursor completamente

## Configuración Avanzada

### Múltiples Bases de Datos por Proyecto

Puedes configurar diferentes bases de datos para diferentes proyectos usando variables de entorno o configuración por workspace.

### Sincronización entre Máquinas

La base de datos SQLite puede sincronizarse usando:
- Dropbox, Google Drive, OneDrive
- Git (si es pequeño)
- Sincronización manual

**Nota:** SQLite no está diseñado para acceso concurrente desde múltiples máquinas. Si necesitas sincronización, considera un enfoque de "última escritura gana" o implementar un sistema de sincronización más sofisticado.

## Ejemplo de Uso en Cursor

Una vez configurado, puedes usar las herramientas directamente en el chat de Cursor:

```
Usuario: Busca información sobre cómo manejar eventos recurrentes en Google Calendar

Cursor (usando mem_search): Encontré 2 entradas relevantes:
1. [pattern] Manejo de eventos recurrentes - "Para eventos recurrentes, usar el campo recurrence..."
2. [decision] Arquitectura de eventos - "Decidimos usar Google Calendar API para eventos..."

Usuario: Muéstrame más detalles de la primera entrada

Cursor (usando mem_get_detail): [Muestra el contenido completo]
```

## Próximos Pasos

- Usa `mem_save` para guardar conocimiento importante mientras trabajas
- Usa `mem_search` para recuperar contexto relevante antes de empezar a trabajar
- Organiza el conocimiento con tags apropiados
- Usa `project_path` para filtrar conocimiento por proyecto
