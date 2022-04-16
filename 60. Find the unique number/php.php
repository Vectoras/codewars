// 60. Find the unique number -> https://www.codewars.com/kata/585d7d5adb20cf33cb000235/train/php

function find_uniq($a) {
  
  // case first element different  
  if ($a[0] !== $a[1]  && $a[0] !== $a[2])
    return $a[0];
  
  // case any middle element different  
  for ($i = 1; $i < count($a)-1; $i++)
    if ($a[$i] !== $a[$i-1]  &&  $a[$i] !== $a[$i+1])
      return $a[$i];
  
  // case lat element different
  return $a[count($a)-1];
  
}