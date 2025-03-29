[private]
help:
    @just --list

version_file := source_directory() / "node_modules" / ".new_version"

publish version="$(read -p 'New version: ' v; echo $v)":
    grep '"version"' package.json
    @echo {{version}} > {{version_file}}
    sed -i 's/"version": "[^"]*"/"version": "'"$(cat {{version_file}})"'"/' package.json
    grep '"version"' package.json
    yarn run package
    git add package.json
    git commit -m "Bump version to $(cat {{version_file}})"
    git tag v$(cat {{version_file}})" -m "v$(cat {{version_file}})"
    yarn npm publish
    git push
    git push --tags
    sed -i~ '/"registry":/d' package.json
    yarn npm publish
    mv package.json~ package.json
