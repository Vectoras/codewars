// 56. Smallest unused ID
// https://www.codewars.com/kata/55eea63119278d571d00006a

fn next_id(ids: &[usize]) -> usize {
    
  if ids == [] {
      return 0;
  }
  
  let mut work_vec: Vec<usize> = ids.to_vec();
  work_vec.sort();
  work_vec.dedup();
  
  if work_vec[work_vec.len()-1] == work_vec.len()-1 {
      return work_vec.len();
  }
  
  for i in 0..work_vec.len() {
      if i != work_vec[i] {
          return i;
      }
  }
  
  return 0;
}