# cosmas-api

The backend for a Community Sharing Management Software:
This is the backend API for a Community Sharing Management App, a platform where farms/communities/communes/homesteads/households can log their produce and inventory and see the inventories of other farms in the community. The API is built in Rust using the Rocket web framework, with a MariaDB database for storing farm data.
<br />
<br />
The point is to share data and subsequently resources to your fellow connected farms, like how organs share, or how teams in companies share resources to fulfill a common goal. A farm might make a request to all its connected members, and the idea is that some other farm comes forth and shares if it can. Extra resources can and should be given to fellow farms rather than hoarding.
<br />
This API along with the frontend should be run on some old PC on each farm in a decentralised way - such that each farm holds its own data and can manage its own inventory even if internet and grid power is lost (we could also support manual physical connections between farms so that they may stay connected while being completely off-grid, but that's not in the scope of this exact API).
A farm will create a static Auth key for other farms to save and use when querying each others' APIs, and there will be another token or some other form of login to get into the frontend app.

# Installation

TODO. For now just install cargo then cargo run this. Remember that Rocket uses nightly Rust.

# Endpoints

| Endpoint                                      | Description                                                                                                                    |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `/api/details` GET, PATCH                     | Actions on the details of this farm (connected farms may only GET)                                                             |
| `/api/resources` GET                          | Returns a list of all resources                                                                                                |
| `/api/requests` GET                           | Returns a list of all requests this farm has (w/ params to specify open/closed/all, etc.)                                      |
| `/api/requests` POST                          | Create a request to this farm                                                                                                  |
| `/api/requests:id` DELETE                     | Delete a request from this farm (accessible by internal/creator of the request only)                                           |
| `/api/resources/:id` GET, POST, PATCH, DELETE | CRUD on the details of a specific resource (set) identified by :id (connected farms may only GET; POST doesn't require an :id) |
| `/api/connections` GET                        | Returns a list of all connected farms (each connected farm can only get/edit/delete its own entry in our connections list)     |
| `/api/connections` POST                       | Add a new connected farm (internal access only)                                                                                |
| `/api/connections/:id` GET, PATCH, DELETE     | More CRUD on a connected farm (each connected farm can only get/edit/delete its own entry in our connections list)             |

Then, with every other farm that we have connected, we can use their side of these `/api/xyz` endpoints which are accessible to us.

## Authorization

Every request to a given farm will use a Bearer token initially given manually in real life between the managers of this app on both farms.
Additionally, each farm will internally have a custom auth header to indicate a user is "logged in" (with another manual key).
<br />
<code contenteditable="false">
Authorization: Bearer <inter-farm_token>
<br />
X-Internal-Token: <internal_token>
</code>
<br />
<br />
The internal token will be designed to update every so often and post itself into a telegram channel or similar to be copied, then applied into the separate frontend of this app. The frontend is meant to be used by as little users on each farm as possible, to prevent leaks (ie. one would be able to view the resources of all other connections via this API).
<br />
The inter-farm token should probably update itself too, and then a farm will be responsible to PATCH that new token to its frontend, and connected farms' `/api/connections/:our-farm-id` endpoint.

# Contributing

Contributions are welcome! Feel free to open an issue or pull request if you have any suggestions or improvements to the API.
