create table economy_log (
    log_id serial primary key,

    log_event text not null, -- The reason

    coins_change integer not null,

    created_at timestamptz not null default now(),
    
    updated_at timestamptz
);

SELECT trigger_updated_at('game_action');

-- This function should be used for modifying a user's coins,
-- ensuring that all changes are logged to this table.
create or replace function update_user_coins(p_user_id uuid, p_coin_change bigint, p_event text)
    returns void as
$$
declare current_coins bigint
begin
    select coins into current_coins
    from users
    where user_id = p_user_id

    update users
    set coins = current_coins + p_coin_change
    where user_id = p_user_id

    insert into economy_log (log_event, coins_change)
    values (p_event, p_coin_change)
end;
$$ language plpgsql;

-- TODO: create a trigger to block/log direct updates, see if it messes with the above function as well