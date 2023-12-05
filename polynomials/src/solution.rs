
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