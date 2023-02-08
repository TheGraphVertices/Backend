# Backend
## The REST-ful backend for the smart meter

Will be responsible for storing all temperature, PPM, (perhaps light?) data sent to the server by the pi.

Respond to REST queries with json, allowing the fontend to access metrics, like average temperature, CO2 saved etc.

To be written in Rust, and use SQlite behind the scenes to store the data.
