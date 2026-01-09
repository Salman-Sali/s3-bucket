use syn::{parse_str, Expr};


pub trait AsExpr {
    fn as_expr(&self) -> Expr;
}

impl AsExpr for String {
    fn as_expr(&self) -> Expr {
        match parse_str(self) {
            Ok(x) => x,
            Err(e) => panic!("Error while converting string `{}` to expr : {}", self, e),
        }
    }
}