<p align="center">
  <img src="images/EH3D_BANNER.svg" alt="EH3D-MOTIONCNC banner" width="100%">
</p>

# 🧭 EH3D-MOTIONCNC

<p align="center"><a href="README.md">🇺🇸 English</a> | 🇪🇸 <b>Español</b> | <a href="README_fra.md">🇫🇷 Français</a> | <a href="README_ita.md">🇮🇹 Italiano</a> | <a href="README_deu.md">🇩🇪 Deutsch</a> | <a href="README_zho.md">🇨🇳 中文</a> | <a href="README_jpn.md">🇯🇵 日本語</a> | <a href="README_por.md">🇵🇹 Português</a></p>

### La base compartida de los programas de control de máquina de EH3D: la vista de un trabajo, los temas y los textos de la interfaz en ocho idiomas

<p align="center">
  <img src="https://img.shields.io/badge/License-GPL%203.0--or--later-blue.svg" alt="GPL 3.0 or later">
  <img src="https://img.shields.io/badge/Stack-Rust-DEA584.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Languages-8-00E5FF.svg" alt="Languages">
  <img src="https://img.shields.io/badge/Version-0.0.2-0f6cbd.svg" alt="Version">
  <img src="https://img.shields.io/badge/Maturity-scaffolding-FFB020.svg" alt="Maturity">
  <img src="https://img.shields.io/badge/Tests-0-lightgrey.svg" alt="Tests">
</p>

---

**Comprobación de honestidad - lo que se ejecuta hoy:** un crate de Rust que usa la vista de un trabajo de `eh3d-view` de EH3D-COMMON, donde se comparte con los demás programas de la suite y tiene sus pruebas. Esa vista tiene la escena de un trabajo sobre la mesa de trabajo (cada agujero con su tamaño de broca, las trayectorias de fresado, cuánto lleva hecho el trabajo, un trabajo de taladrado y uno de fresado superpuestos), la vista en planta desde arriba y la vista isométrica de la máquina ajustada al tamaño de la mesa, una comprobación que dice qué no cabe en la mesa antes de empezar un trabajo, cinco temas (Forge, Neon Arcade, Glass Pro, el gris clásico de Windows y uno que sigue los colores de Windows) y los textos de la interfaz en ocho idiomas. **Todavía no hay ventana ni control de máquina**, nada de esto se ha ejecutado en una máquina y este repositorio no tiene pruebas propias: solo reexporta la vista.

---

> **Estado de seguridad (safety) y de seguridad informática:** este repositorio no es un sistema de seguridad ni maneja ninguna máquina. No abre ningún puerto serie ni conexión de red y no lee ficheros de otras personas. Cuando se añada el control de máquina tendrá que demostrarse primero en un simulador y luego en una máquina real antes de llamarse funcional, y los finales de carrera y una parada de emergencia siguen siendo cosa de la propia unidad de control de la máquina.

## 📋 De un vistazo

| | |
|---|---|
| Versión | `0.0.2` |
| Madurez | Andamiaje |
| Pruebas | 0 |
| Pila tecnológica | Rust |
| En hardware real | No: nada aquí se conecta a una máquina |

**Ir a:** [Resumen](#-resumen) · [Arquitectura](#-arquitectura) · [Inicio rápido](#-inicio-rápido) · [Pruebas](#-pruebas) · [Estructura](#-estructura-del-repositorio) · [Relacionados](#-proyectos-relacionados)

## 🚦 Estado del proyecto

Cada línea es una parte de este repositorio y la luz dice hasta dónde llega hoy. Se escribe con lo que el repositorio puede demostrar (pruebas, compilaciones, ejecuciones), no con lo previsto.

🟢 Hecho y verificado · 🟡 Funciona, con límites conocidos · 🟠 Hecho a medias o sin verificar · 🔴 Sin hacer o sin funcionar

| Área | Estado | Qué significa aquí |
|---|---|---|
| Biblioteca base | 🟡 | Un crate de Rust que reexporta `scene`, `texts` y `theme` de `eh3d-view` con el nombre de este programa; necesita la carpeta de EH3D-COMMON al lado. |
| Escena y vistas | 🟡 | Escritas y probadas en un ordenador en EH3D-COMMON: el trabajo sobre la mesa, las vistas en planta e isométrica, la comprobación de lo que no cabe; todavía ninguna ventana las dibuja. |
| Temas y textos | 🟡 | Cinco temas con comprobación de contraste y los textos de la interfaz en ocho idiomas, probados en EH3D-COMMON; aún no vistos en una ventana. |
| Pruebas | 🟠 | Este repositorio tiene 0 pruebas propias (`cargo test`): la vista se prueba donde vive, en EH3D-COMMON. |
| Ventana y jog | 🔴 | Sin empezar. |
| Control de máquina | 🔴 | Sin empezar: ninguna conexión a una máquina, nada de G-code, ningún trabajo de taladrado o fresado se ejecuta desde aquí. |
| Documentación | 🟡 | README en ocho idiomas, las dos imágenes y el registro de cambios; hay poco que documentar hasta que exista el programa. |

## 🎯 Resumen

* La **biblioteca base** (`src/lib.rs`): reexporta `scene`, `texts` y `theme` de `eh3d-view` de EH3D-COMMON, para que el futuro programa los use con su propio nombre y todos los programas de la suite muestren un trabajo igual.
* La **escena** de un trabajo sobre la mesa: agujeros por tamaño de broca, trayectorias de fresado, el avance del trabajo, un trabajo de taladrado y otro de fresado superpuestos, la vista en planta desde arriba, la vista isométrica de la máquina ajustada al tamaño de la mesa y una comprobación de lo que no cabe en la mesa.
* Los **temas**: Forge (el predeterminado), Neon Arcade, Glass Pro, el gris clásico de Windows y uno que sigue los colores de Windows; cada usuario puede escribir el suyo en un fichero de texto.
* Los **textos de la interfaz** en inglés, español, italiano, alemán, francés, portugués, chino y japonés.
* Lo que será: un programa para taladrar, fresar y hacer CNC 2D y 3D, construido sobre esta base, sobre el ZIP de trabajos de EH3D-COMMON (`eh3d-archive`) y sobre los motores de EH3D-DRILLPRO y EH3D-ROUTEPRO. **Nada de eso existe todavía.**

## 🧩 Papel en EH3D

EH3D-MOTIONCNC es el futuro programa de control de máquina de EH3D y, hoy, solo su base. Depende por ruta de EH3D-COMMON (la carpeta `EH3D-COMMON` debe estar al lado de esta) y todavía no depende de ningún otro repositorio de EH3D.

## 🏛️ Arquitectura

<p align="center">
  <img src="images/architecture.svg" alt="Arquitectura de EH3D-MOTIONCNC: quién lo usará, qué contiene y en qué se apoya" width="100%">
</p>

Cómo encajan las partes: quién lo usará, qué contiene este repositorio y en qué se apoya. La imagen se dibuja a partir de `docs/architecture.json`, se comprueba contra el código y marca como futuro lo que es futuro.

## 🔒 Seguridad y protección

Este repositorio no abre ningún puerto serie, ninguna conexión de red ni ficheros de otras personas; el único texto externo que leen sus partes es un fichero de tema escrito por el usuario, que se comprueba en lugar de fiarse. No maneja una máquina y no es un sistema de seguridad. Véase [SECURITY.md](SECURITY.md).

## 🚀 Inicio rápido

Necesitas una cadena de herramientas estable de Rust y la carpeta `EH3D-COMMON` al lado de esta.

```bash
cargo build --release
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all --check
python -I tools/ci_validate.py
```

`build.bat` / `build.sh` compilan la biblioteca, `test.bat` / `test.sh` ejecutan las pruebas y `run.bat` / `run.sh` avisan de que aún no hay programa y ejecutan las pruebas. Los scripts esperan una tecla al final salvo que se defina `NOPAUSE`.

## 🧪 Pruebas

`cargo test` informa de 0 pruebas aquí: este repositorio solo reexporta la vista, cuyas pruebas están en EH3D-COMMON. El CI también ejecuta `cargo fmt --check` y `cargo clippy` con las advertencias como errores, y `python -I tools/ci_validate.py` comprueba el manifiesto, las versiones, el registro de cambios y los ocho README.

## 🏗️ Compilación y publicación

La versión vive en `Cargo.toml` y en `eh3d.project.json`, y cada versión tiene su entrada en [CHANGELOG.md](CHANGELOG.md). Todavía no hay programa que empaquetar, así que no hay binarios ya hechos.

## 📂 Estructura del repositorio

<p align="center">
  <img src="images/repository-structure.svg" alt="Estructura del repositorio de EH3D-MOTIONCNC" width="100%">
</p>

<details>
<summary>Versión en texto del árbol</summary>

```text
EH3D-MOTIONCNC/
├── src/
│   └── lib.rs                                   reexporta escena, textos y tema de eh3d-view de EH3D-COMMON
├── tools/                                       comprobación de CI, del README y las herramientas de imágenes
├── docs/                                        las fuentes de las imágenes de Arquitectura y Estructura del repositorio
├── .github/                                     flujo de CI, plantillas de issues y pull requests
├── images/                                      banner y las imágenes de Arquitectura y Estructura del repositorio
├── build.sh build.bat                           compilan la biblioteca
├── run.sh run.bat                               avisan de que aún no hay programa y ejecutan las pruebas
├── eh3d.project.json                            manifiesto: versión, rol, madurez y comando de compilación
├── Cargo.toml Cargo.lock                        el paquete de Rust y sus dependencias fijadas
├── CHANGELOG.md                                 qué cambió en cada versión
├── SECURITY.md SUPPORT.md                       cómo informar de una vulnerabilidad y dónde preguntar
└── CONTRIBUTING.md CODE_OF_CONDUCT.md LICENSE   cómo contribuir, el código de conducta y la licencia
```

</details>

## 🗺️ Hoja de ruta y límites conocidos

**Límites conocidos hoy**

* No hay ventana ni control de máquina: nada de esto se ha ejecutado en una máquina.
* No hay pruebas en este repositorio; la vista se prueba en EH3D-COMMON.
* Necesita la carpeta `EH3D-COMMON` al lado, porque depende de ella por ruta.

**Siguiente**

* Estudiar los programas de control de máquina que existen hoy (uno nativo y otro .NET) para decidir qué necesita el operador.
* Una ventana con la escena, los temas y los textos de la interfaz.
* Jog y G-code: conexión con una máquina, primero en un simulador.
* Vistas 2D y 3D de un trabajo en marcha.
* Unir los motores de EH3D-DRILLPRO y EH3D-ROUTEPRO para que un solo programa taladre y fresé.

## 🌍 Idiomas

README y textos existen en inglés, español, francés, italiano, alemán, chino, japonés y portugués de Portugal.

## 🔗 Proyectos relacionados

* [EH3D-COMMON](https://github.com/JuanenRac/EH3D-COMMON): donde viven la vista (`eh3d-view`) y el ZIP de trabajos (`eh3d-archive`)
* [EH3D-DRILLPRO](https://github.com/JuanenRac/EH3D-DRILLPRO): el programa de taladrado cuyo motor se unirá aquí
* [EH3D-ROUTEPRO](https://github.com/JuanenRac/EH3D-ROUTEPRO): el programa de fresado cuyo motor se unirá aquí

## 📚 Documentación

* [Registro de cambios](CHANGELOG.md)
* [Política de seguridad](SECURITY.md) y [soporte](SUPPORT.md)
* La vista de un trabajo está documentada en EH3D-COMMON, donde vive.

## 🤝 Contribuir y comunidad

* Antes de un pull request ejecuta `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` y `python -I tools/ci_validate.py`: todo debe pasar. Véanse [CONTRIBUTING.md](CONTRIBUTING.md) y el [código de conducta](CODE_OF_CONDUCT.md).
* El README existe en ocho idiomas: un cambio en el texto se hace en los ocho ficheros, y las dos imágenes se dibujan a partir de `docs/architecture.json` y `docs/structure.json` con `tools/make_architecture.py` y `tools/make_structure.py`.
* No describas como funcional lo que no se ha ejecutado en una máquina.
* Informa de una vulnerabilidad en privado: [SECURITY.md](SECURITY.md). Preguntas, ideas e informes: electrohobby3d@gmail.com ([SUPPORT.md](SUPPORT.md)).

## 👤 AUTOR

**JuanenRac (Electro Hobby 3D)** · electrohobby3d@gmail.com

## 📜 LICENCIA

Software: GPL-3.0-or-later. Véase [LICENSE](LICENSE). Los diseños de hardware de la suite usan CERN OHL-S.
