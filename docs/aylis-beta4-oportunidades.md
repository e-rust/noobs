# O que o Aylis 0.2.0-beta.4 pode trazer ao NoOBS

Levantamento de 12/11/2026, comparando os commits recentes do Aylis com o código do NoOBS.

- **Commits do Aylis olhados:** `66ceb6d` (0.2.0-beta.3), `bfa34a2`, `f074dc4`, `850180f` e `b5415ea` (0.2.0-beta.4).
- **Versão usada pelo NoOBS:** a cópia de trabalho do Aylis, pela junção `.aylis\sdk`. Ela já compilava o NoOBS sem nenhuma mudança, e os 379 testes da lib passavam. Compile pela IDE (o `build` do MCP do Aylis); o `scripts\build.bat` saiu.

## Situação (12/11/2026)

| Item | Situação |
|---|---|
| Regerar o código (gerador 11) | **Feito.** Os arquivos dos frames agora levam o prefixo da pasta (`settings_general.rs`...). |
| 2. ComboBox do codec | **Feito.** O `cmbCodec` usa `groups` (Hardware / Software) e `disabled_items`. O `CODECS` foi reordenado para os grupos ficarem juntos. A configuração guarda o nome do codec, então não muda nada para quem já usa. |
| 3. FrameTimer | **Feito.** `tmrRipple` e `tmrCountdown` no frmMain, `tmrFlash` no frmPlayer. A onda acaba no `window_hidden` (`app_main::end_ripple`), porque a janela escondida não desenha quadros e a onda pararia no meio. |
| 1. Popover | **Não feito.** Faltam duas coisas no Aylis. (a) O clique fora fecha já no aperto, e o botão que abriu não é o dono (o Popover não tem `dies_with`): clicar no botão de velocidade com o menu aberto fecharia e reabriria o menu. (b) O `placement` alinha pela borda esquerda do botão, e o menu hoje fica centrado nele. |
| 4. Modo escuro e `on_window_created` | **Feito.** `set_dark_mode` e `apply_dark_mode` saíram, e o Aylis acompanha o `theme.dark`. O `finish_show` não tenta mais 60 vezes: sem HWND, a exibição fica pendente (`show_pending`), e o `main_window_created` do frmMain termina. |

Os testes passam (379). **Ainda falta conferir na tela** (o NoOBS estava aberto, então o exe não foi trocado):
- a onda e a contagem do botão de gravar;
- o pulso da pausa;
- a barra de título ao trocar de tema;
- a primeira exibição e a volta da bandeja;
- os grupos do combo de codec;
- os itens de "Mudanças de comportamento" mais abaixo.

**Sobre as referências:**
- As linhas citadas são aproximadas: confira antes de editar.
- Lembre-se da regra do projeto: forms se mudam pelo designer ou pelo MCP da IDE, nunca editando o JSON à mão com a IDE aberta.

## Já vem de graça (só recompilar)

- **`create_in` mais rápido (G21, `bfa34a2`):** caiu de 136 para 22 µs por frame criado. Beneficia:
  - os cards da biblioteca;
  - os `FraTurn` da transcrição, que o teste em `app_player.rs:~2112` mede com 500 falas;
  - as linhas das Configurações.
- **O Label pinta o fundo e o raio do estilo (C10):** os badges `lblPlayerZoom` (estilo `pl_zoom`) e `lblExAudioOnly` (estilo `ex_badge`) passam a sair como desenhados no designer. Confira na tela.

## Vale fazer já

1. **Popover** (C15, `66ceb6d`; doc em `aylis/examples/Landing/assets/docs/componentes/Popover.md`).
   - **Como funciona:** tem `placement` e `gap`, fecha com clique fora e com Esc, e avisa por `on_close`.
   - **Onde entra:** no menu de velocidade do player, hoje montado à mão em `app_player.rs:~637-688` (`speed_menu`, `close_speed`, `place_popup` e o estado `speed_open`).
   - **Resultado:** somem umas 50 linhas e o reparent manual.
   - **Fora deste item:** `lyVolPop` e `lyCcPop` (`sync_popups`, `:~712`) abrem no hover, que o Popover não cobre. Deixe como estão.
   - **Esforço:** baixo a médio.

2. **ComboBox com `disabled_items` e `groups`** (C20, `66ceb6d`).
   - **Hoje:** no `cmbCodec`, `codec_options()` (`app_settings.rs:~382-420`) escreve "(indisponível)" no texto, e `video::codec` (`:~2484`) desfaz a escolha depois.
   - **Com `disabled_items`:** o codec indisponível nem pode ser escolhido.
   - **Com `groups`:** "Hardware" fica separado de "Software".
   - **Esforço:** baixo.

3. **FrameTimer** (P05, `66ceb6d`) no lugar dos laços `set_timeout(16)` das animações.
   - **Onde:**
     - o pulso da pausa (`app_player.rs:~459-477`);
     - o ripple (`app_main.rs:~814-817`);
     - a contagem regressiva (`app_main.rs:~886`).
   - **Resultado:** a animação segue o ritmo do monitor, e o laço dorme quando está desligado.
   - **Esforço:** baixo.

4. **Barra de título escura pelo tema (J04) e `on_window_created` (R03).**
   - **Modo escuro:**
     - o Aylis já aplica o modo escuro pelo `theme.dark`, inclusive ao trocar de tema (`platform-winit/lib.rs`, `dark_title`);
     - por isso `set_dark_mode` e `apply_dark_mode` (`ui.rs:~164-185`, chamados em `bridge.rs:~577`) ficam redundantes.
   - **Espera pela janela:**
     - o `finish_show` (`ui.rs:~381`) tenta de novo até 60 vezes, a cada 16 ms, esperando o HWND;
     - o `on_window_created` do frmMain resolve isso, e é ali que devem entrar o ícone e a posição.
   - **Esforço:** baixo a médio.

## Talvez (depende de mais coisa)

5. **TrayIcon do Aylis, com `popup_menu` e `show_balloon`** (C26/C27; `aylis-controls-base/src/tray.rs`), mais o ícone certo pela escala (C41).
   - **O que ganharia:** substituiria as ~262 linhas do `tray.rs` do NoOBS.
   - **O que ainda falta no Aylis:**
     - recolocar o ícone quando a barra de tarefas reinicia (não há `TaskbarCreated` em `crates/`);
     - o `allow_from_lower_il` do processo elevado (`ui.rs:~740`);
     - o balão do Aylis vira toast pelo serviço de notificação, não pelo `NIF_INFO`.
   - **Quando migrar:** só depois de o Aylis cobrir isso.

6. **Vídeo como textura no Direct2D** (W15, `bfa34a2`: no 4K, cada quadro caiu de 13-17 ms para 1,6 ms).
   - **Hoje:** o player do NoOBS converte o quadro na placa e copia de volta para a memória (`playback/gpu.rs:~154`, `render(.., dst: &mut [u8])`) antes do `FrameView::set_frame`.
   - **O caminho no Aylis:** `GpuSurface::D3D12`, que aceita textura D3D11 com handle NT e `ID3D11Fence` (`aylis-core/canvas.rs:~398`).
   - **O que falta:** o `FrameView` não aceita `GpuSurface`. Seria preciso um controle próprio (ou mudar o FrameView no Aylis), mais a textura compartilhada no `gpu.rs`.
   - **Fora deste item:** `show_video` e `on_frame` são do VideoPlayer (Media Foundation) e não servem para o pipeline FFmpeg do NoOBS.
   - **Avaliação:** ganho grande, esforço alto.

7. **Headless e `ui.advance`** (B04, P08).
   - **O que permitiria:** testar telas sem janela, incluindo layout, foto e tempo.
   - **Hoje:** os testes de `app_settings.rs` (`~:2925`) já criam forms sem janela à mão.
   - **Esforço:** médio.

8. **Itens menores:**
   - **`default_button` e `cancel_button` (J23)** no `fraConfirm`, no lugar do Enter e do Esc tratados à mão (`app_player.rs:~1773-1779`). Não é certo que funcione bem, porque o diálogo é um frame dentro do frmMain, não um form próprio.
   - **`accessible_name` (C29)** nos ~27 botões que só têm ícone nos forms.
   - **`gap` em Layout e Panel (L05) e o form com `auto_size` (L07)** no lugar das medições manuais do `confirm` (`app_view.rs:~430-470`) e dos `set_top` em `app_settings.rs`.

## Não se aplica

- **TitleBar com o menu de layouts do Windows 11 e `window_corners` (`850180f`):** o NoOBS usa a barra de título nativa.
- **`area_select.rs`, `area_frame.rs` e `rec_indicator.rs`:** continuam em Win32 puro. Precisam de `WDA_EXCLUDEFROMCAPTURE` e de janela com cor-chave, e o Aylis não tem nada disso.
- **`show_owned` e `activate_on_show`:** o NoOBS tem uma janela só, com as telas dentro dela.

## Atenção: adaptar ou conferir

- **`GENERATOR_VERSION` subiu de 7 para 11.**
  - **Situação:** `rust/generated/components/version` diz `7`, embora o catálogo de componentes já seja o novo (`client_width` aparece no diff ainda sem commit). Ou seja, quem gerou foi um binário antigo da IDE.
  - **O que fazer:** abra o projeto na IDE atual, rode "Gerar código" e faça o commit junto com o código regerado.
  - **O que deve mudar:**
    - o `mod.rs` perde o `use ...form_handler` (gerador 10);
    - o `create_in` passa a pôr `fill` transparente (gerador 11). Os frames já têm `"fill": "transparent"` ou entram por `Frame`, então não deve mudar nada visível.
- **Mudanças de comportamento para conferir na tela:**
  - **Checked posto direto agora realça (C02):** afeta `btnSpeed.set_checked` e os chips do picker. Provavelmente é o desejado.
  - **O Label soma o `padding` (C05):** o estilo `pl_spk` tem `padding [3,0]`, então o `lblSpeaker` pode ter mudado de altura.
  - **Controle alinhado agora informa `width`/`height` do layout (L13):** pode alterar as alturas que o `fit_page` lê (`app_settings.rs:~1278`). Não é certo.
- **APIs:**
  - Nada que o NoOBS usa foi removido ou renomeado: `markup_runs`/`markup_plain`, `platform::win::hwnd`, `run_with` e `FrameView::set_frame` continuam.
  - Só o `on_message` passou a devolver `u64`, e o NoOBS não o usa.

