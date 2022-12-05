use std::{array, collections::HashMap, fmt};

use crate::{CacauError, CacauResult};

pub type Identifier = String;

#[derive(Debug, Clone)]
pub enum Statement {
    Empty,
    Assignment(Identifier, Expression),
    Expression(Expression),
    FunctionDeclaration(Identifier, Identifier, Vec<Statement>),
    Return(Option<Expression>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Value),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
    UnaryOp(UnaryOperator, Box<Expression>),
    Variable(Identifier),
    FunctionCall(Identifier, Vec<Expression>),
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Less,
    LessOrEqual,
    Equals,
    NotEquals,
    Greater,
    GreaterOrEqual,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Not,
    Minus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Number(i32),
    Bool(bool),
    Unit,
}

impl Value {
    const TYPE_CHECK_ERROR_MESSAGE: &str = "type checks failed";

    pub fn unwrap_number(self) -> i32 {
        match self {
            Value::Number(value) => value,
            _ => unreachable!("{}", Self::TYPE_CHECK_ERROR_MESSAGE),
        }
    }

    pub fn unwrap_bool(self) -> bool {
        match self {
            Value::Bool(value) => value,
            _ => unreachable!("{}", Self::TYPE_CHECK_ERROR_MESSAGE),
        }
    }

    pub fn variant_name(&self) -> &'static str {
        match self {
            Self::Number(_) => "number",
            Self::Bool(_) => "bool",
            Self::Unit => "()",
        }
    }

    pub fn not_op(self) -> CacauResult<Value> {
        match &self {
            Self::Bool(boolean) => Ok(Self::Bool(!boolean)),
            _ => {
                Err(CacauError::InvalidUnaryOperation(
                    "not",
                    self.variant_name(),
                ))
            }
        }
    }

    pub fn minus_op(self) -> CacauResult<Value> {
        match &self {
            Self::Number(value) => Ok(Self::Number(-value)),
            _ => Err(CacauError::InvalidUnaryOperation("-", self.variant_name())),
        }
    }

    pub fn add_op(self, other: Self) -> CacauResult<Value> {
        match (&self, &other) {
            (Self::Number(l), Self::Number(r)) => Ok(Self::Number(l + r)),
            _ => {
                Err(CacauError::InvalidBinaryOperation(
                    self.variant_name(),
                    "add",
                    other.variant_name(),
                ))
            }
        }
    }

    pub fn sub_op(self, other: Self) -> CacauResult<Value> {
        match (&self, &other) {
            (Self::Number(l), Self::Number(r)) => Ok(Self::Number(l - r)),
            _ => {
                Err(CacauError::InvalidBinaryOperation(
                    self.variant_name(),
                    "subtract",
                    other.variant_name(),
                ))
            }
        }
    }

    pub fn mul_op(self, other: Self) -> CacauResult<Value> {
        match (&self, &other) {
            (Self::Number(l), Self::Number(r)) => Ok(Self::Number(l * r)),
            _ => {
                Err(CacauError::InvalidBinaryOperation(
                    self.variant_name(),
                    "multiply",
                    other.variant_name(),
                ))
            }
        }
    }

    pub fn div_op(self, other: Self) -> CacauResult<Value> {
        match (&self, &other) {
            (Self::Number(l), Self::Number(r)) => Ok(Self::Number(l / r)),
            _ => {
                Err(CacauError::InvalidBinaryOperation(
                    self.variant_name(),
                    "divide",
                    other.variant_name(),
                ))
            }
        }
    }

    pub fn less_op(self, other: Self) -> CacauResult<Value> {
        match (&self, &other) {
            (Self::Number(l), Self::Number(r)) => Ok(Self::Bool(l < r)),
            _ => {
                Err(CacauError::InvalidBinaryOperation(
                    self.variant_name(),
                    "compare",
                    other.variant_name(),
                ))
            }
        }
    }

    pub fn less_or_equal_op(self, other: Self) -> CacauResult<Value> {
        match (&self, &other) {
            (Self::Number(l), Self::Number(r)) => Ok(Self::Bool(l <= r)),
            _ => {
                Err(CacauError::InvalidBinaryOperation(
                    self.variant_name(),
                    "compare",
                    other.variant_name(),
                ))
            }
        }
    }

    pub fn equals_op(self, other: Self) -> CacauResult<Value> {
        Ok(Self::Bool(self == other))
    }

    pub fn not_equals_op(self, other: Self) -> CacauResult<Value> {
        Ok(Self::Bool(self != other))
    }

    pub fn greater_op(self, other: Self) -> CacauResult<Value> {
        match (&self, &other) {
            (Self::Number(l), Self::Number(r)) => Ok(Self::Bool(l > r)),
            _ => {
                Err(CacauError::InvalidBinaryOperation(
                    self.variant_name(),
                    "compare",
                    other.variant_name(),
                ))
            }
        }
    }

    pub fn greater_or_equal_op(self, other: Self) -> CacauResult<Value> {
        match (&self, &other) {
            (Self::Number(l), Self::Number(r)) => Ok(Self::Bool(l >= r)),
            _ => {
                Err(CacauError::InvalidBinaryOperation(
                    self.variant_name(),
                    "compare",
                    other.variant_name(),
                ))
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(inner) => inner.fmt(f),
            Self::Number(inner) => inner.fmt(f),
            Self::Unit => write!(f, "()"),
        }
    }
}

pub trait Interpret<T = ()> {
    fn evaluate(&self, vm: &mut Vm) -> CacauResult<T>;
}

impl Interpret for Statement {
    fn evaluate(&self, vm: &mut Vm) -> CacauResult {
        match self {
            Self::Empty => {}
            Self::Expression(expression) => {
                expression.evaluate(vm)?;
            }
            Self::Assignment(identifier, expression) => {
                let value = expression.evaluate(vm)?;
                vm.assign_variable(identifier.clone(), value);
            }
            Self::FunctionDeclaration(identifier, arguments, body) => {
                let function = Function::new(arguments.clone(), body.to_vec());
                vm.declare_function(identifier.clone(), function);
            }
            Statement::Return(_) => todo!(),
        }
        Ok(())
    }
}

impl Interpret<Value> for Expression {
    fn evaluate(&self, vm: &mut Vm) -> CacauResult<Value> {
        match self {
            Self::Literal(value) => Ok(value.clone()),
            Self::BinaryOp(lhs, op, rhs) => {
                let lhs = lhs.evaluate(vm)?;
                let rhs = rhs.evaluate(vm)?;

                match op {
                    BinaryOperator::Add => lhs.add_op(rhs),
                    BinaryOperator::Sub => lhs.sub_op(rhs),
                    BinaryOperator::Mul => lhs.mul_op(rhs),
                    BinaryOperator::Div => lhs.div_op(rhs),
                    BinaryOperator::Less => lhs.less_op(rhs),
                    BinaryOperator::LessOrEqual => lhs.less_or_equal_op(rhs),
                    BinaryOperator::Equals => lhs.equals_op(rhs),
                    BinaryOperator::NotEquals => lhs.not_equals_op(rhs),
                    BinaryOperator::Greater => lhs.greater_op(rhs),
                    BinaryOperator::GreaterOrEqual => lhs.greater_or_equal_op(rhs),
                }
            }
            Self::UnaryOp(op, expression) => {
                let value = expression.evaluate(vm)?;

                match op {
                    UnaryOperator::Not => value.not_op(),
                    UnaryOperator::Minus => value.minus_op(),
                }
            }
            Self::Variable(identifier) => vm.get_variable(identifier),
            Self::FunctionCall(identifier, arguments) => {
                let [expression] =
                    check_and_return_arguments_quantity::<Expression, 1>(arguments, identifier)?;
                let value = expression.evaluate(vm)?;

                // Check built-in functions
                match identifier.as_str() {
                    "dbg" => {
                        println!("{value}");
                        return Ok(value);
                    }
                    "println" => {
                        println!("{value}");
                        return Ok(Value::Unit);
                    }
                    "print" => {
                        print!("{value}");
                        return Ok(Value::Unit);
                    }
                    "assert" => {
                        let value = match value {
                            Value::Bool(value) => value,
                            value => {
                                return Err(CacauError::FunctionWrongArgumentType(
                                    "assert",
                                    "bool",
                                    value.variant_name(),
                                ))
                            }
                        };

                        if !value {
                            return Err(CacauError::AssertionError);
                        }

                        return Ok(Value::Unit);
                    }
                    _ => {}
                }

                // Check user-defined functions
                let function = vm.get_function(identifier)?;

                vm.assign_variable("x".into(), value);
                for statement in &function.body {
                    if let Statement::Return(Some(expression)) = statement {
                        return expression.evaluate(vm);
                    }

                    statement.evaluate(vm)?;
                }

                Ok(Value::Unit)
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Vm {
    variables: HashMap<Identifier, Value>,
    functions: HashMap<Identifier, Function>,
}

impl Vm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn declare_function(&mut self, identifier: Identifier, function: Function) {
        self.functions.insert(identifier, function);
    }

    pub fn get_function(&mut self, identifier: &Identifier) -> CacauResult<Function> {
        self.functions
            .get(identifier)
            .cloned()
            .ok_or_else(|| CacauError::UndefinedFunction(identifier.clone()))
    }

    pub fn assign_variable(&mut self, identifier: Identifier, value: Value) {
        self.variables.insert(identifier, value);
    }

    pub fn get_variable(&mut self, identifier: &Identifier) -> CacauResult<Value> {
        self.variables
            .get(identifier)
            .cloned()
            .ok_or_else(|| CacauError::UndefinedVariable(identifier.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    // Only one for now
    // pub arguments: Vec<Identifier>,
    pub arguments: Identifier,
    pub body: Vec<Statement>,
}

impl Function {
    fn new(arguments: Identifier, body: Vec<Statement>) -> Self {
        Self { arguments, body }
    }
}

fn check_and_return_arguments_quantity<T: Clone, const N: usize>(
    arguments: &[T],
    identifier: &Identifier,
) -> CacauResult<[T; N]> {
    if arguments.len() != N {
        return Err(CacauError::WrongAmountOfArguments(
            identifier.clone(),
            N,
            arguments.len(),
        ));
    }

    let mut args = arguments.iter();

    Ok(array::from_fn(|_| args.next().cloned().unwrap()))
}
