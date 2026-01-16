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
    
    # Print the command being executed
    "PS $((Get-Location).Path)> $Command" | Out-File -FilePath $captureFile -Append -Encoding UTF8
    Write-Host "Running: $Command" -ForegroundColor Cyan
    
    # Use Invoke-Expression to run the command directly in PowerShell instead of cmd
    try {
        $ErrorActionPreference = "Continue"
        $output = Invoke-Expression $Command 2>&1 | Out-String
        $output | Out-File -FilePath $captureFile -Append -Encoding UTF8 -NoNewline
    }
    catch {
        $_.Exception.Message | Out-File -FilePath $captureFile -Append -Encoding UTF8
    }
    
    # Add a blank line for separation
    "" | Out-File -FilePath $captureFile -Append -Encoding UTF8
}

# Run each command and capture output
Run-And-Capture "cargo fmt --all"
Run-And-Capture "cargo clippy --all-targets --all-features" 
Run-And-Capture "cargo check -p ddai_llm"
Run-And-Capture "cargo check -p ddai_cli"
Run-And-Capture "cargo build"
Run-And-Capture "cargo test"
Run-And-Capture "cargo run --bin ddai_cli"
Run-And-Capture "cargo run -p ddai_cli -- init-db"
Run-And-Capture "cargo run -p ddai_cli -- ingest .\data\raw\sample_rules.md --source 'Sample Rules Pack' --title 'Sample Rules Pack (Test Content)' --license 'Internal test content' --attribution 'Created by the developer for ingestion tests'"
Run-And-Capture "cargo run -p ddai_cli -- list-docs"
Run-And-Capture "cargo run -p ddai_cli -- ingest-dnd5eapi --base-url https://www.dnd5eapi.co --source 'dnd5eapi.co (SRD mirror)'"
Run-And-Capture "cargo run -p ddai_cli -- list-docs"
Run-And-Capture "cargo run -p ddai_cli -- list-entities --kind monsters --limit 10"
Run-And-Capture "cargo run -p ddai_cli -- list-entities --kind spells --limit 10"

# Test search functionality
Run-And-Capture "cargo run -p ddai_cli -- search 'advantage' --k 5"
Run-And-Capture "cargo run -p ddai_cli -- search 'goblin' --k 5"
Run-And-Capture "cargo run -p ddai_cli -- search 'bronze dragon' --k 5"
Run-And-Capture "cargo run -p ddai_cli -- search 'dragon armor class' --k 5"

# Test chunk inspection
Run-And-Capture "cargo run -p ddai_cli -- show-chunk 1"

# Test AI-powered ask functionality - these work well
Run-And-Capture "cargo run -p ddai_cli -- ask 'What does advantage do?' --k 6"
Run-And-Capture "cargo run -p ddai_cli -- ask 'What is Armor Class for a goblin?' --k 8"
Run-And-Capture "cargo run -p ddai_cli -- ask 'What is the AC of a Bronze Dragon?' --k 8"
Run-And-Capture "cargo run -p ddai_cli -- ask 'What is the AC of an Adult Bronze Dragon?' --k 8"

# Examples that show the importance of specific queries
# Run-And-Capture "cargo run -p ddai_cli -- ask 'What is the Armor Class of a dragon?' --k 8"  # Too generic - finds spell data
# Run-And-Capture "cargo run -p ddai_cli -- ask 'What are the stats for dragons?' --k 8"        # Too vague - no good matches

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