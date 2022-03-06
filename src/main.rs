use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::sync::RwLock;
use std::str::FromStr;
use strum_macros::EnumString;

use Func::*;

#[derive(Debug, Clone)]
enum Type {
  Num(usize),
  Char(char),
  Lazy {
    arg_vec: Vec<Type>,
    vec: Vec<String>,
  },
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Func {
  Add,
  Sub,
  Lt,
  If,
  nul,
  one,
  Nul,
  Chr,
  Ord,
  Bruh,
}

lazy_static! {
  pub static ref FUNC_HASH: HashMap<&'static str, Vec<Func>> = [
    ("+", vec![Add]),
    ("<", vec![Lt]),
    ("?", vec![If]),
    ("₀", vec![nul]),
    ("₁", vec![one]),
    ("⁰", vec![Nul]),
    ("-", vec![Sub]),
    ("c", vec![Bruh, Chr, Ord]),
  ]
  .into_iter()
  .collect();
  pub static ref LINE_FUNC_VEC: RwLock<Vec<Vec<String>>> = RwLock::new(vec![]);
}

fn main() {
  /* let code = r###"? ₀ ₁ ⁰ ⁰ < ⁰ 3
  + ⁰ 1"###; */
  // let code = r###"? c 56 32 3 5"###;
  let code = r###"c '3'"###;

  for line_func_str in code.split("\n") {
    LINE_FUNC_VEC.write().unwrap().push(
      line_func_str
        .split(" ")
        .map(str::to_string)
        .collect::<Vec<_>>(),
    );
  }

  let arg_vec = vec![Type::Num(1)];
  let line_func_beg = LINE_FUNC_VEC.read().unwrap()[0].clone();

  let mut typ = Type::Lazy {
    arg_vec,
    vec: line_func_beg.clone(),
  };

  println!("{:?}", solve(&mut typ));
}

fn solve(typ_o: &mut Type) -> Option<Type> {
  use Type::*;
  let top = match typ_o {
    Lazy { vec, .. } => vec.remove(0),

    _ => unreachable!(),
  };

  match FUNC_HASH.get(&top.as_str()) {
    Some(f_vec) => {
      f_vec
        .into_iter()
        .map(|f| {
          let typ = &mut typ_o.clone();
          match f {
            Bruh => {
              match (consume(typ), consume(typ)) {
                (Some(Num(a)), Some(Num(b))) => Some(Num(a * 69 + b / 420)),
                _ => None, // _ => unreachable!()
              }
            }

            Chr => {
              match consume(typ) {
                Some(Num(a)) => Some(Char((a as u8) as char)),
                _ => None, // _ => unreachable!()
              }
            }

            Ord => {
              match consume(typ) {
                Some(Char(a)) => Some(Num(a as usize)),
                _ => None, // _ => unreachable!()
              }
            }

            Lt => match (consume(typ), consume(typ)) {
              (Some(Num(a)), Some(Num(b))) => {
                if a < b {
                  Some(Num(1))
                } else {
                  Some(Num(0))
                }
              }
              _ => None,
            },

            Sub => match (consume(typ), consume(typ)) {
              (Some(Num(a)), Some(Num(b))) => Some(Num(a - b)),
              _ => None,
            },

            Add => match (consume(typ), consume(typ)) {
              (Some(Num(a)), Some(Num(b))) => Some(Num(a + b)),
              _ => None,
            },

            If => {
              let tru = consume_lazy(typ); 
              let fal = consume_lazy(typ);

              let cond = match consume(typ) {
                Some(Num(n)) => n,

                _ => {return None;}
              };

              if cond != 0 {
                val(tru)
              } else {
                val(fal)
              }
            }

            Nul => match typ {
              Lazy { arg_vec, .. } => Some(arg_vec[0].clone()),
              _ => None,
            },

            nul => solve_line_func(0, typ),
            one => solve_line_func(1, typ),
            _ => panic!("not implemented"),
          }
        })
        .find(|t| t.is_some())
        .unwrap()
    }
    None => parse_token(top),
  }
}

fn solve_lazy(typ: &mut Type) -> Vec<String> {
  let top = match typ {
    Type::Lazy { vec, .. } => vec.remove(0),

    _ => unreachable!(),
  };
  let mut vec_ret = vec![top.clone()];

  vec_ret.extend(match FUNC_HASH.get(&top.as_str()) {
    // grabbing the first one doesn't work if the char maps to functions of differing lengths sadge pp
    Some(f_vec) => match f_vec.into_iter().nth(0).unwrap() {
        Bruh => solve_lazy_consume(typ, 2),
        Chr => solve_lazy_consume(typ, 1),
        Ord => solve_lazy_consume(typ, 1),
        Add => solve_lazy_consume(typ, 2),
        Sub => solve_lazy_consume(typ, 2),
        Lt => solve_lazy_consume(typ, 2),
        If => solve_lazy_consume(typ, 3),
        Nul => vec![],
        nul => solve_lazy_consume(typ, lf_param_count(0)),
        one => solve_lazy_consume(typ, lf_param_count(1)),

        _ => panic!("not implemented"),
      // }
    },
    None => vec![],
  });

  vec_ret.into_iter().collect::<Vec<_>>()
}

fn solve_line_func(line: usize, typ: &mut Type) -> Option<Type> {
  solve(&mut Type::Lazy {
    vec: LINE_FUNC_VEC.read().unwrap()[line].clone(),
    arg_vec: (0..lf_param_count(line))
      .map(|_| consume(typ).unwrap())
      .collect::<Vec<_>>(),
  })
}

fn lf_param_count(line: usize) -> usize {
  let line_func_vec = LINE_FUNC_VEC.read().unwrap()[line].clone();
  let num_hash: HashMap<&'static str, usize> = [("⁸", 5), ("⁶", 4), ("⁴", 3), ("²", 2), ("⁰", 1)]
    .into_iter()
    .collect();

  match num_hash
    .into_iter()
    .find(|(s, _n)| line_func_vec.contains(&s.to_string()))
  {
    Some((_s, n)) => n,
    None => 0,
  }
}
fn solve_lazy_consume(typ: &mut Type, param_count: usize) -> Vec<String> {
  (0..param_count)
    .map(|_| solve_lazy(typ))
    .flatten()
    .collect::<Vec<_>>()
}

fn val(typ: Type) -> Option<Type> {
  solve(&mut typ.clone())
}

fn consume_lazy(typ: &mut Type) -> Type {
  let typ_solve_vec = solve_lazy(typ);
  // typ_solve_vec.into_iter().map(|t| {

    Type::Lazy {
      arg_vec: match typ.clone() {
        Type::Lazy { arg_vec, .. } => arg_vec,

        _ => unreachable!(),
      },
      vec: typ_solve_vec,
    }
  // }).collect()

  // todo!()
}

fn consume(typ: &mut Type) -> Option<Type> {
  let a = match typ.clone() {
    Type::Lazy { vec, .. } => match vec.clone().into_iter().nth(0) {
      Some(t) => t,
      None => return None
    }

    _ => unreachable!(),
  };

  match parse_token(a) {
    Some(t) => match typ {
      Type::Lazy { vec, .. } => {
        vec.remove(0);
        Some(t)
      }

      _ => unreachable!(),
    },

    None => solve(typ),
  }
}

fn parse_token(token: String) -> Option<Type> {
  let num_re = Regex::new(r###"^(\d+)$"###).unwrap();
  let char_re = Regex::new(r###"^'(.)'$"###).unwrap();
  // let str = "'3'";
  if let Some(cap) = num_re.captures(token.as_str()) {
    Some(Type::Num(
      cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
    ))
    // println!("{:#?}", cap.get(1).unwrap().as_str());
  } else if let Some(cap) = char_re.captures(token.as_str()) {
    Some(Type::Char(
      cap.get(1).unwrap().as_str().chars().nth(0).unwrap(),
    ))
  } else {
    None
  }
}
