use std::fmt::Debug;

/// List of available types of `Expression`.
/// This enum is designed for used as field of `Expr` struct.
///
/// # See Also
/// [Expr]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AtomicType {
    /// Expression Type: `Literal`.
    /// Literal is called as `constant` in mathematics.
    /// Literal indicates decimal numberic constant.
    /// Some special constants (such as `e` and `pi`) is belonged to `Constant`.
    Literal,

    /// Special constant.
    /// Special constants are well-known mathematical constants
    /// such as `e` and `pi`
    Constant,

    /// Unknown variables.
    /// In most case `x` is expected to be used as.
    /// Completely unknown variables (except for `x` and `y`)
    Var,

    /// Other Expressions.
    /// With this type, Expression is no longer *atomic*.
    /// And this means this expression stage is consists of subsequent
    /// expressions and also will cause reculsive function calls.
    /// If expressions are too recursived to over the reculsive function call
    /// limitation, it may cause unexpected crash.
    Ex,
}

/// Lists of available unknown variables.
/// Some chars such as `x`, `y`, `e` are pre-defined.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Vars {
    a,
    b,
    c,
    d,
    /// This variable is pre-defined as *Euler's number `e`*.
    e,
    f,
    g,
    h,
    i,
    j,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    /// This variable is pre-defined as independent `x`.
    x,
    /// This variable is pre-defined as dependnet `y`.
    /// And also independent `y`
    y,
    z,
}

/// Expression type.
/// This typically means *function type* in mathematics
/// such as `cos`, `sin` and others.
/// Inner values indicate operands.
/// Order is down to up.
#[derive(Debug, Clone, Copy)]
pub enum ExprType {
    Constant(f64),
    Poly(Vars),
    Exponent(f64, f64),
    Log(f64, f64),
    None,
}

pub trait Dif: Debug {
    fn differentiate(&self) -> Self
    where
        Self: Sized;
}

/// Expression struct.
#[derive(Debug)]
pub struct Expr {
    pub atomic_type: AtomicType,
    pub expr_type: ExprType,
    pub expr: Option<Box<dyn Dif>>,
}

impl Expr {
    pub fn from(str: String) -> Self {
        todo!()
    }
}

impl Dif for Expr {
    fn differentiate(&self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}

#[derive(Debug)]
pub struct Poly {
    pub var: Vars,
    pub coe: f64,
    pub ex: f64,
}

impl Poly {
    pub fn from(coe: f64, var: Vars, ex: f64) -> Self {
        Self { var, coe, ex }
    }
}

impl Dif for Poly {
    fn differentiate(&self) -> Self
    where
        Self: Sized,
    {
        let coef = self.coe * self.ex;
        let exp = self.ex - 1f64;

        Self {
            var: self.var,
            coe: coef,
            ex: exp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_poly() {
        let x_squre = Poly::from(1f64, Vars::x, 2f64);
        let x_squire_prime = x_squre.differentiate();

        assert_eq!(x_squire_prime.coe, 2f64);
        assert_eq!(x_squire_prime.var, Vars::x);
        assert_eq!(x_squire_prime.ex, 1f64);
    }
}
