// Measurement error allowed to consider a root found
static MEDITION_ERROR: f64 = 0.00005;

// Structure representing a polynomial
#[derive(Debug)]
struct Polynomial {
    degree: i32, // Degree of the polynomial
    coefficients: Vec<f64>, // Polynomial coefficients
}

// Structure representing an environment (interval) where a root of the polynomial may exist
struct Enviorament {
    a: f64, // Lower bound of the interval
    b: f64, // Upper bound of the interval
}

// Implementation of methods for the Enviorament structure
impl Enviorament {
    // Initializes a new environment with the given bounds
    fn init(a: f64, b: f64) -> Self {
        Enviorament { a, b }
    }

    // Searches for a root within the environment by adjusting the bounds
    fn search_root(&mut self, polynomial: &Polynomial) -> bool {
        let gamma: f64 = 0.000005; // Step size for adjusting bounds

        // If the lower bound exceeds the upper bound, remove the environment
        if self.a > self.b {
            return true;
        }

        // Adjust the lower bound if it is farther from the root
        if polynomial.compute(self.a).abs() > MEDITION_ERROR
            && polynomial.compute(self.a).abs() > polynomial.compute(self.b).abs()
        {
            self.a += gamma;
            return false;
        }
        // Adjust the upper bound if it is farther from the root
        if polynomial.compute(self.b).abs() > MEDITION_ERROR
            && polynomial.compute(self.b).abs() > polynomial.compute(self.a).abs()
        {
            self.b -= gamma;
            return false;
        }

        // A root has been found within the interval
        println!(
            "The value of the root is inside this range ({}, {}) AVERAGE: {}",
            self.a, self.b, (self.a + self.b) / 2.0 
        );
        true
    }
}

// Implementation of methods for the Polynomial structure
impl Polynomial {
    // Creates a new polynomial from a vector of coefficients
    fn create(coefficients: Vec<f64>) -> Self {
        let degree = coefficients.len() as i32 - 1;
        Polynomial {
            degree,
            coefficients,
        }
    }

    // Computes the value of the polynomial for a given x
    fn compute(&self, x: f64) -> f64 {
        let mut result = 0.0;
        // Applies the general polynomial formula
        for (i, &coef) in self.coefficients.iter().enumerate() {
            result += coef * x.powi((self.coefficients.len() as i32 - 1) - i as i32);
        }
        result
    }

    // Computes the derivative of the polynomial and returns a new polynomial
    fn derivate(&self) -> Polynomial {
        let mut coeficients_new: Vec<f64> = Vec::new();

        // Applies the derivative rule for each coefficient
        for (i, &coef) in self.coefficients.iter().enumerate() {
            coeficients_new.push(coef * ((self.degree - i as i32) as f64));
        }

        // Removes the last coefficient (constant term)
        coeficients_new.pop();

        Polynomial::create(coeficients_new)
    }
}

// Finds the range where roots of the polynomial may exist
fn find_search_range(polynomial: &Polynomial, init_a: f64, init_b: f64) -> (f64, f64) {
    let mut a: f64 = init_a;
    let mut b: f64 = init_b;

    let gamma: f64 = 0.00005; // Step size for adjusting bounds

    println!("Our polynomial is degree {}", &polynomial.degree);

    loop {
        // If the product is negative, there is a root between a and b
        if polynomial.compute(a) * polynomial.compute(b) < 0.0 {
            println!("There is a root");
            if &polynomial.degree % 2 == 0 {
                println!("Some roots of the polynomial are in: {}, {}", a, b);
            }

            break;
        }

        /*
         * If no root is found, adjust the outer bounds
         * For a, if searching on the negative side, add gamma
         * For b, subtract gamma
         * This is done so that f(a) < f(b), but if f(a)f(b) are the same sign, decrease f(b), and vice versa
         */
        println!("Not yet, a: {} b: {}", a, b);
        if polynomial.compute(a) - polynomial.compute(b) < 0.0 {
            b -= gamma;
        } else {
            a += gamma;
        }

        // If it goes out of bounds, break the loop
        if a > b  {
            break;
        }
    }

    // If no range was found, return the initial bounds again
    if a > b {
        return (init_a, init_b);
    }

    return (a + MEDITION_ERROR, b);
}

// Main function of the program
fn main() {
    let init_a = -5.; // Initial lower bound
    let init_b = 6.; // Initial upper bound

    // Create an example polynomial: x^2
    let polynomail = Polynomial::create(vec![12., -60., -3., 15.]);

    // Find the range where roots may exist
    let (a, b) = find_search_range(&polynomail, init_a, init_b);

    let mut envioraments: Vec<Enviorament> = Vec::new(); // Vector of environments
    let mut counter = 0;
    // Calculate the width of each environment
    let amplitud_enviorament = (b - a).abs() / polynomail.degree as f64 + 0.00000000001;

    // Initialize the environments
    while polynomail.degree - 1 >= counter {
        envioraments.push(Enviorament::init(
            a + counter as f64 * amplitud_enviorament - MEDITION_ERROR - 0.00000000001,
            a + (counter + 1) as f64 * amplitud_enviorament + MEDITION_ERROR,
        ));
        counter += 1;
    }

    // Print the created environments
    for (i, enviorament) in envioraments.iter().enumerate() {
        println!("{} | {} {}", i, enviorament.a, enviorament.b);
    }

    // Main loop to search for roots in the environments
    loop {
        let mut to_remove = Vec::new();
        for (i, enviorament) in envioraments.iter_mut().enumerate() {
            if enviorament.search_root(&polynomail) {
                to_remove.push(i);
            }
        }
        // Remove processed environments (from back to front to avoid index issues)
        for &i in to_remove.iter().rev() {
            envioraments.remove(i);
        }
        // Break the loop if no environments remain
        if envioraments.is_empty() {
            break;
        }
    }

    // Print the derivative of the polynomial
    println!("{:?}", polynomail.derivate());
}
