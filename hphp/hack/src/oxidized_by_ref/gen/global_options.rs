// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
//
// @generated SignedSource<<bd9f8bc1c7e670c6be631f6142bab746>>
//
// To regenerate this file, run:
//   hphp/hack/src/oxidized_by_ref/regen.sh

use arena_trait::TrivialDrop;
use ocamlrep_derive::FromOcamlRepIn;
use ocamlrep_derive::ToOcamlRep;
use serde::Serialize;

#[allow(unused_imports)]
use crate::*;

#[derive(
    Clone,
    Debug,
    FromOcamlRepIn,
    PartialEq,
    PartialOrd,
    Serialize,
    ToOcamlRep
)]
pub struct GlobalOptions<'a> {
    pub tco_experimental_features: s_set::SSet<'a>,
    pub tco_migration_flags: s_set::SSet<'a>,
    pub tco_dynamic_view: bool,
    pub tco_num_local_workers: Option<isize>,
    pub tco_parallel_type_checking_threshold: isize,
    pub tco_defer_class_declaration_threshold: Option<isize>,
    pub tco_max_times_to_defer_type_checking: Option<isize>,
    pub tco_prefetch_deferred_files: bool,
    pub tco_remote_type_check_threshold: Option<isize>,
    pub tco_remote_type_check: bool,
    pub tco_remote_worker_key: Option<&'a str>,
    pub tco_remote_check_id: Option<&'a str>,
    pub tco_remote_max_batch_size: isize,
    pub tco_remote_min_batch_size: isize,
    pub tco_num_remote_workers: isize,
    pub so_remote_version_specifier: Option<&'a str>,
    pub so_remote_worker_vfs_checkout_threshold: isize,
    pub so_naming_sqlite_path: Option<&'a str>,
    pub po_auto_namespace_map: &'a [(&'a str, &'a str)],
    pub po_codegen: bool,
    pub po_deregister_php_stdlib: bool,
    pub po_disallow_toplevel_requires: bool,
    pub po_disable_nontoplevel_declarations: bool,
    pub po_disable_static_closures: bool,
    pub po_allow_goto: bool,
    pub po_allow_unstable_features: bool,
    pub tco_log_inference_constraints: bool,
    pub tco_disallow_array_typehint: bool,
    pub tco_disallow_array_literal: bool,
    pub tco_language_feature_logging: bool,
    pub tco_unsafe_rx: bool,
    pub tco_disallow_scrutinee_case_value_type_mismatch: bool,
    pub tco_timeout: isize,
    pub tco_disallow_invalid_arraykey: bool,
    pub tco_disallow_byref_dynamic_calls: bool,
    pub tco_disallow_byref_calls: bool,
    pub allowed_fixme_codes_strict: i_set::ISet<'a>,
    pub allowed_fixme_codes_partial: i_set::ISet<'a>,
    pub codes_not_raised_partial: i_set::ISet<'a>,
    pub log_levels: s_map::SMap<'a, isize>,
    pub po_disable_lval_as_an_expression: bool,
    pub tco_shallow_class_decl: bool,
    pub po_rust_parser_errors: bool,
    pub profile_type_check_duration_threshold: f64,
    pub profile_type_check_twice: bool,
    pub profile_total_typecheck_duration: bool,
    pub profile_owner: Option<&'a str>,
    pub profile_desc: &'a str,
    pub tco_like_type_hints: bool,
    pub tco_union_intersection_type_hints: bool,
    pub tco_coeffects: bool,
    pub tco_like_casts: bool,
    pub tco_simple_pessimize: f64,
    pub tco_complex_coercion: bool,
    pub tco_disable_partially_abstract_typeconsts: bool,
    pub error_codes_treated_strictly: i_set::ISet<'a>,
    pub tco_check_xhp_attribute: bool,
    pub tco_check_redundant_generics: bool,
    pub tco_disallow_unresolved_type_variables: bool,
    pub tco_disallow_trait_reuse: bool,
    pub tco_disallow_invalid_arraykey_constraint: bool,
    pub po_enable_class_level_where_clauses: bool,
    pub po_disable_legacy_soft_typehints: bool,
    pub po_allowed_decl_fixme_codes: i_set::ISet<'a>,
    pub po_allow_new_attribute_syntax: bool,
    pub tco_global_inference: bool,
    pub tco_gi_reinfer_types: &'a [&'a str],
    pub tco_ordered_solving: bool,
    pub tco_const_static_props: bool,
    pub po_disable_legacy_attribute_syntax: bool,
    pub tco_const_attribute: bool,
    pub po_const_default_func_args: bool,
    pub po_const_default_lambda_args: bool,
    pub po_disallow_silence: bool,
    pub po_abstract_static_props: bool,
    pub po_disable_unset_class_const: bool,
    pub po_parser_errors_only: bool,
    pub tco_check_attribute_locations: bool,
    pub glean_service: &'a str,
    pub glean_hostname: &'a str,
    pub glean_port: isize,
    pub glean_reponame: &'a str,
    pub symbol_write_root_path: &'a str,
    pub symbol_write_hhi_path: &'a str,
    pub symbol_write_ignore_paths: &'a [&'a str],
    pub symbol_write_index_paths: &'a [&'a str],
    pub symbol_write_include_hhi: bool,
    pub po_disallow_func_ptrs_in_constants: bool,
    pub tco_error_php_lambdas: bool,
    pub tco_disallow_discarded_nullable_awaitables: bool,
    pub po_enable_xhp_class_modifier: bool,
    pub po_disable_xhp_element_mangling: bool,
    pub po_disable_xhp_children_declarations: bool,
    pub po_enable_enum_classes: bool,
    pub po_disable_modes: bool,
    pub po_disable_hh_ignore_error: bool,
    pub po_disable_array: bool,
    pub po_disable_array_typehint: bool,
    pub tco_enable_systemlib_annotations: bool,
    pub tco_higher_kinded_types: bool,
    pub tco_method_call_inference: bool,
    pub tco_report_pos_from_reason: bool,
    pub tco_typecheck_sample_rate: f64,
    pub tco_enable_sound_dynamic: bool,
    pub po_disallow_hash_comments: bool,
}
impl<'a> TrivialDrop for GlobalOptions<'a> {}
