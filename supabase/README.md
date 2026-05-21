# Deploy do Backend de Licenças — AutoREC Turbo

## 1. Criar projeto no Supabase

1. Acesse https://supabase.com → New Project
2. Nome: `autorec-turbo`
3. Anote: **Project URL** e **anon key** (Settings > API)

---

## 2. Criar o banco de dados

No painel do Supabase → **SQL Editor** → colar e executar:

```
supabase/migrations/001_licenses.sql
```

---

## 3. Deploy das Edge Functions

Instale o CLI:
```bash
brew install supabase/tap/supabase
```

Faça login e link ao projeto:
```bash
supabase login
supabase link --project-ref SEU_PROJECT_REF
```

Configure os secrets:
```bash
supabase secrets set HOTMART_WEBHOOK_TOKEN=seu_token_secreto_aqui
```
(O `SUPABASE_URL` e `SUPABASE_SERVICE_ROLE_KEY` são injetados automaticamente)

Deploy das funções:
```bash
supabase functions deploy validate-license
supabase functions deploy hotmart-webhook
```

URLs resultantes:
- `https://SEU_PROJECT_REF.supabase.co/functions/v1/validate-license`
- `https://SEU_PROJECT_REF.supabase.co/functions/v1/hotmart-webhook`

---

## 4. Atualizar o app Rust

No arquivo `src-tauri/src/license.rs`, substituir as duas constantes:

```rust
const SUPABASE_LICENSE_URL: &str = "https://SEU_PROJECT_REF.supabase.co/functions/v1/validate-license";
const SUPABASE_ANON_KEY: &str = "eyJhbGci...sua_anon_key";
```

---

## 5. Configurar webhook na Hotmart

1. Hotmart → Ferramentas → Webhooks → Adicionar webhook
2. URL: `https://SEU_PROJECT_REF.supabase.co/functions/v1/hotmart-webhook`
3. Eventos: **PURCHASE_APPROVED** e **PURCHASE_COMPLETE**
4. Hottok: o mesmo valor definido em `HOTMART_WEBHOOK_TOKEN`

---

## 6. Configurar envio de e-mail da chave (recomendado)

Opção mais simples: **Resend** (https://resend.com — free tier generoso)

```bash
supabase secrets set RESEND_API_KEY=re_xxxxxxxxxxxx
```

Descomentar e adaptar a linha `await sendLicenseEmail(email, key)` em
`supabase/functions/hotmart-webhook/index.ts`.

---

## 7. Gerar licença de teste manualmente

No SQL Editor do Supabase, executar:
```sql
insert into licenses (key, email, hotmart_order)
values ('DEV-TESTE-0000-0000', 'dev@autorec.com.br', 'dev-manual');
```

No app, use a chave `DEV-TESTE` (o código Rust já bypass qualquer key que começa com `DEV-`).

---

## Fluxo completo

```
Cliente compra no Hotmart
    ↓ webhook POST (com hottok)
hotmart-webhook Edge Function
    ↓ gera AREC-XXXX-XXXX-XXXX
    ↓ INSERT em licenses
    ↓ (opcional) envia e-mail com a chave
Cliente recebe a chave por e-mail
    ↓ digita no AutoREC Turbo
validate-license Edge Function
    ↓ verifica status + contagem de ativações
    ↓ registra machine_id em activations
App libera instalação das cenas
```
