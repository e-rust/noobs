@echo off
REM ---------------------------------------------------------------------
REM  build.bat - compila o NoOBS (rust\) e poe o exe no runtime\.
REM
REM  Uso:
REM      scripts\build.bat           cargo build --release
REM      scripts\build.bat debug     cargo build (mais rapido, sem otimizacao)
REM
REM  Saida: runtime\NoOBS.exe (na raiz do repositorio), ao lado das DLLs do
REM  FFmpeg - e de la que o app roda. O runtime e SO isso: a interface (forms
REM  do Iron) e as traducoes (locales\) vao compiladas dentro do exe.
REM  O instalador sai do Iron: iron-tool publish NoOBS.ironproj --installer.
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

REM  O rust\Cargo.toml acha o Iron em .iron\sdk, uma juncao ate a instalacao desta maquina
REM  (fora do git). A IDE e o iron-tool criam; aqui, pelo registro do instalador do Iron.
if not exist "%ROOT%\.iron\sdk\crates\iron\Cargo.toml" (
  set "SDK="
  for /f "skip=2 tokens=2,*" %%a in ('reg query "HKCU\Software\Iron" /v SdkDir 2^>nul') do set "SDK=%%b"
  if not defined SDK (
    echo [build] ERRO: o Iron nao esta instalado ^(HKCU\Software\Iron^). Instale a IDE do Iron
    echo         ou abra o NoOBS.ironproj nela uma vez, que cria a juncao .iron\sdk.
    exit /b 1
  )
  if exist "%ROOT%\.iron\sdk" rmdir "%ROOT%\.iron\sdk"
  if not exist "%ROOT%\.iron" mkdir "%ROOT%\.iron"
  mklink /J "%ROOT%\.iron\sdk" "!SDK!" >nul
)

set "PROFILE=release"
set "FLAGS=--release"
if /i "%~1"=="debug" (
  set "PROFILE=debug"
  set "FLAGS="
)

echo [build] cargo build %FLAGS% ...
cargo build %FLAGS%
if errorlevel 1 (
  echo [build] ERRO: cargo build falhou.
  exit /b 1
)

REM  A pasta de saida e a do cargo (a compartilhada do Iron, pelo .cargo\config.toml).
set "TDIR=%CD%\target"
for /f "delims=" %%t in ('powershell -NoProfile -Command "(cargo metadata --format-version 1 --no-deps | ConvertFrom-Json).target_directory"') do set "TDIR=%%t"
copy /y "%TDIR%\%PROFILE%\noobs.exe" "%ROOT%\runtime\NoOBS.exe" >nul
if errorlevel 1 (
  echo [build] ERRO: nao consegui copiar o exe ^(o NoOBS de runtime\ esta aberto?^).
  exit /b 1
)
echo [build] OK: runtime\NoOBS.exe ^(%PROFILE%^)
exit /b 0
