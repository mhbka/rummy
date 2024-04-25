create table game
(   
    game_id serial primary key,

    game_metadata jsonb not null, -- The Rummy variant played, any customizable settings, etc.

    created_at timestamptz not null default now(),
    
    updated_at timestamptz
);

SELECT trigger_updated_at('game');