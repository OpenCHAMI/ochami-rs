.PHONY: convert-spec convert-spec-hsm convert-spec-bss

# Regenerate every committed OpenAPI 3.0 spec from its Swagger 2.0 source.
convert-spec: convert-spec-hsm convert-spec-bss

convert-spec-hsm:
	npx --yes swagger2openapi@7.0.8 src/hsm/ochami_api_docs.yaml -o /tmp/hsm-raw.json
	jq -f scripts/patch-hsm-spec.jq /tmp/hsm-raw.json > src/hsm/ochami_api_docs.openapi3.json
	rm /tmp/hsm-raw.json

convert-spec-bss:
	npx --yes swagger2openapi@7.0.8 src/bss/ochami_api_docs.yaml -o /tmp/bss-raw.json
	jq -f scripts/patch-bss-spec.jq /tmp/bss-raw.json > src/bss/ochami_api_docs.openapi3.json
	rm /tmp/bss-raw.json
