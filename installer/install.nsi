!ifndef VERSION
!define VERSION "dev"
!endif

!define APP_NAME "WSL Network Manager"
!define APP_ID   "WSLNetMan"

Name "${APP_NAME} ${VERSION}"
OutFile "dist\wslnetman-${VERSION}-setup.exe"
InstallDir "$PROGRAMFILES\WSLNetMan"
ShowInstDetails show
ShowUninstDetails show
RequestExecutionLevel admin

!include "MUI2.nsh"
!define MUI_ABORTWARNING
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_LANGUAGE "English"

Section "Install"
  SetOutPath "$INSTDIR"
  File "staging\wslnetman.exe"

  CreateDirectory "$SMPROGRAMS\WSLNetMan"
  CreateShortCut "$SMPROGRAMS\WSLNetMan\WSLNetMan.lnk" "$INSTDIR\wslnetman.exe"

  WriteUninstaller "$INSTDIR\Uninstall.exe"

  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "DisplayName" "${APP_NAME}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "DisplayVersion" "${VERSION}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "Publisher" "WSLNetMan"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "DisplayIcon" "$INSTDIR\wslnetman.exe"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}" "UninstallString" "$INSTDIR\Uninstall.exe"
SectionEnd

Section "Uninstall"
  Delete "$SMPROGRAMS\WSLNetMan\WSLNetMan.lnk"
  RMDir  "$SMPROGRAMS\WSLNetMan"

  Delete "$INSTDIR\wslnetman.exe"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir  "$INSTDIR"

  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_ID}"
SectionEnd
