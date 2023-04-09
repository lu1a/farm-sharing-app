CREATE TABLE farm_details (
  id INT NOT NULL AUTO_INCREMENT,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  joined_date DATETIME NOT NULL DEFAULT NOW(),
  address VARCHAR(255),
  display_address BOOL NOT NULL DEFAULT true,
  primary_produce VARCHAR(255),
  website VARCHAR(255),
  PRIMARY KEY (id)
);