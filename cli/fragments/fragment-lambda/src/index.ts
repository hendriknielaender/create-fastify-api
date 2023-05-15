import * as url from 'node:url';
import server from 'server'


function isMainModule() {
  if (import.meta.url.startsWith('file:')) { 
    const modulePath = url.fileURLToPath(import.meta.url);
    if (process.argv[1] === modulePath) { 
      return true;
    }
  }
  return false;
}

process.on('unhandledRejection', (err) => {
  console.error(err);
  process.exit(1);
});

const port = +server.config.API_PORT;
const host = server.config.API_HOST;

if (!isMainModule()) {
  // called directly i.e. "node app"
  server.listen({ host, port }, (err) => {
    if (err) console.error(err)
    console.log(`server listening on ${port}`)
  })
} 

export default server;