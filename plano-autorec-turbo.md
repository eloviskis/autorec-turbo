# Plano de Desenvolvimento — Configurador Automático de OBS (estilo "AutoREC Turbo Install")

**Escopo:** macOS primeiro · versão completa (instala OBS + injeta cenas + branding + licenciamento)
**Promessa a entregar:** 1 clique → ~2 minutos → OBS abre pronto pra gravar.

---

## 1. O que o produto realmente é (desmistificando)

Não é um software de gravação. É um **instalador + um pacote de configuração do OBS + uma camada de personalização e licença**. O OBS faz todo o trabalho pesado; o seu app só prepara o terreno. Isso é ótimo: o risco técnico é baixo e o valor percebido é alto (o cliente paga pela conveniência e pelo método de cenas, não pela tecnologia).

Os três artefatos centrais:

1. **App instalador** (.app/.dmg) que orquestra tudo.
2. **Pacote de cena** — um Scene Collection do OBS em JSON, com todas as cenas já montadas, mais os assets (overlays, faixas).
3. **Camada de licença + branding** — valida a compra e troca nome/redes/cores antes de aplicar.

---

## 2. Como o OBS guarda a configuração (a base de tudo)

No macOS, tudo vive em `~/Library/Application Support/obs-studio/`:

- `basic/scenes/*.json` → cada Scene Collection é **um arquivo JSON**.
- `basic/profiles/<nome>/` → cada perfil é uma pasta com `basic.ini` (resolução, FPS, encoder, caminho de saída) + JSONs.
- `global.ini` → qual coleção/perfil estão ativos.

**O perigo nº 1 (precisa resolver no código):** as cenas guardam caminhos absolutos e IDs de dispositivo da máquina onde foram criadas. Se você só copiar o JSON, a câmera e a captura de tela abrem **vazias** no PC do cliente. Por isso o app precisa de um passo de **normalização pós-cópia**: reescrever o JSON apontando para o device de vídeo/áudio default do cliente e para os assets no caminho local correto.

---

## 3. Arquitetura (Mac-first)

```
┌─────────────────────────────────────────────────────────┐
│  App instalador (.app empacotado em .dmg)                │
│                                                           │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────┐  │
│  │ UI de setup │→ │ Orquestrador │→ │ Verificador de  │  │
│  │ (nome/cores)│  │  (núcleo)    │  │ licença (online)│  │
│  └─────────────┘  └──────┬───────┘  └─────────────────┘  │
│                          │                                │
│       ┌──────────────────┼──────────────────┐            │
│       ▼                  ▼                  ▼            │
│  Detecta/instala   Copia + normaliza   Injeta branding   │
│      OBS            scene JSON         (faixa/nome/cor)   │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
         ~/Library/Application Support/obs-studio/
```

### Stack recomendada

| Camada | Escolha | Por quê |
|---|---|---|
| App desktop | **Electron** ou **Tauri** | Electron: você já domina React, ecossistema enorme. Tauri: binário muito menor (~3–10 MB vs ~120 MB), mais leve, Rust no core. Para Mac-first com futuro Windows, **Tauri** é a aposta mais elegante; se quer velocidade de entrega, **Electron**. |
| UI de setup | React + Tailwind | Reaproveita seu stack. |
| Manipulação dos JSON | Node/Rust nativo | Ler, normalizar e gravar os Scene Collections. |
| Backend de licença | **Supabase** (você já usa) | Tabela de licenças + Edge Function de validação. Barato e rápido. |
| Distribuição | `.dmg` assinado e notarizado | Sem isso o Gatekeeper bloqueia (ver seção 7). |

> **Decisão a tomar:** Electron (entrega rápida) vs Tauri (produto mais leve/profissional). Recomendo **Tauri** se o cronograma permitir — combina com o posicionamento "leve, 2 minutos".

---

## 4. As cenas (o coração do "método")

Você precisa montar **uma vez, à mão, no seu Hackintosh (macOS)** o Scene Collection ideal, com estas cenas (espelhando a oferta deles):

1. **Você + Slides** — captura de janela (PowerPoint/Keynote/navegador) + webcam num canto, com faixa de nome.
2. **Tela + Explicação** — Display/Window Capture cheia + webcam pequena.
3. **Tablet / Escrita** — captura de janela do app de escrita (ex.: tela espelhada do iPad) + webcam.
4. **Cena pré-evento / intervalo** — tela "Já começamos!" com overlay.
5. **Live / Reunião** — layout otimizado para câmera virtual (Instagram/YouTube/Zoom).
6. **Horizontal e Vertical** — duplicar as principais em canvas 1920×1080 e 1080×1920.

Depois de montar, você **exporta o JSON** e ele vira o template embutido no app. Os overlays (faixas, fundos) viram PNGs no pacote de assets.

**Branding dinâmico:** a faixa com nome/redes pode ser (a) um PNG gerado na hora a partir dos inputs do usuário, ou (b) uma fonte de **Texto (GDI/FreeType)** dentro do OBS, que o app preenche editando o JSON. A opção (b) é mais limpa e editável depois pelo cliente — recomendo.

---

## 5. Fluxo de execução do instalador (o "1 clique")

1. **Boas-vindas + licença** — cliente cola a chave (recebida via Hotmart). App valida online no Supabase.
2. **Personalização** — nome, @ das redes, cor de destaque. (Pode ter um preset "pular e usar padrão".)
3. **Detecção do OBS** — verifica se o OBS existe em `/Applications`. Se não, baixa e instala automaticamente (ou abre o instalador oficial).
4. **Backup de segurança** — se já houver config, faz cópia em `obs-studio-backup-<data>/`. **Crítico** para não destruir o setup de quem já usa OBS (a FAQ deles trata disso).
5. **Injeção** — copia o Scene Collection + assets para `~/Library/Application Support/obs-studio/`.
6. **Normalização** — reescreve os JSON: aponta câmera/microfone para os devices default, corrige caminhos dos assets, injeta o branding.
7. **Ativação** — edita `global.ini` para deixar a coleção/perfil novos como ativos.
8. **Conclusão** — botão "Abrir OBS". OBS sobe já pronto.

> **Importante:** OBS precisa estar **fechado** durante a injeção, senão ele sobrescreve seus arquivos ao sair. O app deve detectar e pedir para fechar.

---

## 6. Sistema de licença (versão completa)

Fluxo mínimo viável e robusto:

- **Geração:** ao aprovar a compra, um **webhook da Hotmart** chama uma Edge Function do Supabase que cria a licença (chave única + e-mail do comprador) e a envia por e-mail.
- **Validação:** o app envia a chave; a função confere se existe, está ativa e dentro do limite de ativações.
- **Vínculo de máquina:** registra um hash de hardware (ex.: derivado do serial via `ioreg`) para limitar nº de instalações por licença — a oferta deles sugere 1 licença por SO; defina sua política (1 ou 2 máquinas).
- **Vitalício:** sem expiração; só controla nº de ativações. Combina com o "uso vitalício" prometido.

Tabela `licenses`: `key`, `email`, `status`, `max_activations`, `activations[]`, `created_at`.

---

## 7. O ponto que mais trava no Mac: assinatura e notarização

Sem isso, o cliente vê "app de desenvolvedor não identificado" e muitos desistem — mata a promessa de simplicidade.

- Precisa de **Apple Developer Program** (US$ 99/ano).
- **Code signing** do .app + **notarização** junto à Apple + **stapling** do ticket.
- Pedir **permissões** necessárias: o app que injeta arquivos no diretório do usuário não precisa de permissões especiais, mas o **OBS** vai pedir acesso a Câmera, Microfone e **Gravação de Tela** (Screen Recording) na primeira execução. Inclua isso nas "aulas de orientação" para o cliente liberar nas Preferências do Sistema.

> Você roda Hackintosh — atenção: assinar/notarizar exige conta Apple válida e funciona normalmente, mas teste o fluxo de Gatekeeper numa máquina/instalação limpa, porque o seu ambiente de dev já tem tudo liberado e pode mascarar problemas.

---

## 8. Roadmap em fases

### Fase 0 — Validação técnica (2–3 dias)
- Montar o Scene Collection à mão no OBS do Mac.
- Exportar o JSON, copiar para outra conta de usuário/máquina limpa e provar que **normalizando os devices** as cenas funcionam. Esse é o teste que valida o produto inteiro.

### Fase 1 — Núcleo do instalador (1–1,5 semana)
- App Tauri/Electron com UI mínima.
- Detecção do OBS, backup, cópia de arquivos, normalização dos JSON, edição do `global.ini`.
- Botão "Abrir OBS". Sem licença e sem branding ainda — só fazer o OBS abrir pronto.

### Fase 2 — Personalização/branding (3–5 dias)
- Tela de inputs (nome/redes/cor).
- Injeção dos valores nas fontes de Texto do JSON e/ou geração de PNG de faixa.

### Fase 3 — Licenciamento (3–5 dias)
- Supabase: tabela + Edge Functions (validar/ativar).
- Webhook da Hotmart → criação e envio de chave.
- Tela de ativação no app.

### Fase 4 — Empacotamento e distribuição (3–5 dias)
- `.dmg`, code signing, notarização, stapling.
- Teste em macOS limpo (Intel e Apple Silicon — atenção a universal binary).

### Fase 5 — Conteúdo de apoio + lançamento (paralelo)
- "Aulas de orientação para uso" (a oferta promete isso).
- Página de FAQ cobrindo: não apaga OBS atual, permissões de tela/câmera, suporte.

**Estimativa total:** ~4 a 6 semanas de desenvolvimento focado para a v1 Mac.

---

## 9. Riscos e mitigação

| Risco | Impacto | Mitigação |
|---|---|---|
| Caminhos/devices quebrados no PC do cliente | Cenas vazias = produto "não funciona" | Passo de normalização robusto + fallback "selecione sua câmera" na primeira abertura |
| Cliente já tem OBS configurado | Apaga o trabalho dele | Backup automático obrigatório + opção de criar coleção nova ao invés de sobrescrever |
| Gatekeeper bloqueia o app | Abandono na instalação | Notarização Apple (obrigatória) |
| Permissão de Gravação de Tela negada | OBS grava tela preta | Aula de orientação + detecção e aviso no app |
| Versão futura do OBS muda schema do JSON | Quebra o template | Fixar versão testada do OBS + validar versão na instalação |
| Pirataria da chave | Perda de receita | Vínculo por hardware + limite de ativações |

---

## 10. Diferenciais possíveis (pra superar o original)

- **Reset/Reaplicar** num clique (eles vendem "reconfigurações" como benefício — entregue de fábrica).
- **Atualização de cenas OTA**: novos layouts entregues sem reinstalar.
- **Detector de Stream Deck** e mapeamento automático de botões para trocar de cena (eles citam Stream Deck na FAQ).
- **Modo vertical real**: profile separado 1080×1920 com um clique, já que Reels/Shorts é o que mais cresce.
- **Cross-platform depois**: o núcleo (manipular JSON) é o mesmo no Windows — só muda o caminho (`%APPDATA%\obs-studio`) e o empacotamento.

---

## 11. Próximos passos imediatos

1. Decidir **Tauri vs Electron**.
2. Definir política de licença (quantas máquinas por compra).
3. Montar o Scene Collection-modelo no seu Mac e rodar a **Fase 0** — é o experimento que prova que o produto é viável antes de escrever o instalador.
