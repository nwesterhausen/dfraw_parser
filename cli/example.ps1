$existingSize = (Get-Item example-vanilla-raws.json).length

cargo run -- -o example-vanilla-raws.json -P --vanilla "C:\Program Files (x86)\Steam\steamapps\common\Dwarf Fortress"
# cargo run -- -c -r "../lib/tests/data/creature_amphibians.txt" -r "../lib/tests/data/c_variation_default.txt" -o creature_amphibians.json -P

function ConvertTo-HumanReadable {
    param(
        [Parameter(Mandatory=$true)]
        [int64]$Bytes
    )

    if ($Bytes -ge 1PB) { [string]::Format("{0:0.00} PB", $Bytes / 1PB) }
    elseif ($Bytes -ge 1TB) { [string]::Format("{0:0.00} TB", $Bytes / 1TB) }
    elseif ($Bytes -ge 1GB) { [string]::Format("{0:0.00} GB", $Bytes / 1GB) }
    elseif ($Bytes -ge 1MB) { [string]::Format("{0:0.00} MB", $Bytes / 1MB) }
    elseif ($Bytes -ge 1KB) { [string]::Format("{0:0.00} KB", $Bytes / 1KB) }
    else { [string]::Format("{0:0.00} B", $Bytes) }
}

# Print the prior size of example-vanilla-raws.json (powershell)
ConvertTo-HumanReadable -Bytes $existingSize

# Print the human-readable size of example-vanilla-raws.json (powershell)
$size = (Get-Item example-vanilla-raws.json).length
ConvertTo-HumanReadable -Bytes $size
