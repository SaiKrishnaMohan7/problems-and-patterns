import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    include: ['problems/**/*.test.ts'],
    coverage: {
      provider: 'v8',
      include: ['problems/**/solution.ts'],
      exclude: ['**/*.test.ts', '**/node_modules/**']
    }
  }
});
