# Cómo Vincular GitHub a Cursor

Guía paso a paso para vincular tu cuenta de GitHub con Cursor IDE.

## 🔐 Opción 1: Personal Access Token (Recomendada para empezar)

### Paso 1: Crear Token en GitHub

1. Ve a [GitHub.com](https://github.com) e inicia sesión
2. Click en tu avatar (arriba derecha) → **Settings**
3. En el menú lateral izquierdo: **Developer settings**
4. Click en **Personal access tokens** → **Tokens (classic)**
5. Click en **Generate new token** → **Generate new token (classic)**
6. Configura el token:
   - **Note**: `Cursor IDE - IDE_Memory`
   - **Expiration**: Elige una fecha (o "No expiration" si prefieres)
   - **Scopes**: Marca **`repo`** (acceso completo a repositorios)
     - Esto incluye: `repo:status`, `repo_deployment`, `public_repo`, `repo:invite`, `security_events`
7. Click en **Generate token**
8. **⚠️ IMPORTANTE**: Copia el token inmediatamente (solo se muestra una vez)
   - Ejemplo: `ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx`

### Paso 2: Configurar Git en Cursor

Abre la terminal en Cursor (`Ctrl + Ñ`) y ejecuta:

```bash
git config --global user.name "Mostremos"
git config --global user.email "marcelo_pcsolution@live.com"
```

Verifica:

```bash
git config --global --list
```

### Paso 3: Usar el Token

Cuando hagas `git push` por primera vez:

1. Cursor te pedirá credenciales
2. **Usuario**: `Mostremos`
3. **Contraseña**: Pega el token que copiaste (NO tu contraseña de GitHub)

**Nota**: Windows puede guardar las credenciales automáticamente. Si quieres guardarlas manualmente:

```bash
git config --global credential.helper manager-core
```

---

## 🚀 Opción 2: GitHub CLI (Más Automático)

### Paso 1: Instalar GitHub CLI

**Windows (con Chocolatey):**
```bash
choco install gh
```

**O descarga manual:**
- Ve a: https://cli.github.com/
- Descarga e instala el instalador para Windows

### Paso 2: Autenticar

Abre la terminal en Cursor y ejecuta:

```bash
gh auth login
```

Sigue las instrucciones:
1. **¿Qué cuenta quieres usar?** → GitHub.com
2. **¿Cuál es tu protocolo preferido?** → HTTPS
3. **¿Autenticar Git con tus credenciales de GitHub?** → Yes
4. **¿Cómo prefieres autenticar?** → Login with a web browser
5. Se abrirá el navegador → Click en **Authorize**
6. Copia el código que aparece → Pégalo en la terminal

### Paso 3: Verificar

```bash
gh auth status
```

Deberías ver:
```
✓ Logged in to github.com as Mostremos
```

### Paso 4: Configurar Git (si no lo has hecho)

```bash
git config --global user.name "Mostremos"
git config --global user.email "marcelo_pcsolution@live.com"
```

---

## 🔌 Opción 3: Extensión de GitHub en Cursor

### Paso 1: Instalar Extensión

1. En Cursor: `Ctrl + Shift + X` (abrir Extensions)
2. Busca: **"GitHub"** (la oficial de GitHub)
3. Click en **Install**

### Paso 2: Autenticar

1. `Ctrl + Shift + P` (Command Palette)
2. Escribe: `GitHub: Sign in`
3. Selecciona la opción
4. Se abrirá el navegador para autorizar
5. Click en **Authorize**

### Paso 3: Verificar

1. `Ctrl + Shift + P`
2. Escribe: `GitHub: Show Output`
3. Deberías ver tu usuario autenticado

---

## ⚙️ Opción 4: Git Credential Manager (Windows)

Si tienes Git para Windows instalado, puedes usar el Credential Manager:

```bash
git config --global credential.helper manager-core
```

Luego, cuando hagas `git push`:
- Se abrirá una ventana de Windows para autenticar
- Ingresa tu usuario y contraseña de GitHub (o token)

---

## ✅ Verificar que Funciona

Después de configurar cualquiera de las opciones:

### Verificar Configuración Git

```bash
git config --global --list
```

Deberías ver:
```
user.name=Mostremos
user.email=marcelo_pcsolution@live.com
```

### Verificar Autenticación GitHub (si usas GitHub CLI)

```bash
gh auth status
```

### Probar con tu Repositorio

```bash
cd "D:\Proyectos\AI\Memoria persistente IDEs\IDE-Memory_MCP-Server"
git remote -v
```

Deberías ver:
```
origin  https://github.com/Mostremos/IDE_Memory.git (fetch)
origin  https://github.com/Mostremos/IDE_Memory.git (push)
```

### Probar Push

```bash
git add .
git commit -m "Test: Verificar conexión GitHub"
git push
```

Si funciona, verás tus cambios en GitHub.

---

## 🎯 Recomendación

**Para empezar rápido**: Usa **Opción 1 (Personal Access Token)**
- Es la más simple
- Funciona inmediatamente
- No requiere instalar nada adicional

**Para automatización**: Usa **Opción 2 (GitHub CLI)**
- Más cómodo a largo plazo
- Integración mejor con GitHub
- Permite hacer más cosas desde la terminal

**Para integración visual**: Usa **Opción 3 (Extensión GitHub)**
- Interfaz visual en Cursor
- Ver issues, PRs, etc. desde el editor

---

## 🚨 Solución de Problemas

### Error: "Authentication failed"

1. Verifica que el token tenga permisos `repo`
2. Verifica que no haya expirado
3. Intenta crear un nuevo token

### Error: "Permission denied"

1. Verifica que tengas acceso al repositorio
2. Verifica que el token tenga el scope `repo`
3. Verifica que el repositorio exista en GitHub

### Error: "Repository not found"

1. Verifica que el repositorio existe: https://github.com/Mostremos/IDE_Memory
2. Verifica la URL del remote: `git remote -v`
3. Verifica que tengas acceso al repositorio

### Windows no guarda las credenciales

```bash
git config --global credential.helper manager-core
```

O guarda manualmente en Windows Credential Manager:
1. Windows → Buscar "Credential Manager"
2. Windows Credentials
3. Agregar credencial genérica
4. Internet o red: `git:https://github.com`
5. Usuario: `Mostremos`
6. Contraseña: Tu token

---

## 📝 Notas Importantes

- **Nunca compartas tu token**: Es como una contraseña
- **Si pierdes el token**: Crea uno nuevo y revoca el anterior
- **Tokens expiran**: Configura una fecha de expiración razonable
- **Scope mínimo**: Solo marca `repo` si solo necesitas acceso a repositorios

---

**¡Listo!** Una vez configurado, podrás hacer push/pull desde Cursor sin problemas.