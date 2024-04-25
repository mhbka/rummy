create table game_action
(
    action_id uuid primary key default uuid_generate_v1mc(), -- Doesn't matter if its serial so I made it uuid.

    user_id serial references "user"(user_id) on delete restrict,

    game_id serial references game(game_id) on delete restrict,

    action_type text not null, -- The action type; the exact possible types are set and enforced server-side.

    action_metadata jsonb, -- Any additional metadata about the action

    created_at timestamptz not null default now(),
    
    updated_at timestamptz
);

SELECT trigger_updated_at('game_action');