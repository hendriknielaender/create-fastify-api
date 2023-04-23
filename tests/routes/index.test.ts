import server from '../../src/server';
import assert from 'assert/strict';
import { describe, test } from 'node:test';

describe('GET /', () => {
  test('Should return hello world', async () => {
    const response = await server.inject({
      method: 'GET',
      path: '/',
    });
    assert.equal(response.statusCode,200);
    assert.deepEqual(response.json(),{ hello: 'world' });
  });
});
