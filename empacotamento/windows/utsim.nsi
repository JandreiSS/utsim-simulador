; Instalador do UTSim para Windows (NSIS 3).
;
; Instala por usuário, sem exigir administrador, em %LOCALAPPDATA%\Programs\UTSim.
; Compilar com scripts/instalador-windows.sh, que monta a pasta de origem e chama:
;   makensis /DVERSAO=0.1.0 /DORIGEM=<pasta> /DICONE=<utsim.ico> /DSAIDA=<instalador.exe> utsim.nsi
;
; Decisão sobre associação de arquivos: nenhuma. O Windows associa pela última parte do nome,
; então "modelo.utsim.ron" cairia na extensão ".ron", genérica demais para tomarmos. O modelo
; abre com "utsim-view modelo.ron" ou arrastando o arquivo sobre utsim-view.exe.

Unicode true

!ifndef VERSAO
  !define VERSAO "0.0.0"
!endif
!ifndef ORIGEM
  !error "Defina /DORIGEM=<pasta montada por scripts/preparar-pacote.sh>"
!endif
!ifndef ICONE
  !error "Defina /DICONE=<caminho do utsim.ico>"
!endif
!ifndef SAIDA
  !define SAIDA "utsim-instalador.exe"
!endif

!include "MUI2.nsh"
!include "LogicLib.nsh"
!include "WinMessages.nsh"

Name "UTSim ${VERSAO}"
OutFile "${SAIDA}"
RequestExecutionLevel user
InstallDir "$LOCALAPPDATA\Programs\UTSim"
InstallDirRegKey HKCU "Software\UTSim" "Pasta"
SetCompressor /SOLID lzma
BrandingText "UTSim ${VERSAO}"

!define MUI_ICON "${ICONE}"
!define MUI_UNICON "${ICONE}"
!define MUI_ABORTWARNING
!define MUI_FINISHPAGE_RUN "$INSTDIR\bin\utsim-view.exe"
!define MUI_FINISHPAGE_RUN_PARAMETERS "--novo"
!define MUI_FINISHPAGE_RUN_TEXT "Abrir o UTSim agora"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "PortugueseBR"

VIProductVersion "${VERSAO}.0"
VIAddVersionKey /LANG=${LANG_PORTUGUESEBR} "ProductName" "UTSim"
VIAddVersionKey /LANG=${LANG_PORTUGUESEBR} "FileDescription" "Instalador do UTSim"
VIAddVersionKey /LANG=${LANG_PORTUGUESEBR} "FileVersion" "${VERSAO}"
VIAddVersionKey /LANG=${LANG_PORTUGUESEBR} "LegalCopyright" "Copyright (C) 2026 Jandrei"

; Remove de $R0 (lista de pastas separadas por ";") a entrada "$INSTDIR\bin".
; Mesmo código para o instalador e para o desinstalador (que exige o prefixo "un.").
!macro DEFINIR_FILTRAR_PATH PREFIXO
Function ${PREFIXO}FiltrarPath
  Push $R1
  Push $R2
  Push $R3
  Push $R4
  StrCpy $R1 ""
  StrCpy $R2 ""
  StrCpy $R3 0
  laco:
    StrCpy $R4 $R0 1 $R3
    IntOp $R3 $R3 + 1
    ${If} $R4 == ";"
    ${OrIf} $R4 == ""
      ${If} $R2 != ""
      ${AndIf} $R2 != "$INSTDIR\bin"
        ${If} $R1 == ""
          StrCpy $R1 $R2
        ${Else}
          StrCpy $R1 "$R1;$R2"
        ${EndIf}
      ${EndIf}
      StrCpy $R2 ""
      ${If} $R4 == ""
        Goto fim
      ${EndIf}
    ${Else}
      StrCpy $R2 "$R2$R4"
    ${EndIf}
    Goto laco
  fim:
  StrCpy $R0 $R1
  Pop $R4
  Pop $R3
  Pop $R2
  Pop $R1
FunctionEnd
!macroend
!insertmacro DEFINIR_FILTRAR_PATH ""
!insertmacro DEFINIR_FILTRAR_PATH "un."

Section "UTSim (programas, exemplos, manual e ativos)" SecPrincipal
  SectionIn RO

  SetOutPath "$INSTDIR\bin"
  File "${ORIGEM}\bin\utsim.exe"
  File "${ORIGEM}\bin\utsim-view.exe"

  SetOutPath "$INSTDIR"
  File "${ORIGEM}\README.md"
  File "${ORIGEM}\CHANGELOG.md"
  File /r "${ORIGEM}\examples"
  File /r "${ORIGEM}\assets"
  File /r "${ORIGEM}\manual"

  WriteUninstaller "$INSTDIR\desinstalar.exe"
  WriteRegStr HKCU "Software\UTSim" "Pasta" "$INSTDIR"

  ; Entrada em "Aplicativos instalados" (por usuário).
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\UTSim" "DisplayName" "UTSim"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\UTSim" "DisplayVersion" "${VERSAO}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\UTSim" "Publisher" "Projeto UTSim"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\UTSim" "DisplayIcon" "$INSTDIR\bin\utsim-view.exe"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\UTSim" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\UTSim" "UninstallString" '"$INSTDIR\desinstalar.exe"'
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\UTSim" "QuietUninstallString" '"$INSTDIR\desinstalar.exe" /S'
  WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\UTSim" "NoModify" 1
  WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\UTSim" "NoRepair" 1
SectionEnd

Section "Atalhos no Menu Iniciar" SecAtalhos
  SetShellVarContext current
  SetOutPath "$INSTDIR"
  CreateDirectory "$SMPROGRAMS\UTSim"
  CreateShortcut "$SMPROGRAMS\UTSim\UTSim.lnk" "$INSTDIR\bin\utsim-view.exe" "--novo" "$INSTDIR\bin\utsim-view.exe" 0
  CreateShortcut "$SMPROGRAMS\UTSim\UTSim (exemplos).lnk" "$INSTDIR\examples"
  ; Com o manual em HTML, abre a página inicial; sem ele (só Markdown), abre a pasta.
  !if /FileExists "${ORIGEM}\manual\index.html"
    CreateShortcut "$SMPROGRAMS\UTSim\Manual do UTSim.lnk" "$INSTDIR\manual\index.html"
  !else
    CreateShortcut "$SMPROGRAMS\UTSim\Manual do UTSim.lnk" "$INSTDIR\manual"
  !endif
  CreateShortcut "$SMPROGRAMS\UTSim\Desinstalar o UTSim.lnk" "$INSTDIR\desinstalar.exe"
SectionEnd

Section /o "Adicionar o comando utsim ao PATH (terminal)" SecPath
  ReadRegStr $R0 HKCU "Environment" "Path"
  Call FiltrarPath
  ${If} $R0 == ""
    StrCpy $R0 "$INSTDIR\bin"
  ${Else}
    StrCpy $R0 "$R0;$INSTDIR\bin"
  ${EndIf}
  WriteRegExpandStr HKCU "Environment" "Path" "$R0"
  SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
SectionEnd

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
  !insertmacro MUI_DESCRIPTION_TEXT ${SecPrincipal} "Os programas utsim e utsim-view, os modelos de exemplo, o manual, os modelos 3D e os ativos de terceiros com suas licenças."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecAtalhos} "Cria a pasta UTSim no Menu Iniciar, com atalhos para o editor, os exemplos e o manual."
  !insertmacro MUI_DESCRIPTION_TEXT ${SecPath} "Permite digitar utsim em qualquer terminal novo (só para o seu usuário)."
!insertmacro MUI_FUNCTION_DESCRIPTION_END

Section "Uninstall"
  SetShellVarContext current

  ; Tira a pasta bin do PATH do usuário, se estiver lá.
  ReadRegStr $R0 HKCU "Environment" "Path"
  ${If} $R0 != ""
    Call un.FiltrarPath
    WriteRegExpandStr HKCU "Environment" "Path" "$R0"
    SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=5000
  ${EndIf}

  Delete "$SMPROGRAMS\UTSim\UTSim.lnk"
  Delete "$SMPROGRAMS\UTSim\UTSim (exemplos).lnk"
  Delete "$SMPROGRAMS\UTSim\Manual do UTSim.lnk"
  Delete "$SMPROGRAMS\UTSim\Desinstalar o UTSim.lnk"
  RMDir "$SMPROGRAMS\UTSim"

  ; Apaga só o que o instalador criou, nunca a pasta escolhida inteira.
  RMDir /r "$INSTDIR\bin"
  RMDir /r "$INSTDIR\examples"
  RMDir /r "$INSTDIR\assets"
  RMDir /r "$INSTDIR\manual"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\CHANGELOG.md"
  Delete "$INSTDIR\desinstalar.exe"
  RMDir "$INSTDIR"

  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\UTSim"
  DeleteRegKey HKCU "Software\UTSim"
SectionEnd
