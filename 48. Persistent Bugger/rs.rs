// 48. Persistent Bugger.
// https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec

fn persistence(num: u64) -> u64 {
  let mut k :u64 = 0;
  let mut n :u64 = num;
  
  while n > 9 {
      let mut p :u64 = 1;
      
      while n > 0 {
          p *= n%10;
          n /= 10;
      }
      
      n = p;
      k += 1;
  }
  
  return k;
}