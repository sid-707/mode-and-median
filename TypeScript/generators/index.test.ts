import { assertEquals } from "https://deno.land/std@0.178.0/testing/asserts.ts";
import { generateSortedList } from "./index.ts";

Deno.test("generateSortedList generates list from range", () => {
    const range = 3;
  const list = generateSortedList(range);

    assertEquals(list.length, range);
  });