#![allow(non_snake_case)]
#![allow(dead_code)]
use crate::config::from_string;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fmt::{Display, Formatter};

const DEFAULT_ENV: [(&str, &str); 14] = [
                ("WINEPREFIX", "/opt/windows/windows7"),
                ("WINEARCH", "win64"),
                ("WINE_LARGE_ADDRESS_AWARE","1"),
                ("DXVK_HUD", "1"),
                //("WINEDEBUG", "warn+all"),
                ("DISPLAY", ":0.0"),
                (
                    "Path",
                    r"c:\\windows;c:\\windows\\system;c:\\windows\\syswow64;c:\\windows\\system32",
                ),
                ("WINEDLLOVERRIDES", "d3d10core,d3d11,d3d12,d3d9,d3dcompiler_33,d3dcompiler_34,d3dcompiler_35,d3dcompiler_36,d3dcompiler_37,d3dcompiler_38,d3dcompiler_39,d3dcompiler_40,d3dcompiler_41,d3dcompiler_42,d3dcompiler_43,d3dcompiler_46,d3dcompiler_47,d3dx10,d3dx10_33,d3dx10_34,d3dx10_35,d3dx10_36,d3dx10_37,d3dx10_38,d3dx10_39,d3dx10_40,d3dx10_41,d3dx10_42,d3dx10_43,d3dx11_42,d3dx11_43,d3dx9_24,d3dx9_25,d3dx9_26,d3dx9_27,d3dx9_28,d3dx9_29,d3dx9_30,d3dx9_31,d3dx9_32,d3dx9_33,d3dx9_34,d3dx9_35,d3dx9_36,d3dx9_37,d3dx9_38,d3dx9_39,d3dx9_40,d3dx9_41,d3dx9_42,d3dx9_43,dxgi,nvapi,nvapi64,nvml=n;winemenubuilder="),
                //("MANGOHUD","1"),
                //("MANGOHUD_DLSYM","1"),
                ("DRI_PRIME","1"),
                ("__NV_PRIME_RENDER_OFFLOAD","1"),
                ("__GLX_VENDOR_LIBRARY_NAME","nvidia"),
                ("__VK_LAYER_NV_optimus","NVIDIA_only"),
                ("VK_ICD_FILENAMES", "/usr/share/vulkan/icd.d/nvidia_icd.json"),
                ("DXVK_ENABLE_NVAPI", "1"),
                ("WINE", "/usr/bin/wine"),
            ];

const ENV_KEY: &str = "env";

pub struct Env(HashMap<String, String>);
impl Env {
    pub fn get_map(&self) -> &HashMap<String, String> {
        &self.0
    }
    pub fn from_string(c: &str, with_sys_env:bool) -> Result<Env, ()> {
        match from_string(c, ENV_KEY) {
            Ok(mut e) => {
                with_sys_env.then(||e.extend(std::env::vars().collect::<HashMap<String, String>>()));
                Ok(Env { 0: e })
            },
            _ => Err(()),
        }
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
    pub fn cstr(&self)-> Vec<Box<CStr>>{
        self.0
        .iter()
        .map(|(k, v)|CString::new(format!("{}={}", k, v)).unwrap().into_boxed_c_str())
        .collect::<Vec<Box<CStr>>>()
    }
}

impl Default for Env {
    fn default() -> Self {
        Env {
            0: HashMap::from(DEFAULT_ENV.map(|(k, v)| (k.to_string(), v.to_string()))),
        }
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut fmt_str = String::new();
        fmt_str.push_str("Env { ");
        for item in &self.0 {
            fmt_str.push_str(&*format!("{}=\"{}\", ", item.0, item.1))
        }
        fmt_str.push_str(" }");
        write!(f, "{}", fmt_str)
    }
}
