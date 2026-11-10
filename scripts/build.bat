@echo off
REM ---------------------------------------------------------------------
REM  build.bat - compila o NoOBS com o compilador do Aylis e poe o exe no runtime\.
REM
REM  Uso:
REM      scripts\build.bat           aylis-tool build NoOBS.aylisproj --release
REM      scripts\build.bat debug     aylis-tool build NoOBS.aylisproj (sem otimizacao)
REM
REM  O aylis-tool build gera o codigo dos forms (rust\generated), verifica o
REM  projeto e so entao compila (o cargo), parando no primeiro erro - o mesmo
REM  que o Compilar da IDE.
REM
REM  Saida: runtime\NoOBS.exe (na raiz do repositorio), ao lado das DLLs do
REM  FFmpeg - e de la que o app roda. O runtime e SO isso: a interface (forms
REM  do Aylis) e as traducoes (locales\) vao compiladas dentro do exe.
REM  O instalador sai do Aylis: aylis-tool publish NoOBS.aylisproj --installer.
REM ---------------------------------------------------------------------
setlocal EnableDelayedExpansion
set "ROOT=%~dp0.."
cd /d "%ROOT%\rust"

where cargo >nul 2>nul
if errorlevel 1 (
  echo [build] ERRO: cargo nao encontrado. Instale o Rust ^(rustup.rs^).
  exit /b 1
)
if not exist "%ROOT%\runtime\avcodec-61.dll" (
  echo [build] ERRO: runtime\avcodec-61.dll nao existe - faltam as DLLs do FFmpeg.
  echo         Copie pra runtime\ ^(na raiz^), do OBS portatil 32.1.2 ^(bin\64bit^):
  echo         avcodec-61 avformat-61 avutil-59 swscale-8 swresample-5
  echo         libx264-164 zlib srt librist ^(.dll^).
  exit /b 1
)

REM  O rust\Cargo.toml acha o Aylis em .aylis\sdk, uma juncao ate a instalacao desta maquina
REM  (fora do git). A IDE e o aylis-tool criam; aqui, pelo registro do instalador do Aylis.
if not exist "%ROOT%\.aylis\sdk\crates\aylis\Cargo.toml" (
  set "SDK="
  for /f "skip=2 tokens=2,*" %%a in ('reg query "HKCU\Software\Aylis" /v SdkDir 2^>nul') do set "SDK=%%b"
  if not defined SDK (
    echo [build] ERRO: o Aylis nao esta instalado ^(HKCU\Software\Aylis^). Instale a IDE do Aylis
    echo         ou abra o NoOBS.aylisproj nela uma vez, que cria a juncao .aylis\sdk.
    exit /b 1
  )
  if exist "%ROOT%\.aylis\sdk" rmdir "%ROOT%\.aylis\sdk"
  if not exist "%ROOT%\.aylis" mkdir "%ROOT%\.aylis"
  mklink /J "%ROOT%\.aylis\sdk" "!SDK!" >nul
)

REM  O compilador do Aylis (aylis-tool.exe): o da instalacao (InstallDir do
REM  registro do instalador, HKCU\Software\Aylis) ou, sem instalacao, o do
REM  repositorio do Aylis compilado (.aylis\sdk\target\release ou debug).
set "TOOL="
for /f "skip=2 tokens=2,*" %%a in ('reg query "HKCU\Software\Aylis" /v InstallDir 2^>nul') do (
  if exist "%%b\aylis-tool.exe" set "TOOL=%%b\aylis-tool.exe"
)
if not defined TOOL if exist "%ROOT%\.aylis\sdk\target\release\aylis-tool.exe" set "TOOL=%ROOT%\.aylis\sdk\target\release\aylis-tool.exe"
if not defined TOOL if exist "%ROOT%\.aylis\sdk\target\debug\aylis-tool.exe" set "TOOL=%ROOT%\.aylis\sdk\target\debug\aylis-tool.exe"
if not defined TOOL (
  echo [build] ERRO: aylis-tool.exe nao encontrado. Instale a IDE do Aylis ou compile o
  echo         repositorio dele ^(cargo build --release -p aylis-tools^).
  exit /b 1
)

set "PROFILE=release"
set "FLAGS=--release"
if /i "%~1"=="debug" (
  set "PROFILE=debug"
  set "FLAGS="
)

echo [build] aylis-tool build NoOBS.aylisproj %FLAGS% ...
echo         ^(!TOOL!^)
"!TOOL!" build "%ROOT%\NoOBS.aylisproj" %FLAGS%
if errorlevel 1 (
  echo [build] ERRO: o compilador do Aylis falhou.
  exit /b 1
)

REM  A pasta de saida e a do cargo que o aylis-tool chamou (a compartilhada do Aylis,
REM  pelo .cargo\config.toml que ele grava).
set "TDIR=%CD%\target"
for /f "delims=" %%t in ('powershell -NoProfile -Command "(cargo metadata --format-version 1 --no-deps | ConvertFrom-Json).target_directory"') do set "TDIR=%%t"
copy /y "%TDIR%\%PROFILE%\noobs.exe" "%ROOT%\runtime\NoOBS.exe" >nul
if errorlevel 1 (
  echo [build] ERRO: nao consegui copiar o exe ^(o NoOBS de runtime\ esta aberto?^).
  exit /b 1
)
echo [build] OK: runtime\NoOBS.exe ^(%PROFILE%^)
exit /b 0
