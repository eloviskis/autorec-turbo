-- Script para inserir licenças manualmente (testes/suporte)
-- Execute no SQL Editor do Supabase

-- ====================================================
-- LICENÇA ADMIN VITALÍCIA (uso interno / proprietário)
-- ====================================================
insert into licenses (key, email, hotmart_order, max_activations)
values ('AREC-ADMN-MSTR-2026', 'eloisa@autorec.com.br', 'admin-vitalicio', 99);

-- Licença de teste (DEV)
insert into licenses (key, email, hotmart_order)
values ('DEV-TESTE-0000-0000', 'dev@autorec.com.br', 'dev-manual');

-- Licença para um cliente manual
-- insert into licenses (key, email, hotmart_order)
-- values ('AREC-XXXX-YYYY-ZZZZ', 'cliente@email.com', 'manual-001');

-- Ver todas as licenças
-- select l.key, l.email, l.status, count(a.id) as ativacoes
-- from licenses l
-- left join activations a on a.license_id = l.id
-- group by l.id
-- order by l.created_at desc;

-- Revogar uma licença
-- update licenses set status = 'revoked', revoked_at = now()
-- where key = 'AREC-XXXX-YYYY-ZZZZ';

-- Ver ativações de uma chave específica
-- select a.machine_id, a.activated_at
-- from activations a
-- join licenses l on l.id = a.license_id
-- where l.key = 'AREC-XXXX-YYYY-ZZZZ';

-- Resetar ativações (quando cliente troca de máquina)
-- delete from activations
-- where license_id = (select id from licenses where key = 'AREC-XXXX-YYYY-ZZZZ')
--   and machine_id = 'ID_DA_MAQUINA_ANTIGA';
