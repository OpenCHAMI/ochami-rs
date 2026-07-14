# Patches applied to the PCS spec before progenitor consumes it.
# Each patch has an explanation. Periodically review whether upstream fixed
# the underlying issue and drop the patch.

# ----------------------------------------------------------------------------
# Patch 1: Add operationIds to every endpoint.
#
# Progenitor 0.8 refuses to codegen operations without operationId, failing
# with UnexpectedFormat("path <path> is missing operation ID"). The upstream
# PCS spec declares no operationIds at all (all 13 operations lack one).
# Synthesised snake_case ids follow the pattern <resource>_<action>[_<suffix>]
# — same style used by HSM and BSS patches.
# TODO: file upstream PR to add these operationIds at source.
# ----------------------------------------------------------------------------
  .paths["/transitions"].post.operationId                     = "transitions_post"
| .paths["/transitions"].get.operationId                      = "transitions_get"
| .paths["/transitions/{transitionID}"].get.operationId       = "transition_get"
| .paths["/transitions/{transitionID}"].delete.operationId    = "transition_delete"
| .paths["/power-status"].get.operationId                     = "power_status_get"
| .paths["/power-status"].post.operationId                    = "power_status_post"
| .paths["/power-cap/snapshot"].post.operationId              = "power_cap_snapshot_post"
| .paths["/power-cap"].patch.operationId                      = "power_cap_patch"
| .paths["/power-cap"].get.operationId                        = "power_cap_get"
| .paths["/power-cap/{taskID}"].get.operationId               = "power_cap_task_get"
| .paths["/liveness"].get.operationId                         = "liveness_get"
| .paths["/readiness"].get.operationId                        = "readiness_get"
| .paths["/health"].get.operationId                           = "health_get"

# ----------------------------------------------------------------------------
# Patch 2: Rewrite `non_empty_string_list` to a plain string.
#
# The upstream spec declares `non_empty_string_list` as
# `{type: array, items: {type: string}, minItems: 1}` and uses it as a
# query parameter on `GET /power-status`. Progenitor 0.8 emits it as a
# newtype wrapping `Vec<String>` and then calls `.to_string()` on it
# when building the query string, which fails to compile because
# `Vec<String>: !Display`.
#
# The `GET /power-status` endpoint isn't used by any in-crate consumer
# (only `POST /power-status` is), so downgrading the schema to a plain
# `type: string` avoids the codegen breakage. Callers who need the
# multi-value semantics of the array can use POST (which takes a JSON
# body). TODO: file upstream typify PR to give `Vec`-newtypes a
# `Display` impl, or upstream OpenCHAMI PR to drop this endpoint.
# ----------------------------------------------------------------------------
| .components.schemas.non_empty_string_list = {
    "type": "string",
    "description": "downgraded from array to string in ochami-rs patch to work around a progenitor codegen limitation; see scripts/patch-pcs-spec.jq"
  }

# ----------------------------------------------------------------------------
# Patch 3: Strip non-2xx responses.
#
# Same shape as HSM patch 4 and BSS patch 2. PCS attaches typed error
# schemas to nearly every non-2xx response; progenitor 0.8's
# method.rs:1264 assertion (`response_types.len() <= 1`) trips on the
# resulting set. We keep only 2xx entries; callers handle non-2xx via
# `Error::UnexpectedResponse(reqwest::Response)` (map_err routes those
# into `Error::OchamiError { status, body }`).
# TODO: file upstream PR / consider per-op typed-error reintroduction.
# ----------------------------------------------------------------------------
| (
    .paths |= (with_entries(
      .value |= (with_entries(
        if (.value | type) == "object" and (.value | has("responses"))
        then .value.responses |= with_entries(
          select(.key | test("^2[0-9][0-9]$"))
        )
        else .
        end
      ))
    ))
  )
