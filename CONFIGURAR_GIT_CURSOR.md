# Configurar Git en Cursor IDE

Esta guía te muestra cómo configurar Git en Cursor para automatizar tareas de control de versiones.

## 🔧 Configuración Inicial de Git

### Paso 1: Configurar Git Globalmente (si no lo has hecho)

Abre la terminal en Cursor (`Ctrl + Ñ` o `View → Terminal`) y ejecuta:

```bash
git config --global user.name "Mostremos"
git config --global user.email "marcelo_pcsolution@live.com"
```

Verifica la configuración:

```bash
git config --global --list
```

### Paso 2: Configurar Credenciales de GitHub

#### Opción A: Personal Access Token (Recomendado)

1. Ve a GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic)
2. Click en **"Generate new token (classic)"**
3. Configura:
   - **Note**: "Cursor IDE - IDEMemoryMCPServer"
   - **Expiration**: Elige una fecha (o "No expiration" si prefieres)
   - **Scopes**: Marca `repo` (acceso completo a repositorios)
4. Click en **"Generate token"**
5. **Copia el token** (solo se muestra una vez)

Cuando hagas `git push`, usa:
- **Usuario**: `Mostremos`
- **Contraseña**: El token que copiaste (no tu contraseña de GitHub)

#### Opción B: GitHub CLI (Más Automático)

Instala GitHub CLI:

```bash
# Windows (con Chocolatey)
choco install gh

# O descarga desde: https://cli.github.com/
```

Autentica:

```bash
gh auth login
```

Sigue las instrucciones en pantalla.

## 📦 Configurar el Repositorio en Cursor

### Paso 1: Inicializar Git en el Proyecto

1. Abre el proyecto en Cursor: `D:\Proyectos\AI\Memoria persistente IDEs\IDE-Memory_MCP-Server`
2. Abre la terminal (`Ctrl + Ñ`)
3. Ejecuta:

```bash
git init
```

### Paso 2: Agregar el Remote de GitHub

**Primero crea el repositorio en GitHub** (ver `GUIA_GITHUB.md`), luego:

```bash
git remote add origin https://github.com/Mostremos/IDE_Memory.git
```

Verifica:

```bash
git remote -v
```

Deberías ver:
```
origin  https://github.com/Mostremos/IDE_Memory.git (fetch)
origin  https://github.com/Mostremos/IDE_Memory.git (push)
```

## 🎯 Usar Git desde Cursor (Interfaz Visual)

### Panel de Control de Código Fuente

1. **Abrir el panel**: Click en el ícono de Git en la barra lateral izquierda (o `Ctrl + Shift + G`)
2. Verás:
   - **Changes**: Archivos modificados
   - **Staged Changes**: Archivos listos para commit

### Hacer Commit desde Cursor

1. **Ver cambios**: El panel de Source Control muestra todos los archivos modificados
2. **Stage changes**: 
   - Click en el **"+"** al lado de cada archivo para agregarlo
   - O click en **"Stage All Changes"** para agregar todo
3. **Escribir mensaje**: En el campo "Message" arriba
4. **Commit**: Click en el ícono de check (✓) o `Ctrl + Enter`
5. **Push**: Click en el ícono de sincronización (↻) o `Ctrl + Shift + P` → "Git: Push"

### Atajos de Teclado Útiles

- `Ctrl + Shift + G`: Abrir Source Control
- `Ctrl + Enter`: Hacer commit
- `Ctrl + Shift + P`: Abrir Command Palette
  - `Git: Push` - Subir cambios
  - `Git: Pull` - Bajar cambios
  - `Git: Commit` - Hacer commit
  - `Git: Stage All Changes` - Agregar todos los cambios

## 🔄 Flujo de Trabajo Automatizado

### Workflow Básico Diario

1. **Hacer cambios** en el código
2. **Ver cambios** en Source Control (`Ctrl + Shift + G`)
3. **Stage changes** (click en "+" o "Stage All Changes")
4. **Commit** con mensaje descriptivo:
   ```
   Fix: Corregir formato de respuesta tools/list
   Add: Sistema de métricas
   Update: Mejorar documentación
   ```
5. **Push** (click en ↻ o `Ctrl + Shift + P` → "Git: Push")

### Crear Branch para Nueva Feature

1. `Ctrl + Shift + P` → `Git: Create Branch`
2. Escribe el nombre: `feature/nueva-funcionalidad`
3. Trabaja en el branch
4. Commit y push normalmente
5. Cuando termines, crea un Pull Request en GitHub

## ⚙️ Configuración Avanzada en Cursor

### Settings de Git en Cursor

1. `Ctrl + ,` (abrir Settings)
2. Busca "git"
3. Configuraciones útiles:
   - `git.enabled`: true (habilitar Git)
   - `git.autofetch`: true (actualizar automáticamente)
   - `git.confirmSync`: false (no pedir confirmación para push/pull)
   - `git.enableSmartCommit`: true (commit automático al stage)

### Extensiones Útiles (Opcional)

1. `Ctrl + Shift + X` (Extensions)
2. Busca e instala:
   - **GitLens**: Visualización avanzada de Git
   - **Git Graph**: Ver historial visual
   - **Git History**: Ver historial de archivos

## 🚨 Solución de Problemas

### Error: "Authentication failed"

1. Verifica tus credenciales:
   ```bash
   git config --global user.name
   git config --global user.email
   ```

2. Usa Personal Access Token en lugar de contraseña

3. O configura Git Credential Manager:
   ```bash
   git config --global credential.helper manager-core
   ```

### Error: "Repository not found"

- Verifica que el repositorio existe en GitHub
- Verifica la URL del remote:
  ```bash
  git remote -v
  ```

### Error: "Permission denied"

- Verifica que tienes acceso al repositorio en GitHub
- Verifica que el token tiene permisos `repo`

## 📝 Comandos Útiles desde Terminal

Aunque Cursor tiene interfaz visual, a veces es útil usar la terminal:

```bash
# Ver estado
git status

# Ver diferencias
git diff

# Ver historial
git log --oneline

# Ver branches
git branch

# Cambiar de branch
git checkout nombre-branch

# Crear y cambiar a nuevo branch
git checkout -b feature/nueva-funcionalidad

# Subir branch nuevo
git push -u origin nombre-branch
```

## ✅ Checklist de Configuración

- [ ] Git configurado globalmente (nombre y email)
- [ ] Personal Access Token creado en GitHub
- [ ] Repositorio creado en GitHub (`IDE_Memory`)
- [ ] Git inicializado en el proyecto (`git init`)
- [ ] Remote agregado (`git remote add origin ...`)
- [ ] Primer commit hecho
- [ ] Primer push exitoso

## 🎉 ¡Listo!

Una vez configurado, puedes:
- Ver cambios en tiempo real en Source Control
- Hacer commits con un click
- Push automático desde la interfaz
- Crear branches fácilmente
- Ver historial visual

**Consejo**: Haz commits frecuentes con mensajes descriptivos. Es mejor muchos commits pequeños que uno grande.