use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Verifica si un archivo existe
fn existe_archivo(ruta: &str) -> bool {
    Path::new(ruta).exists()
}

/// Lee la configuración del usuario en .gitconfig
fn mostrar_usuario_git(ruta: &str) {
    match fs::read_to_string(ruta) {
        Ok(contenido) => {
            let nombre = contenido
                .lines()
                .find_map(|line| line.trim_start().strip_prefix("name = "))
                .unwrap_or("Desconocido")
                .trim();

            let email = contenido
                .lines()
                .find_map(|line| line.trim_start().strip_prefix("email = "))
                .unwrap_or("Desconocido")
                .trim();

            println!("📌 Configuración activa: {} <{}>", nombre, email);
        }
        Err(_) => {
            println!("ℹ️ No se pudo leer la configuración de usuario del .gitconfig");
        }
    }
}

/// Elimina la credencial de GitHub en Windows
fn borrar_credencial_github() {
    println!("🗝️ Eliminando credencial git:https://github.com");

    let salida = Command::new("cmd")
        .args(["/C", "cmdkey /delete:git:https://github.com"])
        .output();

    match salida {
        Ok(output) if output.status.success() => println!("✅ Credencial eliminada"),
        _ => println!("ℹ️ La credencial no existía o ya estaba eliminada"),
    }
}

/// Cambia configuraciones solo si hay exactamente dos archivos
fn cambiar_configuracion_git() {
    let carpeta_usuario = match env::var("USERPROFILE") {
        Ok(path) => path,
        Err(_) => {
            println!("❌ No se pudo detectar la carpeta del usuario");
            return;
        }
    };

    let ruta_base = format!("{}\\.gitconfig", carpeta_usuario);
    let ruta_1 = format!("{}\\.gitconfig1", carpeta_usuario);
    let ruta_2 = format!("{}\\.gitconfig2", carpeta_usuario);

    let existe_base = existe_archivo(&ruta_base);
    let existe_1 = existe_archivo(&ruta_1);
    let existe_2 = existe_archivo(&ruta_2);

    let cantidad_existentes = [existe_base, existe_1, existe_2]
        .iter()
        .filter(|&&x| x)
        .count();

    println!("📁 Usuario: {}", carpeta_usuario);
    println!("📄 Archivos detectados: {}", cantidad_existentes);

    if cantidad_existentes != 2 {
        println!("ℹ️ Regla de seguridad: solo se actúa con EXACTAMENTE 2 archivos");
        return;
    }

    // Caso: .gitconfig + .gitconfig1
    if existe_base && existe_1 {
        println!("🔄 Intercambiando .gitconfig ↔ .gitconfig1");

        fs::rename(&ruta_base, &ruta_2)
            .unwrap_or_else(|_| println!("❌ Error renombrando .gitconfig a .gitconfig2"));
        fs::rename(&ruta_1, &ruta_base)
            .unwrap_or_else(|_| println!("❌ Error renombrando .gitconfig1 a .gitconfig"));

        borrar_credencial_github();
        mostrar_usuario_git(&ruta_base);

        println!("✅ Cambio realizado");
        return;
    }

    // Caso: .gitconfig + .gitconfig2
    if existe_base && existe_2 {
        println!("🔄 Intercambiando .gitconfig ↔ .gitconfig2");

        fs::rename(&ruta_base, &ruta_1)
            .unwrap_or_else(|_| println!("❌ Error renombrando .gitconfig a .gitconfig1"));
        fs::rename(&ruta_2, &ruta_base)
            .unwrap_or_else(|_| println!("❌ Error renombrando .gitconfig2 a .gitconfig"));

        borrar_credencial_github();
        mostrar_usuario_git(&ruta_base);

        println!("✅ Cambio realizado");
        return;
    }

    println!("ℹ️ Caso no permitido, no se realizó ningún cambio");
}

/// Función principal
fn main() {
    println!("\n🔧 Gestor ultra seguro de .gitconfig\n");

    if let Err(e) = std::panic::catch_unwind(|| cambiar_configuracion_git()) {
        println!("❌ Error inesperado");
        println!("{:?}", e);
    }
}
