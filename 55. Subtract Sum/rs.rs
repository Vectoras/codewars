// 55 - Subtract the Sum
// https://www.codewars.com/kata/56c5847f27be2c3db20009c3

// Rust - Subtract the Sum
// https://www.codewars.com/kata/56c5847f27be2c3db20009c3/train/rust

fn subtract_sum(n: u32) -> &'static str {

    println!("this is the number: {}", n);
    let dic_array: [&'static str; 100] = [
    "kiwi",
    "pear",
    "kiwi",
    "banana",
    "melon",
    "banana",
    "melon",
    "pineapple",
    "apple",
    "pineapple",
    "cucumber",
    "pineapple",
    "cucumber",
    "orange",
    "grape",
    "orange",
    "grape",
    "apple",
    "grape",
    "cherry",
    "pear",
    "cherry",
    "pear",
    "kiwi",
    "banana",
    "kiwi",
    "apple",
    "melon",
    "banana",
    "melon",
    "pineapple",
    "melon",
    "pineapple",
    "cucumber",
    "orange",
    "apple",
    "orange",
    "grape",
    "orange",
    "grape",
    "cherry",
    "pear",
    "cherry",
    "pear",
    "apple",
    "pear",
    "kiwi",
    "banana",
    "kiwi",
    "banana",
    "melon",
    "pineapple",
    "melon",
    "apple",
    "cucumber",
    "pineapple",
    "cucumber",
    "orange",
    "cucumber",
    "orange",
    "grape",
    "cherry",
    "apple",
    "cherry",
    "pear",
    "cherry",
    "pear",
    "kiwi",
    "pear",
    "kiwi",
    "banana",
    "apple",
    "banana",
    "melon",
    "pineapple",
    "melon",
    "pineapple",
    "cucumber",
    "pineapple",
    "cucumber",
    "apple",
    "grape",
    "orange",
    "grape",
    "cherry",
    "grape",
    "cherry",
    "pear",
    "cherry",
    "apple",
    "kiwi",
    "banana",
    "kiwi",
    "banana",
    "melon",
    "banana",
    "melon",
    "pineapple",
    "apple",
    "pineapple",
  ];
      let mut m: usize = n as usize;
      let mut sum: usize = 0;
      while m > 100 {
          while m > 0 {
          sum = sum + (m % 10) as usize;
          m = m / 10;
          }
          m = sum;
          println!("this is m: {}", m);
      }
      return dic_array[m-2];

      //return "apple";
      // this kata is broken on codewars
  }
  