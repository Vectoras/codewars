// 23 -> Beginner Series #3 Sum of Numbers
// https://www.codewars.com/kata/55f2b110f61eb01779000053

function getSum(a, b) {
  if (a === b) return a;

  if (a > b) {
    a = a + b;
    b = a - b;
    a = a - b;
  }

  let s = 0;
  for (let i = a; i <= b; i++) s += i;

  return s;
}
