# Patches applied to swagger2openapi output before progenitor consumes it.
# Each patch has an explanation. Periodically review whether upstream fixed
# the underlying issue and drop the patch.

# ----------------------------------------------------------------------------
# Patch 1: Widen ResourceName pattern
#
# Widen ResourceName pattern to accept real HSM labels ("compute", "worker" —
# spec's ^[0-9a-f_\-.] rejects letters beyond 'f'). Server-side may still
# enforce narrower; this patch fixes client-side deserialization.
# TODO: file upstream PR to fix at source.
# ----------------------------------------------------------------------------
.components.schemas.ResourceName.pattern = "^[0-9a-zA-Z._-]{1,}$"

# ----------------------------------------------------------------------------
# Patch 2: Add operationIds to /locks/* endpoints
#
# Progenitor 0.8 refuses to codegen operations without operationId, failing
# with UnexpectedFormat("path <path> is missing operation ID"). The upstream
# HSM spec omits operationId on the 13 /locks/* operations (all other
# operations in the spec declare theirs). Follows csm-rs BSS Task 0 precedent
# (F.1 patch in 2026-06-13-progenitor-bss-output-reference.md).
#
# Names use snake_case <verb>_locks_<suffix>. No in-crate caller currently
# consumes any /locks/* endpoint; adding operationIds preserves them in the
# generated Client for future wrappers rather than dropping them from the
# spec. TODO: file upstream PR to add these operationIds at source.
# ----------------------------------------------------------------------------
| .paths["/locks/reservations/remove"].post.operationId = "postLocksReservationsRemove"
| .paths["/locks/reservations/release"].post.operationId = "postLocksReservationsRelease"
| .paths["/locks/reservations"].post.operationId = "postLocksReservations"
| .paths["/locks/service/reservations/release"].post.operationId = "postLocksServiceReservationsRelease"
| .paths["/locks/service/reservations"].post.operationId = "postLocksServiceReservations"
| .paths["/locks/service/reservations/renew"].post.operationId = "postLocksServiceReservationsRenew"
| .paths["/locks/service/reservations/check"].post.operationId = "postLocksServiceReservationsCheck"
| .paths["/locks/status"].post.operationId = "postLocksStatus"
| .paths["/locks/status"].get.operationId = "getLocksStatus"
| .paths["/locks/lock"].post.operationId = "postLocksLock"
| .paths["/locks/unlock"].post.operationId = "postLocksUnlock"
| .paths["/locks/repair"].post.operationId = "postLocksRepair"
| .paths["/locks/disable"].post.operationId = "postLocksDisable"

# ----------------------------------------------------------------------------
# Patch 3: Drop duplicate application/problem+json response content types
#
# Every response in the spec that declares application/problem+json ALSO
# declares application/json with the exact same schema (grep-verified). The
# duplication trips progenitor 0.8's method.rs:1264 assertion
# (`response_types.len() <= 1`) because progenitor treats the two media
# types as distinct branches even when their schemas match, and the pattern
# repeats across ~120 operations. Dropping the problem+json entry leaves
# application/json (same schema) as the sole response content type, which
# progenitor handles cleanly. Real HSM servers may respond with the
# problem+json content type header; the generated client will then take the
# UnexpectedResponse branch on those responses. Wrappers can decode the
# body as Problem7807 explicitly in map_err if needed.
# TODO: file upstream PR to drop the duplicate at source.
# ----------------------------------------------------------------------------
| (
    .paths |= (with_entries(
      .value |= (with_entries(
        if (.value | type) == "object" and (.value | has("responses"))
        then .value.responses |= with_entries(
          if (.value | type) == "object" and (.value | has("content"))
          then .value.content |= del(."application/problem+json")
          else .
          end
        )
        else .
        end
      ))
    ))
  )

# ----------------------------------------------------------------------------
# Patch 4: Strip non-2xx and `default` responses
#
# Progenitor 0.8's success-extractor groups 2xx and `default` responses and
# asserts (method.rs:1264) that the resulting set of response types has
# exactly one element. The spec uses `Problem7807` for `default`/4xx/5xx and
# the resource schema for 2xx, so the set has >1 element on nearly every
# operation and codegen panics. csm-rs HSM Task 0 identified the same
# pattern (three-patch remediation in
# csm-rs/docs/superpowers/plans/2026-06-13-progenitor-output-reference.md
# § F).
#
# We keep only 2xx response entries. Downstream callers handle non-2xx as
# `Error::UnexpectedResponse(reqwest::Response)` and read the body manually
# if they need Problem7807 details. Typed error responses can be
# reintroduced later by selective, per-operation edits — not required for
# Phase 1.
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
