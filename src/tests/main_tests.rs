use workctl::sync_flag;


#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn can_make_ctrl_c_handler() {
    let (control_c_flag_tx, control_c_called_flag_rx) = sync_flag::new_syncflag(false);
    crate::setup_control_c(control_c_flag_tx);
}


