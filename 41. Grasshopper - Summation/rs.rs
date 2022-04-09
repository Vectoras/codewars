// 41. Grasshopper - Summation
// https://www.codewars.com/kata

fn summation(n: i32) -> i32 {    
  let mut sum:i32 = 0;    
  for i in 1..n+1 {
      println!("i: {}", i);
      sum += i;
  }    
  return sum;
}