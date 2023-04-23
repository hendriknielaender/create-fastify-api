import server from '../src/server.js';
import assert from 'assert/strict';
import { describe, test } from 'node:test';

describe('Server', () => {
  test('Should return server instance', async () => {
    assert.equal(typeof server,'object');
    await server.close();
  });
});
