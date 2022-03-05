use lazy_static::lazy_static;
use std::sync::RwLock;
use std::collections::HashMap;
use Func::*;

#[derive(Debug, Clone)]
struct Type {
  arg_vec: Vec<usize>,
  vec: Vec<String>,
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
}

lazy_static! {
  pub static ref FUNC_HASH: HashMap<&'static str, Func> = [
    ("+", Add),
    ("<", Lt,),
    ("?", If),
    ("₀", nul),
    ("₁", one),
    ("⁰", Nul),
    ("-", Sub)
  ]
  .into_iter()
  .collect();
  pub static ref LINE_FUNC_VEC: RwLock<Vec<Vec<String>>> = RwLock::new(vec![]);
}

fn main() {
  let code = r###"? ₀ ₁ ⁰ ⁰ < ⁰ 3
+ ⁰ 1"###;

  for line_func_str in code.split("\n") {
    LINE_FUNC_VEC.write().unwrap().push(
      line_func_str
        .split(" ")
        .map(str::to_string)
        .collect::<Vec<_>>(),
    );
  }

  let arg_vec = vec![5];
  let line_func_beg = LINE_FUNC_VEC.read().unwrap()[0].clone();

  let mut typ = Type {
    arg_vec,
    vec: line_func_beg.clone(),
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

      Sub => {
        let a = consume(typ);
        let b = consume(typ);
        a - b
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

      nul => solve_line_func(0, typ),
      one => solve_line_func(1, typ),
      _ => panic!("not implemented"),
    },
    None => top.parse::<usize>().unwrap(),
  }
}

fn solve_line_func(line: usize, typ: &mut Type) -> usize {
  solve(&mut Type {
    vec: LINE_FUNC_VEC.read().unwrap()[line].clone(),
    arg_vec: (0..lf_param_count(line))
      .map(|_| consume(typ))
      .collect::<Vec<_>>(),
  })
}

fn solve_lazy(typ: &mut Type) -> Vec<String> {
  let top = typ.vec.remove(0);
  let mut vec_ret = vec![top.clone()];

  vec_ret.extend(match FUNC_HASH.get(&top.as_str()) {
    Some(f) => match f {
      Add => solve_lazy_consume(typ, 2),
      Sub => solve_lazy_consume(typ, 2),
      Lt => solve_lazy_consume(typ, 2),

      If => solve_lazy_consume(typ, 3),
      Nul => vec![],
      nul => solve_lazy_consume(typ, lf_param_count(0)),
      one => solve_lazy_consume(typ, lf_param_count(1)),

      _ => panic!("not implemented"),
    },
    None => vec![],
  });

  vec_ret.into_iter().collect::<Vec<_>>()
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

fn val(typ: Type) -> usize {
  // println!("val my guy {typ:#?}");
  solve(&mut typ.clone())
  // todo!()
}

fn consume_lazy(typ: &mut Type) -> Type {
  Type {
    arg_vec: typ.clone().arg_vec,
    vec: solve_lazy(typ),
  }

  // todo!()
}

fn consume(typ: &mut Type) -> usize {
  let a = typ.vec.clone().into_iter().nth(0).unwrap();
  match a.parse::<usize>() {
    Ok(t) => {
      typ.vec.remove(0);
      t
    }
    Err(_) => solve(typ),
  }
}
