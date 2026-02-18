# Solución de Errores de Configuración en Cursor

## Problema: Error de Configuración en Cursor Settings

Si Cursor marca un error en "Settings/Installed MCP Servers/", sigue estos pasos:

## Solución 1: Verificar que el Binario Existe

```powershell
Test-Path "d:\Desarrollo\_Editores Code\_IDE-memory\IDE_Memory.exe"
```

Si retorna `False`, copia el binario:
```powershell
Copy-Item "D:\Proyectos\AI\Memoria persistente IDEs\IDE_Memory_MCP-Server\target\release\IDE_Memory.exe" -Destination "d:\Desarrollo\_Editores Code\_IDE-memory\IDE_Memory.exe" -Force
```

## Solución 2: Probar el Binario Manualmente

```powershell
cd "d:\Desarrollo\_Editores Code\_IDE-memory"
.\IDE_Memory.exe --help
```

Debería mostrar la ayuda. Si hay error, el binario puede estar corrupto o faltar dependencias.

## Solución 3: Verificar el Formato del JSON

Tu `mcp.json` debe verse así:

```json
{
  "mcpServers": {
    "ide_memory": {
      "command": "d:\\Desarrollo\\_Editores Code\\_IDE-memory\\IDE_Memory.exe",
      "args": [
        "--database",
        "d:\\Desarrollo\\_Editores Code\\_IDE-memory\\ide_memory.db"
      ]
    }
  }
}
```

**Puntos importantes:**
- Usa barras dobles `\\` en las rutas de Windows
- O usa barras normales `/` (también funciona en Windows)
- El JSON debe ser válido (verifica con un validador JSON)

## Solución 4: Probar el Servidor MCP Manualmente

```powershell
cd "d:\Desarrollo\_Editores Code\_IDE-memory"
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | .\IDE_Memory.exe --database ide_memory.db
```

**Debería retornar:**
```json
{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"tools":{}},"protocolVersion":"2024-11-05","serverInfo":{"name":"IDE_Memory","version":"0.1.0"}}}
```

**Si hay error:**
- Verifica permisos de escritura en el directorio
- Verifica que la ruta de la base de datos sea correcta
- Revisa que no haya otros procesos usando la base de datos

## Solución 5: Verificar Logs de Cursor

1. Abre Cursor IDE
2. Ve a `Help` → `Toggle Developer Tools`
3. Busca en la consola errores relacionados con MCP
4. Los errores te dirán exactamente qué está mal

## Solución 6: Formato Alternativo de Rutas

Si las barras dobles causan problemas, prueba con barras normales:

```json
{
  "mcpServers": {
    "ide_memory": {
      "command": "d:/Desarrollo/_Editores Code/_IDE-memory/IDE_Memory.exe",
      "args": [
        "--database",
        "d:/Desarrollo/_Editores Code/_IDE-memory/ide_memory.db"
      ]
    }
  }
}
```

## Solución 7: Usar Ruta Absoluta con Comillas

A veces Cursor necesita las rutas entre comillas:

```json
{
  "mcpServers": {
    "ide_memory": {
      "command": "\"d:\\Desarrollo\\_Editores Code\\_IDE-memory\\IDE_Memory.exe\"",
      "args": [
        "--database",
        "d:\\Desarrollo\\_Editores Code\\_IDE-memory\\ide_memory.db"
      ]
    }
  }
}
```

## Solución 8: Verificar Versión del Binario

Asegúrate de usar la versión más reciente compilada:

```powershell
cd "D:\Proyectos\AI\Memoria persistente IDEs\IDE_Memory_MCP-Server"
cargo build --release
Copy-Item "target\release\IDE_Memory.exe" -Destination "d:\Desarrollo\_Editores Code\_IDE-memory\IDE_Memory.exe" -Force
```

## Solución 9: Deshabilitar Métricas Temporalmente

Si las métricas causan problemas, puedes deshabilitarlas:

```json
{
  "mcpServers": {
    "ide_memory": {
      "command": "d:\\Desarrollo\\_Editores Code\\_IDE-memory\\IDE_Memory.exe",
      "args": [
        "--database",
        "d:\\Desarrollo\\_Editores Code\\_IDE-memory\\ide_memory.db",
        "--no-metrics"
      ]
    }
  }
}
```

## Solución 10: Verificar Variables de Entorno

Algunos servidores MCP necesitan variables de entorno. Aunque este no las requiere, verifica que no haya conflictos.

## Pasos de Diagnóstico

1. ✅ Binario existe y es ejecutable
2. ✅ Binario responde a `--help`
3. ✅ Binario responde correctamente a `initialize` via stdio
4. ✅ JSON en `mcp.json` es válido
5. ✅ Rutas son correctas y accesibles
6. ✅ No hay otros procesos usando la base de datos
7. ✅ Cursor se reinició completamente después de cambiar `mcp.json`

## Después de Corregir

1. **Reinicia Cursor completamente** (cierra todas las ventanas)
2. Abre cualquier proyecto
3. Verifica en `Settings/Installed MCP Servers/` que el servidor aparezca sin error
4. Prueba usar las herramientas MCP en el chat

## Si Nada Funciona

1. Verifica la versión de Cursor (debe soportar MCP)
2. Revisa la documentación oficial de MCP en Cursor
3. Prueba con un servidor MCP de ejemplo para verificar que MCP funciona en tu instalación
4. Revisa los logs de Cursor para errores específicos

## Cambios Recientes

**Versión actual** incluye:
- ✅ Sin mensajes a stderr en modo stdio (evita confundir a Cursor)
- ✅ Solo JSON-RPC válido en stdout
- ✅ Manejo correcto de errores sin escribir a stderr

Estos cambios deberían resolver la mayoría de problemas de configuración.
