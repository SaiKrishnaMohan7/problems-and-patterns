/**
 * Encode and Decode Strings
 *
 * Pattern: arrays-hashing
 * Difficulty: Medium
 *
 * Approach:
 * Use length-prefix encoding: For each string, store its length followed by a delimiter,
 * then the string itself. This way even if the delimiter appears in the string content,
 * we know exactly how many characters to read based on the length prefix.
 *
 * Format: {length}{delimiter}{string}{length}{delimiter}{string}...
 * Example: ["hello", "world"] -> "5%hello5%world"
 *
 * Time Complexity: O(n) where n is total length of all strings
 * Space Complexity: O(n) for the encoded string
 */

export function encode(strs: string[]): string {
  let encodedStr: string;
  let delimiter: string;

  encodedStr = '';
  delimiter = '%';

  for (let i = 0; i < strs.length; i++) {
    const str = strs[i];
    const strLength = str.length;
    encodedStr = `${encodedStr}${strLength}${delimiter}${str}`;
  }

  return encodedStr;
}

export function decode(encodedStr: string): string[] {
  let result: string[];
  let i: number;
  let delimiter: string;

  result = [];
  i = 0;
  delimiter = '%';

  while (i < encodedStr.length) {
    let j = i;

    // Find the delimiter to get the length
    while (encodedStr[j] !== delimiter) {
      j++;
    }

    const strLength = parseInt(encodedStr.slice(i, j));
    const beginningIdxOfStr = j + 1;
    const endingIdxOfStr = beginningIdxOfStr + strLength;
    const str = encodedStr.slice(beginningIdxOfStr, endingIdxOfStr);
    result.push(str);

    i = endingIdxOfStr;
  }

  return result;
}