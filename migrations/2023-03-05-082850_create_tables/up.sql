CREATE TABLE users (
  id TEXT NOT NULL PRIMARY KEY,
  fname TEXT NOT NULL,
  lname TEXT NOT NULL,
  address TEXT NOT NULL
);

CREATE TABLE frame (
  uid TEXT NOT NULL,
  datetime TEXT NOT NULL, --Must be of format 1970-01-01T00:00:00.000000Z
  temp REAL NOT NULL,
  ppm REAL NOT NULL,
  light REAL NOT NULL,
  boiler BOOLEAN NOT NULL,
  PRIMARY KEY (uid, datetime) --Ensures that every record is unique
);
