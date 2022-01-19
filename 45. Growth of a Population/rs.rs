// 45. Growth of a Population
// https://www.codewars.com/kata/563b662a59afc2b5120000c6

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 { 
    
  let percentage :f32 = (percent / 100.0) as f32;
  let mut current_population :f32 = p0 as f32;
  let mut n_years :i32 = 0;
  
  while current_population < p as f32{
      n_years += 1;
      current_population += (current_population*percentage).trunc() + aug as f32;
  }
  
  return n_years;
}