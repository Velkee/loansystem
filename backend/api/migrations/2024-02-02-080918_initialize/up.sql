CREATE TYPE status_enum AS ENUM ('In use', 'In stock', 'Used in home office', 'Temporary', 'Lost', 'To be repaired', 'Return supplier', 'Sold', 'Stolen', 'Assigned loan office', 'Loaned out', 'Discarded', 'Other');

CREATE TABLE categories (
    name varchar(255) PRIMARY KEY
);

CREATE TABLE post_offices (
    code int PRIMARY KEY,
    name varchar(255) NOT NULL
);

CREATE TABLE locations (
    name varchar(255) PRIMARY KEY,
    address varchar(255),
    post_office int REFERENCES post_offices ON DELETE SET NULL ON UPDATE CASCADE
);

CREATE TABLE departments (
    name varchar(255) PRIMARY KEY,
    location varchar(255) REFERENCES locations ON DELETE SET NULL ON UPDATE CASCADE
);

CREATE TABLE buildings (
    name varchar(255) PRIMARY KEY,
    address varchar(255),
    location varchar(255) REFERENCES locations ON DELETE SET NULL ON UPDATE CASCADE,
    post_office int REFERENCES post_offices ON DELETE SET NULL ON UPDATE CASCADE
);

CREATE TABLE people (
    id varchar(255) PRIMARY KEY,
    name text NOT NULL,
    location varchar(255) REFERENCES locations ON DELETE SET NULL ON UPDATE CASCADE,
    department varchar(255) REFERENCES departments ON DELETE SET NULL ON UPDATE CASCADE,
    building varchar(255) REFERENCES buildings ON DELETE SET NULL ON UPDATE CASCADE
);

CREATE TABLE rooms (
    name varchar(255) PRIMARY KEY,
    building varchar(255) NOT NULL REFERENCES buildings ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE devices (
    id varchar(255) PRIMARY KEY,
    serial_number varchar(255),
    category varchar(255) REFERENCES categories ON DELETE SET NULL ON UPDATE CASCADE,
    status status_enum NOT NULL,
    person varchar(255) REFERENCES people ON DELETE SET NULL ON UPDATE CASCADE,
    location varchar(255) REFERENCES locations ON DELETE SET NULL ON UPDATE CASCADE,
    department varchar(255) REFERENCES departments ON DELETE SET NULL ON UPDATE CASCADE,
    building varchar(255) REFERENCES buildings ON DELETE SET NULL ON UPDATE CASCADE,
    room varchar(255) REFERENCES rooms ON DELETE SET NULL
);
