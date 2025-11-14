import { describe, test, expect } from 'vitest';
import { encode, decode } from './solution';

describe('Encode and Decode Strings', () => {
  test('Example 1: Basic strings', () => {
    const input = ["hello", "world"];
    const encoded = encode(input);
    const decoded = decode(encoded);

    expect(decoded).toEqual(input);
  });

  test('Example 2: Single string', () => {
    const input = ["neet", "code", "love", "you"];
    const encoded = encode(input);
    const decoded = decode(encoded);

    expect(decoded).toEqual(input);
  });

  test('Example 3: Strings with delimiter character', () => {
    const input = ["we", "say", ":", "yes", "%", "!"];
    const encoded = encode(input);
    const decoded = decode(encoded);

    expect(decoded).toEqual(input);
  });

  test('Edge case: Empty array', () => {
    const input: string[] = [];
    const encoded = encode(input);
    const decoded = decode(encoded);

    expect(decoded).toEqual(input);
    expect(encoded).toBe("");
  });

  test('Edge case: Array with empty strings', () => {
    const input = ["", "", ""];
    const encoded = encode(input);
    const decoded = decode(encoded);

    expect(decoded).toEqual(input);
  });

  test('Edge case: Single empty string', () => {
    const input = [""];
    const encoded = encode(input);
    const decoded = decode(encoded);

    expect(decoded).toEqual(input);
  });

  test('Complex case: Mixed empty and non-empty strings', () => {
    const input = ["hello", "", "world", "", "test"];
    const encoded = encode(input);
    const decoded = decode(encoded);

    expect(decoded).toEqual(input);
  });

  test('Complex case: Strings with special characters', () => {
    const input = ["#@!", "123", "a b c", "new\nline"];
    const encoded = encode(input);
    const decoded = decode(encoded);

    expect(decoded).toEqual(input);
  });

  test('Complex case: Long strings', () => {
    const input = ["a".repeat(1000), "b".repeat(500)];
    const encoded = encode(input);
    const decoded = decode(encoded);

    expect(decoded).toEqual(input);
  });
});
