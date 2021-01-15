/// macro that maps a character or its symbolic
/// representation to a variant in the `KeyCode`
/// enum. This doesn't have all entries, but is
/// sufficient for most use cases
/// ```
/// app.keys().pressed(btn!("up"));
/// app.keys().pressed(btn!("A"));
/// app.keys().pressed(btn!("a"));
/// app.keys().pressed(btn!(" "));
///
/// app.mouse().pressed(btn!("lmb"));
/// app.mouse().pressed(btn!("rmb"));
/// app.mouse().pressed(btn!("mmb"));
/// ```
#[macro_export]
macro_rules! btn
{
    // mouse
    ("lmb") => { framework::input::MouseButton::Left };
    ("LMB") => { framework::input::MouseButton::Left };
    ("rmb") => { framework::input::MouseButton::Right };
    ("RMB") => { framework::input::MouseButton::Right };
    ("mmb") => { framework::input::MouseButton::Middle };
    ("MMB") => { framework::input::MouseButton::Middle };

    // keyboard
    ("1") => { framework::input::KeyCode::Key1 };
    ("2") => { framework::input::KeyCode::Key2 };
    ("3") => { framework::input::KeyCode::Key3 };
    ("4") => { framework::input::KeyCode::Key4 };
    ("5") => { framework::input::KeyCode::Key5 };
    ("6") => { framework::input::KeyCode::Key6 };
    ("7") => { framework::input::KeyCode::Key7 };
    ("8") => { framework::input::KeyCode::Key8 };
    ("9") => { framework::input::KeyCode::Key9 };
    ("0") => { framework::input::KeyCode::Key0 };

    ("a") => { framework::input::KeyCode::A };
    ("b") => { framework::input::KeyCode::B };
    ("c") => { framework::input::KeyCode::C };
    ("d") => { framework::input::KeyCode::D };
    ("e") => { framework::input::KeyCode::E };
    ("f") => { framework::input::KeyCode::F };
    ("g") => { framework::input::KeyCode::G };
    ("h") => { framework::input::KeyCode::H };
    ("i") => { framework::input::KeyCode::I };
    ("j") => { framework::input::KeyCode::J };
    ("k") => { framework::input::KeyCode::K };
    ("l") => { framework::input::KeyCode::L };
    ("m") => { framework::input::KeyCode::M };
    ("n") => { framework::input::KeyCode::N };
    ("o") => { framework::input::KeyCode::O };
    ("p") => { framework::input::KeyCode::P };
    ("q") => { framework::input::KeyCode::Q };
    ("r") => { framework::input::KeyCode::R };
    ("s") => { framework::input::KeyCode::S };
    ("t") => { framework::input::KeyCode::T };
    ("u") => { framework::input::KeyCode::U };
    ("v") => { framework::input::KeyCode::V };
    ("w") => { framework::input::KeyCode::W };
    ("x") => { framework::input::KeyCode::X };
    ("y") => { framework::input::KeyCode::Y };
    ("z") => { framework::input::KeyCode::Z };

    ("A") => { framework::input::KeyCode::A };
    ("B") => { framework::input::KeyCode::B };
    ("C") => { framework::input::KeyCode::C };
    ("D") => { framework::input::KeyCode::D };
    ("E") => { framework::input::KeyCode::E };
    ("F") => { framework::input::KeyCode::F };
    ("G") => { framework::input::KeyCode::G };
    ("H") => { framework::input::KeyCode::H };
    ("I") => { framework::input::KeyCode::I };
    ("J") => { framework::input::KeyCode::J };
    ("K") => { framework::input::KeyCode::K };
    ("L") => { framework::input::KeyCode::L };
    ("M") => { framework::input::KeyCode::M };
    ("N") => { framework::input::KeyCode::N };
    ("O") => { framework::input::KeyCode::O };
    ("P") => { framework::input::KeyCode::P };
    ("Q") => { framework::input::KeyCode::Q };
    ("R") => { framework::input::KeyCode::R };
    ("S") => { framework::input::KeyCode::S };
    ("T") => { framework::input::KeyCode::T };
    ("U") => { framework::input::KeyCode::U };
    ("V") => { framework::input::KeyCode::V };
    ("W") => { framework::input::KeyCode::W };
    ("X") => { framework::input::KeyCode::X };
    ("Y") => { framework::input::KeyCode::Y };
    ("Z") => { framework::input::KeyCode::Z };

    ("esc") => { framework::input::KeyCode::Escape };
    ("ESC") => { framework::input::KeyCode::Escape };

    ("f1")  => { framework::input::KeyCode::F1 };
    ("f2")  => { framework::input::KeyCode::F2 };
    ("f3")  => { framework::input::KeyCode::F3 };
    ("f4")  => { framework::input::KeyCode::F4 };
    ("f5")  => { framework::input::KeyCode::F5 };
    ("f6")  => { framework::input::KeyCode::F6 };
    ("f7")  => { framework::input::KeyCode::F7 };
    ("f8")  => { framework::input::KeyCode::F8 };
    ("f9")  => { framework::input::KeyCode::F9 };
    ("f10") => { framework::input::KeyCode::F10 };
    ("f11") => { framework::input::KeyCode::F11 };
    ("f12") => { framework::input::KeyCode::F12 };
    ("f13") => { framework::input::KeyCode::F13 };
    ("f14") => { framework::input::KeyCode::F14 };
    ("f15") => { framework::input::KeyCode::F15 };
    ("f16") => { framework::input::KeyCode::F16 };
    ("f17") => { framework::input::KeyCode::F17 };
    ("f18") => { framework::input::KeyCode::F18 };
    ("f19") => { framework::input::KeyCode::F19 };
    ("f20") => { framework::input::KeyCode::F20 };
    ("f21") => { framework::input::KeyCode::F21 };
    ("f22") => { framework::input::KeyCode::F22 };
    ("f23") => { framework::input::KeyCode::F23 };
    ("f24") => { framework::input::KeyCode::F24 };
    ("F1")  => { framework::input::KeyCode::F1 };
    ("F2")  => { framework::input::KeyCode::F2 };
    ("F3")  => { framework::input::KeyCode::F3 };
    ("F4")  => { framework::input::KeyCode::F4 };
    ("F5")  => { framework::input::KeyCode::F5 };
    ("F6")  => { framework::input::KeyCode::F6 };
    ("F7")  => { framework::input::KeyCode::F7 };
    ("F8")  => { framework::input::KeyCode::F8 };
    ("F9")  => { framework::input::KeyCode::F9 };
    ("F10") => { framework::input::KeyCode::F10 };
    ("F11") => { framework::input::KeyCode::F11 };
    ("F12") => { framework::input::KeyCode::F12 };
    ("F13") => { framework::input::KeyCode::F13 };
    ("F14") => { framework::input::KeyCode::F14 };
    ("F15") => { framework::input::KeyCode::F15 };
    ("F16") => { framework::input::KeyCode::F16 };
    ("F17") => { framework::input::KeyCode::F17 };
    ("F18") => { framework::input::KeyCode::F18 };
    ("F19") => { framework::input::KeyCode::F19 };
    ("F20") => { framework::input::KeyCode::F20 };
    ("F21") => { framework::input::KeyCode::F21 };
    ("F22") => { framework::input::KeyCode::F22 };
    ("F23") => { framework::input::KeyCode::F23 };
    ("F24") => { framework::input::KeyCode::F24 };

    ("left")  => { framework::input::KeyCode::Left  };
    ("up")    => { framework::input::KeyCode::Up    };
    ("right") => { framework::input::KeyCode::Right };
    ("down")  => { framework::input::KeyCode::Down  };

    ("back")      => { framework::input::KeyCode::Back };
    ("backspace") => { framework::input::KeyCode::Back };
    ("delete")    => { framework::input::KeyCode::Back };

    ("return") => { framework::input::KeyCode::Return };
    ("enter")  => { framework::input::KeyCode::Return };
    ("\n)")    => { framework::input::KeyCode::Return };
    
    (" ")  => { framework::input::KeyCode::Space };
    ("'")  => { framework::input::KeyCode::Apostrophe };
    ("*")  => { framework::input::KeyCode::Asterisk };
    ("\\") => { framework::input::KeyCode::Backslash };
    (":")  => { framework::input::KeyCode::Colon };
    (",")  => { framework::input::KeyCode::Comma };
    ("=")  => { framework::input::KeyCode::Equals };
    ("[")  => { framework::input::KeyCode::LBracket };
    ("]")  => { framework::input::KeyCode::RBracket };
    ("-")  => { framework::input::KeyCode::Minus };
    (".")  => { framework::input::KeyCode::Period };
    ("+")  => { framework::input::KeyCode::Plus };
    (";")  => { framework::input::KeyCode::Semicolon };
    ("/")  => { framework::input::KeyCode::Slash };

    ("    ") => { framework::input::KeyCode::Tab };
    ("\t")   => { framework::input::KeyCode::Tab };
}