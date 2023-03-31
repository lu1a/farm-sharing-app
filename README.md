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
TODO

# Endpoints
TODO

# Contributing
Contributions are welcome! Feel free to open an issue or pull request if you have any suggestions or improvements to the API.
