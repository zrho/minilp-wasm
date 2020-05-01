use wasm_bindgen::prelude::*;
use minilp;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptimizationDirection {
    Minimize,
    Maximize
}

impl Into<minilp::OptimizationDirection> for OptimizationDirection {
    fn into(self) -> minilp::OptimizationDirection {
        match self {
            OptimizationDirection::Minimize => minilp::OptimizationDirection::Minimize,
            OptimizationDirection::Maximize => minilp::OptimizationDirection::Maximize,
        }
    }
}

#[serde(rename_all = "lowercase")]
#[derive(Clone, Debug, Copy, Deserialize)]
pub enum ComparisonOp {
    Eq,
    Le,
    Ge,
}

impl Into<minilp::ComparisonOp> for ComparisonOp {
    fn into(self) -> minilp::ComparisonOp {
        match self {
            ComparisonOp::Eq => minilp::ComparisonOp::Eq,
            ComparisonOp::Le => minilp::ComparisonOp::Le,
            ComparisonOp::Ge => minilp::ComparisonOp::Ge,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct Variable(usize);

#[derive(Clone, Debug, Deserialize)]
pub struct Summand {
    variable: Variable,
    coefficient: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Constraint {
    expression: Vec<Summand>,
    comparison: ComparisonOp,
    constant: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct VariableDeclaration {
    minimum: Option<f64>,
    maximum: Option<f64>,
    coefficient: f64,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    Infeasible,
    Unbounded,
    BadFormat,
}

impl Into<Error> for minilp::Error {
    fn into(self) -> Error {
        match self {
            minilp::Error::Infeasible => Error::Infeasible,
            minilp::Error::Unbounded => Error::Unbounded,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Solution {
    objective: f64,
    values: Vec<f64>,
}

fn to_solution(solution: &minilp::Solution, vars: &[minilp::Variable]) -> Solution {
    Solution {
        objective: solution.objective(),
        values: vars.iter().map(|var| solution[*var]).collect()
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "lowercase")]
pub enum Return {
    Success(Solution),
    Error(Error)
}

#[derive(Clone, Debug, Deserialize)]
pub struct Problem {
    direction: OptimizationDirection,
    variables: Vec<VariableDeclaration>,
    constraints: Vec<Constraint>,
}

#[wasm_bindgen]
pub fn solve(problem: &JsValue) -> JsValue {
    let problem: Problem = match problem.into_serde() {
        Ok(problem) => problem,
        Err(_) => return JsValue::from_serde(&Return::Error(Error::BadFormat)).unwrap()
    };

    let mut solver = minilp::Problem::new(problem.direction.into());
    
    let solver_vars: Vec<minilp::Variable> = problem.variables.iter().map(|var| {
        let minimum = var.minimum.unwrap_or(-f64::INFINITY);
        let maximum = var.maximum.unwrap_or(f64::INFINITY);
        solver.add_var(var.coefficient, (minimum, maximum))
    }).collect();

    problem.constraints.iter().for_each(|constraint| {
        solver.add_constraint(
            constraint.expression.iter().map(|summand| {
                (solver_vars[summand.variable.0], summand.coefficient)
            }),
            constraint.comparison.into(),
            constraint.constant
        )
    });

    let ret = match solver.solve() {
        Ok(solution) => Return::Success(to_solution(&solution, &solver_vars)),
        Err(error) => Return::Error(error.into())
    };

    JsValue::from_serde(&ret).unwrap()
}
