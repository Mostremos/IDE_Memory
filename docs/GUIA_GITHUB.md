# Guía: Subir Proyecto a GitHub

## 📋 Preparación del Proyecto

### ✅ Archivos ya preparados:
- ✅ `.gitignore` - Configurado para ignorar archivos innecesarios
- ✅ `LICENSE` - Licencia MIT
- ✅ `README.md` - Documentación completa
- ✅ `Cargo.toml` - Configuración del proyecto

## 🚀 Pasos para Subir a GitHub

### Paso 1: Crear el Repositorio en GitHub

1. Ve a [GitHub.com](https://github.com) e inicia sesión
2. Haz clic en el botón **"+"** (arriba a la derecha) → **"New repository"**
3. Configura el repositorio:
   - **Repository name**: `IDE_Memory` (sin guiones medios)
   - **Description**: "MCP Server para memoria persistente en IDEs (Cursor, Claude Code, etc.)"
   - **Visibility**: 
     - ✅ **Public** (recomendado para proyectos open source)
     - ⚠️ **Private** (solo si quieres mantenerlo privado)
   - **NO marques** "Add a README file" (ya tienes uno)
   - **NO marques** "Add .gitignore" (ya tienes uno)
   - **NO marques** "Choose a license" (ya tienes LICENSE)
4. Haz clic en **"Create repository"**

### Paso 2: Inicializar Git Localmente

Abre una terminal en la carpeta del proyecto:

```bash
cd "D:\Proyectos\AI\Memoria persistente IDEs\IDE-Memory_MCP-Server"
```

Inicializa Git (si no está inicializado):

```bash
git init
```

### Paso 3: Agregar Archivos

Agrega todos los archivos al staging:

```bash
git add .
```

Verifica qué archivos se agregaron:

```bash
git status
```

### Paso 4: Hacer el Primer Commit

```bash
git commit -m "Initial commit: IDE-Memory MCP Server v0.1.0"
```

### Paso 5: Conectar con GitHub

GitHub te mostrará comandos después de crear el repositorio. Usa estos:

```bash
git remote add origin https://github.com/Mostremos/IDE_Memory.git
```

(Reemplaza `TU-USUARIO` con tu nombre de usuario de GitHub)

### Paso 6: Subir el Código

```bash
git branch -M main
git push -u origin main
```

Si te pide credenciales:
- **Usuario**: Tu nombre de usuario de GitHub
- **Contraseña**: Usa un **Personal Access Token** (no tu contraseña normal)
  - Cómo crear un token: GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic) → Generate new token

## 📝 Sobre Protección de Nombres

### ¿Se puede "robar" el nombre?

**Respuesta corta**: No hay un registro oficial de nombres de software, pero GitHub protege el nombre del repositorio.

### Protección en GitHub:

1. **Nombre del repositorio**: Una vez que creas `tu-usuario/IDE-Memory_MCP-Server`, ese nombre está "reservado" bajo tu cuenta
   - Otros usuarios no pueden crear un repositorio con el mismo nombre bajo tu cuenta
   - Otros usuarios SÍ pueden crear repositorios con el mismo nombre bajo sus propias cuentas

2. **Nombres de usuario/organización**: Si creas una organización, puedes proteger mejor el nombre

### Lo que SÍ se protege:

- ✅ **Código fuente**: Protegido por copyright automáticamente
- ✅ **Licencia**: La licencia MIT protege tu código (pero permite uso libre)
- ✅ **Repositorio**: El nombre del repositorio está "reservado" en tu cuenta

### Lo que NO se protege:

- ❌ **Idea/concepto**: Cualquiera puede crear algo similar
- ❌ **Nombre del software**: Otros pueden usar nombres similares
- ❌ **Funcionalidad**: No puedes impedir que otros hagan algo similar

### Recomendaciones:

1. **Elige un nombre único**: `IDE-Memory_MCP-Server` es bastante específico
2. **Marca registrada (opcional)**: Si quieres protección legal del nombre, puedes registrar una marca, pero es costoso y no es necesario para proyectos open source
3. **Documentación clara**: Un buen README y documentación ayudan a establecer tu proyecto como la "versión oficial"

## 🔐 Licencia MIT

Tu proyecto usa la licencia MIT, que significa:

- ✅ Otros pueden usar, modificar y distribuir tu código
- ✅ Otros deben incluir tu copyright y la licencia
- ✅ Otros pueden usar tu código en proyectos comerciales
- ✅ No ofreces garantías

**Ventaja**: Es la licencia más popular y aceptada en la comunidad open source.

## 📦 Archivos que NO se suben (gracias a .gitignore):

- `target/` - Archivos compilados
- `*.db` - Bases de datos
- `*.pdb` - Archivos de debug
- Archivos temporales y de configuración local

## 🎯 Próximos Pasos Después de Subir:

1. **Agregar badges** al README (opcional):
   ```markdown
   ![License](https://img.shields.io/badge/license-MIT-blue.svg)
   ```

2. **Crear releases** cuando tengas versiones estables:
   - GitHub → Releases → Create a new release
   - Sube los binarios compilados (IDE_Memory.exe, etc.)

3. **Agregar issues y pull requests** para colaboración

4. **Agregar GitHub Actions** para CI/CD (opcional)

## 💡 Consejos:

- **Commits descriptivos**: Usa mensajes claros como "Fix: Corregir formato de tools/list"
- **Branches**: Usa branches para features nuevas (`git checkout -b feature/nueva-funcionalidad`)
- **Tags**: Marca versiones importantes (`git tag v0.1.0`)

## ❓ ¿Problemas?

Si tienes errores al hacer `git push`:
- Verifica que tengas acceso a internet
- Verifica tus credenciales de GitHub
- Asegúrate de haber creado el repositorio en GitHub primero

---

**¡Listo!** Tu proyecto estará disponible en `https://github.com/Mostremos/IDE_Memory`