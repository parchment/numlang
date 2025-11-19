import { toWords } from "./cardinal";
import { toOrdinal, toWordsOrdinal } from "./ordinal";
import { fromWords } from "./parse";

const numlang = {
  toWords,
  toOrdinal,
  toWordsOrdinal,
  fromWords,
};

export { toWords, toOrdinal, toWordsOrdinal, fromWords };
export default numlang;
