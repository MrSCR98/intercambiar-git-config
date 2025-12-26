# 🔄 **INTERCAMBIAR GIT CONFIG**

Programa sencillo para **alternar configuraciones de Git** en Windows, de forma **segura y rápida**.

---

## 💾 **Descarga**

[🔗 **Descargar ejecutable hecho en rust**](https://github.com/MrSCR98/intercambiar-git-config/releases/download/Ejecutable/Intercambiar-Git-Config.exe)

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

## 🎨 Cómo cambiar el icono del ejecutable (opcional)

Si quieres personalizar el icono del archivo .exe:

1. Descarga la herramienta **rcedit**:  
   [Descargar rcedit](https://github.com/electron/rcedit/releases)
2. Crea tu imagen en formato PNG y luego conviértela a .ICO (por ejemplo, 64x64px).
3. Ejecuta el siguiente comando en la misma carpeta donde esté tu .exe e icono:

```
rcedit-x64.exe "intercambiar-git-config.exe" --set-icon "favicon_64x64.ico" --set-file-version "1.0.0" --set-product-version "1.0.0"
```

**Consejo:** Para que el icono se vea bien en todos los tamaños, asegúrate de usar un archivo .ICO que contenga varias resoluciones (16x16, 32x32, 64x64, etc.).

---

🧠 **Resumen rápido:**  
> *Si no hay exactamente 2 archivos válidos, el programa no hace nada.  
> Solo intercambia nombres cuando es seguro y elimina la credencial de GitHub (`git:https://github.com`) automáticamente.*
