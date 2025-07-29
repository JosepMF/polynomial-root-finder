# Polynomial Root Finder / Buscador de raíces de polinomios

## English

### What is this project for?
This Rust project is designed to evaluate polynomial functions and find their roots within a given interval. It uses simple algorithms to search for roots and can also compute the derivative of a polynomial.

### How does it work?
- You define a polynomial by its coefficients (highest degree first).
- The program searches for intervals where roots may exist.
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
3. **Edit the polynomial:**
   Change the coefficients in the `main.rs` file:
   ```rust
   let polynomail = Polynomial::create(vec![12., -60., -3., 15.]);
   ```
   (This example is for a degree 3 polynomial)

---

## Castellano

### ¿Para qué sirve este proyecto?
Este proyecto en Rust está diseñado para evaluar funciones polinómicas y encontrar sus raíces dentro de un intervalo dado. Utiliza algoritmos sencillos para buscar raíces y también puede calcular la derivada de un polinomio.

### ¿Cómo funciona?
- Defines un polinomio mediante sus coeficientes (empezando por el de mayor grado).
- El programa busca intervalos donde pueden existir raíces.
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
3. **Edita el polinomio:**
   Cambia los coeficientes en el archivo `main.rs`:
   ```rust
   let polynomail = Polynomial::create(vec![12., -60., -3., 15.]);
   ```
   (Este ejemplo es para un polinomio de grado 3)
