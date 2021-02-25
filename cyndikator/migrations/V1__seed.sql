create table if not exists feeds (
  id integer primary key,
  url text unique not null,
  title text,
  ttl integer default 60,
  last_fetch text default null
);

create table if not exists items (
  id integer primary key,
  title text,
  feed_id integer,

  pub_date text,
  guid text,

  foreign key (feed_id) 
    references feeds(id)
    on delete cascade
);

create table if not exists actions (
  id integer primary key,
  conditions text,
  action text
)

