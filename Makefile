.PHONY: convert-spec convert-spec-hsm convert-spec-bss convert-spec-pcs

# Regenerate every committed OpenAPI 3.0 spec from its Swagger 2.0 source.
convert-spec: convert-spec-hsm convert-spec-bss convert-spec-pcs

convert-spec-hsm:
	npx --yes swagger2openapi@7.0.8 src/hsm/ochami_api_docs.yaml -o /tmp/hsm-raw.json
	jq -f scripts/patch-hsm-spec.jq /tmp/hsm-raw.json > src/hsm/ochami_api_docs.openapi3.json
	rm /tmp/hsm-raw.json

convert-spec-bss:
	npx --yes swagger2openapi@7.0.8 src/bss/ochami_api_docs.yaml -o /tmp/bss-raw.json
	jq -f scripts/patch-bss-spec.jq /tmp/bss-raw.json > src/bss/ochami_api_docs.openapi3.json
	rm /tmp/bss-raw.json

# PCS's source YAML is already OpenAPI 3.0; swagger2openapi handles it as a
# pass-through (also normalising YAML → JSON), so we reuse the same tool.
convert-spec-pcs:
	npx --yes swagger2openapi@7.0.8 src/pcs/ochami_api_docs.yaml -o /tmp/pcs-raw.json
	jq -f scripts/patch-pcs-spec.jq /tmp/pcs-raw.json > src/pcs/ochami_api_docs.openapi3.json
	rm /tmp/pcs-raw.json
