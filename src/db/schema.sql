create table if not exists feeds(
  id integer primary key,
  name varchar,
  url varchar unique not null ,
  ttl integer not null
);

create table if not exists tracking(
  id integer primary key,
  feed integer not null unique,
  last_fetch integer not null,

  foreign key(feed) references feeds(id)
);
