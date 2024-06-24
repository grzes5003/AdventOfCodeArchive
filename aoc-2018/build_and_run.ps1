$buildDir = ".\build"
$executableName = "\src\Debug\aoc2018.exe"

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

$executablePath = Join-Path $buildDir $executableName

if (-Not (Test-Path $executablePath)) {
    Write-Error "Executable not found: $executablePath"
    exit 1
}

& $executablePath

# Check if the execution was successful
if ($LASTEXITCODE -ne 0) {
    Write-Error "Execution of the program failed."
    exit $LASTEXITCODE
}

Write-Output "Program executed successfully."
