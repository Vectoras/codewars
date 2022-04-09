// 59. Square Every Digit -> https://www.codewars.com/kata/546e2562b03326a88e000020/train/typescript

export class Kata {
  static squareDigits(num: number): number {
    return [...String(num)].reduce((result: number, current: string): number => {
      const square: number = Number(current) * Number(current);
      return result * Math.pow(10, String(square).length) + square;
    }, 0);
  }
}
