CREATE TABLE
  `user` (
    `id` BIGINT (11) UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR (100) NOT NULL,
    `status` ENUM (
      'provisional_registration',
      'official_registration'
    ) NOT NULL,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
  ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
