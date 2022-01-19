// 42. Square(n) Sum
// https://www.codewars.com/kata/515e271a311df0350d00000f

fn square_sum(mut vec: Vec<i32>) -> i32 {    
  let mut s:i32 = 0;
  
  while let Some (n) = vec.pop() {
      s += n*n;
  }
  
  return s;
}