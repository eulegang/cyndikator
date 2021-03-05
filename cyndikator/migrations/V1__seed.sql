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
  url text,
  feed_id integer,

  description text,
  categories text,

  foreign key (feed_id) 
    references feeds(id)
    on delete cascade
);
