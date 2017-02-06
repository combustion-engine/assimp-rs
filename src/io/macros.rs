macro_rules! user_data {
    ($file:expr) => {unsafe {
        c_assert!(!$file.is_null());

        let user_data = (*$file).user_data as *mut _;

        c_assert!(!user_data.is_null());

        &mut *user_data
    }}
}
