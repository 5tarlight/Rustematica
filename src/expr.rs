use std::fmt::{Debug, Display};

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

impl Vars {
    /// Convert variable as printable str (`&'static str`)
    ///
    /// # Example
    /// ```
    /// use rustemetica::expr::Vars;
    /// assert_eq!(Vars::a.to_str(), "a");
    /// assert_eq!(Vars::z.to_str(), "z");
    /// ```
    pub fn to_str(self) -> &'static str {
        use Vars::*;

        match self {
            a => "a",
            b => "b",
            c => "c",
            d => "d",
            e => "e",
            f => "f",
            g => "g",
            h => "h",
            i => "i",
            j => "j",
            k => "k",
            l => "l",
            m => "m",
            n => "n",
            o => "o",
            p => "p",
            q => "q",
            r => "r",
            s => "s",
            t => "t",
            u => "u",
            v => "v",
            w => "w",
            x => "x",
            y => "y",
            z => "z",
            // _ => "?",
        }
    }
}

/// Expression type.
/// This typically means *function type* in mathematics
/// such as `cos`, `sin` and others.
/// Inner values indicate operands.
/// Order is down to up.
#[derive(Debug)]
pub enum AtomicExpr {
    // Constant(f64),
    Poly(Poly),
    Constant(Const),
    Exponent(),
    Log(),
}

/// Dif trait means a struct which implements Dif
/// can be differentiated.
pub trait Dif: Debug {
    /// Differentiate self.
    fn differentiate(&self) -> AtomicExpr
    where
        Self: Sized;
}

/// This struct is one term of Polynonmial Expression.
#[derive(Debug)]
pub struct Poly {
    /// Variable of term.
    pub var: Vars,
    /// Coefficient of term.
    pub coe: f64,
    /// Exponent of term.
    pub ex: f64,
}

impl Poly {
    pub fn from(coe: f64, var: Vars, ex: f64) -> Self {
        Self { var, coe, ex }
    }
}

impl Dif for Poly {
    /// Differentiate polynomial expression.
    /// `ax^b` will be `abx^(b-1)`.
    fn differentiate(&self) -> AtomicExpr
    where
        Self: Sized,
    {
        let coef = self.coe * self.ex;
        let exp = self.ex - 1f64;

        if self.ex != 1. {
            let poly = Poly {
                var: self.var,
                coe: coef,
                ex: exp,
            };

            AtomicExpr::Poly(poly)
        } else {
            let con = Const { value: self.coe };
            AtomicExpr::Constant(con)
        }
    }
}

impl Display for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ex == 1. {
            write!(f, "{}{}", self.coe, self.var.to_str())
        } else {
            write!(f, "{}{}^{}", self.coe, self.var.to_str(), self.ex)
        }
    }
}

/// This struct means Constant value
/// such as `5`, `-12`.
/// When differentiated, it would be `0f64`
#[derive(Debug)]
pub struct Const {
    pub value: f64,
}

impl Const {
    pub fn from(value: f64) -> Self {
        Self { value }
    }

    pub fn to_f64(&self) -> f64 {
        self.value
    }
}

impl Display for Const {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Dif for Const {
    /// Differentiate constant.
    /// Every differentiated constant will be `0f64`.
    fn differentiate(&self) -> AtomicExpr
    where
        Self: Sized,
    {
        let con = Const { value: 0f64 };
        AtomicExpr::Constant(con)
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    pub fn test_poly() {
        let x_squre = Poly::from(1f64, Vars::x, 2f64);
        if let AtomicExpr::Poly(poly) = x_squre.differentiate() {
            assert_eq!(poly.coe, 2f64);
            assert_eq!(poly.var, Vars::x);
            assert_eq!(poly.ex, 1f64);
        } else {
            panic!("Differentiated value should be Poly!");
        }
    }

    #[test]
    pub fn vars_to_string() {
        assert_eq!(Vars::a.to_str(), "a");
        assert_eq!(Vars::b.to_str(), "b");
        assert_eq!(Vars::c.to_str(), "c");
        assert_eq!(Vars::d.to_str(), "d");
        assert_eq!(Vars::e.to_str(), "e");
        assert_eq!(Vars::f.to_str(), "f");
        assert_eq!(Vars::g.to_str(), "g");
        assert_eq!(Vars::h.to_str(), "h");
        assert_eq!(Vars::i.to_str(), "i");
        assert_eq!(Vars::j.to_str(), "j");
        assert_eq!(Vars::k.to_str(), "k");
        assert_eq!(Vars::l.to_str(), "l");
        assert_eq!(Vars::m.to_str(), "m");
        assert_eq!(Vars::n.to_str(), "n");
        assert_eq!(Vars::o.to_str(), "o");
        assert_eq!(Vars::p.to_str(), "p");
        assert_eq!(Vars::q.to_str(), "q");
        assert_eq!(Vars::r.to_str(), "r");
        assert_eq!(Vars::s.to_str(), "s");
        assert_eq!(Vars::t.to_str(), "t");
        assert_eq!(Vars::u.to_str(), "u");
        assert_eq!(Vars::v.to_str(), "v");
        assert_eq!(Vars::w.to_str(), "w");
        assert_eq!(Vars::x.to_str(), "x");
        assert_eq!(Vars::y.to_str(), "y");
        assert_eq!(Vars::z.to_str(), "z");
    }

    #[test]
    pub fn create_const() {
        let five = Const::from(5.);
        assert_eq!(five.value, 5f64);
    }

    #[test]
    pub fn dif_const() {
        if let AtomicExpr::Constant(con) = Const::from(5.).differentiate() {
            assert_eq!(con.value, 0f64);
        } else {
            panic!("Differentiated value should be zero");
        }
    }

    #[test]
    pub fn dif_poly_to_const() {
        if let AtomicExpr::Constant(con) = Poly::from(3., Vars::x, 1.).differentiate() {
            assert_eq!(con.to_f64(), 3.);
        } else {
            panic!("Differentiated value should be Constant");
        }
    }
}
