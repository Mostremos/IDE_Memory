# IDE-Memory MCP Server

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)

Servidor MCP (Model Context Protocol) para dotar de memoria persistente a cualquier IDE (Cursor, Claude Code, OpenCode, etc.).

## ¿Qué es esto?

Este servidor MCP permite que IDEs como Cursor, Claude Code, OpenCode, etc. tengan acceso a memoria persistente entre sesiones. En lugar de perder todo el contexto cuando cierras una conversación, el conocimiento importante se guarda y puede recuperarse cuando lo necesites.

## Características

- ✅ **Memoria Persistente**: Guarda conocimiento importante entre sesiones
- ✅ **Progressive Disclosure**: 3 capas de búsqueda eficiente (resumen → timeline → detalle)
- ✅ **Búsqueda Full-Text**: Usa SQLite FTS5 para búsquedas rápidas y relevantes
- ✅ **Tipos de Conocimiento**: Decisiones, bugs resueltos, patrones, configuraciones, contexto, resúmenes
- ✅ **Ahorro de Tokens**: Solo carga lo que necesitas, cuando lo necesitas (~97% de ahorro)
- ✅ **Sistema de Métricas**: Registro automático de uso y rendimiento

## Tecnologías

Este proyecto está construido con:

- **Rust**: Lenguaje de programación principal
- **SQLite** (rusqlite): Base de datos embebida con soporte FTS5
- **Tokio**: Runtime asíncrono
- **Serde/Serde JSON**: Serialización JSON para el protocolo MCP
- **Clap**: Parsing de argumentos de línea de comandos

**Nota**: Este es un servidor CLI que se ejecuta en segundo plano. No usa Tauri ni ninguna interfaz gráfica. Se comunica con el IDE mediante JSON-RPC sobre stdio (stdin/stdout).

## Instalación

### Requisitos

- Rust 1.70+ (`rustup install stable`)
- Cargo (incluido con Rust)

### Compilar

```bash
git clone https://github.com/Mostremos/IDE_Memory.git
cd IDE_Memory
cargo build --release
```

El binario estará en `target/release/IDE_Memory` (o `target/release/IDE_Memory.exe` en Windows).

## Configuración en Cursor IDE

### Paso 1: Preparar el servidor

1. Compila el proyecto:
   ```bash
   cargo build --release
   ```

2. Copia el binario a una ubicación permanente (ej: `~/bin/IDE_Memory` o `C:\bin\IDE_Memory.exe`)

3. Hazlo ejecutable (Linux/Mac):
   ```bash
   chmod +x ~/bin/IDE_Memory
   ```

### Paso 2: Configurar Cursor

1. Abre Cursor IDE

2. Ve a Configuración → MCP Servers (o busca "MCP" en configuración)

3. Agrega un nuevo servidor MCP con esta configuración:

**Windows:**
```json
{
  "mcpServers": {
    "ide_memory": {
      "command": "C:\\ruta\\a\\IDE_Memory.exe",
      "args": ["--database", "C:\\ruta\\a\\ide_memory.db"]
    }
  }
}
```

**Linux/Mac:**
```json
{
  "mcpServers": {
    "ide_memory": {
      "command": "/ruta/a/IDE_Memory",
      "args": ["--database", "/ruta/a/ide_memory.db"]
    }
  }
}
```

### Paso 3: Reiniciar Cursor

Reinicia Cursor IDE para que cargue el servidor MCP.

## Uso

Una vez configurado, el servidor expone estas herramientas MCP:

### `mem_search`

Busca conocimiento relevante usando búsqueda compacta (Progressive Disclosure Capa 1).

**Parámetros:**
- `query` (string, requerido): Consulta de búsqueda
- `limit` (integer, opcional): Número máximo de resultados (default: 5)

**Ejemplo:**
```json
{
  "name": "mem_search",
  "arguments": {
    "query": "cómo manejar eventos recurrentes",
    "limit": 3
  }
}
```

**Respuesta:**
```json
[
  {
    "id": 1,
    "knowledge_type": "pattern",
    "title": "Manejo de eventos recurrentes en Google Calendar",
    "summary": "Para eventos recurrentes, usar el campo recurrence...",
    "tags": ["calendar", "recurrence"],
    "relevance_score": 0.95
  }
]
```

### `mem_save`

Guarda una entrada de conocimiento en la memoria persistente.

**Parámetros:**
- `knowledge_type` (string, requerido): "decision", "bugfix", "pattern", "config", "context", o "summary"
- `title` (string, requerido): Título de la entrada
- `content` (string, requerido): Contenido completo
- `summary` (string, requerido): Resumen compacto (~100 tokens)
- `tags` (array de strings, opcional): Tags para categorización
- `project_path` (string, opcional): Ruta del proyecto

**Ejemplo:**
```json
{
  "name": "mem_save",
  "arguments": {
    "knowledge_type": "decision",
    "title": "Usar SQLite para memoria persistente",
    "content": "Decidimos usar SQLite porque...",
    "summary": "SQLite elegido para memoria persistente por ser ligero y sin dependencias externas.",
    "tags": ["arquitectura", "sqlite"],
    "project_path": "/ruta/al/proyecto"
  }
}
```

### `mem_get_detail`

Obtiene el detalle completo de una entrada (Progressive Disclosure Capa 3).

**Parámetros:**
- `id` (integer, requerido): ID de la entrada

### `mem_timeline`

Obtiene el timeline de una entrada (Progressive Disclosure Capa 2).

**Parámetros:**
- `id` (integer, requerido): ID de la entrada

## Arquitectura

```
┌─────────────────┐
│   Cursor IDE    │
│  (cualquier     │
│   proyecto)     │
└────────┬────────┘
         │ MCP Protocol (JSON-RPC sobre stdio)
         │
┌────────▼────────────────────┐
│  IDE_Memory (Rust CLI)      │
│  - Lee de stdin             │
│  - Escribe a stdout         │
│  - Expone herramientas MCP │
│  - Maneja memoria           │
└────────┬────────────────────┘
         │
┌────────▼────────┐
│  SQLite DB      │
│  (ide_memory.db)│
│  - FTS5 index   │
│  - Timeline      │
└─────────────────┘
```

### Comunicación

El servidor se comunica con el IDE mediante:
- **Entrada**: JSON-RPC por `stdin`
- **Salida**: JSON-RPC por `stdout`
- **Sin red**: Todo es local, sin puertos ni HTTP

## Progressive Disclosure

El sistema usa Progressive Disclosure en 3 capas para ahorrar tokens:

1. **Capa 1 (mem_search)**: Solo resúmenes (~100 tokens cada uno)
2. **Capa 2 (mem_timeline)**: Timeline de eventos relacionados
3. **Capa 3 (mem_get_detail)**: Contenido completo (solo si realmente lo necesitas)

Esto permite ahorrar ~97% de tokens comparado con cargar todo el contexto siempre.

## Base de Datos

La base de datos SQLite se crea automáticamente en la ubicación especificada con `--database`. 

**Estructura:**
- `knowledge`: Entradas principales de conocimiento
- `knowledge_fts`: Índice FTS5 para búsqueda rápida
- `knowledge_timeline`: Historial de eventos por entrada

## Métricas

El servidor incluye un sistema de métricas opcional que registra:

- Tiempo de respuesta por solicitud
- Tamaño de respuestas
- Tasa de éxito/error
- Estadísticas por herramienta

Ver métricas:
```bash
IDE_Memory.exe --database ide_memory.db --stats
```

## Desarrollo

### Estructura del Proyecto

```
IDE_Memory/
├── src/
│   ├── main.rs          # Punto de entrada, manejo de argumentos CLI
│   ├── memory.rs        # Lógica de memoria persistente (SQLite)
│   ├── mcp_server.rs    # Implementación del protocolo MCP (JSON-RPC)
│   └── metrics.rs       # Sistema de métricas
├── Cargo.toml
├── LICENSE
└── README.md
```

### Ejecutar en modo desarrollo

```bash
cargo run -- --database memory.db
```

### Ejecutar tests

```bash
cargo test
```

### Comandos disponibles

```bash
# Ver ayuda
IDE_Memory.exe --help

# Especificar base de datos
IDE_Memory.exe --database /ruta/a/ide_memory.db

# Ver estadísticas
IDE_Memory.exe --database ide_memory.db --stats

# Deshabilitar métricas
IDE_Memory.exe --database ide_memory.db --no-metrics
```

## Troubleshooting

### El servidor no aparece en Cursor

1. Verifica que el binario existe y es ejecutable
2. Verifica la ruta en la configuración de Cursor
3. Revisa los logs de Cursor para errores
4. Reinicia Cursor completamente

### Error de base de datos

Asegúrate de que la ruta especificada con `--database` sea accesible y que tengas permisos de escritura.

### El servidor no responde

Verifica que estés usando el modo `stdio` (default). El modo HTTP aún no está implementado.

### Error de autenticación en GitHub

Si estás clonando el repositorio, asegúrate de tener configurado Git correctamente. Ver `CONFIGURAR_GIT_CURSOR.md` para más detalles.

## Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Fork el repositorio
2. Crea un branch para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push al branch (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

## Inspiración y Agradecimientos

Este proyecto fue inspirado por varias ideas y conversaciones de la comunidad de desarrolladores:

### Inspiración Principal

- **[Engram](https://github.com/Gentleman-Programming/engram)**: Concepto original de memoria persistente para IDEs. Este proyecto adapta la idea para funcionar como servidor MCP independiente usando Rust y SQLite.

### Comunidad y Conversaciones

- **Discusiones en Twitter/X**: Las conversaciones sobre la necesidad de memoria persistente en IDEs y cómo manejar el límite de contexto inspiraron este proyecto. Especialmente la [discusión sobre memoria persistente en IDEs](https://x.com/G_Programming/status/2023656851531854236) que motivó la creación de este servidor MCP.
- **Comunidad de desarrolladores**: Todas las personas que compartieron sus ideas, experiencias y necesidades sobre memoria persistente en IDEs.

### Agradecimientos

Agradecemos a:
- Todos los desarrolladores que comparten conocimiento públicamente
- La comunidad open source que hace crecer el ecosistema
- Quienes discuten y proponen soluciones a problemas comunes
- Los creadores de herramientas que inspiran nuevas implementaciones

**Gracias por hacer crecer la comunidad y compartir conocimiento. Este proyecto existe gracias a esas conversaciones e ideas compartidas.**

## Autor

**Mostremos**

- GitHub: [@Mostremos](https://github.com/Mostremos)
- Email: marcelo_pcsolution@live.com

---

⭐ Si este proyecto te resulta útil, considera darle una estrella en GitHub.