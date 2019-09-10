# 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
# 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
# 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
# 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48

CREATE TABLE `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `name` varchar(100) NOT NULL,
  `address` varchar(100) NOT NULL,
  `public_key` varchar(100) NOT NULL,
  `balance` varchar(100) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

# id keep the same order with index in identities array
CREATE TABLE `identities` (
  `id` int(11) NOT NULL,
  `owner_id` int(11) NOT NULL,
  `identity_hash` varchar(100) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

# id keep the same order with index in tokens array
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
CREATE TABLE `litentry_index` (
  `id` int(11) NOT NULL DEFAULT 0,
  `identity_index` int(11) NOT NULL DEFAULT -1,
  `token_index` int(11) NOT NULL DEFAULT -1,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;


CREATE TABLE `new_identity_event` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `block_hash` varchar(100) NOT NULL,
  `address` varchar(100) NOT NULL,
  `identity_hash` varchar(100) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `new_token_event` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `block_hash` varchar(100) NOT NULL,
  `owner_address` varchar(100) NOT NULL,
  `identity_hash` varchar(100) NOT NULL,
  `token_hash` varchar(100) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;


