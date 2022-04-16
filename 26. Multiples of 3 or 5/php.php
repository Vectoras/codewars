// 26. Multiples of 3 or 5
// https://www.codewars.com/kata/514b92a657cdc65150000006/train/php

function solution($number){
  
  if ($number < 0) return 0;
  
  $result = 0;  
  for ($i; $i < $number; $i++) {
    if ($i % 3 === 0 || $i % 5 === 0)
      $result += $i;
  }
  
  return $result;
  
}