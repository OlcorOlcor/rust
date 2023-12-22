mod solution;

use solution::*;
use std::collections::HashMap;

use solution::Expression;
use solution::Visitor;

struct PostfixConvertor {
    result: Vec<String>,
}

impl PostfixConvertor {
    pub fn transform(expression: &dyn Expression) -> String {
        let mut visitor = Self::new();
        expression.accept(&mut visitor);
        visitor.consume()
    }
    fn new() -> Self {
        PostfixConvertor { result: Vec::new() }
    }
    fn consume(self) -> String { self.result.join(" ") }
}

impl Visitor for PostfixConvertor {
    fn visit_const(&mut self, cst: &Const) {
        self.result.push(format!("{}", cst.value()));
    }

    fn visit_var(&mut self, var: &Variable) {
        self.result.push(format!("{}", var.name()));
    }

    fn visit_sum(&mut self, sum: &Sum) {
        sum.left().accept(self);
        sum.right().accept(self);
        self.result.push(format!("+"));
    }

    fn visit_product(&mut self, product: &Product) {
        product.left().accept(self);
        product.right().accept(self);
        self.result.push(format!("*"));
    }
}

fn main() {
    let c = Const::new(12);
    let c3 = Const::new(12);
    let c2 = Const::new(2);
    let a = Variable::new("a".to_string());
    let b = Variable::new("b".to_string());
    let exp = Sum::new(c, Product::new(a, b));
    let values = HashMap::from([("a".to_string(), 2), ("b".to_string(), 10)]);
    match Evaluate::transform(exp.as_ref(), &values) {
        Ok(value) => println!("{}", value),
        Err(_) => println!("Missing variable."),
    }
    
    let exp2 = Sum::new(c3, Variable::new("a".to_string()));
    let a = Sum::new(Const::new(1), Const::new(1));
    let mut values2: HashMap<_, &dyn Expression> = HashMap::new();
    values2.insert("a".to_string(), a.as_ref());
    let exp2 = Substitute::transform(exp2.as_ref(), &values2);
    
    println!("{}", PostfixConvertor::transform(exp2.as_ref()));

}