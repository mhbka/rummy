-- As a style choice, we prefer to avoid plurals in table names, mainly because it makes queries read better.
--
-- For our user table, quoting the table name is recommended by IntelliJ's tooling because `user` is a keyword,
-- though Postgres seems to handle it fine in most contexts either way.
create table "user"
(
    user_id       serial primary key,

    username      text collate "case_insensitive" unique not null,

    email         text collate "case_insensitive" unique not null,

    bio           text                                   not null default '',

    image         text,

    coins         bigint                                 not null default 0,

    password_hash text                                   not null,

    -- If you want to be really pedantic you can add a trigger that enforces this column will never change,
    -- but that seems like overkill for something that's relatively easy to enforce in code-review.
    created_at    timestamptz                            not null default now(),
    
    updated_at    timestamptz
);

-- Apply our `updated_at` trigger is as easy as this.
select trigger_updated_at('"user"');

