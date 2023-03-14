# How to use this API

## To create a user
POST https://url/user/

json data:
```json
{
  fname: "FirstName",
  lname: "LastName",
  address: "Address",
  password: "VerySecurePassword"
}
```
If a record matching the data sent does not exist, the server will reply with a uuid string.

Keep this stored! Every query from now on will use it for verification.

## To push new data to a user's log of sensor data:

POST https://url/data/

json data:
```json
{
  uid: "<INSERT UUID HERE>",   
  //Temperature in degrees C
  temp: 20,
  //Parts per million in room
  ppm: 192,
  //Light in lumens
  light: 700,
  //Boiler on or off as a boolean
  boiler: false
}
```
Response will be a HTTP response code.

## To get data:

GET https://url/data/{user_id}

Response will be of format:
```json
{
  //average temperature
  temp: 20,
  //average PPM
  ppm: 192,
  //average light
  light: 700, 
  //Average boiler state (Whether it is on more often, or off more often.)
  boiler: true,
}
```
If the UID matches no users, all values will be at 0.

## To toggle sensors remotely

PUT https://url/user/appliance

json data:
```json
{
  uid: "<INSERT-UUID>",
  appliance-type: "(Either 'Boiler' or 'Lights')",
  on_off: "boolean true/false",
}
```

## To get the UID of an already existing user:

GET https://url/user

json data:
```json
{
  fname: "Firstname",
  lname: "Lastname",
  address: "Address", 
  password: "VerySecurePassword"
}
```
## To get data of user from UID:

GET https://url/user/{user_id}
 
response will be:
```json
{
  fname: "Firstname",
  lname: "Lastname",
  address: "Address",
  password_hash: "<SOME GARBLED TEXT>"
}
```
