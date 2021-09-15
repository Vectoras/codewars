// 21 -> Is a number prime?
// https://www.codewars.com/kata/5262119038c0985a5b00029f

function isPrime(num) {
  if (num <= 1) return false;
  for (let i = 3; i < num / 2; i + 2) if (num % i === 0) return false;
  return true;
}

// !!! Needs optimisation to fit into the 12s limit on codewars for their tests (numbers up to 2^31)
