// Counting sheep...
// https://www.codewars.com/kata/54edbc7200b811e956000556

fn count_sheep(sheep: &[bool]) -> u8 {
  let mut results: u8 = 0;
  for i in sheep.iter() {
      if *i == true {
          results += 1;
      }
  }
  return results
}