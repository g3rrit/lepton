
macro_rules! comp_warn {
  ($fmt:expr) => (print!(concat!("Compiler Warning|: ", $fmt, "\n")));
  ($fmt:expr, $($arg:tt)*) => (print!(concat!("Compiler Warning|: ", $fmt, "\n"), $($arg)*));
}

pub fn enum_eq<E>(e0: &E, e1: &E) -> bool {
  std::mem::discriminant(e0) == std::mem::discriminant(e1)
}



// -- EXPERIMENTAL ----------------------

/*
pub fn cast<T, U>(e: T) -> U {
  unsafe { std::mem::transmute::<T, U>(e) }
}
*/

pub type void = u32;
pub type void_ptr = *mut void;

macro_rules! comb_fold {
  ( $nt:ty, $($mem_id:ident, $mem_ty:ty)* ) => {{
    fn _fold_fn(node_vec: Vec<void_ptr>) -> void_ptr {
      node_it = node_vec.iter();
      unsafe {
        Box::new($nt {
          $( $mem_id: *cast<*void, *$mem_ty>(node_it.next()))*
        });
      }
    }
  }}
}

