function socketsPort() {
  const environmentServerPort = process.env.SERVER_PORT;
  const numberEnvironmentServerPort = Number(environmentServerPort);

  if (Number.isNaN(numberEnvironmentServerPort)) return 6001;
  return numberEnvironmentServerPort;
}

export default {
  SERVER_PORT: process.env.SERVER_PORT ?? '3001',
  SOCKETS_PORT: socketsPort(),
};
