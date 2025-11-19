import { toWords } from "./cardinal";
import { toOrdinal, toWordsOrdinal } from "./ordinal";
import { fromWords } from "./parse";
import { tokenise } from "./tokenise";

const numlang = {
  toWords,
  toOrdinal,
  toWordsOrdinal,
  fromWords,
  tokenise,
};

export { toWords, toOrdinal, toWordsOrdinal, fromWords, tokenise };
export default numlang;
