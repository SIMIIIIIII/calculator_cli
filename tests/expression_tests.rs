use calculator_cli::{CalcError::ExpressionConstruction, Expression, Operator::{self, Add, Multiply, NoOperator}};

#[test]
fn create_expression_fail_for_leaf_with_no_value() {
    let expression = Expression::new(
        true,
        Operator::NoOperator,
        None,
        None,
        None
    );
    assert!(expression.is_err());
    assert_eq!(ExpressionConstruction, expression.expect_err("Testing expect_err"));
}

#[test]
fn create_expression_fail_for_leaf_with_operator() {
    let expression = Expression::new(
        true,
        Operator::Add,
        Some(5.0),
        None,
        None
    );
    assert!(expression.is_err());
    assert_eq!(ExpressionConstruction, expression.expect_err("Testing expect_err"));
}

#[test]
fn create_expression_success_for_leaf() {
    let expression = Expression::new(
        true,
        Operator::NoOperator,
        Some(5.0),
        None,
        None
    );
    assert!(expression.is_ok());

    let exp = expression.unwrap();

    assert!(exp.leaf);
    assert!(exp.left.is_none());
    assert!(exp.right.is_none());
    assert!(exp.value.is_some());
    assert_eq!(NoOperator, exp.operator);
    assert_eq!(5.0, exp.value.unwrap());
}

#[test]
fn create_expression_fail_for_expr_with_no_operator() {
    let expression = Expression::new(
        false,
        Operator::NoOperator,
        None,
        None,
        None
    );
    assert!(expression.is_err());
    assert_eq!(ExpressionConstruction, expression.expect_err("Testing expect_err"));
}

#[test]
fn create_expression_fail_for_expr_with_value() {
    let expression = Expression::new(
        false,
        Operator::Add,
        Some(5.0),
        None,
        None
    );
    assert!(expression.is_err());
    assert_eq!(ExpressionConstruction, expression.expect_err("Testing expect_err"));
}

#[test]
fn create_expression_sucess() {
    let left = Expression::new(
        true,
        NoOperator,
        Some(5.0),
        None,
        None
    );
    assert!(left.is_ok());

    let right = Expression::new(
        true,
        NoOperator,
        Some(3.0),
        None,
        None
    );
    assert!(right.is_ok());

    let expression = Expression::new(
        false,
        Operator::Add,
        None,
        Some(Box::new(left.unwrap())),
        Some(Box::new(right.unwrap()))
    );
    assert!(expression.is_ok());

    let exp = expression.unwrap();

    assert!(!exp.leaf);
    assert!(exp.value.is_none());
    assert!(exp.left.is_some());
    assert!(exp.right.is_some());
    assert_eq!(Operator::Add, exp.operator);
}

#[test]
fn add_two_leaves() {
    let left = Expression::new(
        true,
        NoOperator,
        Some(5.0),
        None,
        None
    );
    assert!(left.is_ok());

    let right = Expression::new(
        true,
        NoOperator,
        Some(3.0),
        None,
        None
    );
    assert!(right.is_ok());

    let expression = Expression::new(
        false,
        Operator::Add,
        None,
        Some(Box::new(left.unwrap())),
        Some(Box::new(right.unwrap()))
    ).unwrap();

    let result = expression.eval();
    assert!(result.is_ok());
    
    assert_eq!(8.0, result.unwrap());
}

#[test]
fn eval_left_leaf_and_right_expression() {
    let left_fact = Expression::new(
        true,
        NoOperator,
        Some(5.0),
        None,
        None
    );
    assert!(left_fact.is_ok());

    let right_fact = Expression::new(
        true,
        NoOperator,
        Some(0.0),
        None,
        None
    );
    assert!(right_fact.is_ok());

    let fact_expression = Expression::new(
        false,
        Operator::Factorial,
        None,
        Some(Box::new(left_fact.unwrap())),
        Some(Box::new(right_fact.unwrap()))
    );
    assert!(fact_expression.is_ok());

    let left_mult =  Expression::new(
        true,
        NoOperator,
        Some(2.0),
        None,
        None
    );
    assert!(left_mult.is_ok());

    let expression = Expression::new(
        false,
        Multiply,
        None,
        Some(Box::new(left_mult.unwrap())),
        Some(Box::new(fact_expression.unwrap()))
    );
    assert!(expression.is_ok());

    let res = expression.unwrap().eval();

    assert!(res.is_ok());
    assert_eq!(240.0, res.unwrap());

}

#[test]
fn eval_right_leaf_and_left_expression() {
    let left_fact = Expression::new(
        true,
        NoOperator,
        Some(5.0),
        None,
        None
    );
    assert!(left_fact.is_ok());

    let right_fact = Expression::new(
        true,
        NoOperator,
        Some(0.0),
        None,
        None
    );
    assert!(right_fact.is_ok());

    let fact_expression = Expression::new(
        false,
        Operator::Factorial,
        None,
        Some(Box::new(left_fact.unwrap())),
        Some(Box::new(right_fact.unwrap()))
    );
    assert!(fact_expression.is_ok());

    let left_mult =  Expression::new(
        true,
        NoOperator,
        Some(2.0),
        None,
        None
    );
    assert!(left_mult.is_ok());

    let expression = Expression::new(
        false,
        Multiply,
        None,
        Some(Box::new(fact_expression.unwrap())),
        Some(Box::new(left_mult.unwrap()))
    );
    assert!(expression.is_ok());

    let res = expression.unwrap().eval();

    assert!(res.is_ok());
    assert_eq!(240.0, res.unwrap());

}

#[test]
fn create_simple_expression_from_str() {
    let expression = Expression::from("   -5 + 3   ");
    assert!(expression.is_ok());

    let exp = expression.unwrap();
    
    assert!(!exp.leaf);
    assert_eq!(Add, exp.operator);
    assert!(exp.left.is_some());
    assert!(exp.right.is_some());

    let left = exp.clone().left.unwrap();
    assert!(left.leaf);
    assert!(left.value.is_some());
    assert_eq!(-5.0, left.value.unwrap());

    let right = exp.clone().right.unwrap();
    assert!(right.leaf);
    assert!(right.value.is_some());
    assert_eq!(3.0, right.value.unwrap());

    
    let result = exp.eval();
    assert!(result.is_ok());

    assert_eq!(-2.0, result.unwrap());
}

#[test]
fn create_exepression_with_bracket() {
    let expression = Expression::from("   5 + 3 - ((-2*3)/2) - 9    ");
    assert!(expression.is_ok());
    let result = expression.unwrap().eval();

    assert!(result.is_ok());
    assert_eq!(2.0, result.unwrap());
}

#[test]
fn create_factorial_expression_with_bracket() {
    let expression = Expression::from("   ((2^2) + 1) !   ");
    assert!(expression.is_ok());
    let result = expression.unwrap().eval();

    assert!(result.is_ok());
    assert_eq!(120.0, result.unwrap());
}