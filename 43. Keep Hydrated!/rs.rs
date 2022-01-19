// 43. Keep Hydrated!
// https://www.codewars.com/kata/582cb0224e56e068d800003c

fn litres(time: f64) -> i32 {
  return (time * 0.5).trunc() as i32;
}