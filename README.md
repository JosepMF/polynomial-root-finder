# Polynomial Root Finder / Buscador de raíces de polinomios

## English

### What is this project for?
This Rust project is designed to evaluate polynomial functions and find their roots within a given interval. It uses simple algorithms to search for roots and can also compute the derivative of a polynomial.

### How does it work?
- You define a polynomial by its coefficients (highest degree first).
- You must specify a search range with `init_a` (lower bound) and `init_b` (upper bound) in the code. This range is required for the root-finding algorithm to work.
- The program searches for intervals where roots may exist within this range.
- It subdivides the interval and iteratively refines the search to find roots with a specified measurement error.
- It prints the intervals containing roots and the average value as an approximation.
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
- Imprime los intervalos que contienen raíces y el valor promedio como aproximación.
- También puede imprimir la derivada del polinomio.

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
