
$buildDir = ".\build"

if (-Not (Test-Path $buildDir)) {
    New-Item -ItemType Directory -Path $buildDir
}

cmake -S . -B $buildDir

if ($LASTEXITCODE -ne 0) {
    Write-Error "CMake configuration failed."
    exit $LASTEXITCODE
}

cmake --build $buildDir

if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed."
    exit $LASTEXITCODE
}

Write-Output "Build completed successfully."
