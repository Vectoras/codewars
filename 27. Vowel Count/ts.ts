// 27. Vowel Count -> https://www.codewars.com/kata/54ff3102c1bad923760001f3

export class Kata {
  static getCount(str: string): number {
    let result: number = 0;

    [...str].forEach((c) => {
      if ('aeiou'.includes(c)) result += 1;
    });

    return result;
  }
}
