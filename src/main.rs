use lazy_static::lazy_static;
use std::{collections::HashMap, num};
use Func::*;

#[derive(Debug, Clone)]
struct Type {
  arg_vec: Vec<usize>,
  vec: Vec<String>,
  vec_orig: Vec<String>,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Func {
  Add,
  Sub,
  Lt,
  If,
  nul,
  Nul,
}

lazy_static! {
  pub static ref FUNC_HASH: HashMap<&'static str, Func> = [
    ("+", Add),
    ("<", Lt,),
    ("?", If),
    ("₀", nul),
    ("⁰", Nul),
    ("-", Sub)
  ]
  .into_iter()
  .collect();
}

fn main() {
  let input = "? ₀ + ⁰ 1 ⁰ < ⁰ 3";
  // if it's not 0, then return number otherwise return number + 1

  let input_vec = &mut input.split(" ").map(|s| s.to_string()).collect::<Vec<_>>();
  let arg_vec = vec![5];

  let mut typ = Type {
    arg_vec,
    vec: input_vec.clone(),
    vec_orig: input_vec.clone(),
  };

  println!("{:?}", solve(&mut typ));
}

fn solve(typ: &mut Type) -> usize {
  let top = typ.vec.remove(0);

  match FUNC_HASH.get(&top.as_str()) {
    Some(f) => match f {
      Lt => {
        let a = consume(typ);
        let b = consume(typ);
        if a < b {
          1
        } else {
          0
        }
      }

      Add => {
        let a = consume(typ);
        let b = consume(typ);
        a + b
      }

      If => {
        let tru = consume_lazy(typ);
        let fal = consume_lazy(typ);
        let cond = consume(typ);

        if cond != 0 {
          val(tru)
        } else {
          val(fal)
        }
      }

      Nul => typ.arg_vec[0],

      nul => solve(&mut Type {
        vec: typ.vec_orig.clone(),
        vec_orig: typ.vec_orig.clone(),
        arg_vec: (0..typ.arg_vec.len())
          .map(|_| consume(typ))
          .collect::<Vec<_>>(),
      }),

      _ => panic!("not implemented"),
    },
    None => top.parse::<usize>().unwrap(),
  }
}

fn solve_lazy(typ: &mut Type) -> Vec<String> {
  let top = typ.vec.remove(0);
  let mut vec_ret = vec![top.clone()];

  vec_ret.extend(match FUNC_HASH.get(&top.as_str()) {
    Some(f) => match f {
      Add => solve_lazy_consume(typ, 2),
      Lt => solve_lazy_consume(typ, 2),

      If => solve_lazy_consume(typ, 3),
      Nul => vec![],
      nul => solve_lazy_consume(typ, typ.arg_vec.len()),

      _ => panic!("not implemented"),
    },
    None => vec![],
  });

  vec_ret.into_iter().collect::<Vec<_>>()
}

fn solve_lazy_consume(typ: &mut Type, param_count: usize) -> Vec<String> {
  (0..param_count)
    .map(|_| solve_lazy(typ))
    .flatten()
    .collect::<Vec<_>>()
}

fn val(typ: Type) -> usize {
  // println!("val my guy {typ:#?}");
  solve(&mut typ.clone())
  // todo!()
}

fn consume_lazy(typ: &mut Type) -> Type {
  Type {
    arg_vec: typ.clone().arg_vec,
    vec_orig: typ.clone().vec_orig,
    vec: solve_lazy(typ),
  }

  // todo!()
}

fn consume(typ: &mut Type) -> usize {
  // let mut vec_ret = vec![];

  // for _ in 0..amount {
  let a = typ.vec.clone().into_iter().nth(0).unwrap();
  match a.parse::<usize>() {
    Ok(t) => {
      typ.vec.remove(0);
      t
    }
    Err(_) => {
      solve(typ)
      // energy_solve(typ.vec_orig.clone() ,vec, arg_vec.clone())
    }
  }
}
