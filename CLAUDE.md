# NoOBS — guia para agentes

O NoOBS é um projeto do Aylis (`NoOBS.aylisproj`): forms em `forms\`, temas em `themes\`,
imagens em `assets\`, traduções em `locales\` e o código em `rust\`. Texto em português do Brasil.

## Compilar, executar e mexer no projeto: pelo MCP do Aylis

Use o servidor MCP `aylis` (a IDE do Aylis), não scripts nem o `cargo` direto:

- `open_project` com o `NoOBS.aylisproj`, depois `build` (espera terminar e devolve os erros),
  `run_without_debug` ou `run` para executar, `output` para as mensagens.
- O `build` gera o código dos forms, confere o projeto, compila e põe o `NoOBS.exe` em
  `runtime\` (o `output` do projeto), ao lado das DLLs do FFmpeg.
- Com o projeto aberto na IDE, forms, temas, `.aylisproj` e código mudam pelo MCP
  (`edit_code`, `set_property`, `save`...), não pelas ferramentas de arquivo: a IDE guarda o
  que está aberto e perderia a mudança feita por fora.
- O instalador: `publish` (ou Publicar na IDE).

Não há script de compilação no repositório: o `scripts\build.bat` saiu em 12/11/2026 (procurava
o `aylis-tool` num lugar que o Aylis não usa mais).

## Publicar uma versão

O passo a passo está em `docs\publicar-versao.md`. O `scripts\release.bat` (de 17/11/2026) faz a
release inteira pelo `aylis-tool` (em `%USERPROFILE%\.aylis\target\debug`, ou em `AYLIS_TOOL`):
versão no `.aylisproj`, testes, instalador, commit, tag, envio e release no GitHub. Ele mexe no
`.aylisproj` por fora da IDE: com o projeto aberto nela, feche antes. Ao mudar o script, rode com
`/simular`, que confere tudo sem mudar nada.
