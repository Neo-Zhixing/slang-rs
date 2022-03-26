#!/usr/bin/env bash

# Check that bindgen is installed.
if ! [ -x "$(command -v bindgen)" ]; then
  echo 'Error: bindgen is not installed.' >&2
  echo 'Install bindgen with "cargo install bindgen"' >&2
  exit 1
fi

echo "Fetching release info..."
github_release_json=$(curl -s https://api.github.com/repos/shader-slang/slang/releases/latest)
github_release_url=$(echo "$github_release_json" \
| grep "browser_download_url.*-source.zip" \
| cut -d '"' -f 4)
github_release_version=$(echo "$github_release_json" | grep 'tag_name' | cut -d '"' -f 4)
echo "Found release: $github_release_version"
echo -n "${github_release_version:1}" > version.txt

temp_dir=$(mktemp -d)

echo "Downloading source..."
curl --location -o "$temp_dir/source.zip" "$github_release_url"

echo "Unzipping source..."
unzip -q "$temp_dir/source.zip"  -d "$temp_dir/source"

echo "Generating bindings..."
bindgen "$temp_dir/source/slang.h" \
  --size_t-is-usize \
  --allowlist-function "sp[A-Z].*" \
  --allowlist-var "SLANG_[A-Z].*" \
  --allowlist-type "slang_.*" \
  --allowlist-type "I?Slang[A-Z].*" \
  -- -x c++ -std=c++20 > "$(dirname $0)/src/bindings.rs"

echo "Binding generated to $(dirname $0)/src/bindings.rs"
rm -rf "$temp_dir"
