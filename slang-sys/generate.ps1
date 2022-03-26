if ($null -eq (Get-Command "bindgen" -ErrorAction SilentlyContinue)) 
{ 
  Write-Host "Error: bindgen is not installed."
  Write-Host "Install bindgen with 'cargo install bindgen'"
  Exit
}

Remove-Item -Recurse -ErrorAction Ignore "$env:temp\source"
Remove-Item -Recurse -ErrorAction Ignore "$env:temp\source.zip"
$PSDefaultParameterValues['Out-File:Encoding'] = 'utf8'

$githubLatestReleases = "https://api.github.com/repos/shader-slang/slang/releases/latest"

Write-Host "Fetching release info..."
$githubLatestReleasesJson = ((Invoke-WebRequest $gitHubLatestReleases) | ConvertFrom-Json)
$githubLatestReleaseVersion = $githubLatestReleasesJson.tag_name
Write-Host "Found release: $githubLatestReleaseVersion"
$githubLatestReleaseVersion.substring(1) | Out-File -Encoding ASCII -NoNewline -FilePath "$PSScriptRoot\version.txt"

$githubLatestSourceURL = ($githubLatestReleasesJson.assets.browser_download_url | Select-String "-source.zip").ToString()

Write-Host "Downloading source..."
Invoke-WebRequest -Uri $githubLatestSourceURL -OutFile "$env:temp\source.zip"

Write-Host "Unzipping source..."
Expand-Archive -Path "$env:temp\source.zip" -DestinationPath "$env:temp\source"

Write-Host "Generating bindings..."
bindgen "$env:temp\source\slang.h" `
  --size_t-is-usize `
  --allowlist-function "sp[A-Z].*" `
  --allowlist-var "SLANG_[A-Z].*" `
  --allowlist-type "slang_.*" `
  --allowlist-type "I?Slang[A-Z].*" `
  -- -x c++ -std=c++20 > "$PSScriptRoot\src\bindings.rs"

Write-Host "Binding generated to $PSScriptRoot\src\bindings.rs"
Remove-Item -Recurse "$env:temp\source"
Remove-Item "$env:temp\source.zip"
