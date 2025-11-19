import { toWords } from "./cardinal";
import { toOrdinal, toWordsOrdinal } from "./ordinal";
import { fromWords } from "./parse";
import { tokenise } from "./tokenise";
import { expandUnit, abbreviateUnit } from "./unit";

const numlang = {
  toWords,
  toOrdinal,
  toWordsOrdinal,
  fromWords,
  tokenise,
  expandUnit,
  abbreviateUnit,
};

export {
  toWords,
  toOrdinal,
  toWordsOrdinal,
  fromWords,
  tokenise,
  expandUnit,
  abbreviateUnit,
};
export default numlang;
