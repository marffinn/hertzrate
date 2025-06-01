[Setup]
AppName=HertzRate
AppVersion=0.1.0
AppPublisher=HertzRate
AppPublisherURL=https://github.com/your-username/hertzrate
AppSupportURL=https://github.com/your-username/hertzrate/issues
AppUpdatesURL=https://github.com/your-username/hertzrate/releases
DefaultDirName={autopf}\HertzRate
DefaultGroupName=HertzRate
AllowNoIcons=yes
LicenseFile=LICENSE
OutputDir=installer
OutputBaseFilename=HertzRate-Setup-v0.1.0
; SetupIconFile=icon.ico
Compression=lzma
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
ArchitecturesAllowed=x64
ArchitecturesInstallIn64BitMode=x64

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "quicklaunchicon"; Description: "{cm:CreateQuickLaunchIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked; OnlyBelowVersion: 6.1
Name: "addtopath"; Description: "Add to PATH environment variable"; GroupDescription: "System Integration"

[Files]
Source: "target\release\hertzrate-gui.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "target\release\hertzrate.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "README.md"; DestDir: "{app}"; Flags: ignoreversion
Source: "LICENSE"; DestDir: "{app}"; Flags: ignoreversion; AfterInstall: CreateLicenseIfMissing

[Icons]
Name: "{group}\HertzRate"; Filename: "{app}\hertzrate-gui.exe"; IconFilename: "{app}\hertzrate-gui.exe"
Name: "{group}\HertzRate CLI"; Filename: "{app}\hertzrate.exe"; IconFilename: "{app}\hertzrate.exe"
Name: "{group}\{cm:UninstallProgram,HertzRate}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\HertzRate"; Filename: "{app}\hertzrate-gui.exe"; Tasks: desktopicon; IconFilename: "{app}\hertzrate-gui.exe"
Name: "{userappdata}\Microsoft\Internet Explorer\Quick Launch\HertzRate"; Filename: "{app}\hertzrate-gui.exe"; Tasks: quicklaunchicon; IconFilename: "{app}\hertzrate-gui.exe"

[Registry]
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Tasks: addtopath; Check: NeedsAddPath('{app}')

[Run]
Filename: "{app}\hertzrate-gui.exe"; Description: "{cm:LaunchProgram,HertzRate}"; Flags: nowait postinstall skipifsilent

[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE,
    'SYSTEM\CurrentControlSet\Control\Session Manager\Environment',
    'Path', OrigPath)
  then begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
end;

procedure CreateLicenseIfMissing();
var
  LicenseFile: string;
begin
  LicenseFile := ExpandConstant('{app}\LICENSE');
  if not FileExists(LicenseFile) then
  begin
    SaveStringToFile(LicenseFile, 
      'MIT License' + #13#10 + #13#10 +
      'Copyright (c) 2024 HertzRate' + #13#10 + #13#10 +
      'Permission is hereby granted, free of charge, to any person obtaining a copy' + #13#10 +
      'of this software and associated documentation files (the "Software"), to deal' + #13#10 +
      'in the Software without restriction, including without limitation the rights' + #13#10 +
      'to use, copy, modify, merge, publish, distribute, sublicense, and/or sell' + #13#10 +
      'copies of the Software, and to permit persons to whom the Software is' + #13#10 +
      'furnished to do so, subject to the following conditions:' + #13#10 + #13#10 +
      'The above copyright notice and this permission notice shall be included in all' + #13#10 +
      'copies or substantial portions of the Software.' + #13#10 + #13#10 +
      'THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR' + #13#10 +
      'IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,' + #13#10 +
      'FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE' + #13#10 +
      'AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER' + #13#10 +
      'LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,' + #13#10 +
      'OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE' + #13#10 +
      'SOFTWARE.', False);
  end;
end;
