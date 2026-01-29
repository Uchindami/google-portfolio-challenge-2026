function filterArray(array) {
  return array.filter((num) => num % 2 === 0);
}
function addNumbers(a, b) {
  return a + b;
}
function indexValue(array) {
  array.forEach((item, i) => console.log(item, "index of ", i));
}

indexValue([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
