# lemur

lemur pronounced lemur, is a simple (and insecure) http tunneling tool i built to be able to access my self-hosted kubernetes cluster behind a firewall on a private network.

Currently only supports one connection. But this could be expanded to many connections.

# Usage

1. Start by spinning up a remote machine that has a publicly exposed ip address. I'm using `GCP Compute Engine` as they provide a free tier machine that is automatically exposed to the internet. (Checkout Terraform code)
2. Get the right binary to the remote machine ie. linux if using a loniux operating system or windows compiled if using windoes operating system.
3. Start the proxy/mitm server by running the binary `./lemur proxy 0.0.0.0:80 0.0.0.0:8080` or `./lemur.exe proxy 0.0.0.0:80 0.0.0.0:8080`, use port 80, 8080 or 443 as these are usually allowed through firewalls.
4. Start server (where commands are to be received): `./lemur server <remote-ip>:80`, should now connect to the proxy.
5. Start the client `./lemur client <remote-ip>:8080`
6. Have fun

## how it works

lemur uses a free compute engine instance in google cloud as a proxy/man in the middle. This works because the instance is exposed on the public internet meaning it is possible to connect to that machine from inside a private network and establish a "tunnel". Subsequently this tunnel can be used to send traffick the other way.

The proxy in the cloud waits for two connections, one from the server (where commands are to be executed) and from a client(where commands are sent).

This is a "one-way" tunnel from client to server.

## Roadmap

- TLS
- Manage multiple connections
- Long lived shell processes
- Recovery and state
