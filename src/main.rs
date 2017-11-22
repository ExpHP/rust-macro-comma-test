
#![cfg_attr(feature = "nightly", feature(concat_idents))]
#![cfg_attr(feature = "nightly", feature(mpsc_select))]

fn main() {
    let _ = _main();
}

fn _main() -> Result<(), ()> {
    use ::std::io::prelude::*;
    let mut stdout = ::std::io::stdout();

    /*----------------------------*/
    // Macros which very closely resemble the syntax of function calls
    // or some other grammar production which supports trailing commas.

    assert!(true);
    assert!(true, ); //~ ERROR: unexpected end of macro invocation
    assert!(true, "hello");
    assert!(true, "hello",); //~ ERROR: unexpected end of macro invocation
    assert!(true, "hello {}", "world");
    assert!(true, "hello {}", "world",);

    assert_eq!(1, 1);
    assert_eq!(1, 1,);  // (fixed in 1.22)
    assert_eq!(1, 1, "hello");
    assert_eq!(1, 1, "hello",);
    assert_eq!(1, 1, "hello {}", "world");
    assert_eq!(1, 1, "hello {}", "world",);

    assert_ne!(1, 2);
    assert_ne!(1, 2,);  // (fixed in 1.22)
    assert_ne!(1, 2, "hello");
    assert_ne!(1, 2, "hello",);
    assert_ne!(1, 2, "hello {}", "world");
    assert_ne!(1, 2, "hello {}", "world",);

    column!();

    compile_error!("lel");
    //~^ ERROR: lel
    compile_error!("lel",); //~ ERROR: compile_error! takes 1 argument
    //~^ ERROR: lel

    // The stub definition of concat! in std lies.
    // The compiler builtin supports trailing commas.
    concat!();
    concat!("hello");
    concat!("hello",);
    concat!("hello", " world");
    concat!("hello", " world",);

    debug_assert!(true);
    debug_assert!(true, ); //~ ERROR: unexpected end of macro invocation
    debug_assert!(true, "hello");
    debug_assert!(true, "hello",); //~ ERROR: unexpected end of macro invocation
    debug_assert!(true, "hello {}", "world");
    debug_assert!(true, "hello {}", "world",);

    debug_assert_eq!(1, 1);
    debug_assert_eq!(1, 1,); //  (fixed in 1.22)
    debug_assert_eq!(1, 1, "hello");
    debug_assert_eq!(1, 1, "hello",);
    debug_assert_eq!(1, 1, "hello {}", "world");
    debug_assert_eq!(1, 1, "hello {}", "world",);

    debug_assert_ne!(1, 2);
    debug_assert_ne!(1, 2,); //  (fixed in 1.22)
    debug_assert_ne!(1, 2, "hello");
    debug_assert_ne!(1, 2, "hello",);
    debug_assert_ne!(1, 2, "hello {}", "world");
    debug_assert_ne!(1, 2, "hello {}", "world",);

    // The stub definition of env! in std lies.
    // The compiler builtin supports a trailing comma.
    env!("PATH");
    env!("PATH",);

    eprint!("hello");
    eprint!("hello",);
    eprint!("hello {}", "world");
    eprint!("hello {}", "world",);

    eprintln!();
    eprintln!("hello");
    eprintln!("hello",);
    eprintln!("hello {}", "world");
    eprintln!("hello {}", "world",);

    file!();

    format!("hello");
    format!("hello",);
    format!("hello {}", "world");
    format!("hello {}", "world",);

    format_args!("hello");
    format_args!("hello",);
    format_args!("hello {}", "world");
    format_args!("hello {}", "world",);

    include!("dumdum.rs");
    include!("dumdum.rs",); //~ ERROR: include! takes 1 argument

    include_bytes!("dumdum.rs");
    include_bytes!("dumdum.rs",); //~ ERROR: include_bytes! takes 1 argument

    include_str!("dumdum.rs");
    include_str!("dumdum.rs",); //~ ERROR: include_str! takes 1 argument

    line!();

    module_path!();

    option_env!("PATH");
    option_env!("PATH",); //~ ERROR: option_env! takes 1 argument

    // this is just to silence some unreachable code warnings
    fn f() -> bool { false }

    if f() { panic!(); }
    if f() { panic!("hello"); }
    if f() { panic!("hello",); } //~ ERROR: unexpected end of macro invocation
    if f() { panic!("hello {}", "world"); }
    if f() { panic!("hello {}", "world",); }

    print!("hello");
    print!("hello",);
    print!("hello {}", "world");
    print!("hello {}", "world",);

    println!();
    println!("hello");
    println!("hello",);
    println!("hello {}", "world");
    println!("hello {}", "world",);

    try!(Ok(()));
    try!(Ok(()),); //~ ERROR: no rules expected the token `,`

    if f() { unimplemented!(); }
    if f() { unimplemented!("hello"); }
    if f() { unimplemented!("hello",); }
    if f() { unimplemented!("hello {}", "world"); }
    if f() { unimplemented!("hello {}", "world",); }

    if f() { unreachable!(); }
    if f() { unreachable!("hello"); }
    if f() { unreachable!("hello",); } //~ ERROR: unexpected end of macro invocation
    if f() { unreachable!("hello {}", "world"); }
    if f() { unreachable!("hello {}", "world",); }

    vec![] as Vec<()>;
    vec![0];
    vec![0,];
    vec![0, 1];
    vec![0, 1,];

    let _ = write!(&mut stdout, "hello");
    let _ = write!(&mut stdout, "hello",);
    let _ = write!(&mut stdout, "hello {}", "world");
    let _ = write!(&mut stdout, "hello {}", "world",);

    let _ = writeln!(&mut stdout);
    let _ = writeln!(&mut stdout,); //~ ERROR: unexpected end of macro invocation
    let _ = writeln!(&mut stdout, "hello");
    let _ = writeln!(&mut stdout, "hello",);
    let _ = writeln!(&mut stdout, "hello {}", "world");
    let _ = writeln!(&mut stdout, "hello {}", "world",);

    /*----------------------------*/
    // Macros not shown above because their syntax is unusual in some way.
    // In these cases, the argument for trailing comma support may be less compelling.

    cfg!(unix);
    cfg!(unix,); //~ ERROR: expected 1 cfg-pattern

    #[cfg(feature = "nightly")]
    {
        // The stub definition of concat_idents! in std lies.
        // The compiler builtin supports trailing commas.
        concat_idents!(foo)();
        concat_idents!(foo,)();
        concat_idents!(foo, bar)();
        concat_idents!(foo, bar,)();
    }

    #[cfg(feature = "nightly")]
    {
        use ::std::sync::mpsc;
        // (odd note: select! reassigns the identifiers given to it for
        //            receivers, so we need multiple receivers just to write
        //            a snippet that typechecks)
        let receivers = || {
            let (ta, ra) = mpsc::channel();
            let (tb, rb) = mpsc::channel();
            ta.send(()).unwrap();
            tb.send(()).unwrap();
            (ra, rb)
        };

        // ------------------
        // -- expr bodies --

        let (ra, _) = receivers();
        select! {
            _ = ra.recv() => ()
        }

        let (ra, _) = receivers();
        select! {
            _ = ra.recv() => (), //~ ERROR: unexpected end of macro invocation
        }

        let (ra, rb) = receivers();
        select! {
            _ = ra.recv() => (),
            _ = rb.recv() => ()
        }

        let (ra, rb) = receivers();
        select! {
            _ = ra.recv() => (),
            _ = rb.recv() => (), //~ ERROR: unexpected end of macro invocation
        }

        // ------------------
        // -- block bodies --

        let (ra, _) = receivers();
        select! {
            _ = ra.recv() => {}
        }

        let (ra, _) = receivers();
        select! {
            _ = ra.recv() => {}, //~ ERROR: unexpected end of macro invocation
        }

        let (ra, rb) = receivers();
        select! {
            _ = ra.recv() => {},
            _ = rb.recv() => {}
        }

        let (ra, rb) = receivers();
        select! {
            _ = ra.recv() => {},
            _ = rb.recv() => {}, //~ ERROR: unexpected end of macro invocation
        }

        // ----------------------------------------------------
        // -- block bodies without commas (allowed in match) --

        let (ra, rb) = receivers();
        select! {
            _ = ra.recv() => {}
            _ = rb.recv() => {} //~ ERROR: no rules expected the token `_`
        }
    }

    // stringify! is N/A

    thread_local! {
        #[allow(unused)] pub static A: () = ();
    }

    thread_local! {
        #[allow(unused)] pub static AA: () = ()   // strangely, this is allowed
    }

    thread_local! {
        #[allow(unused)] pub static AAA: () = ();
        #[allow(unused)] pub static AAAA: () = ();
    }

    thread_local! {
        #[allow(unused)] pub static AAAAG: () = ();
        #[allow(unused)] pub static AAAAGH: () = ()   // strangely, this is allowed
    }

    /*----------------------------*/
    // BONUS ROUND!
    // Macros that allow what appears to be a format message without any arguments,
    // and must forever continue to support this for backwards compatibility.
    // (here, errors are GOOD)

/*
    assert!(true, "{}");
    assert_eq!(1, 1, "{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    assert_ne!(1, 2, "{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    debug_assert!(true, "{}");
    debug_assert_eq!(1, 1, "{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    debug_assert_ne!(1, 2, "{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    eprint!("{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    eprintln!("{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    format!("{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    format_args!("{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    if f() { panic!("{}"); }
    print!("{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    println!("{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    unimplemented!("{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    unreachable!("{}");
    write!(&mut stdout, "{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
    writeln!(&mut stdout, "{}"); //~ ERROR: error: 1 positional argument in format string, but no arguments were given
*/

    /*----------------------------*/
    Ok(())
}

// these are here for testing concat_idents!
#[cfg(feature = "nightly")]
fn foo() {}
#[cfg(feature = "nightly")]
fn foobar() {}

