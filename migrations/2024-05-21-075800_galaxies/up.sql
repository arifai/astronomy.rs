CREATE TABLE constellations (
  id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  "name" VARCHAR NOT NULL,
  created_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE galaxies (
  id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  constellation_id UUID NOT NULL,
  "name" VARCHAR NOT NULL,
  "image" TEXT NOT NULL,
  note TEXT,
  created_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  CONSTRAINT fk_constellation FOREIGN KEY (constellation_id) REFERENCES constellations (id) ON DELETE CASCADE
);

CREATE TABLE planets (
  id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
  galaxy_id UUID NOT NULL,
  "name" VARCHAR NOT NULL,
  "image" TEXT NOT NULL,
  short_description VARCHAR,
  details TEXT,
  created_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  CONSTRAINT fk_galaxy FOREIGN KEY (galaxy_id) REFERENCES galaxies (id) ON DELETE CASCADE
);

INSERT INTO constellations ("name") VALUES ('Andromeda'), ('Centaurus'), ('Sagitarius (centre)'), ('Virgo');

INSERT INTO galaxies (constellation_id, "name", "image", note) VALUES
  ((SELECT id FROM constellations WHERE "name" = 'Andromeda' LIMIT 1), 'Andromeda Galaxy', 'https://upload.wikimedia.org/wikipedia/commons/thumb/9/98/Andromeda_Galaxy_%28with_h-alpha%29.jpg/2560px-Andromeda_Galaxy_%28with_h-alpha%29.jpg', 'Andromeda is the closest big galaxy to the Milky Way and is expected to collide with the Milky Way around 4.5 billion years from now. The two will eventually merge into a single new galaxy called Milkdromeda.'),
  ((SELECT id FROM constellations WHERE "name" = 'Centaurus' LIMIT 1), 'Backward Galaxy', 'https://upload.wikimedia.org/wikipedia/commons/8/8c/NGC_4622HSTFull.jpg', NULL),
  ((SELECT id FROM constellations WHERE "name" = 'Sagitarius (centre)' LIMIT 1), 'Milky Way Galaxy', 'https://upload.wikimedia.org/wikipedia/commons/thumb/4/43/ESO-VLT-Laser-phot-33a-07.jpg/2560px-ESO-VLT-Laser-phot-33a-07.jpg', 'The galaxy containing the Sun and its Solar System, and therefore Earth.'),
  ((SELECT id FROM constellations WHERE "name" = 'Virgo' LIMIT 1), 'Sombrero Galaxy', 'https://upload.wikimedia.org/wikipedia/commons/thumb/5/5e/M104_ngc4594_sombrero_galaxy_hi-res.jpg/2560px-M104_ngc4594_sombrero_galaxy_hi-res.jpg', 'Also known as Messier Object 104 or M104');

INSERT INTO planets (galaxy_id, "name", "image", short_description, details) VALUES 
  ((SELECT id FROM galaxies WHERE "name" = 'Milky Way Galaxy' LIMIT 1), 'Mercury', 'https://upload.wikimedia.org/wikipedia/commons/4/4a/Mercury_in_true_color.jpg', 'First planet from the sun', 'Mercury is the first planet from the Sun and the smallest in the Solar System. In English, it is named after the ancient Roman god Mercurius (Mercury), god of commerce and communication, and the messenger of the gods.'),
  ((SELECT id FROM galaxies WHERE "name" = 'Milky Way Galaxy' LIMIT 1), 'Venus', 'https://upload.wikimedia.org/wikipedia/commons/b/b2/Venus_2_Approach_Image.jpg', 'Second planet from the sun', 'Venus is the second planet from the Sun. It is a terrestrial planet and is the closest in mass and size to its orbital neighbour Earth.'),
  ((SELECT id FROM galaxies WHERE "name" = 'Milky Way Galaxy' LIMIT 1), 'Earth', 'https://upload.wikimedia.org/wikipedia/commons/thumb/c/cb/The_Blue_Marble_%28remastered%29.jpg/1920px-The_Blue_Marble_%28remastered%29.jpg', 'Third planet from the sun', 'Earth is the third planet from the Sun and the only astronomical object known to harbor life. This is enabled by Earth being an ocean world, the only one in the Solar System sustaining liquid surface water.'),
  ((SELECT id FROM galaxies WHERE "name" = 'Milky Way Galaxy' LIMIT 1), 'Mars', 'https://upload.wikimedia.org/wikipedia/commons/0/0c/Mars_-_August_30_2021_-_Flickr_-_Kevin_M._Gill.png', 'Fourth planet from the sun', 'Mars is the fourth planet from the Sun. The surface of Mars is orange-red because it is covered in iron(III) oxide dust, giving it the nickname "the Red Planet".'),
  ((SELECT id FROM galaxies WHERE "name" = 'Milky Way Galaxy' LIMIT 1), 'Jupiter', 'https://upload.wikimedia.org/wikipedia/commons/c/c1/Jupiter_New_Horizons.jpg', 'Fifth planet from the sun', 'Jupiter is the fifth planet from the Sun and the largest in the Solar System. A gas giant, Jupiter s mass is more than two and a half times that of all the other planets in the Solar System combined and slightly less than one one-thousandth the mass of the Sun.'),
  ((SELECT id FROM galaxies WHERE "name" = 'Milky Way Galaxy' LIMIT 1), 'Saturn', 'https://upload.wikimedia.org/wikipedia/commons/thumb/c/c7/Saturn_during_Equinox.jpg/2880px-Saturn_during_Equinox.jpg', 'Sixth planet from the sun', 'Saturn is the sixth planet from the Sun and the second-largest in the Solar System, after Jupiter. It is a gas giant with an average radius of about nine-and-a-half times that of Earth.'),
  ((SELECT id FROM galaxies WHERE "name" = 'Milky Way Galaxy' LIMIT 1), 'Uranus', 'https://upload.wikimedia.org/wikipedia/commons/6/69/Uranus_Voyager2_color_calibrated.png', 'Seventh planet from the sun', 'Uranus is the seventh planet from the Sun. It is a gaseous cyan-coloured ice giant. Most of the planet is made of water, ammonia, and methane in a supercritical phase of matter, which in astronomy is called ice or volatiles.'),
  ((SELECT id FROM galaxies WHERE "name" = 'Milky Way Galaxy' LIMIT 1), 'Neptune', 'https://upload.wikimedia.org/wikipedia/commons/b/b9/Neptune_Voyager2_color_calibrated.png', 'Eighth planet from the sun', 'Neptune is the eighth and farthest known planet from the Sun. It is the fourth-largest planet in the Solar System by diameter, the third-most-massive planet, and the densest giant planet. It is 17 times the mass of Earth, and slightly more massive than fellow ice giant Uranus.');