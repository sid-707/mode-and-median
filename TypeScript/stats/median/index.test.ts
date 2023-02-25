import { assertEquals } from "https://deno.land/std@0.178.0/testing/asserts.ts";
import { getMedian } from "./index.ts";

Deno.test("getMedian returns the median for a list with even number of elements", () => {
    const list = [1, 2, 3, 4, 4, 5];
    assertEquals(getMedian(list), 3.5);
  });

Deno.test("getMedian returns the median for a list with odd number of elements", () => {
    const list = [1, 2, 3, 4, 5];
    assertEquals(getMedian(list), 3);
  });