// 34. Convert a String to a Number!
// https://www.codewars.com/kata/544675c6f971f7399a000e79

fn string_to_number(s: &str) -> i32 {
  //     s.parse::<i32>().unwrap()
      s.parse().unwrap()    
  }