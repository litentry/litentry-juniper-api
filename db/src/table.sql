
CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(100) NOT NULL,
  `address` varchar(100) NOT NULL,
  `public_key` varchar(100) NOT NULL,
  `balance` varchar(100) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;


CREATE TABLE `identities` (
  `id` int(11) NOT NULL,
  `owner_id` int(11) NOT NULL,
  `identity_hash` varchar(100) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `tokens` (
  `id` int(11) NOT NULL,
  `owner_id` int(11) NOT NULL,
  `identity_id` int(11) NOT NULL,
  `token_hash` varchar(100) NOT NULL,
  `cost` varchar(100) NOT NULL,
  `data` varchar(100) NOT NULL,
  `data_type` varchar(100) NOT NULL,
  `expired` varchar(100) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

# id always 0, default value for index is -1.
CREATE TABLE `litentryIndex` (
  `id` int(11) NOT NULL DEFAULT 0,
  `identity_index` int(11) NOT NULL DEFAULT -1,
  `token_index` int(11) NOT NULL DEFAULT -1,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;


