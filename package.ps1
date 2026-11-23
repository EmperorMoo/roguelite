$Version = "v0.1.0"
$ReleaseDir = "dist\Roguelite-$Version-windows"
$ZipPath = "dist\Roguelite-$Version-windows.zip"

# 1. Build the Windows release executable
cargo build --release
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

# 2. Clean old build folder and create fresh structure
if (Test-Path "dist") { Remove-Item -Recurse -Force "dist" }
New-Item -ItemType Directory -Force -Path $ReleaseDir | Out-Null

# 3. Copy the .exe and assets folder side-by-side
Copy-Item "target\release\Roguelite.exe" -Destination $ReleaseDir
Copy-Item -Recurse "assets" -Destination "$ReleaseDir\assets"

# 4. Zip it up for GitHub Releases
Compress-Archive -Path $ReleaseDir -DestinationPath $ZipPath

Write-Host "Done! Upload $ZipPath to GitHub Releases."
