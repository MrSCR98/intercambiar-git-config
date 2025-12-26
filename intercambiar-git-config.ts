import { rename, access } from "fs/promises";
import { join } from "path";

/**
 * Verifica si un archivo existe
 */
async function existeArchivo(ruta: string): Promise<boolean> {
  try {
    await access(ruta);
    return true;
  } catch {
    return false;
  }
}

/**
 * Cambia configuraciones solo si hay EXACTAMENTE dos archivos
 */
async function cambiarConfiguracionGit() {
  const carpetaUsuario = process.env.USERPROFILE;

  if (!carpetaUsuario) {
    console.log("❌ No se pudo detectar la carpeta del usuario");
    return;
  }

  const rutaGitconfig = join(carpetaUsuario, ".gitconfig");
  const rutaGitconfig1 = join(carpetaUsuario, ".gitconfig1");
  const rutaGitconfig2 = join(carpetaUsuario, ".gitconfig2");

  const existeBase = await existeArchivo(rutaGitconfig);
  const existeUno = await existeArchivo(rutaGitconfig1);
  const existeDos = await existeArchivo(rutaGitconfig2);

  const cantidadExistentes =
    Number(existeBase) + Number(existeUno) + Number(existeDos);

  console.log(`📁 Usuario: ${carpetaUsuario}`);
  console.log(`📄 Archivos detectados: ${cantidadExistentes}`);

  // Regla principal
  if (cantidadExistentes !== 2) {
    console.log("ℹ️ Regla de seguridad: solo se actúa con EXACTAMENTE 2 archivos");
    return;
  }

  // Casos permitidos
  if (existeBase && existeUno) {
    console.log("🔄 Intercambiando .gitconfig ↔ .gitconfig1");
    await rename(rutaGitconfig, rutaGitconfig2);
    await rename(rutaGitconfig1, rutaGitconfig);
    console.log("✅ Cambio realizado");
    return;
  }

  if (existeBase && existeDos) {
    console.log("🔄 Intercambiando .gitconfig ↔ .gitconfig2");
    await rename(rutaGitconfig, rutaGitconfig1);
    await rename(rutaGitconfig2, rutaGitconfig);
    console.log("✅ Cambio realizado");
    return;
  }

  console.log("ℹ️ Caso no permitido, no se realizó ningún cambio");
}

/**
 * Función principal
 */
async function principal() {
  console.log("\n🔧 Gestor ultra seguro de .gitconfig\n");
  try {
    await cambiarConfiguracionGit();
  } catch (error) {
    console.log("❌ Error inesperado");
    console.error(error);
  }
}

// Ejecutar
principal();
