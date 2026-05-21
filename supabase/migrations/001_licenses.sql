-- Tabela de licenças do AutoREC Turbo
create table if not exists licenses (
  id            uuid primary key default gen_random_uuid(),
  key           text unique not null,          -- ex: AREC-XXXX-XXXX-XXXX
  email         text not null,                 -- email do comprador (Hotmart)
  hotmart_order text,                          -- ID do pedido Hotmart
  status        text not null default 'active' -- active | revoked
    check (status in ('active', 'revoked')),
  max_activations int not null default 2,
  created_at    timestamptz not null default now(),
  revoked_at    timestamptz
);

-- Tabela de ativações (uma linha por máquina)
create table if not exists activations (
  id           uuid primary key default gen_random_uuid(),
  license_id   uuid not null references licenses(id) on delete cascade,
  machine_id   text not null,                 -- SHA-256 do serial (do app)
  activated_at timestamptz not null default now(),
  unique (license_id, machine_id)             -- mesma máquina não ativa duas vezes
);

-- RLS: nenhum cliente acessa diretamente — só via Edge Function com service_role
alter table licenses    enable row level security;
alter table activations enable row level security;

-- Bloqueia tudo por padrão (Edge Function usa service_role que ignora RLS)
create policy "no direct access" on licenses    for all using (false);
create policy "no direct access" on activations for all using (false);
