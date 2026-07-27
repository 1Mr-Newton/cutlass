# Cutlass

Cutlass es un editor de video gratuito y de código abierto con un asistente de IA integrado.
Edita en la línea de tiempo de la manera habitual, o describe la edición en lenguaje sencillo
y el asistente la ejecutará como comandos regulares de la línea de tiempo que puedes revisar y
deshacer.

Se encuentra en fase alpha temprana. El ciclo de edición central funciona, pero se esperan detalles sin pulir y un
formato de proyecto que aún no se ha estabilizado.

## Qué funciona hoy

**Edición de línea de tiempo**

- Importación de video, audio e imágenes en una línea de tiempo de múltiples pistas.
- Cortar, recortar, dividir, mover, duplicar, vincular/desvincular, ripple-delete, selección múltiple.
- Cambios de velocidad (planos o en rampa), reversa, recorte, giro, mover/escalar/rotar
  (escala por eje), opacidad.
- Texto con estilo, colores sólidos, formas y stickers incluidos, estáticos y animados.
- Animaciones de entrada/salida/combo desde un catálogo.
- Fotogramas clave (keyframes) en transformaciones y ajustes de efectos, con un editor de gráficas y ajustes preestablecidos de suavizado (easing); las rutas de movimiento soportan tangentes bezier.
- Modos de fusión, estilos de capa (sombra / resplandor / contorno / fondo), recorte animable y desenfoque de movimiento (motion blur) de transformación por clip.
- Ajustes preestablecidos de lienzo (16:9, 9:16, 1:1, 4:5, 21:9) y un color de fondo.

**Efectos y color**

- Pasadas de efectos por GPU: desenfoque gaussiano, viñeteado, pixelado, además de parámetros tipados
  (incluyendo duotono). El resto del catálogo (enfocar, glitch, grano, resplandor,
  y así sucesivamente) es seleccionable pero se renderiza como una operación nula (no-op) hasta que su shader esté disponible.
- Geometría de máscara por clip (lineal, espejo, círculo, rectángulo, corazón, estrella) y
  chroma key.
- Ajustes preestablecidos de filtros y ajuste de color expandido (11 deslizadores) por clip, además de
  pasadas de ajuste / efecto / filtro en toda la pista que gradúan todo lo que esté debajo.
- Transiciones: el crossfade y el wipe-left están implementados; las demás entradas del catálogo
  se reproducen actualmente como un crossfade.

**Audio**

- Envolventes de volumen, pan stereo y manejadores de desvanecimiento (fade) arrastrables.
- Los cambios de velocidad remuestrean el audio, incluyendo las rampas. El tono sigue la velocidad
  por ahora; el estiramiento que preserva el tono está planeado pero no construido.
- Reducción de ruido por clip (RNNoise).

**Vista previa y exportación**

- Vista previa en GPU en vivo con scrubbing y reproducción.
- Exportación a H.264/AAC MP4.

**El asistente de IA**

Describe una edición y el asistente la aplica a través de los mismos comandos que
utiliza la interfaz de usuario, por lo que su trabajo aparece en la línea de tiempo como si fuera el tuyo y se deshace en
un solo paso. La vista previa de ejecución simulada (dry-run) está activada por defecto: ves el plan antes de que cualquier cosa
cambie. El asistente es opcional y el editor funciona perfectamente sin él.

## Instalación

Descarga una versión desde la [página de releases](https://github.com/1Mr-Newton/cutlass/releases).

- **macOS** (Apple Silicon): descomprime y arrastra `Cutlass.app` a Aplicaciones.
  En el primer inicio, haz clic derecho en la app y elige **Abrir**; las versiones aún no están
  notarizadas. La decodificación/codificación de medios utiliza AVFoundation del sistema, por lo que
  no hay nada más que instalar.
- **Windows** (x64): descomprime y ejecuta, o utiliza el instalador Setup.exe. La decodificación/codificación de medios utiliza Media Foundation, por lo que no hay nada más que instalar. Las versiones no están
  firmadas por ahora; SmartScreen advertirá en la primera ejecución.
- **Linux**: solo versiones de vista previa. La interfaz de usuario funciona, pero el backend de medios de Linux
  aún no está implementado, por lo que los medios importados no se reproducirán.

## Configuración del asistente de IA

Cutlass no incluye un modelo. Usa **Local** (Ollama / LM Studio), **OpenRouter**
para la nube, o **Advanced** para cualquier endpoint compatible con OpenAI. El cuadro de diálogo de Ajustes
es la ruta habitual; o crea el archivo `~/.cutlass/config.toml`:

```toml
# Local (Ollama / LM Studio) — solo modelos seleccionados
[ai]
source = "local"
base_url = "http://localhost:11434/v1"
model = "qwen3:14b"

# O nube de OpenRouter — una sola clave, slugs seleccionados
# [ai]
# source = "openrouter"
# model = "openai/gpt-5.6-sol"
# api_key = "sk-or-…"
# # api_key_env = "OPENROUTER_API_KEY"
```

La clave permanece en ese archivo o en tu entorno; nunca se escribe en los
archivos del proyecto. Los modelos locales pequeños funcionan, pero su capacidad de llamado a herramientas es menos
confiable, razón por la cual el dry-run es el valor predeterminado.

## Proyectos

Cutlass gestiona tus proyectos al estilo de CapCut. No hay botón de guardar: cada edición
se guarda automáticamente, y la pantalla de inicio enumera tus proyectos para reabrirlos o eliminarlos.
Cambia el nombre de un proyecto desde la barra de título.

**Open file…** importa un archivo `.cutlass` externo a tus proyectos, y
**Export** renderiza un `.mp4`. Los medios se referencian desde donde se encuentren en
el disco, por lo que un proyecto de otra máquina puede pedirte que vuelvas a vincular sus medios.

## Compilar desde el código fuente

Necesitas una cadena de herramientas de Rust estable y reciente. No hay bibliotecas de medios de terceros
que instalar; la decodificación/codificación es nativa de la plataforma (AVFoundation y
VideoToolbox en plataformas Apple, Media Foundation en Windows).

```bash
cargo run -p cutlass-desktop
# o abrir directamente un archivo:
cargo run -p cutlass-desktop -- path/to/video.mp4
```

Compilar y probar todo:

```bash
cargo build --workspace
cargo test --workspace
```

La aplicación SwiftUI para iOS/macOS reside en `apps/cutlass-ios-macos` (construida con Xcode
sobre el mismo motor a través de `cutlass-mobile`), y la aplicación de prueba de Android en
`apps/cutlass-android`.

## Contribuciones

Consulta [CONTRIBUTING.md](CONTRIBUTING.md) para la configuración, la disposición del proyecto y el
estilo de commits/PR. Cada crate tiene su propio README bajo `crates/`, y las notas de empaquetado se encuentran en [packaging/](packaging/README.md).

Gran parte de Cutlass está escrito con herramientas de codificación de IA y revisado por mantenedores.
Las contribuciones se juzgan por lo que hacen, no por cómo fueron creadas.

## Licencia

Licencia dual bajo [Apache-2.0](LICENSE-APACHE) o [MIT](LICENSE-MIT), según tu preferencia.
