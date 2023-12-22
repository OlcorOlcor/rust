use std::collections::HashMap;

#[derive(Clone)]
pub struct Const {
    num: i32,
}

#[derive(Clone)]
pub struct Variable {
    var: String,
}

impl Const {
    pub fn new(num: i32) -> Const {
        Const { num }
    }

    pub fn value(&self) -> i32 {
        self.num
    }
}

impl Variable {
    pub fn new(var: String) -> Variable {
        Variable { var }
    }

    pub fn name(&self) -> String {
        self.var.clone()
    }
}
pub struct Sum {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Sum {
    pub fn new(left: impl Expression + 'static, right: impl Expression + 'static) -> Sum {
        Sum { left: Box::new(left), right: Box::new(right) }
    }

    pub fn left(&self) -> &dyn Expression {
        self.left.as_ref()
    }

    pub fn right(&self) -> &dyn Expression {
        self.right.as_ref()
    }
}

pub struct Product {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>
}

impl Product {
    pub fn new(left: impl Expression + 'static, right: impl Expression + 'static) -> Product {
        Product { left: Box::new(left), right: Box::new(right) }
    }

    pub fn left(&self) -> &dyn Expression {
        self.left.as_ref()
    }

    pub fn right(&self) -> &dyn Expression {
        self.right.as_ref()
    }
}

#[derive(Debug)]
pub enum Error {
    UnboundVariable,
}

pub trait Expression {
    fn evaluate(&self, map: &HashMap<String, i32>) -> Result<i32, Error>;
    fn accept(&self, visitor: &mut dyn Visitor);
    fn clone_exp(&self) -> Box<dyn Expression>;
}

impl Expression for Const {
    fn evaluate(&self, _map: &HashMap<String, i32>) -> Result<i32, Error> {
        Ok(self.num)
    }

    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_const(self)
    }

    fn clone_exp(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl AsRef<Const> for Const {
    fn as_ref(&self) -> &Const {
        self
    }
}

impl Expression for Variable {
    fn evaluate(&self, map: &HashMap<String, i32>) -> Result<i32, Error> {
        map.get(&self.var).cloned().ok_or(Error::UnboundVariable)
    }

    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_var(self)
    }

    fn clone_exp(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl AsRef<Variable> for Variable {
    fn as_ref(&self) -> &Variable {
        self
    }
}

impl Expression for Sum {
    fn evaluate(&self, map: &HashMap<String, i32>) -> Result<i32, Error> {
        Ok(self.left.as_ref().evaluate(map)? + self.right.as_ref().evaluate(map)?)
    }

    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_sum(self)
    }

    fn clone_exp(&self) -> Box<dyn Expression> {
        Box::new(Sum { left: self.left().clone_exp(), right: self.right().clone_exp() })
    }
}

impl AsRef<Sum> for Sum {
    fn as_ref(&self) -> &Sum {
        self
    }
}

impl Expression for Product {
    fn evaluate(&self, map: &HashMap<String, i32>) -> Result<i32, Error> {
        Ok(self.left.as_ref().evaluate(map)? * self.right.as_ref().evaluate(map)?)
    }

    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_product(self)
    }

    fn clone_exp(&self) -> Box<dyn Expression> {
        Box::new(Product { left: self.left().clone_exp(), right: self.right().clone_exp() })
    }
}

impl AsRef<Product> for Product {
    fn as_ref(&self) -> &Product {
        self
    }
}
pub trait Visitor {
    fn visit_const(&mut self, cst: &Const);
    fn visit_var(&mut self, var: &Variable);
    fn visit_sum(&mut self, sum: &Sum);
    fn visit_product(&mut self, product: &Product);
}

pub struct Evaluate {
    result: i32,
    succ: bool,
    map: HashMap<String, i32>
}

impl Evaluate {
    pub fn transform(exp: &dyn Expression, map: &HashMap<String, i32>) -> Result<i32, Error> {
        let mut visitor = Self::new(map);
        exp.accept(&mut visitor);
        if visitor.succ {
            return Ok(visitor.result);
        } else {
            return Err(Error::UnboundVariable);
        }
    }

    pub fn new(map: &HashMap<String, i32>) -> Self {
        Evaluate { result: 0, succ: true, map: map.clone() }
    }
}

impl Visitor for Evaluate {
    fn visit_const(&mut self, cst: &Const) {
        let res = cst.evaluate(&self.map);
        match res {
            Ok(value) => self.result += value,
            Err(_) => { self.succ = false; },
        }
    }

    fn visit_var(&mut self, var: &Variable) {
        let res = var.evaluate(&self.map);
        match res {
            Ok(value) => self.result += value,
            Err(_) => { self.succ = false; },
        }
    }

    fn visit_sum(&mut self, sum: &Sum) {
        let res = sum.evaluate(&self.map);
        match res {
            Ok(value) => self.result += value,
            Err(_) => { self.succ = false; },
        }
    }

    fn visit_product(&mut self, product: &Product) {
        let res = product.evaluate(&self.map);
        match res {
            Ok(value) => self.result += value,
            Err(_) => { self.succ = false; },
        }
    }
}

pub struct Substitute<'a> {
    result: Option<Box<dyn Expression>>,
    map: &'a HashMap<String, &'a dyn Expression>
}

impl<'a> Substitute<'a> {
    pub fn transform(exp: &dyn Expression, map: &'a HashMap<String, &dyn Expression>) -> Box<dyn Expression> {
        let mut visitor = Self::new(map);
        exp.accept(&mut visitor);
        visitor.result.unwrap()
    }

    fn new(map: &'a HashMap<String, &dyn Expression>) -> Self {
        Substitute { result: None, map }
    }
}

impl<'a> Visitor for Substitute<'a> {
    fn visit_const(&mut self, cst: &Const) {
        self.result = Some(Box::new(cst.clone()));
    }

    fn visit_var(&mut self, var: &Variable) {
        let v = self.map.get(&var.var).ok_or(Error::UnboundVariable);
        match v {
            Ok(exp) => { self.result = Some(exp.clone_exp()); },
            Err(_) => { self.result = Some(var.clone_exp()) }
        }
    }

    fn visit_sum(&mut self, sum: &Sum) {
        sum.left().accept(self);
        let left = self.result.take().unwrap();
        sum.right().accept(self);
        self.result = Some(Box::new(Sum {left, right: self.result.take().unwrap()}));
    }

    fn visit_product(&mut self, product: &Product) {
        product.left().accept(self);
        let left = self.result.take().unwrap();
        product.right().accept(self);
        self.result = Some(Box::new(Product {left, right: self.result.take().unwrap()}));
    }
}