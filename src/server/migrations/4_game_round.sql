create domain round_placing as integer
check (value >= 1 || value == -1); -- round_placing==-1 is equivalent to DNF; otherwise, it must be >=1.

create table game_round
(
    round_id serial primary key,

    user_id serial references "user"(user_id) on delete restrict,

    game_id serial references game(game_id) on delete restrict,

    points integer not null,

    placing round_placing not null,

    created_at timestamptz not null default now(),
    
    updated_at timestamptz 
);

SELECT trigger_updated_at('game_round');