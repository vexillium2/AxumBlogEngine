CREATE DATABASE test;
use test;

CREATE TABLE `user`
(
    `username` varchar(255) NOT NULL PRIMARY KEY COMMENT 'username',
    `password` varchar(255) NOT NULL COMMENT 'password'
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4 COMMENT ='user';
