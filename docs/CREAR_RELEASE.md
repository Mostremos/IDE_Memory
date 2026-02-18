# Cómo Crear un Release en GitHub

Guía para crear releases con ejecutables compilados.

## 📦 Qué Incluir en un Release

### ✅ SÍ incluir:
- **Ejecutable compilado**: `IDE_Memory.exe` (Windows)
- **README de instalación**: Instrucciones rápidas
- **Changelog**: Lista de cambios de la versión

### ❌ NO incluir:
- **Bases de datos**: Se crean automáticamente al ejecutar
- **Código fuente**: Ya está en el repositorio
- **Archivos temporales**: `.db`, `.pdb`, etc.

## 🚀 Pasos para Crear un Release

### Paso 1: Compilar el Ejecutable

```bash
cd "D:\Proyectos\AI\Memoria persistente IDEs\IDE-Memory_MCP-Server"
cargo build --release
```

El ejecutable estará en: `target/release/IDE_Memory.exe`

### Paso 2: Preparar el ZIP

Crea una carpeta temporal con:

```
IDE_Memory_v0.1.0/
├── IDE_Memory.exe          # Ejecutable
├── README_INSTALL.md       # Instrucciones de instalación
└── LICENSE                 # Licencia (opcional)
```

**Con PowerShell:**
```powershell
# Crear carpeta temporal
New-Item -ItemType Directory -Path "release_temp" -Force

# Copiar ejecutable
Copy-Item "target\release\IDE_Memory.exe" -Destination "release_temp\IDE_Memory.exe"

# Crear README de instalación
@"
# Instalación Rápida

## Windows

1. Descarga `IDE_Memory.exe`
2. Copia a una ubicación permanente (ej: `C:\bin\`)
3. Configura en Cursor IDE según la documentación del repositorio

## Configuración en Cursor

Agrega a tu `mcp.json`:

\`\`\`json
{
  "mcpServers": {
    "ide_memory": {
      "command": "C:\\ruta\\a\\IDE_Memory.exe",
      "args": ["--database", "C:\\ruta\\a\\ide_memory.db"]
    }
  }
}
\`\`\`

Ver documentación completa en: https://github.com/Mostremos/IDE_Memory
"@ | Out-File -FilePath "release_temp\README_INSTALL.md" -Encoding UTF8

# Crear ZIP
Compress-Archive -Path "release_temp\*" -DestinationPath "IDE_Memory_v0.1.0_Windows.zip" -Force

# Limpiar
Remove-Item -Path "release_temp" -Recurse -Force
```

### Paso 3: Crear el Release en GitHub

1. Ve a tu repositorio: https://github.com/Mostremos/IDE_Memory
2. Click en **"Releases"** (lado derecho, o ve a `/releases`)
3. Click en **"Create a new release"**
4. Configura:
   - **Tag version**: `v0.1.0` (o la versión que corresponda)
   - **Release title**: `IDE Memory v0.1.0`
   - **Description**: 
     ```markdown
     ## 🎉 Primera Release
     
     ### Características
     - Servidor MCP para memoria persistente
     - Progressive Disclosure (3 capas)
     - Sistema de métricas
     - Soporte para Cursor IDE
     
     ### Instalación
     1. Descarga `IDE_Memory_v0.1.0_Windows.zip`
     2. Extrae el contenido
     3. Copia `IDE_Memory.exe` a una ubicación permanente
     4. Configura en Cursor según la documentación
     
     ### Cambios
     - Versión inicial
     - Implementación completa del servidor MCP
     - Sistema de memoria persistente con SQLite
     ```
   - **Attach binaries**: Arrastra el archivo ZIP
5. Click en **"Publish release"**

## 📝 Template de Descripción para Releases

```markdown
## 🎉 IDE Memory v0.1.0

### ✨ Nuevas Características
- [Lista de nuevas funcionalidades]

### 🐛 Correcciones
- [Lista de bugs corregidos]

### 📦 Instalación

**Windows:**
1. Descarga `IDE_Memory_v0.1.0_Windows.zip`
2. Extrae y copia `IDE_Memory.exe` a `C:\bin\` (o tu ubicación preferida)
3. Configura en Cursor IDE según la [documentación](https://github.com/Mostremos/IDE_Memory#configuración-en-cursor-ide)

### 📚 Documentación

Ver [README completo](https://github.com/Mostremos/IDE_Memory) para más detalles.

### 🔗 Enlaces
- [Documentación completa](https://github.com/Mostremos/IDE_Memory)
- [Reportar issues](https://github.com/Mostremos/IDE_Memory/issues)
```

## 🔄 Automatizar con Script

Puedes crear un script para automatizar el proceso:

**`create_release.ps1`:**
```powershell
param(
    [string]$Version = "0.1.0"
)

$ReleaseName = "IDE_Memory_v$Version"
$TempDir = "release_temp"
$ZipFile = "${ReleaseName}_Windows.zip"

# Limpiar
if (Test-Path $TempDir) { Remove-Item $TempDir -Recurse -Force }
if (Test-Path $ZipFile) { Remove-Item $ZipFile -Force }

# Compilar
Write-Host "Compilando..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error al compilar" -ForegroundColor Red
    exit 1
}

# Crear estructura
New-Item -ItemType Directory -Path $TempDir -Force | Out-Null
Copy-Item "target\release\IDE_Memory.exe" -Destination "$TempDir\IDE_Memory.exe"

# Crear README
@"
# IDE Memory v$Version - Instalación Rápida

## Windows

1. Copia \`IDE_Memory.exe\` a una ubicación permanente (ej: \`C:\bin\`)
2. Configura en Cursor IDE según la documentación del repositorio

Ver documentación completa: https://github.com/Mostremos/IDE_Memory
"@ | Out-File -FilePath "$TempDir\README_INSTALL.md" -Encoding UTF8

# Crear ZIP
Write-Host "Creando ZIP..." -ForegroundColor Yellow
Compress-Archive -Path "$TempDir\*" -DestinationPath $ZipFile -Force

# Limpiar
Remove-Item $TempDir -Recurse -Force

Write-Host "✅ Release preparado: $ZipFile" -ForegroundColor Green
Write-Host "Ahora sube $ZipFile a GitHub Releases" -ForegroundColor Cyan
```

**Uso:**
```powershell
.\create_release.ps1 -Version "0.1.0"
```

## 🏷️ Versionado

Usa [Semantic Versioning](https://semver.org/):
- **MAJOR**: Cambios incompatibles (1.0.0 → 2.0.0)
- **MINOR**: Nuevas funcionalidades compatibles (1.0.0 → 1.1.0)
- **PATCH**: Correcciones de bugs (1.0.0 → 1.0.1)

## 📋 Checklist Antes de Publicar

- [ ] Código compilado sin errores
- [ ] Ejecutable probado localmente
- [ ] ZIP creado con solo lo necesario
- [ ] README de instalación incluido
- [ ] Changelog actualizado
- [ ] Tag creado en Git (`git tag v0.1.0`)
- [ ] Tag subido (`git push origin v0.1.0`)
- [ ] Release publicado en GitHub

## 🔗 Eliminar Credenciales de Windows (Para Máquinas Compartidas)

Si necesitas eliminar credenciales guardadas:

1. **Windows Credential Manager**:
   - Windows → Buscar "Credential Manager"
   - Windows Credentials
   - Busca `git:https://github.com`
   - Click en la flecha → Remove

2. **Desde línea de comandos**:
   ```powershell
   cmdkey /list | findstr git
   # Luego eliminar con:
   cmdkey /delete:git:https://github.com
   ```

3. **Git config**:
   ```bash
   git config --global --unset credential.helper
   ```

---

**Nota**: Las bases de datos NO se incluyen porque:
- Se crean automáticamente al ejecutar
- Son específicas de cada usuario
- Pueden ser grandes
- Contienen información personal