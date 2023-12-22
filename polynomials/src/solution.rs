use std::ops;

#[derive(Debug)]
pub struct Polynomial<'a> {
    coefficients: Vec<i32>,
    variables: Vec<&'a str>,
    exponents: Vec<i32>
} 

impl<'a> Polynomial<'a> {
    pub fn builder() -> PolynomialBuilder<'a> {
        PolynomialBuilder::new()
    }
}
pub struct PolynomialBuilder<'a> {
    coefficients: Vec<i32>,
    variables: Vec<&'a str>,
    exponents: Vec<i32>
}

impl<'a> PolynomialBuilder<'a> {
    fn new() -> PolynomialBuilder<'a> {
        PolynomialBuilder { 
            coefficients: Vec::new(),
            variables: Vec::new(),
            exponents: Vec::new()
        }
    }
    
    pub fn add(mut self, c: i32, v: &'a str, e: i32) -> PolynomialBuilder<'a> {
        self.coefficients.push(c);
        self.variables.push(v);
        self.exponents.push(e);
        self
    }

    pub fn build(self) -> Polynomial<'a> {
        Polynomial { 
            coefficients: self.coefficients,
            variables: self.variables,
            exponents: self.exponents
        }
    }
}

impl<'a> ops::Add<Polynomial<'a>> for Polynomial<'a> {
    type Output = Polynomial<'a>;

    fn add(self, mut other: Polynomial<'a>) -> Polynomial<'a> {
        let mut new: Vec<(i32, &'a str, i32)> = Vec::new(); 
        for i in 0..self.coefficients.len() {
            let first_pol = (self.coefficients[i], self.variables[i], self.exponents[i]);
            let mut pushed: bool = false;
            for j in 0..other.coefficients.len() {
                let second_pol = (other.coefficients[j], other.variables[j], other.exponents[j]);
                if first_pol.1 == second_pol.1 && first_pol.2 == second_pol.2 {
                    if first_pol.0 + second_pol.0 != 0 {
                        new.push((first_pol.0 + second_pol.0, first_pol.1, first_pol.2));
                    }
                    other.coefficients.remove(j);
                    other.variables.remove(j);
                    other.exponents.remove(j);
                    pushed = true;
                    break;
                }
            }
            if !pushed {
                new.push((first_pol.0, first_pol.1, first_pol.2));
            }
        }

        // push remaining polynomials from other
        for i in 0..other.coefficients.len() {
            new.push((other.coefficients[i], other.variables[i], other.exponents[i]));
        }

        let mut builder = Polynomial::builder();
        for i in 0..new.len() {
            builder = builder.add(new[i].0, new[i].1, new[i].2);
        }
        builder.build()
    }
}

impl<'a> PartialEq for Polynomial<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.coefficients.len() != other.coefficients.len() {
            return false;
        }
        let len = self.coefficients.len();
        let mut found: bool = false;
        for i in 0..len {
            for j in 0..len {
                if self.coefficients[i] == other.coefficients[j] && self.exponents[i] == other.exponents[j] && self.variables[i] == other.variables[j] {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        return true;
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}