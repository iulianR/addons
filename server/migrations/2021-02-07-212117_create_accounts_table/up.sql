CREATE TABLE accounts
(
  id uuid NOT NULL,
  username varchar(100) NOT NULL,
  email varchar(100) NOT NULL,
  password varchar(150) NOT NULL,
  PRIMARY KEY (id),
  UNIQUE (email)
);

