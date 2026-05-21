// supabase/functions/validate-license/index.ts
// Valida e ativa licenças — chamada pelo app AutoREC Turbo

import { createClient } from "jsr:@supabase/supabase-js@2";

const supabase = createClient(
  Deno.env.get("SUPABASE_URL")!,
  Deno.env.get("SUPABASE_SERVICE_ROLE_KEY")!, // service_role ignora RLS
);

interface RequestBody {
  key: string;
  machine_id: string;
  action: "validate" | "activate";
}

Deno.serve(async (req) => {
  if (req.method !== "POST") {
    return json({ error: "Method not allowed" }, 405);
  }

  let body: RequestBody;
  try {
    body = await req.json();
  } catch {
    return json({ error: "Invalid JSON" }, 400);
  }

  const { key, machine_id, action } = body;

  if (!key || !machine_id || !action) {
    return json({ error: "Missing fields: key, machine_id, action" }, 400);
  }

  // Busca a licença
  const { data: license, error: licenseErr } = await supabase
    .from("licenses")
    .select("id, email, status, max_activations")
    .eq("key", key.trim().toUpperCase())
    .single();

  if (licenseErr || !license) {
    return json({ valid: false, activated: false, error: "Chave não encontrada." });
  }

  if (license.status !== "active") {
    return json({ valid: false, activated: false, error: "Licença revogada." });
  }

  // Verifica se esta máquina já está ativada
  const { data: existingActivation } = await supabase
    .from("activations")
    .select("id")
    .eq("license_id", license.id)
    .eq("machine_id", machine_id)
    .maybeSingle();

  if (existingActivation) {
    // Máquina já ativada — apenas validar (não consome slot)
    return json({ valid: true, activated: true, email: license.email });
  }

  // Conta ativações existentes
  const { count } = await supabase
    .from("activations")
    .select("id", { count: "exact", head: true })
    .eq("license_id", license.id);

  const currentActivations = count ?? 0;

  if (action === "validate") {
    // Só informa se ainda há slots disponíveis
    const canActivate = currentActivations < license.max_activations;
    return json({ valid: true, activated: false, can_activate: canActivate, email: license.email });
  }

  // action === "activate"
  if (currentActivations >= license.max_activations) {
    return json({
      valid: true,
      activated: false,
      error: `Limite de ${license.max_activations} ativações atingido. Contate o suporte.`,
    });
  }

  // Registra a ativação
  const { error: activationErr } = await supabase
    .from("activations")
    .insert({ license_id: license.id, machine_id });

  if (activationErr) {
    return json({ valid: true, activated: false, error: "Erro ao registrar ativação." }, 500);
  }

  return json({ valid: true, activated: true, email: license.email });
});

function json(data: unknown, status = 200) {
  return new Response(JSON.stringify(data), {
    status,
    headers: { "Content-Type": "application/json" },
  });
}
