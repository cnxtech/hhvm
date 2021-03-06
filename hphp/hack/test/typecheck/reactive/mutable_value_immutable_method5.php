<?hh // strict
class A {
  <<__Rx, __MutableReturn>>
  public static function make(): A {
    return new A();
  }

  <<__Rx>>
  public static function immutableMethod(A $a): int {
    return 42;
  }
}

<<__Rx>>
function f(): void {
  $a = \HH\Rx\mutable(A::make());
  A::immutableMethod($a);
}
