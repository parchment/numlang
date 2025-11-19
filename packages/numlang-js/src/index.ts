import { toWords } from "./cardinal";
import { toOrdinal, toWordsOrdinal } from "./ordinal";
import { fromWords } from "./parse";
import { tokenise } from "./tokenise";
import { expandUnit, abbreviateUnit } from "./unit";
import { toPlural, toSingular } from "./plural";

const numlang = {
  toWords,
  toOrdinal,
  toWordsOrdinal,
  fromWords,
  tokenise,
  expandUnit,
  abbreviateUnit,
  toPlural,
  toSingular,
};

export {
  toWords,
  toOrdinal,
  toWordsOrdinal,
  fromWords,
  tokenise,
  expandUnit,
  abbreviateUnit,
  toPlural,
  toSingular,
};
export default numlang;
