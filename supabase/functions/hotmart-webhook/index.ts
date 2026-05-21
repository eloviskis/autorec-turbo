// supabase/functions/hotmart-webhook/index.ts
// Recebe notificações da Hotmart e cria licenças automaticamente

import { createClient } from "jsr:@supabase/supabase-js@2";

const supabase = createClient(
  Deno.env.get("SUPABASE_URL")!,
  Deno.env.get("SUPABASE_SERVICE_ROLE_KEY")!,
);

// Token secreto configurado no painel Hotmart (Ferramentas > Webhooks)
const HOTMART_TOKEN = Deno.env.get("HOTMART_WEBHOOK_TOKEN")!;

// Gera chave no formato AREC-XXXX-XXXX-XXXX
function generateKey(): string {
  const chars = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789"; // sem I, O, 0, 1
  const segment = () =>
    Array.from({ length: 4 }, () => chars[Math.floor(Math.random() * chars.length)]).join("");
  return `AREC-${segment()}-${segment()}-${segment()}`;
}

Deno.serve(async (req) => {
  if (req.method !== "POST") {
    return new Response("Method not allowed", { status: 405 });
  }

  // Validação do token Hotmart (header hottok)
  const token = req.headers.get("x-hotmart-hottok");
  if (token !== HOTMART_TOKEN) {
    console.error("Invalid Hotmart token");
    return new Response("Unauthorized", { status: 401 });
  }

  let payload: Record<string, unknown>;
  try {
    payload = await req.json();
  } catch {
    return new Response("Invalid JSON", { status: 400 });
  }

  const event = payload?.event as string;

  // Eventos que geram licença
  const PURCHASE_APPROVED = "PURCHASE_APPROVED";
  const PURCHASE_COMPLETE = "PURCHASE_COMPLETE";

  if (event !== PURCHASE_APPROVED && event !== PURCHASE_COMPLETE) {
    // Ignora outros eventos (cancelamento, etc.) mas retorna 200 para Hotmart não retentar
    return new Response("Ignored", { status: 200 });
  }

  const data = payload?.data as Record<string, unknown> | undefined;
  const buyer = data?.buyer as Record<string, unknown> | undefined;
  const purchase = data?.purchase as Record<string, unknown> | undefined;

  const email = (buyer?.email as string)?.toLowerCase()?.trim();
  const orderId = (purchase?.order_date as string) ?? String(Date.now());

  if (!email) {
    console.error("No buyer email in payload", JSON.stringify(payload));
    return new Response("Missing email", { status: 400 });
  }

  // Gera chave única (tentativas para evitar colisão improvável)
  let key = "";
  for (let i = 0; i < 5; i++) {
    const candidate = generateKey();
    const { data: existing } = await supabase
      .from("licenses")
      .select("id")
      .eq("key", candidate)
      .maybeSingle();
    if (!existing) {
      key = candidate;
      break;
    }
  }

  if (!key) {
    console.error("Failed to generate unique key");
    return new Response("Key generation failed", { status: 500 });
  }

  // Insere a licença
  const { error } = await supabase.from("licenses").insert({
    key,
    email,
    hotmart_order: orderId,
  });

  if (error) {
    console.error("DB insert error", error);
    return new Response("DB error", { status: 500 });
  }

  console.log(`License created: ${key} for ${email}`);

  // Opcional: enviar e-mail com a chave (via Resend, SendGrid, etc.)
  // await sendLicenseEmail(email, key);

  return new Response(JSON.stringify({ ok: true, key }), {
    status: 200,
    headers: { "Content-Type": "application/json" },
  });
});
