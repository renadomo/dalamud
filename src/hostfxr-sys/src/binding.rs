/* automatically generated by rust-bindgen 0.55.1 */

pub type size_t = ::std::os::raw::c_ulonglong;
pub type wchar_t = ::std::os::raw::c_ushort;
pub type char_t = wchar_t;
pub const hostfxr_delegate_type_hdt_com_activation: hostfxr_delegate_type = 0;
pub const hostfxr_delegate_type_hdt_load_in_memory_assembly: hostfxr_delegate_type = 1;
pub const hostfxr_delegate_type_hdt_winrt_activation: hostfxr_delegate_type = 2;
pub const hostfxr_delegate_type_hdt_com_register: hostfxr_delegate_type = 3;
pub const hostfxr_delegate_type_hdt_com_unregister: hostfxr_delegate_type = 4;
pub const hostfxr_delegate_type_hdt_load_assembly_and_get_function_pointer: hostfxr_delegate_type =
    5;
pub type hostfxr_delegate_type = ::std::os::raw::c_int;
pub type hostfxr_main_fn = ::std::option::Option<
    unsafe extern "C" fn(argc: ::std::os::raw::c_int, argv: *mut *const char_t) -> i32,
>;
pub type hostfxr_main_startupinfo_fn = ::std::option::Option<
    unsafe extern "C" fn(
        argc: ::std::os::raw::c_int,
        argv: *mut *const char_t,
        host_path: *const char_t,
        dotnet_root: *const char_t,
        app_path: *const char_t,
    ) -> i32,
>;
pub type hostfxr_error_writer_fn =
    ::std::option::Option<unsafe extern "C" fn(message: *const char_t)>;
pub type hostfxr_set_error_writer_fn = ::std::option::Option<
    unsafe extern "C" fn(error_writer: hostfxr_error_writer_fn) -> hostfxr_error_writer_fn,
>;
pub type hostfxr_handle = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hostfxr_initialize_parameters {
    pub size: size_t,
    pub host_path: *const char_t,
    pub dotnet_root: *const char_t,
}
#[test]
fn bindgen_test_layout_hostfxr_initialize_parameters() {
    assert_eq!(
        ::std::mem::size_of::<hostfxr_initialize_parameters>(),
        24usize,
        concat!("Size of: ", stringify!(hostfxr_initialize_parameters))
    );
    assert_eq!(
        ::std::mem::align_of::<hostfxr_initialize_parameters>(),
        8usize,
        concat!("Alignment of ", stringify!(hostfxr_initialize_parameters))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<hostfxr_initialize_parameters>())).size as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(hostfxr_initialize_parameters),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<hostfxr_initialize_parameters>())).host_path as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(hostfxr_initialize_parameters),
            "::",
            stringify!(host_path)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<hostfxr_initialize_parameters>())).dotnet_root as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(hostfxr_initialize_parameters),
            "::",
            stringify!(dotnet_root)
        )
    );
}
pub type hostfxr_initialize_for_dotnet_command_line_fn = ::std::option::Option<
    unsafe extern "C" fn(
        argc: ::std::os::raw::c_int,
        argv: *mut *const char_t,
        parameters: *const hostfxr_initialize_parameters,
        host_context_handle: *mut hostfxr_handle,
    ) -> i32,
>;
pub type hostfxr_initialize_for_runtime_config_fn = ::std::option::Option<
    unsafe extern "C" fn(
        runtime_config_path: *const char_t,
        parameters: *const hostfxr_initialize_parameters,
        host_context_handle: *mut hostfxr_handle,
    ) -> i32,
>;
pub type hostfxr_get_runtime_property_value_fn = ::std::option::Option<
    unsafe extern "C" fn(
        host_context_handle: hostfxr_handle,
        name: *const char_t,
        value: *mut *const char_t,
    ) -> i32,
>;
pub type hostfxr_set_runtime_property_value_fn = ::std::option::Option<
    unsafe extern "C" fn(
        host_context_handle: hostfxr_handle,
        name: *const char_t,
        value: *const char_t,
    ) -> i32,
>;
pub type hostfxr_get_runtime_properties_fn = ::std::option::Option<
    unsafe extern "C" fn(
        host_context_handle: hostfxr_handle,
        count: *mut size_t,
        keys: *mut *const char_t,
        values: *mut *const char_t,
    ) -> i32,
>;
pub type hostfxr_run_app_fn =
    ::std::option::Option<unsafe extern "C" fn(host_context_handle: hostfxr_handle) -> i32>;
pub type hostfxr_get_runtime_delegate_fn = ::std::option::Option<
    unsafe extern "C" fn(
        host_context_handle: hostfxr_handle,
        type_: hostfxr_delegate_type,
        delegate: *mut *mut ::std::os::raw::c_void,
    ) -> i32,
>;
pub type hostfxr_close_fn =
    ::std::option::Option<unsafe extern "C" fn(host_context_handle: hostfxr_handle) -> i32>;
pub type load_assembly_and_get_function_pointer_fn = ::std::option::Option<
    unsafe extern "C" fn(
        assembly_path: *const char_t,
        type_name: *const char_t,
        method_name: *const char_t,
        delegate_type_name: *const char_t,
        reserved: *mut ::std::os::raw::c_void,
        delegate: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type component_entry_point_fn = ::std::option::Option<
    unsafe extern "C" fn(
        arg: *mut ::std::os::raw::c_void,
        arg_size_in_bytes: i32,
    ) -> ::std::os::raw::c_int,
>;
