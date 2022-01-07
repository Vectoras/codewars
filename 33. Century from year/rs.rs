// 33. Century from year
// https://www.codewars.com/kata/5a3fe3dde1ce0e8ed6000097

fn century(year: u32) -> u32 {
  //     if year % 100 == 0 { year / 100 } else { year / 100 + 1 }
      (year as f32 / 100.0).ceil() as u32
  }