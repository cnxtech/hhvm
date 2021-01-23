<?hh
<<file: __EnableUnstableFeatures('enum_atom')>>

interface IBox {}
class Box<T> implements IBox {
  public function __construct(public T $data) {}
}
function f<reify X, T>(<<__Atom>> HH\MemberOf<X, Box<T>> $elt) : T {
    return $elt->data;
}

<<__EntryPoint>>
 function main() {
    $x = "A";
    echo("Hello " . f<string, string>($x) . "!\n");
}
