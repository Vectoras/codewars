// 44. Beginner - Lost Without a Map
// https://www.codewars.com/kata/57f781872e3d8ca2a000007e

fn maps(values: &Vec<i32>) -> Vec<i32> {
    
  let mut result: Vec<i32> = Vec::new();
  
  for i in 0..values.len() {
      if let Some (n) = values.get(i) {
          result.push(n*2);
      }
  }
  
  return result;
}
