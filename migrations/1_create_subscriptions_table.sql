-- migrations/1_create_subscriptions_table.sql
-- Create Subscriptions Table
create table if not exists subscriptions(
    id uuid not null,
    email text not null unique,
    name text not null,
    subscribed_at timestamptz not null,

    primary key (id)
);
