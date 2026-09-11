[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$OutputPath,
    [string]$SourceRoot,
    [string]$DestinationRoot
)

$ErrorActionPreference = 'Stop'
$VerifierVersion = 'HVA-LWC-001-V03-PRESERVATION-VERIFIER-1.0'
$ReceiptSchema = 'HVA-LWC-001-V03-PRESERVATION-RECEIPT-1'

if ([string]::IsNullOrWhiteSpace($SourceRoot)) {
    $SourceRoot = Join-Path $env:LOCALAPPDATA 'Temp'
}
if ([string]::IsNullOrWhiteSpace($DestinationRoot)) {
    $documents = [Environment]::GetFolderPath('MyDocuments')
    $DestinationRoot = Join-Path $documents 'H!veAI-Preservation\HVA-LWC-001-V02'
}

function Get-Manifest {
    param([Parameter(Mandatory = $true)][string]$Root)

    $rootItem = Get-Item -LiteralPath $Root -Force -ErrorAction Stop
    if (-not $rootItem.PSIsContainer) {
        throw 'ROOT_NOT_DIRECTORY'
    }
    if (($rootItem.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
        throw 'ROOT_REPARSE_POINT'
    }

    $records = New-Object 'System.Collections.Generic.List[string]'
    $stack = New-Object 'System.Collections.Generic.Stack[string]'
    $stack.Push($rootItem.FullName)
    $fileCount = [int64]0
    $totalBytes = [int64]0

    while ($stack.Count -gt 0) {
        $directory = $stack.Pop()
        $entries = [IO.Directory]::EnumerateFileSystemEntries($directory)
        foreach ($entryPath in $entries) {
            $entry = Get-Item -LiteralPath $entryPath -Force -ErrorAction Stop
            if (($entry.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
                throw 'REPARSE_POINT_NOT_FOLLOWED'
            }
            if ($entry.PSIsContainer) {
                $stack.Push($entry.FullName)
                continue
            }

            $relative = $entry.FullName.Substring($rootItem.FullName.Length).TrimStart('\', '/')
            $relative = $relative.Replace('\', '/')
            $hash = (Get-FileHash -LiteralPath $entry.FullName -Algorithm SHA256 -ErrorAction Stop).Hash.ToLowerInvariant()
            [void]$records.Add(('{0}`t{1}`t{2}' -f $relative, [int64]$entry.Length, $hash))
            $fileCount++
            $totalBytes += [int64]$entry.Length
        }
    }

    $records.Sort([System.StringComparer]::Ordinal)
    $manifestText = [String]::Join("`n", $records)
    $manifestBytes = [Text.Encoding]::UTF8.GetBytes($manifestText)
    $digestBytes = [Security.Cryptography.SHA256]::Create().ComputeHash($manifestBytes)
    $digest = ([BitConverter]::ToString($digestBytes)).Replace('-', '').ToLowerInvariant()

    return [pscustomobject]@{
        FileCount = $fileCount
        TotalBytes = $totalBytes
        ManifestSha256 = $digest
    }
}

function New-PairResult {
    param(
        [Parameter(Mandatory = $true)][hashtable]$Definition,
        [Parameter(Mandatory = $true)][string]$SourceBase,
        [Parameter(Mandatory = $true)][string]$DestinationBase
    )

    $sourcePath = Join-Path $SourceBase $Definition.SourceRelative
    $destinationPath = Join-Path $DestinationBase $Definition.DestinationRelative
    $result = [ordered]@{
        sourceLabel = $Definition.SourceLabel
        destinationLabel = $Definition.DestinationLabel
        sourceFileCount = $null
        destinationFileCount = $null
        sourceTotalBytes = $null
        destinationTotalBytes = $null
        sourceManifestSha256 = $null
        destinationManifestSha256 = $null
        countMatch = $false
        bytesMatch = $false
        manifestMatch = $false
        finalResult = 'BLOCKED'
    }

    if (-not (Test-Path -LiteralPath $sourcePath -PathType Container)) {
        $result.finalResult = 'BLOCKED_SOURCE_MISSING'
        return [pscustomobject]$result
    }
    if (-not (Test-Path -LiteralPath $destinationPath -PathType Container)) {
        $result.finalResult = 'BLOCKED_DESTINATION_MISSING'
        return [pscustomobject]$result
    }

    try {
        $sourceManifest = Get-Manifest -Root $sourcePath
        $destinationManifest = Get-Manifest -Root $destinationPath
        $result.sourceFileCount = $sourceManifest.FileCount
        $result.destinationFileCount = $destinationManifest.FileCount
        $result.sourceTotalBytes = $sourceManifest.TotalBytes
        $result.destinationTotalBytes = $destinationManifest.TotalBytes
        $result.sourceManifestSha256 = $sourceManifest.ManifestSha256
        $result.destinationManifestSha256 = $destinationManifest.ManifestSha256
        $result.countMatch = $sourceManifest.FileCount -eq $destinationManifest.FileCount
        $result.bytesMatch = $sourceManifest.TotalBytes -eq $destinationManifest.TotalBytes
        $result.manifestMatch = $sourceManifest.ManifestSha256 -eq $destinationManifest.ManifestSha256
        if ($result.countMatch -and $result.bytesMatch -and $result.manifestMatch) {
            if ($Definition.AllowEmpty -and $sourceManifest.FileCount -eq 0 -and $destinationManifest.FileCount -eq 0) {
                $result.finalResult = 'EMPTY'
            } else {
                $result.finalResult = 'PASS'
            }
        } else {
            $result.finalResult = 'FAIL'
        }
    } catch {
        $result.finalResult = 'BLOCKED_UNSAFE_OR_INACCESSIBLE'
    }

    return [pscustomobject]$result
}

$definitions = @(
    @{
        SourceLabel = 'HVA-LWC-001-V01-parent-preservation'
        DestinationLabel = 'HVA-LWC-001-V02-parent-preservation'
        SourceRelative = 'HVA-LWC-001-V01-parent-preservation'
        DestinationRelative = 'parent-preservation'
        AllowEmpty = $false
    },
    @{
        SourceLabel = 'H-veAI-consolidation-retired-20260911'
        DestinationLabel = 'HVA-LWC-001-V02-retired-H-veAI-20260911'
        SourceRelative = 'H-veAI-consolidation-retired-20260911'
        DestinationRelative = 'retired-H-veAI-20260911'
        AllowEmpty = $false
    },
    @{
        SourceLabel = 'HVA-LWC-001-V01-deduplicated'
        DestinationLabel = 'HVA-LWC-001-V02-deduplicated'
        SourceRelative = 'HVA-LWC-001-V01-deduplicated'
        DestinationRelative = 'deduplicated'
        AllowEmpty = $false
    },
    @{
        SourceLabel = 'HVA-LWC-001-V01-retired-loose-H!veAI'
        DestinationLabel = 'HVA-LWC-001-V02-retired-loose-H!veAI'
        SourceRelative = 'HVA-LWC-001-V01-retired-loose-H!veAI'
        DestinationRelative = 'retired-loose-H!veAI'
        AllowEmpty = $true
    }
)

$pairs = @($definitions | ForEach-Object {
    New-PairResult -Definition $_ -SourceBase $SourceRoot -DestinationBase $DestinationRoot
})
$requiredPairs = @($pairs | Where-Object { $_.sourceLabel -ne 'HVA-LWC-001-V01-retired-loose-H!veAI' })
$overallResult = if (@($requiredPairs | Where-Object { $_.finalResult -ne 'PASS' }).Count -eq 0) { 'PASS' } else { 'PRESERVATION_BLOCKED' }

$receipt = [ordered]@{
    verifierVersion = $VerifierVersion
    schema = $ReceiptSchema
    verificationTimestampUtc = [DateTime]::UtcNow.ToString('o')
    reparsePointsFollowed = $false
    privacyMode = 'logical-labels-only'
    overallResult = $overallResult
    pairs = $pairs
}

$outputParent = Split-Path -Parent $OutputPath
if (-not [string]::IsNullOrWhiteSpace($outputParent)) {
    New-Item -ItemType Directory -Force -Path $outputParent | Out-Null
}
$utf8NoBom = New-Object Text.UTF8Encoding($false)
$outputFullPath = [IO.Path]::GetFullPath($OutputPath)
[IO.File]::WriteAllText($outputFullPath, ($receipt | ConvertTo-Json -Depth 6), $utf8NoBom)

foreach ($pair in $pairs) {
    Write-Output ('{0}: {1} files/{2} bytes -> {3} files/{4} bytes [{5}]' -f $pair.sourceLabel, $pair.sourceFileCount, $pair.sourceTotalBytes, $pair.destinationFileCount, $pair.destinationTotalBytes, $pair.finalResult)
}
Write-Output "OVERALL_RESULT=$overallResult"
if ($overallResult -ne 'PASS') {
    exit 2
}
exit 0
