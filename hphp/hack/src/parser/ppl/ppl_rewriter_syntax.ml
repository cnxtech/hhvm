(**
 * Copyright (c) 2018, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the BSD-style license found in the
 * LICENSE file in the "hack" directory of this source tree. An additional grant
 * of patent rights can be found in the PATENTS file in the same directory.
 *)

module List = Core_list
(* Includes helpful builders *)
module CoroutineSyntax = Coroutine_syntax

open CoroutineSyntax

let ppl_infer_type_string = [ "PPL"; "Inference"; "Infer"; ]
let ppl_infer_type_name_syntax =
  make_qualified_name_syntax ppl_infer_type_string ~has_leading:true
(* If this is changed, also change
 * /src/parser/smart_constructors/coroutine_smart_constructor.ml *)
let ppl_macro_string = Coroutine_smart_constructor.ppl_macro_string
let receiver_string = "$__recv"
let receiver_variable_syntax = make_variable_expression_syntax receiver_string
let suspend_token_syntax = make_token_syntax TokenKind.Suspend
(* Special method names reserved by the Infer class *)
let reserved_method_names =
  [ "sample"
  ; "factor"
  ; "observe"
  ; "condition"
  ]

(* Determines whether a string is in the list of reserved method names *)
let is_infer_method method_string =
  List.mem reserved_method_names method_string
