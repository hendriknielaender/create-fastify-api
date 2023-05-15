import fastify from 'fastify';
import awsLambdaFastify from '@fastify/aws-lambda'
import config from './plugins/config';
import routes from './routes/index';

const server = fastify({
  ajv: {
    customOptions: {
      removeAdditional: "all",
      coerceTypes: true,
      useDefaults: true,
    }
  },
  logger: {
    level: process.env['LOG_LEVEL'] ?? "info",
  },
});

await server.register(config);
await server.register(routes);
export const handler = awsLambdaFastify(server);
await server.ready();

export default server;
