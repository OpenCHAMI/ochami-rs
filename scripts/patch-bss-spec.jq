# Patches applied to swagger2openapi output before progenitor consumes it.
# Each patch has an explanation. Periodically review whether upstream fixed
# the underlying issue and drop the patch.

# ----------------------------------------------------------------------------
# Patch 1: Add operationIds to endpoints that lack them.
#
# Progenitor 0.8 refuses to codegen operations without operationId, failing
# with UnexpectedFormat("path <path> is missing operation ID"). The upstream
# BSS spec declares operationIds only on 4 of 18 operations (all the cloud-init
# and bootscript ones). Every other operation gets a synthesized snake_case
# id matching the existing style (`<resource>_<verb>[_<suffix>]`). No
# in-crate caller currently consumes most of these; adding operationIds
# preserves them in the generated Client for future wrappers rather than
# stripping them from the spec.
# TODO: file upstream PR to add these operationIds at source.
# ----------------------------------------------------------------------------
  .paths["/boot/v1/bootparameters"].get.operationId    = "bootparameters_get"
| .paths["/boot/v1/bootparameters"].post.operationId   = "bootparameters_post"
| .paths["/boot/v1/bootparameters"].put.operationId    = "bootparameters_put"
| .paths["/boot/v1/bootparameters"].patch.operationId  = "bootparameters_patch"
| .paths["/boot/v1/bootparameters"].delete.operationId = "bootparameters_delete"
| .paths["/boot/v1/hosts"].get.operationId             = "hosts_get"
| .paths["/boot/v1/hosts"].post.operationId            = "hosts_post"
| .paths["/boot/v1/dumpstate"].get.operationId         = "dumpstate_get"
| .paths["/boot/v1/endpoint-history"].get.operationId  = "endpoint_history_get"
| .paths["/boot/v1/service/status"].get.operationId    = "service_status_get"
| .paths["/boot/v1/service/storage/status"].get.operationId = "service_storage_status_get"
| .paths["/boot/v1/service/hsm"].get.operationId       = "service_hsm_get"
| .paths["/boot/v1/service/version"].get.operationId   = "service_version_get"
| .paths["/boot/v1/service/status/all"].get.operationId = "service_status_all_get"

# ----------------------------------------------------------------------------
# Patch 2: Strip non-2xx responses
#
# Progenitor 0.8's success-extractor groups 2xx and `default` responses and
# asserts (method.rs:1264) that the resulting set of response types has
# exactly one element. BSS's spec attaches an error schema to nearly every
# non-2xx response, so the set has >1 element on almost every operation and
# codegen panics.
#
# We keep only 2xx response entries. Downstream callers handle non-2xx as
# `Error::UnexpectedResponse(reqwest::Response)` and read the body manually
# if they need typed error details. Same fix pattern as HSM Task 0 patch 4.
# TODO: file upstream PR / consider selective per-op typed-error reintroduction.
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
