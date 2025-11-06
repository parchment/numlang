export interface NumberToWordsOptions {
  useCommas?: boolean;
  useAnd?: boolean;
  appendOnly?: boolean;
  uppercase?: boolean;
  capitalize?: boolean;
}

export const defaultOptions: NumberToWordsOptions = {
  useCommas: false,
  useAnd: false,
  appendOnly: false,
  uppercase: false,
  capitalize: false,
};
