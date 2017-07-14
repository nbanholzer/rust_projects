fn main() {
  let mut cur_tri_num : u64 = 5050;
  let mut delta: u64  = 101;
  let mut factor_count : u64= 0;
  while factor_count <= 500 {
    let mut i = 1;
    factor_count = 0;
    while i <= cur_tri_num {
      if cur_tri_num % i == 0 {
        factor_count+=1;
      }
      i+=1;
    }
    cur_tri_num += delta;
    delta+=1;
  }
  println!("{}", cur_tri_num);
}