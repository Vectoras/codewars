// 50. Replace With Alphabet Position
// https://www.codewars.com/kata/546f922b54af40e1e90001da

fn alphabet_position(text: &str) -> String {
    
  //     println!("{}", text);
      
      let mut result = String::new();
      
      for c in text.chars() {
          if c >= 'a' && c <= 'z' {
              let l = format!("{} ", c as u8 - 'a' as u8 + 1);
              result.push_str(&l);      
          } else if c >= 'A' && c <= 'Z' {
              let l = format!("{} ", c as u8 - 'A' as u8 + 1);
              result.push_str(&l);
          }
      }
      
      result.remove(result.len()-1);
      
      return result;
  }