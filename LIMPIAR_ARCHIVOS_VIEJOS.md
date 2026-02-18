# Limpieza de Archivos Antiguos

## Archivos a Eliminar

En el directorio `d:\Desarrollo\_Editores Code\_IDE-memory\` hay archivos antiguos que ya no se necesitan:

### Archivos Obsoletos

1. **`cursor-memory.db`** - Base de datos antigua (renombrada a `ide_memory.db`)
2. **`cursor-memory_metrics.db`** - Base de datos de métricas antigua (renombrada a `ide_memory_metrics.db`)
3. **`memory_metrics.db`** - Base de datos de métricas de prueba (si no contiene datos importantes)
4. **`ide-memory.db`** - Base de datos con guión medio (renombrada a `ide_memory.db`)
5. **`ide-memory_metrics.db`** - Base de datos de métricas con guión medio (renombrada a `ide_memory_metrics.db`)
6. **`IDE-Memory.exe`** - Binario con guión medio (renombrado a `IDE_Memory.exe`)

### Archivos Actuales (NO ELIMINAR)

✅ **`IDE_Memory.exe`** - Binario actual del servidor MCP
✅ **`ide_memory.db`** - Base de datos principal actual
✅ **`ide_memory_metrics.db`** - Base de datos de métricas actual

## Script de Limpieza (PowerShell)

Ejecuta este script para eliminar los archivos antiguos:

```powershell
cd "d:\Desarrollo\_Editores Code\_IDE-memory"

# Verificar qué archivos se van a eliminar
Write-Host "Archivos que se eliminarán:" -ForegroundColor Yellow
Get-ChildItem -Filter "cursor-memory*" | Select-Object Name
Get-ChildItem -Filter "memory_metrics.db" | Select-Object Name
Get-ChildItem -Filter "ide-memory*" | Select-Object Name
Get-ChildItem -Filter "IDE-Memory.exe" | Select-Object Name

# Confirmar antes de eliminar
$confirm = Read-Host "¿Eliminar estos archivos? (S/N)"
if ($confirm -eq "S" -or $confirm -eq "s") {
    # Eliminar archivos antiguos
    Remove-Item "cursor-memory.db" -ErrorAction SilentlyContinue
    Remove-Item "cursor-memory_metrics.db" -ErrorAction SilentlyContinue
    Remove-Item "memory_metrics.db" -ErrorAction SilentlyContinue
    Remove-Item "ide-memory.db" -ErrorAction SilentlyContinue
    Remove-Item "ide-memory_metrics.db" -ErrorAction SilentlyContinue
    Remove-Item "IDE-Memory.exe" -ErrorAction SilentlyContinue
    
    Write-Host "✅ Archivos antiguos eliminados" -ForegroundColor Green
} else {
    Write-Host "❌ Operación cancelada" -ForegroundColor Red
}
```

## Comandos Manuales

Si prefieres eliminar manualmente:

```powershell
cd "d:\Desarrollo\_Editores Code\_IDE-memory"

# Eliminar base de datos antigua
Remove-Item "cursor-memory.db" -ErrorAction SilentlyContinue

# Eliminar métricas antiguas
Remove-Item "cursor-memory_metrics.db" -ErrorAction SilentlyContinue

# Eliminar métricas de prueba (opcional)
Remove-Item "memory_metrics.db" -ErrorAction SilentlyContinue

# Eliminar archivos con guión medio (versión antigua)
Remove-Item "ide-memory.db" -ErrorAction SilentlyContinue
Remove-Item "ide-memory_metrics.db" -ErrorAction SilentlyContinue
Remove-Item "IDE-Memory.exe" -ErrorAction SilentlyContinue
```

## Verificación

Después de eliminar, verifica que solo queden los archivos actuales:

```powershell
cd "d:\Desarrollo\_Editores Code\_IDE-memory"
Get-ChildItem | Select-Object Name, Length, LastWriteTime
```

**Deberías ver solo:**
- `IDE_Memory.exe`
- `ide_memory.db`
- `ide_memory_metrics.db` (si las métricas están habilitadas)

## Nota Importante

⚠️ **Antes de eliminar**, asegúrate de que:
1. El servidor MCP está funcionando correctamente con `ide_memory.db`
2. No necesitas los datos antiguos de `cursor-memory.db` o `ide-memory.db`
3. Has hecho una copia de seguridad si los datos son importantes

## Migración de Datos (Opcional)

Si necesitas migrar datos de la base de datos antigua a la nueva:

```powershell
# Usar sqlite3 para exportar e importar datos
sqlite3 cursor-memory.db ".dump" | sqlite3 ide_memory.db
```
