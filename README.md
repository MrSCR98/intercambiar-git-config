# 🔄 **INTERCAMBIAR GIT CONFIG**

Programa sencillo para **alternar configuraciones de Git** en Windows, de forma **segura y rápida**.

---

## 💾 **Descarga**

[🔗 **Descargar ejecutable**](https://github.com/MrSCR98/eliminar-cache-windows/releases/download/Ejecutable/borrar-cache.exe)

---

## ⚙️ **Cómo usar el programa**

### 1️⃣ Archivos necesarios

Debes tener **exactamente 2 archivos** de configuración de Git en tu carpeta de usuario:

```cmd
C:\Users\TU_USUARIO
```

Archivos válidos:
- **`.gitconfig`** → configuración activa  
- **`.gitconfig1`** o **`.gitconfig2`** → configuración alternativa  

---

### 2️⃣ Ejemplo de contenido de un `.gitconfig`

Ejemplo básico de configuración:

```ini
[user]
    name = TU_NOMBRE
    email = ejemplo@ejemplo.com
```

Cada archivo puede tener **usuarios, emails o configuraciones distintas**.

---

### 3️⃣ Ejecutar el programa

Ejecuta el programa:
- Haciendo **doble clic**
- O desde CMD / PowerShell

```cmd
intercambiar-git-config.exe
```

---

### 4️⃣ Qué ocurre al ejecutarlo

#### ✅ Casos que **sí funcionan**
- `.gitconfig` + `.gitconfig1` → se intercambian los nombres  
- `.gitconfig` + `.gitconfig2` → se intercambian los nombres  

Además, **el programa elimina automáticamente la credencial de GitHub** almacenada en Windows (`git:https://github.com`) para evitar que Git use credenciales antiguas.

> ⚠️ Ten en cuenta que al cambiar de usuario de Git, **cualquier acción que requiera autenticación** (push, pull, clone privado, etc.) te pedirá iniciar sesión nuevamente o introducir un token de acceso.

---

#### ❌ Casos en los que **NO pasa nada**
- No existe ningún archivo  
- Solo existe **1 archivo**  
- Existen **3 o más archivos**  
- Existen `.gitconfig1` y `.gitconfig2` pero **no** `.gitconfig`  

> 🔒 El programa **nunca borra, sobrescribe ni modifica** el contenido de los archivos.  
> Solo cambia los nombres cuando es **seguro**.

---

### 5️⃣ Cómo quitar la credencial de GitHub manualmente (opcional)

Si quieres hacerlo tú mismo sin ejecutar el programa:

1. Abre **Panel de control**  
2. Ve a **Cuentas de usuario → Administrar credenciales**  
3. Selecciona **Credenciales de Windows**  
4. Busca la entrada `git:https://github.com`  
5. Haz clic en **Quitar**

> ⚠️ Esto es exactamente lo que hace el programa automáticamente después de intercambiar las configuraciones.  

---

## 💻 **Uso para desarrolladores / pruebas**

Si quieres probar o modificar el script, necesitas **Bun**:

### 1️⃣ Descarga el proyecto
Clona o descarga los archivos del repositorio.

### 2️⃣ Instala Bun
Sigue las instrucciones oficiales de [Bun](https://bun.com/) para tu sistema.

### 3️⃣ Abre el proyecto y descarga dependencias
```cmd
bun install
```

### 4️⃣ Ejecutar en modo desarrollo
```cmd
bun run dev
```

> Esto mostrará los **logs en tiempo real** y la ventana no se cierra automáticamente, perfecto para depurar.

### 5️⃣ Crear el ejecutable optimizado para Windows
```cmd
bun run exe
```

> Esto genera el `.exe` listo para uso normal.

---

🧠 **Resumen rápido:**  
> *Si no hay exactamente 2 archivos válidos, el programa no hace nada.  
> Solo intercambia nombres cuando es seguro y elimina la credencial de GitHub (`git:https://github.com`) automáticamente.*
