# Polynomial Root Finder / Buscador de raíces de polinomios

## English

### What is this project for?
This Rust project is designed to evaluate polynomial functions and find their roots within a given interval. It uses simple algorithms to search for roots and can also compute the derivative of a polynomial.

### How does it work?
- You define a polynomial by its coefficients (highest degree first).
- You must specify a search range with `init_a` (lower bound) and `init_b` (upper bound) in the code. This range is required for the root-finding algorithm to work.
- The program searches for intervals where roots may exist within this range.
- It subdivides the interval and iteratively refines the search to find roots with a specified measurement error.
- Results are previewed in two ways:
  - As a range where both ends are roots of the polynomial.
  - As a range whose ends are very close, and the arithmetic mean of the bounds is shown as the result.
- It can also print the derivative of the polynomial.

### How to use
1. **Build the project:**
   ```sh
   cargo build
   ```
2. **Run the project:**
   ```sh
   cargo run
   ```
3. **Edit the polynomial and search range:**
   Change the coefficients and the search range in the `main.rs` file:
   ```rust
   let init_a = -5.; // Lower bound (required)
   let init_b = 6.;  // Upper bound (required)
   let polynomail = Polynomial::create(vec![12., -60., -3., 15.]);
   ```
   (This example is for a degree 3 polynomial)

---

## Castellano

### ¿Para qué sirve este proyecto?
Este proyecto en Rust está diseñado para evaluar funciones polinómicas y encontrar sus raíces dentro de un intervalo dado. Utiliza algoritmos sencillos para buscar raíces y también puede calcular la derivada de un polinomio.

### ¿Cómo funciona?
- Defines un polinomio mediante sus coeficientes (empezando por el de mayor grado).
- Es obligatorio especificar un rango de búsqueda con `init_a` (límite inferior) y `init_b` (límite superior) en el código. Este rango es necesario para que el algoritmo de búsqueda de raíces funcione.
- El programa busca intervalos donde pueden existir raíces dentro de ese rango.
- Subdivide el intervalo y refina la búsqueda iterativamente para encontrar raíces con un error de medición especificado.
- Los resultados se previsualizan de dos formas:
  - Como un rango donde ambos extremos son raíces del polinomio.
  - Como un rango cuyos extremos son muy próximos y su media aritmética se muestra como resultado.
- También puede imprimir la derivada del polinomio.
---

### Cómo usarlo
1. **Compila el proyecto:**
   ```sh
   cargo build
   ```
2. **Ejecuta el proyecto:**
   ```sh
   cargo run
   ```
3. **Edita el polinomio y el rango de búsqueda:**
   Cambia los coeficientes y el rango de búsqueda en el archivo `main.rs`:
   ```rust
   let init_a = -5.; // Límite inferior (requerido)
   let init_b = 6.;  // Límite superior (requerido)
   let polynomail = Polynomial::create(vec![12., -60., -3., 15.]);
   ```
   (Este ejemplo es para un polinomio de grado 3)

## 中文 (Chinese Mandarin)

### 这个项目的用途是什么？
这个 Rust 项目用于在给定区间内评估多项式函数并寻找其根。它使用简单的算法搜索根，并且可以计算多项式的导数。

### 工作原理
- 你需要用系数（从最高次开始）定义一个多项式。
- 必须在代码中指定搜索区间 `init_a`（下界）和 `init_b`（上界），这是根查找算法所必需的。
- 程序会在该区间内搜索可能存在根的区间。
- 它会细分区间并通过迭代方式精确查找根，精度由测量误差决定。
- 结果有两种预览方式：
  - 区间两端都是多项式的根。
  - 区间两端非常接近，显示区间的算术平均值作为结果。
- 还可以打印多项式的导数。

### 如何使用
1. **构建项目：**
   ```sh
   cargo build
   ```
2. **运行项目：**
   ```sh
   cargo run
   ```
3. **编辑多项式和搜索区间：**
   在 `main.rs` 文件中更改系数和搜索区间：
   ```rust
   let init_a = -5.; // 下界（必需）
   let init_b = 6.;  // 上界（必需）
   let polynomail = Polynomial::create(vec![12., -60., -3., 15.]);
   ```
   （此示例为三次多项式）
