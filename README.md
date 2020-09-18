# repro-compiler-error

I am trying to fix a compiler error and I cannot figure it out.  The function signature dht_split_read is from the dht-hal-drv
crate with unrelated arguments removed.

        error[E0277]: the trait bound `Pa16<Input<Floating>>: _embedded_hal_digital_InputPin` is not satisfied
        --> src/main.rs:21:20
        |
        21 |     dht_split_read(&mut in_pin);
        |                    ^^^^^^^^^^^ the trait `_embedded_hal_digital_InputPin` is not implemented for `Pa16<Input<Floating>>`
        |
        = note: required because of the requirements on the impl of `embedded_hal::digital::v2::InputPin` for `Pa16<Input<Floating>>`
        = note: required for the cast to the object type `dyn embedded_hal::digital::v2::InputPin<Error = ()>`

        error: aborting due to previous error

        For more information about this error, try `rustc --explain E0277`.
        error: could not compile `repro`

        To learn more, run the command again with --verbose.


I do know what `_embedded_hal_digital_InputPin` is?  I presume this was generated in a macro.  I certainly don't know how I would
go about implementing that.
