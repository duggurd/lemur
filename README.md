# lemur

lemur pronounced lemur, is a simple (and insecure) http tunneling tool i built to be able to access my self-hosted kubernetes cluster behind a firewall on a private network.

Currently only supports one connection. But this could be expanded to many connections.

## how it works

lemur uses a free compute engine instance in google cloud as a proxy/man in the middle. This works because the instance is exposed on the public internet meaning it is possible to connect to that machine from inside a private network and establish a "tunnel". Subsequently this tunnel can be used to send traffick the other way.

The proxy in the cloud waits for two connections, one from the server (where commands are to be executed) and from a client(where commands are sent).

This is a "one-way" tunnel from client to server.

## Roadmap

- TLS
- Manage multiple connections
