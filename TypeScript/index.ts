/*
Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

Generate list randomly
*/

import { generateSortedList } from "./generators/index.ts";
import { getMedian, getMode } from "./stats/index.ts";

const range = parseInt(Deno.args[0]);

const list: number[] = generateSortedList(range);

console.log("sorted list: ", list);
console.log();

const median = getMedian(list);

console.log("median: ", median);

let mode = getMode(list);

console.log("mode: ", mode);
