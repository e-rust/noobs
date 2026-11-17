@echo off
rem Sobe uma versão nova do NoOBS: troca a versão, roda os testes, gera o instalador,
rem faz o commit e a tag, envia para o GitHub e cria a release com o instalador.
rem
rem   scripts\release.bat <versão> <notas.md> [/simular] [/sem-testes]
rem
rem   scripts\release.bat 0.0.14-beta C:\temp\notas.md /simular
rem   scripts\release.bat 0.0.14-beta C:\temp\notas.md
rem
rem O passo a passo está em docs\publicar-versao.md.

setlocal EnableExtensions DisableDelayedExpansion
chcp 65001 >nul
cd /d "%~dp0.."

set "VERSAO=%~1"
set "NOTAS=%~f2"
set "SIMULAR=0"
set "TESTES=1"
for %%a in (%3 %4) do (
    if /i "%%~a"=="/simular" set "SIMULAR=1"
    if /i "%%~a"=="/sem-testes" set "TESTES=0"
)
if "%VERSAO%"=="" goto :uso
if "%~2"=="" goto :uso

if not defined AYLIS_TOOL set "AYLIS_TOOL=%USERPROFILE%\.aylis\target\debug\aylis-tool.exe"
set "TAG=v%VERSAO%"
set "INSTALADOR=dist\windows\NoOBS-%VERSAO%-setup.exe"
set "MSG=%TEMP%\noobs-commit-%VERSAO%.txt"
set "NOTAS_FINAIS=%TEMP%\noobs-notas-%VERSAO%.md"
set "PRE="
if not "%VERSAO:-=%"=="%VERSAO%" set "PRE=--prerelease"

echo.
echo == NoOBS %TAG%
echo.
echo -- Conferindo
rem Ao mexer neste script: um parêntese dentro de um echo de um bloco (...) fecha o bloco;
rem as mensagens de erro passam por :erro, fora dos blocos.

powershell -NoProfile -Command "if ($env:VERSAO -notmatch '^\d+\.\d+\.\d+(-[0-9A-Za-z.]+)?$') { exit 1 }"
if errorlevel 1 (set "ERRO=Versão inválida: %VERSAO% (use o formato 0.0.14-beta)" & goto :erro)
if not exist "%NOTAS%" (set "ERRO=As notas não existem: %NOTAS%" & goto :erro)
where git >nul 2>&1 || (set "ERRO=O git não está no PATH." & goto :erro)
where gh >nul 2>&1 || (set "ERRO=O GitHub CLI (gh) não está no PATH." & goto :erro)
where cargo >nul 2>&1 || (set "ERRO=O cargo não está no PATH." & goto :erro)
if not exist "%AYLIS_TOOL%" (set "ERRO=O aylis-tool não está em %AYLIS_TOOL% (defina AYLIS_TOOL com o caminho dele)." & goto :erro)
gh auth status >nul 2>&1 || (set "ERRO=O gh não está conectado ao GitHub: rode gh auth login." & goto :erro)

for /f "delims=" %%b in ('git rev-parse --abbrev-ref HEAD') do set "RAMO=%%b"
if not "%RAMO%"=="main" (set "ERRO=O git está no ramo %RAMO%: a versão sai da main." & goto :erro)

rem Só o cache da conferência (rust\generated) pode estar mudado: ele vai no commit da versão.
git status --porcelain --untracked-files=no | findstr /v /c:"rust/generated/" >nul
if not errorlevel 1 (set "ERRO=Há mudanças sem commit (git status): faça o commit delas antes." & goto :erro)

git fetch -q origin || (set "ERRO=Não deu para falar com o GitHub (git fetch)." & goto :erro)
for /f %%n in ('git rev-list --count HEAD..origin/main') do set "ATRAS=%%n"
if not "%ATRAS%"=="0" (set "ERRO=A main daqui está %ATRAS% commit(s) atrás do GitHub: rode git pull." & goto :erro)

git rev-parse -q --verify "refs/tags/%TAG%" >nul && (set "ERRO=A tag %TAG% já existe aqui (git tag -d %TAG% apaga)." & goto :erro)
git ls-remote --exit-code --tags origin "refs/tags/%TAG%" >nul 2>&1 && (set "ERRO=A tag %TAG% já existe no GitHub." & goto :erro)
gh release view "%TAG%" >nul 2>&1 && (set "ERRO=A release %TAG% já existe no GitHub." & goto :erro)

rem O NoOBS rodando de dist\windows trava o exe e as DLLs de lá: a publicação não consegue
rem gravar por cima. Renomear não serve de teste (o Windows deixa renomear um exe em uso).
powershell -NoProfile -Command "$em=@(); Get-ChildItem -LiteralPath 'dist\windows' -File -ErrorAction SilentlyContinue | ForEach-Object { $n=$_.Name; try { $f=[IO.File]::Open($_.FullName, 'Open', 'ReadWrite', 'None'); $f.Close() } catch { $em+=$n } }; if ($em) { Write-Host ('   em uso: '+($em -join ', ')); exit 1 }"
if errorlevel 1 (set "ERRO=Há arquivos de dist\windows em uso: feche o NoOBS que roda de lá." & goto :erro)

for /f "usebackq delims=" %%t in (`powershell -NoProfile -Command "(Get-Content -LiteralPath $env:NOTAS -Encoding UTF8 -TotalCount 1).Trim().TrimEnd('.')"`) do set "TITULO=%%t"
if "%TITULO%"=="" (set "ERRO=A primeira linha das notas (o resumo da versão) está vazia." & goto :erro)

echo    tudo certo.
echo.
echo    Versão:     %VERSAO%  (tag %TAG%)
if defined PRE echo    Tipo:       pré-lançamento (a versão tem sufixo)
if not defined PRE echo    Tipo:       lançamento
echo    Resumo:     %TITULO%
echo    Notas:      %NOTAS%
echo    Testes:     %TESTES%  (1 = roda)
echo    aylis-tool: %AYLIS_TOOL%
echo.

if "%SIMULAR%"=="1" (
    echo -- Simulação: nada foi mudado. Sem /simular, o script faria:
    echo    1. a versão no NoOBS.aylisproj
    echo    2. os testes
    echo    3. o instalador em %INSTALADOR%
    echo    4. o commit, a tag %TAG% e o envio para o GitHub
    echo    5. a release no GitHub com o instalador
    exit /b 0
)

echo -- 1. Versão no NoOBS.aylisproj
powershell -NoProfile -Command "$p='NoOBS.aylisproj'; $s=[IO.File]::ReadAllText($p); $r=[regex]'(\x22publish\x22\s*:\s*\{[^{}]*?\x22version\x22\s*:\s*\x22)[^\x22]*'; if (-not $r.IsMatch($s)) { exit 1 }; $s=$r.Replace($s, '${1}'+$env:VERSAO, 1); [IO.File]::WriteAllText($p, $s, (New-Object Text.UTF8Encoding $false))"
if errorlevel 1 (set "ERRO=Não achei publish.version no NoOBS.aylisproj." & goto :erro_antes_do_commit)

if "%TESTES%"=="1" (
    echo -- 2. Testes
    "%AYLIS_TOOL%" generate NoOBS.aylisproj || (set "ERRO=A geração do código dos forms falhou." & goto :erro_antes_do_commit)
    pushd rust
    cargo test --no-fail-fast
    if errorlevel 1 (popd & (set "ERRO=Os testes falharam." & goto :erro_antes_do_commit))
    popd
) else (
    echo -- 2. Testes: pulados (/sem-testes^)
)

echo -- 3. Instalador
"%AYLIS_TOOL%" publish NoOBS.aylisproj --installer || (set "ERRO=A publicação falhou." & goto :erro_antes_do_commit)
if not exist "%INSTALADOR%" (set "ERRO=A publicação terminou, mas %INSTALADOR% não existe." & goto :erro_antes_do_commit)

rem As notas da release: as que você escreveu, mais a instalação com o SHA-256 do instalador.
powershell -NoProfile -Command "$h=(Get-FileHash -Algorithm SHA256 -LiteralPath $env:INSTALADOR).Hash; $n=(Get-Content -Raw -LiteralPath $env:NOTAS -Encoding UTF8).TrimEnd(); $nl=[Environment]::NewLine; $f=$n+$nl+$nl+'## Instalação'+$nl+'Baixe `NoOBS-'+$env:VERSAO+'-setup.exe` abaixo e execute. Ele instala para o seu usuário, sem pedir administrador, em `%%LOCALAPPDATA%%\NoOBS`.'+$nl+$nl+'Requer Windows 10 ou 11 (64 bits).'+$nl+$nl+'SHA-256 do instalador: `'+$h+'`'+$nl+$nl+'## Atualizações'+$nl+'Quem tem uma versão beta anterior recebe esta pela checagem diária (Configurações → Atualizações).'+$nl; [IO.File]::WriteAllText($env:NOTAS_FINAIS, $f, (New-Object Text.UTF8Encoding $false)); [IO.File]::WriteAllText($env:MSG, 'NoOBS v'+$env:VERSAO+': '+$env:TITULO+$nl, (New-Object Text.UTF8Encoding $false)); Write-Host ('   SHA-256: '+$h)"
if errorlevel 1 (set "ERRO=Não deu para montar as notas." & goto :erro_antes_do_commit)

echo -- 4. Commit, tag e envio
git add NoOBS.aylisproj rust/generated || (set "ERRO=git add falhou." & goto :erro_antes_do_commit)
git commit -q -F "%MSG%" || (set "ERRO=git commit falhou." & goto :erro_antes_do_commit)
git tag "%TAG%" || (set "ERRO=git tag falhou (o commit da versão já foi feito)." & goto :erro)
rem O gh entrega a credencial: o gerenciador de credenciais do Git às vezes para numa
rem janela de escolher a conta.
git -c credential.helper= -c "credential.helper=!gh auth git-credential" push origin main "%TAG%"
if errorlevel 1 (set "ERRO=O envio falhou. O commit e a tag estão só aqui: rode de novo o git push origin main %TAG%." & goto :erro)

echo -- 5. Release no GitHub
gh release create "%TAG%" "%INSTALADOR%" --title "NoOBS %TAG%" --notes-file "%NOTAS_FINAIS%" %PRE%
if errorlevel 1 (set "ERRO=A release não foi criada (a tag já está no GitHub). Notas em %NOTAS_FINAIS%." & goto :erro)

echo.
echo == Pronto: https://github.com/e-rust/noobs/releases/tag/%TAG%
exit /b 0

:uso
echo Uso: scripts\release.bat ^<versão^> ^<notas.md^> [/simular] [/sem-testes]
echo.
echo   scripts\release.bat 0.0.14-beta C:\temp\notas.md /simular
echo   scripts\release.bat 0.0.14-beta C:\temp\notas.md
echo.
echo O passo a passo está em docs\publicar-versao.md.
exit /b 2

:erro_antes_do_commit
echo.
echo ERRO: %ERRO%
echo A versão no NoOBS.aylisproj pode ter mudado. Para desfazer: git checkout -- NoOBS.aylisproj
exit /b 1

:erro
echo.
echo ERRO: %ERRO%
exit /b 1
