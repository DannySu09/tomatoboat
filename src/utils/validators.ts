import type { NewTopic } from "../db/types";

export function validateNewTopic(value: NewTopic) {
  if (value.title.length <= 3) {
    throw new Error('The length of a topic title must at least has 3 characters');
  }

  return true;
}