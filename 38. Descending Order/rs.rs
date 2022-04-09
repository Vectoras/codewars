// 38. Descending Order
// https://www.codewars.com/kata/5467e4d82edf8bbf40000155

fn descending_order(x: u64) -> u64 {
  let mut digits: Vec<_> = x.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
  digits.sort();
  digits.reverse();
  let mut results: String = digits.iter().map( |&id| id.to_string()).collect(); 
  return results.parse::<u64>().unwrap();
}