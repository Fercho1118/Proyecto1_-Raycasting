# 🎮 Naruto Maze - Raycasting Game

Un juego de laberinto 3D desarrollado en Rust utilizando técnicas de raycasting, inspirado en el universo de Naruto. Navega a través de laberintos temáticos con sprites animados y tres niveles de dificultad.

## 🎥 Demo en YouTube
[![Naruto Maze](https://img.youtube.com/vi/E-mm0HbIBZc/0.jpg)](https://youtu.be/E-mm0HbIBZc)

## ✨ Características

- 🌟 **Renderizado 3D**: Motor de raycasting personalizado para visualización en primera persona
- 🎭 **Sprites Animados**: Sprites de Naruto con animaciones reales de GIF
- 🎯 **Múltiples Dificultades**: Tres niveles de laberinto (Fácil, Medio, Difícil)
- 🎮 **Control Dual**: Soporte completo para teclado/mouse y gamepad
- 🎵 **Audio Inmersivo**: Música de fondo y efectos de sonido temáticos
- 📱 **Vista Adaptativa**: Alterna entre modo 2D (vista superior) y 3D (primera persona)
- 🗺️ **Minimapa**: Navegación asistida en modo 3D
- 🎨 **Texturas Temáticas**: Texturas del bosque y Konoha para paredes

## 🚀 Instalación

### Prerrequisitos

- **Rust** (versión 1.70 o superior)
- **Git**
- **Sistema operativo**: Windows, macOS, o Linux

### Clonar el Repositorio

```bash
git clone https://github.com/Fercho1118/Proyecto1_-Raycasting.git
cd Proyecto1_-Raycasting/Proyecto1
```

### Compilar y Ejecutar

```bash
# Compilación en modo debug
cargo run

# Compilación optimizada (recomendado)
cargo run --release
```

## 🎮 Controles

### Teclado

| Tecla | Acción |
|-------|--------|
| `W` / `↑` | Mover hacia adelante |
| `S` / `↓` | Mover hacia atrás |
| `A` | Movimiento lateral izquierdo |
| `D` | Movimiento lateral derecho |
| `←` / `→` | Rotar cámara |
| `M` | Cambiar entre modo 2D/3D |
| `Q` | Volver al menú principal |
| `R` | Reiniciar nivel actual |
| `Mouse` | Rotación de cámara |

### Gamepad (PlayStation/Xbox)

| Botón | Acción |
|-------|--------|
| Stick Izquierdo | Movimiento |
| Stick Derecho | Rotación de cámara |
| Triángulo / Y | Cambiar modo 2D/3D |
| Options / Menu | Volver al menú |
| Share / View | Reiniciar nivel |

## 🏗️ Arquitectura Técnica

### Estructura del Proyecto

```
Proyecto1/
├── src/
│   ├── main.rs          # Bucle principal del juego
│   ├── caster.rs        # Motor de raycasting
│   ├── framebuffer.rs   # Gestión de buffer de frame y texturas
│   ├── player.rs        # Lógica del jugador y controles
│   ├── maze.rs          # Carga y gestión de laberintos
│   ├── sprites.rs       # Sistema de sprites animados
│   ├── audio.rs         # Sistema de audio
│   ├── game_state.rs    # Gestión de estados del juego
│   ├── screens.rs       # Pantallas de menú y victoria
│   └── line.rs          # Algoritmo de línea de Bresenham
├── assets/
│   ├── img/             # Texturas y sprites
│   └── sounds/          # Efectos de audio y música
├── maze_*.txt           # Archivos de laberinto
└── Cargo.toml           # Configuración de dependencias
```

### Tecnologías Utilizadas

- **Raylib-rs**: Motor gráfico y gestión de ventanas
- **Rodio**: Sistema de audio
- **Image**: Procesamiento de imágenes y texturas
- **GIF**: Decodificación de animaciones GIF
- **Fastrand**: Generación de números aleatorios

## 🎯 Algoritmo de Raycasting

El motor implementa raycasting clásico con las siguientes optimizaciones:

1. **Cálculo de Intersecciones**: DDA (Digital Differential Analyzer) para intersecciones eficientes
2. **Mapeo de Texturas**: Interpolación bilineal para texturas de alta calidad
3. **Corrección de Perspectiva**: Eliminación del efecto "ojo de pez"
4. **Renderizado de Sprites**: Proyección 3D de sprites con z-buffering

## 🎨 Sistema de Sprites

- **Animación Real**: Decodificación de GIF frame por frame
- **Paleta de Colores**: Soporte completo para paletas indexadas
- **Transparencia**: Alpha blending para efectos realistas
- **Spawning Inteligente**: Distribución aleatoria con validación de posición

## 🔧 Configuración

### Niveles de Dificultad

- **Fácil**: `maze_easy.txt` - Laberinto 13x9
- **Medio**: `maze_medium.txt` - Laberinto expandido
- **Difícil**: `maze_difficult.txt` - Laberinto complejo

### Rendimiento

- **FPS Target**: 15 FPS 
- **Resolución**: 1300x900 
- **Optimizaciones**: Compilación optimizada automática

## 👨‍💻 Autor

**Fernando Rueda** - [@Fercho1118](https://github.com/Fercho1118)

## 🙏 Agradecimientos

- Inspirado en los clásicos juegos de raycasting como Wolfenstein 3D
- Sprites y temática basados en el anime Naruto
- Comunidad de Rust por las excelentes bibliotecas

---

*¿Te gustó el proyecto? ¡Dale una ⭐ en GitHub!*
