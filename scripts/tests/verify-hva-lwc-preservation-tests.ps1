[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$verifier = Join-Path $PSScriptRoot '..\verify-hva-lwc-preservation.ps1'
$fixture = Join-Path ([IO.Path]::GetTempPath()) ('hva-lwc-v03-' + [Guid]::NewGuid().ToString('N'))
$sourceBase = Join-Path $fixture 'source'
$destinationBase = Join-Path $fixture 'destination'
$receipt = Join-Path $fixture 'receipt.json'
$sourceClass = Join-Path $sourceBase 'HVA-LWC-001-V01-parent-preservation'
$destinationClass = Join-Path $destinationBase 'parent-preservation'

function Assert-True {
    param([bool]$Condition, [string]$Message)
    if (-not $Condition) { throw "ASSERTION_FAILED: $Message" }
}

function Invoke-Receipt {
    param([string]$Path)
    & $verifier -OutputPath $Path -SourceRoot $sourceBase -DestinationRoot $destinationBase | Out-Null
    return $LASTEXITCODE
}

try {
    New-Item -ItemType Directory -Force -Path (Join-Path $sourceClass 'nested') | Out-Null
    New-Item -ItemType Directory -Force -Path $destinationClass | Out-Null
    Set-Content -LiteralPath (Join-Path $sourceClass 'alpha.txt') -Value 'alpha' -NoNewline
    Set-Content -LiteralPath (Join-Path $sourceClass 'nested\beta.txt') -Value 'beta' -NoNewline
    Copy-Item -Path (Join-Path $sourceClass '*') -Destination $destinationClass -Recurse -Force
    foreach ($pair in @(
        @('H-veAI-consolidation-retired-20260911', 'retired-H-veAI-20260911'),
        @('HVA-LWC-001-V01-deduplicated', 'deduplicated')
    )) {
        $additionalSource = Join-Path $sourceBase $pair[0]
        $additionalDestination = Join-Path $destinationBase $pair[1]
        New-Item -ItemType Directory -Force -Path $additionalSource, $additionalDestination | Out-Null
        Copy-Item -Path (Join-Path $sourceClass '*') -Destination $additionalSource -Recurse -Force
        Copy-Item -Path (Join-Path $sourceClass '*') -Destination $additionalDestination -Recurse -Force
    }
    New-Item -ItemType Directory -Force -Path (Join-Path $sourceBase 'HVA-LWC-001-V01-retired-loose-H!veAI'), (Join-Path $destinationBase 'retired-loose-H!veAI') | Out-Null

    $identicalExit = Invoke-Receipt -Path $receipt
    $identical = Get-Content -Raw $receipt | ConvertFrom-Json
    Assert-True ($identicalExit -eq 0 -and $identical.overallResult -eq 'PASS') 'identical trees must PASS'
    $privacyText = Get-Content -Raw $receipt
    Assert-True (-not $privacyText.Contains($fixture)) 'receipt must not contain fixture paths'

    Set-Content -LiteralPath (Join-Path $destinationClass 'alpha.txt') -Value 'bravo' -NoNewline
    $contentExit = Invoke-Receipt -Path (Join-Path $fixture 'content-fail.json')
    $contentReceipt = Get-Content -Raw (Join-Path $fixture 'content-fail.json') | ConvertFrom-Json
    Assert-True ($contentExit -ne 0 -and $contentReceipt.overallResult -eq 'PRESERVATION_BLOCKED') 'same-size content change must FAIL'

    Copy-Item -LiteralPath (Join-Path $sourceClass 'alpha.txt') -Destination (Join-Path $destinationClass 'alpha.txt') -Force
    Remove-Item -LiteralPath (Join-Path $destinationClass 'nested\beta.txt') -Force
    $missingExit = Invoke-Receipt -Path (Join-Path $fixture 'missing-fail.json')
    Assert-True ($missingExit -ne 0) 'missing file must FAIL'

    Copy-Item -LiteralPath (Join-Path $sourceClass 'nested\beta.txt') -Destination (Join-Path $destinationClass 'nested\beta.txt') -Force
    Set-Content -LiteralPath (Join-Path $destinationClass 'extra.txt') -Value 'extra' -NoNewline
    $extraExit = Invoke-Receipt -Path (Join-Path $fixture 'extra-fail.json')
    Assert-True ($extraExit -ne 0) 'extra file must FAIL'

    Remove-Item -LiteralPath (Join-Path $destinationClass 'extra.txt') -Force
    $orderExit = Invoke-Receipt -Path (Join-Path $fixture 'order-pass.json')
    $orderReceipt = Get-Content -Raw (Join-Path $fixture 'order-pass.json') | ConvertFrom-Json
    Assert-True ($orderExit -eq 0 -and $orderReceipt.overallResult -eq 'PASS') 'deterministic ordering must PASS'

    $outside = Join-Path $fixture 'outside.txt'
    Set-Content -LiteralPath $outside -Value 'outside' -NoNewline
    $link = Join-Path $sourceClass 'reparse-link.txt'
    $reparseCreated = $false
    try {
        New-Item -ItemType SymbolicLink -Path $link -Target $outside -ErrorAction Stop | Out-Null
        $reparseCreated = $true
    } catch {
        $reparseDirectory = Join-Path $sourceClass 'reparse-directory'
        $outsideDirectory = Join-Path $fixture 'outside-directory'
        New-Item -ItemType Directory -Force -Path $outsideDirectory | Out-Null
        Set-Content -LiteralPath (Join-Path $outsideDirectory 'escaped.txt') -Value 'outside-directory' -NoNewline
        cmd /c "mklink /J `"$reparseDirectory`" `"$outsideDirectory`"" | Out-Null
        $reparseCreated = $LASTEXITCODE -eq 0
    }
    if ($reparseCreated) {
        $reparseExit = Invoke-Receipt -Path (Join-Path $fixture 'reparse-fail.json')
        Assert-True ($reparseExit -ne 0) 'reparse point must not be followed'
    } else {
        Write-Output 'REPARSE_TEST=SKIPPED_NO_SYMLINK_PRIVILEGE'
    }

    Write-Output 'VERIFIER_TESTS=PASS'
    exit 0
} finally {
    if (Test-Path -LiteralPath $fixture) {
        Remove-Item -LiteralPath $fixture -Recurse -Force
    }
}
