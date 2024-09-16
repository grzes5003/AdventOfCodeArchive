$buildDir = ".\bazel-bin\src"
$executableName = "\aoc-2018.exe"

bazel build //src:aoc-2018

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
