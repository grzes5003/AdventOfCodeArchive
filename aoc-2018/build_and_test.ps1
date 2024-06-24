$buildDir = ".\build"
$executableName = "Debug\runUnitTests.exe"

if (-Not (Test-Path $buildDir)) {
    New-Item -ItemType Directory -Path $buildDir
}

cmake -Dtest=ON -S . -B $buildDir

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

# & ctest
& .\build\test\Debug\runTests.exe

# Check if the execution was successful
if ($LASTEXITCODE -ne 0) {
    Write-Error "Execution of the test failed."
    exit $LASTEXITCODE
}

Write-Output "Tests executed successfully."
