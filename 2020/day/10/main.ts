const parse = (filename: string) =>
  Deno.readTextFile(filename)
    .then((input) =>
      input
        .split("\n")
        .filter((x) => x.length)
        .map((x) => parseInt(x))
    );

const sort = (numbers: number[]) => [
  0,
  ...numbers.sort((a, b) => a - b),
  Math.max(...numbers) + 3,
];

const distance = (sorted: number[]) =>
  sorted
    .map((x, i, a) => x - a[i - 1])
    .slice(1);

const distanceCount = (distances: number[]) =>
  distances.reduce(
    ([ones, threes], val) => ([
      ones + (val === 1 ? 1 : 0),
      threes + (val === 3 ? 1 : 0),
    ]),
    [0, 0],
  );

const options = (sorted: number[]) =>
  sorted
    .reduce((res, item, index, arr) => ({
      ...res,
      [item]:
        arr.slice(index + 1, index + 4).filter((x) => x - item <= 3),
    }), {} as Record<string, [number, number, number, number[]]>);

const testNumbers = await parse("test.txt");
const testSorted = sort(testNumbers);
const testDistances = distance(testSorted);
const testOptions = options(testSorted);

//console.log({ testSorted });
//console.log({ testDistances });
//console.log({ testOptions });
//console.log("2^3 * 7^4")

const sorted = sort(await parse("input.txt"));
const distances = distance(sorted);
const [ones, threes] = distanceCount(distances);

console.log(ones * threes)

const nextPointers = [];

for (let i = 0; i < sorted.length; i++) {
  const thisValue = sorted[i];
  const nextOptions = sorted.filter((v) => v > thisValue && v <= thisValue + 3);

  nextPointers[i] = nextOptions;
}

const count = sorted.map((x) => 0);

for (let i = sorted.length - 1; i >= 0; i--) {
  if (nextPointers.length - 1 === i) {
    count[i] = 1;
  } else {
    count[i] = nextPointers[i]
      .map((v) => {
        const index = sorted.indexOf(v);
        return count[index];
      })
      .reduce((agg, val) => agg + val, 0);
  }
}

console.log(count[0]);

