# Como publicar uma versão nova do NoOBS

Uma versão nova do NoOBS é uma release no GitHub com o instalador
`NoOBS-<versão>-setup.exe`. Quem já tem o NoOBS recebe a versão pela checagem diária
de atualizações, que lê as releases do repositório.

O `scripts\release.bat` faz tudo: troca a versão, roda os testes, gera o instalador, faz
o commit e a tag, envia para o GitHub e cria a release. Você só escreve as notas.

## Resumo

1. Faça o commit e o push das mudanças da versão.
2. Feche o NoOBS e feche o projeto na IDE do Aylis.
3. Escreva as notas num arquivo `.md`.
4. Simule: `scripts\release.bat 0.0.14-beta C:\temp\notas.md /simular`
5. Publique: `scripts\release.bat 0.0.14-beta C:\temp\notas.md`

## O que precisa estar instalado

| Ferramenta | Para quê | Como conferir |
| --- | --- | --- |
| Git | commit, tag e envio | `git --version` |
| GitHub CLI (`gh`), conectado à conta | criar a release e enviar sem pedir senha | `gh auth status` |
| Rust (`cargo`) | rodar os testes | `cargo --version` |
| Aylis (o `aylis-tool`) | gerar o código dos forms e o instalador | o arquivo `%USERPROFILE%\.aylis\target\debug\aylis-tool.exe` |

Se o `gh` não estiver conectado, rode `gh auth login` uma vez.

Se o `aylis-tool` estiver em outro lugar, defina a variável `AYLIS_TOOL` antes de rodar o
script:

```bat
set AYLIS_TOOL=D:\aylis\aylis-tool.exe
```

## Onde fica a versão

A versão fica no `NoOBS.aylisproj`, em `publish` → `version`:

```json
"publish": {
  "app_name": "NoOBS",
  "version": "0.0.13-beta",
```

O script troca esse valor sozinho. Na IDE do Aylis, ela fica nas opções do projeto, na
parte de publicação.

O formato é `X.Y.Z` ou `X.Y.Z-sufixo`:
- **Com sufixo** (`0.0.14-beta`): sai como pré-lançamento no GitHub. É o que o NoOBS usa
  enquanto está em beta.
- **Sem sufixo** (`1.0.0`): sai como lançamento.

A versão nova precisa ser maior que a última. As releases publicadas aparecem com
`gh release list`.

## Passo a passo

### 1. Deixe a main pronta

As mudanças da versão precisam estar com commit feito e enviadas para o GitHub. O script
recusa se houver mudança sem commit, se você não estiver na `main` ou se a `main` local
estiver atrás da do GitHub.

```bat
git status
git push
```

### 2. Feche o NoOBS e o projeto na IDE

- **O NoOBS:** feche pelo ícone da bandeja (Sair). Se ele estiver rodando de
  `dist\windows`, a publicação não consegue trocar o `NoOBS.exe` de lá.
- **O projeto na IDE do Aylis:** com o projeto aberto, a IDE pode gravar o
  `NoOBS.aylisproj` por cima da versão que o script acabou de trocar.

### 3. Escreva as notas

Crie um arquivo `.md`, fora do repositório ou em qualquer pasta. A primeira linha é o
resumo da versão: ela vira o título do commit (`NoOBS v0.0.14-beta: <resumo>`). Não use
`&`, `|`, `<` nem `>` nessa linha.

Exemplo:

```markdown
Correções no marca-texto do print.

## Mudanças
- **Nome da novidade:** o que mudou, do ponto de vista de quem usa.

## Correções
- **O problema:** como era e como ficou.
```

Não escreva a parte de instalação: o script acrescenta no fim das notas a seção
"Instalação", com o SHA-256 do instalador, e a seção "Atualizações".

### 4. Simule

```bat
scripts\release.bat 0.0.14-beta C:\temp\notas.md /simular
```

A simulação faz todas as conferências e mostra o que faria, sem mudar nada. Ela recusa
quando:
- a versão está fora do formato;
- o arquivo de notas não existe ou a primeira linha está vazia;
- falta alguma ferramenta, ou o `gh` não está conectado;
- há mudança sem commit, você não está na `main` ou a `main` está atrás do GitHub;
- a tag ou a release dessa versão já existem;
- o `dist\windows\NoOBS.exe` está em uso.

### 5. Publique

```bat
scripts\release.bat 0.0.14-beta C:\temp\notas.md
```

O script leva alguns minutos e faz, em ordem:

1. troca a versão no `NoOBS.aylisproj`;
2. gera o código dos forms e roda os testes (`cargo test`);
3. gera o instalador em `dist\windows\NoOBS-0.0.14-beta-setup.exe`;
4. faz o commit (do `NoOBS.aylisproj` e do cache em `rust\generated`), cria a tag
   `v0.0.14-beta` e envia os dois para o GitHub;
5. cria a release no GitHub com o instalador e as notas.

No fim, ele mostra o link da release.

Para pular os testes, quando eles acabaram de passar na IDE, acrescente `/sem-testes`:

```bat
scripts\release.bat 0.0.14-beta C:\temp\notas.md /sem-testes
```

### 6. Confira

- Abra o link da release: o instalador tem que estar anexado e as notas, com o SHA-256.
- Se quiser, instale por cima de uma versão anterior para conferir a atualização.

## Quando dá errado

O script para no primeiro erro e diz em que passo parou.

| Parou em | Situação | O que fazer |
| --- | --- | --- |
| Conferindo | nada mudou | corrija o que a mensagem diz e rode de novo |
| 1, 2 ou 3 (versão, testes ou instalador) | só a versão no `NoOBS.aylisproj` mudou | desfaça com `git checkout -- NoOBS.aylisproj`, corrija e rode de novo |
| 4, na tag ou no envio | o commit da versão já existe aqui | veja abaixo |
| 5 (release) | o commit e a tag já estão no GitHub | crie a release à mão (veja abaixo) |

### O envio falhou

O commit e a tag estão só no seu computador. Tente enviar de novo:

```bat
git -c credential.helper= -c "credential.helper=!gh auth git-credential" push origin main v0.0.14-beta
```

Depois crie a release (abaixo).

### Criar a release à mão

As notas completas, com a parte de instalação, ficam em
`%TEMP%\noobs-notas-<versão>.md`.

```bat
gh release create v0.0.14-beta dist\windows\NoOBS-0.0.14-beta-setup.exe --title "NoOBS v0.0.14-beta" --notes-file "%TEMP%\noobs-notas-0.0.14-beta.md" --prerelease
```

Para um lançamento sem sufixo, tire o `--prerelease`.

### Recomeçar uma versão do zero

Se a versão saiu errada e você quer refazê-la com o mesmo número, apague a release e a tag,
no GitHub e aqui:

```bat
gh release delete v0.0.14-beta --yes
git push origin :refs/tags/v0.0.14-beta
git tag -d v0.0.14-beta
```

Quem já baixou a versão continua com ela. Se a release ficou pública por um tempo, é mais
seguro publicar a próxima versão (`0.0.15-beta`) do que reaproveitar o número.

### Arquivos de `dist\windows` em uso

O NoOBS rodando de `dist\windows` trava o exe e as DLLs de lá, e a publicação não consegue
gravar por cima. O script avisa quais arquivos estão em uso.

Feche o NoOBS pela bandeja. Se ele roda como administrador e você não consegue fechá-lo,
mova os arquivos para outra pasta (o Windows deixa mover um exe e as DLLs em uso, desde
que seja no mesmo disco) e rode o script de novo:

```bat
mkdir dist\_em-uso
move dist\windows\* dist\_em-uso\
```

A pasta `dist\_em-uso` pode ser apagada depois que esse NoOBS fechar.
