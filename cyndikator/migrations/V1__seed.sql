create table if not exists feeds (
e  id integer primary key,
  url text unique not null,
  title text,
  last_fetch text
);

create table if not exists items (
  id integer primary key,
  title text,
  feed_id integer,

  pub_date text,
  guid text,

  foreign key (feed_id) 
    references feeds(id)
);

create table if not exists actions (
  id integer primary key,
  feed_id integer,
  conditions text,
  action text,

  foreign key (feed_id)
    references feeds(id)
)

