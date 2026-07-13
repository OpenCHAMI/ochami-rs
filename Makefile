.PHONY: convert-spec

convert-spec:
	npx --yes swagger2openapi@7.0.8 src/hsm/ochami_api_docs.yaml -o /tmp/hsm-raw.json
	jq -f scripts/patch-hsm-spec.jq /tmp/hsm-raw.json > src/hsm/ochami_api_docs.openapi3.json
	rm /tmp/hsm-raw.json
