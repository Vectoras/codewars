// 21 -> Format a string of names like 'Bart, Lisa & Maggie'.
// https://www.codewars.com/kata/53368a47e38700bd8300030d

function list(names) {
  return names
    .map((x) => x.name)
    .join(", ")
    .replace(/,\s(?!.*,\s)/, " & ");
}
