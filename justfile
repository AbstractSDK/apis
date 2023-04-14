build:
  cargo build

# Test everything
test:
  cargo nextest run

format:
  cargo fmt --all

lint:
  cargo clippy --all --all-features -- -D warnings

lintfix:
  cargo clippy --fix --allow-staged --allow-dirty --all-features
  cargo fmt --all

check:
  cargo check --all-features

refresh:
  cargo clean && cargo update

check-codecov:
  cat codecov.yml | curl --data-binary @- https://codecov.io/validate

# Publish crates
publish:
  ./publish/publish.sh

watch:
  cargo watch -x lcheck

watch-test:
  cargo watch -x "nextest run"

wasm:
  ./publish/wasms.sh

wasm-module module:
  RUSTFLAGS='-C link-arg=-s' cargo wasm --package {{module}}

#wasm chain_name:
#  RUSTFLAGS='-C link-arg=-s' cargo ws exec --no-bail cargo wasm
#  if [[ {{chain}} == "terra" ]]; then RUSTFLAGS='-C link-arg=-s' cargo wasm --package dex --features terra --no-default-features; fi

run-script script chain:
  (cd scripts && cargo run --bin {{script}} -- --network-id {{chain}})

full-deploy chain:
  (cd scripts && cargo run --bin full_deploy -- --network-id {{chain}})

publish-schemas version:
  SCHEMA_OUT_DIR=$(cd ../schemas && echo "$PWD") \
  VERSION={{version}} \
    cargo ws exec --no-bail bash -lc 'cargo schema && { outdir="$SCHEMA_OUT_DIR/abstract/${PWD##*/}/$VERSION"; mkdir -p "$outdir"; rm -rf "schema/raw"; cp -a "schema/." "$outdir"; }'