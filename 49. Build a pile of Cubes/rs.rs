// 49. Build a pile of Cubes
// https://www.codewars.com/kata/5592e3bd57b64d00f3000047

fn find_nb(m: u64) -> i32 {
    
  let mut v: u64 = 0;
  let mut n: u64 = 0;
  
  while v < m {
      n += 1;
      v += n*n*n;        
  }
  
  if v == m { return n as i32; }
  else { return -1; }
}