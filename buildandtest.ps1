# Get current timestamp for filename
$timestamp = Get-Date -Format "yyyy-MM-dd_HHmm"
$captureFile = ".\terminal captures\terminalcapture_$timestamp.md"

# Ensure the terminal captures directory exists
if (!(Test-Path ".\terminal captures")) {
    New-Item -ItemType Directory -Path ".\terminal captures" -Force
}

# Create the capture file with metadata header
$metadata = @"
**********************
Windows PowerShell transcript start
Start time: $(Get-Date -Format "yyyyMMddHHmmss")
Username: $env:USERDOMAIN\$env:USERNAME
RunAs User: $env:USERDOMAIN\$env:USERNAME
Configuration Name: 
Machine: $env:COMPUTERNAME (Microsoft Windows NT $(([System.Environment]::OSVersion.Version).ToString()))
Host Application: PowerShell Build and Test Script
Process ID: $PID
PSVersion: $($PSVersionTable.PSVersion.ToString())
PSEdition: $($PSVersionTable.PSEdition)
PSCompatibleVersions: $($PSVersionTable.PSCompatibleVersions -join ', ')
BuildVersion: $($PSVersionTable.BuildVersion.ToString())
CLRVersion: $($PSVersionTable.CLRVersion.ToString())
WSManStackVersion: $($PSVersionTable.WSManStackVersion.ToString())
PSRemotingProtocolVersion: $($PSVersionTable.PSRemotingProtocolVersion.ToString())
SerializationVersion: $($PSVersionTable.SerializationVersion.ToString())
**********************

"@

$metadata | Out-File -FilePath $captureFile -Encoding UTF8

# Add the initial prompt
"PS $((Get-Location).Path)> ./buildandtest.ps1" | Out-File -FilePath $captureFile -Append -Encoding UTF8

# Function to run command and capture output cleanly
function Run-And-Capture {
    param($Command)
    
    # Use Invoke-Expression to run the command directly in PowerShell instead of cmd
    try {
        $ErrorActionPreference = "Continue"
        $output = Invoke-Expression $Command 2>&1 | Out-String
        $output | Out-File -FilePath $captureFile -Append -Encoding UTF8 -NoNewline
    }
    catch {
        $_.Exception.Message | Out-File -FilePath $captureFile -Append -Encoding UTF8
    }
}

# Run each command and capture output
Run-And-Capture "cargo fmt --all"
Run-And-Capture "cargo clippy --all-targets --all-features" 
Run-And-Capture "cargo test"
Run-And-Capture "cargo run --bin ddai_cli"
Run-And-Capture "cargo run -p ddai_cli -- init-db"
Run-And-Capture "cargo run -p ddai_cli -- ingest .\data\raw\sample_rules.md --source 'Sample Rules Pack' --title 'Sample Rules Pack (Test Content)' --license 'Internal test content' --attribution 'Created by the developer for ingestion tests'"
Run-And-Capture "cargo run -p ddai_cli -- list-docs"
Run-And-Capture "cargo run -p ddai_cli -- ingest-dnd5eapi --base-url https://www.dnd5eapi.co --limit 25 --source 'dnd5eapi.co (SRD mirror)'"
Run-And-Capture "cargo run -p ddai_cli -- list-docs"

# Add final prompt and end metadata
"PS $((Get-Location).Path)>" | Out-File -FilePath $captureFile -Append -Encoding UTF8

$endMetadata = @"

**********************
Windows PowerShell transcript end
End time: $(Get-Date -Format "yyyyMMddHHmmss")
**********************
"@

$endMetadata | Out-File -FilePath $captureFile -Append -Encoding UTF8

Write-Host "Output saved to: $captureFile" -ForegroundColor Green