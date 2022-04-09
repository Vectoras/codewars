// 22 -> Is a number prime?
// https://www.codewars.com/kata/5262119038c0985a5b00029f

function isPrime(num) {
  console.log('\n-----------------\n');
  console.log('num: ', num);

  if (num < 1) return false;

  if (num === 2 || num === 3) return true;

  if (num % 2 === 0 || num % 3 === 0) return false;

  for (let i = 1; 6 * i + 1 <= num / 2; i++) {
    console.log('\ni: ', i);
    console.log(`6n-1: ${6 * i - 1}`);
    console.log(`6n+1: ${6 * i + 1}`);
    if (num % (6 * i - 1) === 0 || num % (6 * i + 1) === 0) {
      return false;
    }
  }

  return num > 1;
}

// !!! Needs optimisation to fit into the 12s limit on codewars for their tests (numbers up to 2^31)
