// /// tests that Audio is still Send + Sync(remove this once impl is done)
// fn _test_sync()
// {
//     fn is_sync<T: Sync>() { }
//     fn is_send<T: Send>() { }

//     is_sync::<AudioCtx>(); // compiles only if true
//     is_send::<AudioCtx>();
// }