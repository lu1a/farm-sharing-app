# cosmas-api

### Note from future me:
I'm going to refactor this to be an all-in-1 app. I wrote this as a CRUD API originally just so that it'd be self-evident what each endpoint does and that it's an abstraction over the CRUD actions of a database anyway. I'm feeling HTMX for the "frontend".
Anyway, here's the abstract:

The backend for a Community Sharing Management Software:
This is the backend API for a Community Sharing Management App, a platform where farms/communities/communes/homesteads/households can log their produce and inventory and see the inventories of other farms in the community. The API is built in Rust using the Rocket web framework, with a Postgres database for storing farm data.
<br />
<br />
The point is to share data and subsequently resources to your fellow connected farms, like how organs share, or how teams in companies share resources to fulfill a common goal. A farm might make a request to all its connected members, and the idea is that some other farm comes forth and shares if it can. Extra resources can and should be given to fellow farms rather than hoarding.
<br />
This API along with the frontend should be run on some old PC on each farm in a decentralised way - such that each farm holds its own data and can manage its own inventory even if internet and grid power is lost (we could also support manual physical connections between farms so that they may stay connected while being completely off-grid, but that's not in the scope of this exact API).
A farm will create a static Auth key for other farms to save and use when querying each others' APIs, and there will be another token or some other form of login to get into the frontend app.

# Installation

TODO. For now just install cargo then cargo run this.

# Endpoints

| Endpoint                                      | Description                                                                                                                    |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `/api/details` GET, PATCH                     | Actions on the details of this farm (connected farms may only GET)                                                             |
| `/api/resources` GET                          | Returns a list of all resources                                                                                                |
| `/api/requests` GET                           | Returns a list of all requests this farm has (w/ params to specify open/closed/all, etc.)                                      |
| `/api/requests` POST                          | Create a request to this farm                                                                                                  |
| `/api/requests/:id` DELETE                    | Delete a request from this farm (accessible by internal/creator of the request only)                                           |
| `/api/resources/:id` GET, POST, PATCH, DELETE | CRUD on the details of a specific resource (set) identified by :id (connected farms may only GET; POST doesn't require an :id) |
| `/api/connections` GET                        | Returns a list of all connected farms                                                                                          |
| `/api/connections` POST                       | Add a new connected farm (internal access only)                                                                                |
| `/api/connections/:id` GET, PATCH, DELETE     | More CRUD on a connected farm (each connected farm can only get/edit/delete its own entry in our connections list)             |

Then, with every other farm that we have connected, we can use their side of these `/api/xyz` endpoints which are accessible to us. Maybe I'll make a passthrough local endpoint which points to a connected farm's endpoints.

## Authorisation and Encryption

There are two requirements:

1. Only authorised farms can send and recieve data from each other.
1. All data over the wire must be encrypted.

It would be cool if authorisation happened at the same time as encryption. Like, a farm needs to first physically put in some RSA key to the other farms,
then all request bodies to the new farm would be encrypted one way. If decrypting an incoming msg doesn't work, then the sender inherently isn't authed!
Whenever generating a new set of keys, a farm would be responsible to PATCH a new pub/priv/whatever key to connected farms' `/api/connections/:our-farm-id` endpoint.
🤔 I wonder if that makes sense!
<br />
<br />
Did I mention that ideally the farms should be connected in a closed physical WAN? 😁😏

# Contributing

Contributions are welcome! Feel free to open an issue or pull request if you have any suggestions or improvements to the API.

# Further implementation ideas

- What about people who want to join the network but can't be trusted to see everyone's produce, IP addresses, etc? It would be cool to have secondary farms/houses which a 1st party farm could have connect to them, in a way that the 2nd party farm can only make requests to the 1st party one who auto-forwards it to the other 1st party farms. So if I'm a 1st party farm and I want my good friend Bob to be connected to the network, but the rest of the farms don't know him, then my farm could act as an "opaque passthrough" and his requests only interact with my farm IP address. This way if Bob is a Fed, he can only take me down with him and not the whole network.
