import { assertEquals } from "https://deno.land/std@0.178.0/testing/asserts.ts";
import { getMode } from "./index.ts";

Deno.test("getMode returns the mode for a list", () => {
    const list = [1, 2, 3, 4, 4, 5];
    assertEquals(getMode(list), 4);
  });