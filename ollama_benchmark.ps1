# Benchmark script for Ollama models with ddai_cli ask command
param(
    [string[]]$Models = @("qwen2.5:0.5b", "llama3.2:1b", "phi3.5:3.8b", "gemma2:2b"),
    [int]$Iterations = 10,
    [string]$Question = "What does advantage do?",
    [int]$K = 6
)

Write-Host "=== DDAI CLI Model Benchmark ===" -ForegroundColor Green
Write-Host "Question: '$Question'" -ForegroundColor Cyan
Write-Host "Iterations per model: $Iterations" -ForegroundColor Cyan
Write-Host "Models to test: $($Models -join ', ')" -ForegroundColor Cyan
Write-Host ""

# Pre-compile the binary to avoid timing compilation
Write-Host "Pre-compiling ddai_cli..." -ForegroundColor Yellow
cargo build -p ddai_cli --release | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to compile ddai_cli, exiting..." -ForegroundColor Red
    exit 1
}

# Use the release binary for more consistent timing
$binaryPath = ".\target\release\ddai_cli.exe"

Write-Host "Using compiled binary: $binaryPath" -ForegroundColor Green
Write-Host ""

$results = @()

foreach ($model in $Models) {
    Write-Host "Testing model: $model" -ForegroundColor Yellow
    
    # Check if model is available
    $available = ollama list | Select-String $model
    if (-not $available) {
        Write-Host "  Model $model not found, pulling..." -ForegroundColor Red
        ollama pull $model
        if ($LASTEXITCODE -ne 0) {
            Write-Host "  Failed to pull $model, skipping..." -ForegroundColor Red
            continue
        }
    }
    
    # Set environment variable for this model
    $env:OLLAMA_MODEL = $model
    
    $times = @()
    $successCount = 0
    
    for ($i = 1; $i -le $Iterations; $i++) {
        Write-Host "  Run $i/$Iterations..." -NoNewline
        
        # More precise timing - start right before the process starts
        $startTime = [System.Diagnostics.Stopwatch]::StartNew()
        
        try {
            # Run the compiled binary directly
            $processInfo = New-Object System.Diagnostics.ProcessStartInfo
            $processInfo.FileName = $binaryPath
            $processInfo.Arguments = "ask `"$Question`" --k $K"
            $processInfo.RedirectStandardOutput = $true
            $processInfo.RedirectStandardError = $true
            $processInfo.UseShellExecute = $false
            $processInfo.CreateNoWindow = $true
            
            # Copy current environment variables - fixed syntax
            Get-ChildItem env: | ForEach-Object {
                $processInfo.EnvironmentVariables[$_.Name] = $_.Value
            }
            
            $process = [System.Diagnostics.Process]::Start($processInfo)
            $output = $process.StandardOutput.ReadToEnd()
            $errorOutput = $process.StandardError.ReadToEnd()
            $process.WaitForExit()
            
            $startTime.Stop()
            $duration = $startTime.Elapsed.TotalSeconds
            
            # Check if it was successful
            if ($process.ExitCode -ne 0 -or $errorOutput -match "Error|error|failed") {
                Write-Host " FAILED ($($duration.ToString('F2'))s)" -ForegroundColor Red
                if ($errorOutput) {
                    Write-Host "    Error: $($errorOutput.Substring(0, [Math]::Min(100, $errorOutput.Length)))" -ForegroundColor Red
                }
            } else {
                $times += $duration
                $successCount++
                Write-Host " OK ($($duration.ToString('F2'))s)" -ForegroundColor Green
            }
        }
        catch {
            if ($startTime.IsRunning) { $startTime.Stop() }
            Write-Host " EXCEPTION ($($_.Exception.Message))" -ForegroundColor Red
        }
    }
    
    if ($successCount -gt 0) {
        $avgTime = ($times | Measure-Object -Average).Average
        $minTime = ($times | Measure-Object -Minimum).Minimum  
        $maxTime = ($times | Measure-Object -Maximum).Maximum
        
        $results += [PSCustomObject]@{
            Model = $model
            SuccessfulRuns = $successCount
            AverageTime = $avgTime
            MinTime = $minTime
            MaxTime = $maxTime
            TotalRuns = $Iterations
        }
        
        Write-Host "  Results: Avg=$($avgTime.ToString('F2'))s, Min=$($minTime.ToString('F2'))s, Max=$($maxTime.ToString('F2'))s ($successCount/$Iterations successful)" -ForegroundColor Cyan
    } else {
        Write-Host "  No successful runs for $model" -ForegroundColor Red
    }
    
    Write-Host ""
}

# Display final results
Write-Host "=== BENCHMARK RESULTS ===" -ForegroundColor Green
Write-Host ""

if ($results.Count -gt 0) {
    # Sort by average time
    $sortedResults = $results | Sort-Object AverageTime
    
    Write-Host "Results (sorted by speed):" -ForegroundColor Yellow
    Write-Host ""
    
    $formatString = "{0,-15} {1,4}/{2,-2} {3,8} {4,8} {5,8}"
    Write-Host ($formatString -f "Model", "OK", "Total", "Avg(s)", "Min(s)", "Max(s)") -ForegroundColor White
    Write-Host ("-" * 55) -ForegroundColor White
    
    foreach ($result in $sortedResults) {
        $color = if ($result.SuccessfulRuns -eq $result.TotalRuns) { "Green" } else { "Yellow" }
        Write-Host ($formatString -f 
            $result.Model,
            $result.SuccessfulRuns,
            $result.TotalRuns,
            $result.AverageTime.ToString('F2'),
            $result.MinTime.ToString('F2'),
            $result.MaxTime.ToString('F2')
        ) -ForegroundColor $color
    }
    
    Write-Host ""
    Write-Host "Fastest model: $($sortedResults[0].Model) ($($sortedResults[0].AverageTime.ToString('F2'))s average)" -ForegroundColor Green
    
    # Recommendations
    Write-Host ""
    Write-Host "Recommendations:" -ForegroundColor Yellow
    $fastest = $sortedResults[0]
    Write-Host "- For development/testing: $($fastest.Model)" -ForegroundColor Cyan
    
    if ($sortedResults.Count -gt 1) {
        $balanced = $sortedResults | Where-Object { $_.AverageTime -lt 10 -and $_.Model -notmatch "0\.5b" } | Select-Object -First 1
        if ($balanced) {
            Write-Host "- For better quality: $($balanced.Model)" -ForegroundColor Cyan
        }
    }
} else {
    Write-Host "No successful runs recorded!" -ForegroundColor Red
}

Write-Host ""
Write-Host "Benchmark complete!" -ForegroundColor Green

# Create benchmark results directory if it doesn't exist
if (!(Test-Path ".\benchmark results")) {
    New-Item -ItemType Directory -Path ".\benchmark results" -Force | Out-Null
}

# Save results to file
$timestamp = Get-Date -Format "yyyy-MM-dd_HHmm"
$resultFile = ".\benchmark results\benchmark_results_$timestamp.json"
$results | ConvertTo-Json -Depth 2 | Out-File -FilePath $resultFile -Encoding UTF8
Write-Host "Results saved to: $resultFile" -ForegroundColor Gray