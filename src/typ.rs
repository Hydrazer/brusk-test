/* use Type::*;
use Func::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash)]
pub enum Type {
  Num,
  NumL(&'static str),
  Fn(Box<Type>, Box<Type>)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Func {
  If,
  Gt
  /* Num,
  NumL(&'static str),
  Fn(Box<Type>, Box<Type>) */
}

lazy_static! {
  pub static ref FUNC_HASH: HashMap<Func, Type> = [
    (If, Num.ret(Num).ret(Num).ret(Num).ret(Num)),
    (Gt, Num.ret(Num).ret(Num).ret(Num)) 
  ].into_iter().collect();
}

/* impl Func {
  fn to_typ(&self) -> Type {
    match self {
      If => 
    }
  }
} */


#[allow(non_snake_case)]
pub fn FN(par: Type, ret: Type) -> Type {
  Fn(Box::new(par), Box::new(ret))
}

impl Type {
  pub fn ret(&self, ret: Type) -> Type {
    match self {
      Fn(par, ret_curr) => {
        FN(*par.clone(), ret_curr.ret(ret))
      }

      t => FN(t.clone(), self.clone())
    }
  }

  pub fn par_rep(&self, par: Type) -> Type {
    match self.clone() {
      Fn(par_curr, ret) => {
        if *par_curr.clone() == Num {
          FN(par, *ret.clone())
        } else {
          FN(*par_curr.clone(), ret.par_rep(par))
        }
      }

      Num => par,
      _ => panic!("invalid insert, too many arguments")
    }
    /* match self {
      Fn(par, ret_curr) => {
        FN(*par.clone(), ret_curr.ret(ret))
      }

      t => FN(t.clone(), self.clone())
    } */
  }

  pub fn IF(&self) -> Type {
    // let vec = self.consume(3);

    if vec[2].0 == "0" {
      vec[0] 
    } else {
      vec[1]
    }
  }
  /* pub fn par_rep(&self, par: &'static str) -> Type {
    NumL(par)
    // if self == 
    /* match (self, par) {
      (Num, )
    } */
  } */
  /* pub fn ret(&self, other: Type) -> Type {
    FN(self.clone(), other)  
  } */

  /* pub fn to_ret(&self) -> Type {
    match self.clone() {
      Fn(par, ret) => {
         
        // par.to_ret()
        /* match *par.clone() {
          Fn(p, r) => {
            p.to_ret()
          }
        } */
        // FN(par.to_ret(), ret.to_ret())
        /* if par.to_ret() == Num {
          ret.to_ret()
        } else {
          *par.clone()
          // **par.clone()
        } */
      }
      t => t
    }
  } */
}

impl PartialEq for Type {
  fn eq(&self, other: &Type) -> bool {
    match (self.clone(), other.clone()) {
      (Num, Num) | (NumL(_), NumL(_)) => true,
      (Fn(par_a, ret_a), Fn(par_b, ret_b)) => {
        par_a == par_b && ret_a == ret_b    
      }

      _ => false
    }
    // false
    // format!("{self:?}") == format!("{other:?}") || self.clone().to_ret() == other.clone().to_ret()
    // match (self, other) {
      // Fn(par, ret) => par == 
    // }
  }
}

#[cfg(test)]
mod eq {
  use super::*;
  /* use super::Type;
  use super::Func; */
  #[test]
  fn test() {
    dbg!(&FUNC_HASH[&Gt].par_rep(NumL("3")).par_rep(NumL("2")));
    // dbg!(&FUNC_HASH[&Gt].par_rep(NumL("3")));
    panic!();
  }

  #[test]
  fn final_gen() {
    /* assert_eq!(
      "1 2>3 5".gen(), FN()
    ) */
    // FN()
  }

  /* #[test]
  fn final() {
    assert_eq!(
      "?1 2>3 5".solve(), "2" 
    )
  } */
  
  /* fn a() {
    assert_eq!(Num.par_rep("6"),
    NumL("6"));
  } */

  /* #[test]
  fn b() {
    assert_eq!(Num.ret(Num).par_rep("6").to_ret(), Num);
    // should return a Num that you can par_rep in a function later
  } */
}


fn main() {
  let string = "?1 2<3 5";
  // If
  // (Num -> Num -> Num -> Num) (true-val -> false-val -> cond -> res)
  // Num.ret(Num).ret(Num).ret(Num)
  // Gt
  // (Num -> Num -> Num) (a -> b -> a > b ? a : 0)
  //
  // (Num -> Num -> Num -> Num).par_rep("1").par_rep("2").par_ret((Num -> Num -> Num).par_rep("2").par_rep("5"))
  // (Num -> Num -> Num -> Num).par_rep("1").par_rep("2").par_ret("0")
} */
