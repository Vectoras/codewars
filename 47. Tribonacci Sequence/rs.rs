// 47. Tribonacci Sequence
// https://www.codewars.com/kata/556deca17c58da83c00002db

fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    
  let mut sequence :Vec<f64> = Vec::new();
  
  if n==0 { return sequence; }
  
  if n>0 {sequence.push(signature[0]);}
  if n>1 {sequence.push(signature[1]);}
  if n>2 {sequence.push(signature[2]);}
  
  for i in 3..n {         
      sequence.push(sequence[i-1] + sequence[i-2] + sequence[i-3]);        
  }
  
  return sequence;
}