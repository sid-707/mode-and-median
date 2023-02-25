import { assertEquals } from "https://deno.land/std@0.178.0/testing/asserts.ts";
import { generateList } from "./index.ts";

Deno.test("generateList generates list from range", () => {
    const range = 3;
    const list = generateList(range);

    assertEquals(list.length, range);
  });