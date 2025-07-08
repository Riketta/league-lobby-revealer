use windows::{
    Win32::System::{
        Com::{
            CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx,
            CoInitializeSecurity, EOAC_NONE, RPC_C_AUTHN_LEVEL_DEFAULT,
            RPC_C_IMP_LEVEL_IMPERSONATE,
        },
        Wmi::{
            IWbemLocator, IWbemServices, WBEM_FLAG_FORWARD_ONLY, WBEM_FLAG_RETURN_IMMEDIATELY,
            WBEM_INFINITE, WbemLocator,
        },
    },
    core::{BSTR, VARIANT, w},
};
pub(crate) struct Wmi {
    server: IWbemServices,
}

impl Wmi {
    pub(crate) fn new() -> Self {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED).ok().unwrap();

            CoInitializeSecurity(
                None,
                -1,
                None,
                None,
                RPC_C_AUTHN_LEVEL_DEFAULT,
                RPC_C_IMP_LEVEL_IMPERSONATE,
                None,
                EOAC_NONE,
                None,
            )
            .unwrap();

            let locator: IWbemLocator =
                CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER).unwrap();

            let server = locator
                .ConnectServer(&BSTR::from("root\\cimv2"), None, None, None, 0, None, None)
                .unwrap();

            Self { server }
        }
    }

    pub(crate) fn get_arguments_for_process_with_name(&self, process_name: &str) -> Option<String> {
        let process_name_lowered = process_name.to_lowercase();
        unsafe {
            let query = self
                .server
                .ExecQuery(
                    &BSTR::from("WQL"),
                    &BSTR::from("SELECT * from Win32_Process"),
                    WBEM_FLAG_FORWARD_ONLY | WBEM_FLAG_RETURN_IMMEDIATELY,
                    None,
                )
                .unwrap();

            loop {
                let mut instance = [None; 1];
                let mut returned = 0;
                query
                    .Next(WBEM_INFINITE, &mut instance, &mut returned)
                    .ok()
                    .unwrap();

                if let Some(row) = &instance[0] {
                    let mut process_variant = VARIANT::default();
                    let mut arguments_variant = VARIANT::default();

                    row.Get(w!("Caption"), 0, &mut process_variant, None, None)
                        .unwrap(); // "Name" same as "Caption"?
                    row.Get(w!("CommandLine"), 0, &mut arguments_variant, None, None)
                        .unwrap();

                    let process = process_variant.to_string();
                    let arguments = arguments_variant.to_string();
                    // println!("{process} {arguments}",);

                    if process.to_lowercase() == process_name_lowered {
                        return Some(arguments);
                    }
                } else {
                    break;
                }
            }
        }

        None
    }
}
