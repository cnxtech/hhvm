<?hh // partial
/**
 * Copyright (c) 2014, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *
 */

interface I1 {}

interface I2 extends I1 {}

trait Trait1 {
  require implements I1;
}

trait Trait2 {
  use Trait1;
}

class C1 implements I2 { // OK!
  use Trait1;
}

abstract class C2 { // Need to meet the requirement
  use Trait1;
}
