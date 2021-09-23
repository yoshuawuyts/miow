#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Win32 {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Foundation {
            #[repr(transparent)]
            #[derive(
                :: std :: default :: Default,
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct BOOL(pub i32);
            unsafe impl ::windows::Abi for BOOL {
                type Abi = Self;
                type DefaultType = Self;
            }
            impl BOOL {
                #[inline]
                pub fn as_bool(self) -> bool {
                    !(self.0 == 0)
                }
                #[inline]
                pub fn ok(self) -> ::windows::Result<()> {
                    if self.as_bool() {
                        Ok(())
                    } else {
                        Err(::windows::Error::from_win32())
                    }
                }
                #[inline]
                #[track_caller]
                pub fn unwrap(self) {
                    self.ok().unwrap();
                }
                #[inline]
                #[track_caller]
                pub fn expect(self, msg: &str) {
                    self.ok().expect(msg);
                }
            }
            impl ::std::convert::From<BOOL> for bool {
                fn from(value: BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<&BOOL> for bool {
                fn from(value: &BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<bool> for BOOL {
                fn from(value: bool) -> Self {
                    if value {
                        BOOL(1)
                    } else {
                        BOOL(0)
                    }
                }
            }
            impl ::std::convert::From<&bool> for BOOL {
                fn from(value: &bool) -> Self {
                    (*value).into()
                }
            }
            impl ::std::cmp::PartialEq<bool> for BOOL {
                fn eq(&self, other: &bool) -> bool {
                    self.as_bool() == *other
                }
            }
            impl ::std::cmp::PartialEq<BOOL> for bool {
                fn eq(&self, other: &BOOL) -> bool {
                    *self == other.as_bool()
                }
            }
            impl std::ops::Not for BOOL {
                type Output = Self;
                fn not(self) -> Self::Output {
                    if self.as_bool() {
                        BOOL(0)
                    } else {
                        BOOL(1)
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, BOOL> for bool {
                fn into_param(self) -> ::windows::Param<'a, BOOL> {
                    ::windows::Param::Owned(self.into())
                }
            }
            pub unsafe fn CloseHandle<'a>(hobject: impl ::windows::IntoParam<'a, HANDLE>) -> BOOL {
                #[cfg(windows)]
                {
                    #[link(name = "kernel32")]
                    extern "system" {
                        fn CloseHandle(hobject: HANDLE) -> BOOL;
                    }
                    ::std::mem::transmute(CloseHandle(hobject.into_param().abi()))
                }
                #[cfg(not(windows))]
                unimplemented!("Unsupported target OS");
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: default :: Default,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct HANDLE(pub isize);
            unsafe impl ::windows::Handle for HANDLE {
                fn is_invalid(&self) -> bool {
                    self.0 == 0 || self.0 == -1
                }
                fn ok(self) -> ::windows::Result<Self> {
                    if self.is_invalid() {
                        Err(::windows::Error::from_win32())
                    } else {
                        Ok(self)
                    }
                }
            }
            unsafe impl ::windows::Abi for HANDLE {
                type Abi = Self;
                type DefaultType = Self;
            }
            pub const INVALID_HANDLE_VALUE: HANDLE = HANDLE(-1i32 as _);
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct PSTR(pub *mut u8);
            impl PSTR {
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            unsafe impl ::windows::Abi for PSTR {
                type Abi = Self;
                type DefaultType = Self;
                unsafe fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for &str {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: fmt :: Debug,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
            )]
            #[repr(transparent)]
            pub struct PWSTR(pub *mut u16);
            impl PWSTR {
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PWSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            unsafe impl ::windows::Abi for PWSTR {
                type Abi = Self;
                type DefaultType = Self;
                unsafe fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for &str {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[cfg(windows)]
            impl<'a> ::windows::IntoParam<'a, PWSTR> for &::std::ffi::OsStr {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    use std::os::windows::ffi::OsStrExt;
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_wide()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[cfg(windows)]
            impl<'a> ::windows::IntoParam<'a, PWSTR> for ::std::ffi::OsString {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    use std::os::windows::ffi::OsStrExt;
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_wide()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod NetworkManagement {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod IpHelper {
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct ADDRESS_FAMILY(pub u32);
                pub const AF_INET: ADDRESS_FAMILY = ADDRESS_FAMILY(2u32);
                pub const AF_INET6: ADDRESS_FAMILY = ADDRESS_FAMILY(23u32);
                pub const AF_UNSPEC: ADDRESS_FAMILY = ADDRESS_FAMILY(0u32);
                impl ::std::convert::From<u32> for ADDRESS_FAMILY {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for ADDRESS_FAMILY {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for ADDRESS_FAMILY {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for ADDRESS_FAMILY {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for ADDRESS_FAMILY {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for ADDRESS_FAMILY {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::std::ops::Not for ADDRESS_FAMILY {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Networking {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod WinSock {
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct IN6_ADDR {
                    pub u: IN6_ADDR_0,
                }
                impl IN6_ADDR {}
                impl ::std::default::Default for IN6_ADDR {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for IN6_ADDR {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for IN6_ADDR {}
                unsafe impl ::windows::Abi for IN6_ADDR {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub union IN6_ADDR_0 {
                    pub Byte: [u8; 16],
                    pub Word: [u16; 8],
                }
                impl IN6_ADDR_0 {}
                impl ::std::default::Default for IN6_ADDR_0 {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for IN6_ADDR_0 {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for IN6_ADDR_0 {}
                unsafe impl ::windows::Abi for IN6_ADDR_0 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct IN_ADDR {
                    pub S_un: IN_ADDR_0,
                }
                impl IN_ADDR {}
                impl ::std::default::Default for IN_ADDR {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for IN_ADDR {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for IN_ADDR {}
                unsafe impl ::windows::Abi for IN_ADDR {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub union IN_ADDR_0 {
                    pub S_un_b: IN_ADDR_0_0,
                    pub S_un_w: IN_ADDR_0_1,
                    pub S_addr: u32,
                }
                impl IN_ADDR_0 {}
                impl ::std::default::Default for IN_ADDR_0 {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for IN_ADDR_0 {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for IN_ADDR_0 {}
                unsafe impl ::windows::Abi for IN_ADDR_0 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct IN_ADDR_0_0 {
                    pub s_b1: u8,
                    pub s_b2: u8,
                    pub s_b3: u8,
                    pub s_b4: u8,
                }
                impl IN_ADDR_0_0 {}
                impl ::std::default::Default for IN_ADDR_0_0 {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for IN_ADDR_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_S_un_b_e__Struct")
                            .field("s_b1", &self.s_b1)
                            .field("s_b2", &self.s_b2)
                            .field("s_b3", &self.s_b3)
                            .field("s_b4", &self.s_b4)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IN_ADDR_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self.s_b1 == other.s_b1
                            && self.s_b2 == other.s_b2
                            && self.s_b3 == other.s_b3
                            && self.s_b4 == other.s_b4
                    }
                }
                impl ::std::cmp::Eq for IN_ADDR_0_0 {}
                unsafe impl ::windows::Abi for IN_ADDR_0_0 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct IN_ADDR_0_1 {
                    pub s_w1: u16,
                    pub s_w2: u16,
                }
                impl IN_ADDR_0_1 {}
                impl ::std::default::Default for IN_ADDR_0_1 {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for IN_ADDR_0_1 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_S_un_w_e__Struct")
                            .field("s_w1", &self.s_w1)
                            .field("s_w2", &self.s_w2)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for IN_ADDR_0_1 {
                    fn eq(&self, other: &Self) -> bool {
                        self.s_w1 == other.s_w1 && self.s_w2 == other.s_w2
                    }
                }
                impl ::std::cmp::Eq for IN_ADDR_0_1 {}
                unsafe impl ::windows::Abi for IN_ADDR_0_1 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = unsafe extern "system" fn(
                    dwerror: u32,
                    cbtransferred: u32,
                    lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                    dwflags: u32,
                );
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct SCOPE_ID {
                    pub Anonymous: SCOPE_ID_0,
                }
                impl SCOPE_ID {}
                impl ::std::default::Default for SCOPE_ID {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for SCOPE_ID {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for SCOPE_ID {}
                unsafe impl ::windows::Abi for SCOPE_ID {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub union SCOPE_ID_0 {
                    pub Anonymous: SCOPE_ID_0_0,
                    pub Value: u32,
                }
                impl SCOPE_ID_0 {}
                impl ::std::default::Default for SCOPE_ID_0 {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for SCOPE_ID_0 {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for SCOPE_ID_0 {}
                unsafe impl ::windows::Abi for SCOPE_ID_0 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct SCOPE_ID_0_0 {
                    pub _bitfield: u32,
                }
                impl SCOPE_ID_0_0 {}
                impl ::std::default::Default for SCOPE_ID_0_0 {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for SCOPE_ID_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("_bitfield", &self._bitfield)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for SCOPE_ID_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self._bitfield == other._bitfield
                    }
                }
                impl ::std::cmp::Eq for SCOPE_ID_0_0 {}
                unsafe impl ::windows::Abi for SCOPE_ID_0_0 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct SOCKADDR {
                    pub sa_family: u16,
                    pub sa_data: [super::super::System::SystemServices::CHAR; 14],
                }
                impl SOCKADDR {}
                impl ::std::default::Default for SOCKADDR {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for SOCKADDR {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("SOCKADDR")
                            .field("sa_family", &self.sa_family)
                            .field("sa_data", &self.sa_data)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for SOCKADDR {
                    fn eq(&self, other: &Self) -> bool {
                        self.sa_family == other.sa_family && self.sa_data == other.sa_data
                    }
                }
                impl ::std::cmp::Eq for SOCKADDR {}
                unsafe impl ::windows::Abi for SOCKADDR {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct SOCKADDR_IN {
                    pub sin_family: u16,
                    pub sin_port: u16,
                    pub sin_addr: IN_ADDR,
                    pub sin_zero: [super::super::System::SystemServices::CHAR; 8],
                }
                impl SOCKADDR_IN {}
                impl ::std::default::Default for SOCKADDR_IN {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for SOCKADDR_IN {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for SOCKADDR_IN {}
                unsafe impl ::windows::Abi for SOCKADDR_IN {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct SOCKADDR_IN6 {
                    pub sin6_family: u16,
                    pub sin6_port: u16,
                    pub sin6_flowinfo: u32,
                    pub sin6_addr: IN6_ADDR,
                    pub Anonymous: SOCKADDR_IN6_0,
                }
                impl SOCKADDR_IN6 {}
                impl ::std::default::Default for SOCKADDR_IN6 {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for SOCKADDR_IN6 {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for SOCKADDR_IN6 {}
                unsafe impl ::windows::Abi for SOCKADDR_IN6 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub union SOCKADDR_IN6_0 {
                    pub sin6_scope_id: u32,
                    pub sin6_scope_struct: SCOPE_ID,
                }
                impl SOCKADDR_IN6_0 {}
                impl ::std::default::Default for SOCKADDR_IN6_0 {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for SOCKADDR_IN6_0 {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for SOCKADDR_IN6_0 {}
                unsafe impl ::windows::Abi for SOCKADDR_IN6_0 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct SOCKADDR_STORAGE {
                    pub ss_family: u16,
                    pub __ss_pad1: [super::super::System::SystemServices::CHAR; 6],
                    pub __ss_align: i64,
                    pub __ss_pad2: [super::super::System::SystemServices::CHAR; 112],
                }
                impl SOCKADDR_STORAGE {}
                impl ::std::default::Default for SOCKADDR_STORAGE {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for SOCKADDR_STORAGE {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("SOCKADDR_STORAGE")
                            .field("ss_family", &self.ss_family)
                            .field("__ss_pad1", &self.__ss_pad1)
                            .field("__ss_align", &self.__ss_align)
                            .field("__ss_pad2", &self.__ss_pad2)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for SOCKADDR_STORAGE {
                    fn eq(&self, other: &Self) -> bool {
                        self.ss_family == other.ss_family
                            && self.__ss_pad1 == other.__ss_pad1
                            && self.__ss_align == other.__ss_align
                            && self.__ss_pad2 == other.__ss_pad2
                    }
                }
                impl ::std::cmp::Eq for SOCKADDR_STORAGE {}
                unsafe impl ::windows::Abi for SOCKADDR_STORAGE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: clone :: Clone,
                    :: std :: marker :: Copy,
                    :: std :: fmt :: Debug,
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                )]
                #[repr(transparent)]
                pub struct SOCKET(pub usize);
                impl ::std::default::Default for SOCKET {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                unsafe impl ::windows::Handle for SOCKET {}
                unsafe impl ::windows::Abi for SOCKET {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                pub const SOCKET_ERROR: i32 = -1i32;
                pub const SOL_SOCKET: u32 = 65535u32;
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct WSABUF {
                    pub len: u32,
                    pub buf: super::super::Foundation::PSTR,
                }
                impl WSABUF {}
                impl ::std::default::Default for WSABUF {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for WSABUF {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("WSABUF")
                            .field("len", &self.len)
                            .field("buf", &self.buf)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for WSABUF {
                    fn eq(&self, other: &Self) -> bool {
                        self.len == other.len && self.buf == other.buf
                    }
                }
                impl ::std::cmp::Eq for WSABUF {}
                unsafe impl ::windows::Abi for WSABUF {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                pub unsafe fn WSAGetLastError() -> WSA_ERROR {
                    #[cfg(windows)]
                    {
                        #[link(name = "ws2_32")]
                        extern "system" {
                            fn WSAGetLastError() -> WSA_ERROR;
                        }
                        ::std::mem::transmute(WSAGetLastError())
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn WSAGetOverlappedResult<'a>(
                    s: impl ::windows::IntoParam<'a, SOCKET>,
                    lpoverlapped: *const super::super::System::SystemServices::OVERLAPPED,
                    lpcbtransfer: *mut u32,
                    fwait: impl ::windows::IntoParam<'a, super::super::Foundation::BOOL>,
                    lpdwflags: *mut u32,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "ws2_32")]
                        extern "system" {
                            fn WSAGetOverlappedResult(
                                s: SOCKET,
                                lpoverlapped : * const super::super::System::SystemServices:: OVERLAPPED,
                                lpcbtransfer: *mut u32,
                                fwait: super::super::Foundation::BOOL,
                                lpdwflags: *mut u32,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(WSAGetOverlappedResult(
                            s.into_param().abi(),
                            ::std::mem::transmute(lpoverlapped),
                            ::std::mem::transmute(lpcbtransfer),
                            fwait.into_param().abi(),
                            ::std::mem::transmute(lpdwflags),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn WSAIoctl<'a>(
                    s: impl ::windows::IntoParam<'a, SOCKET>,
                    dwiocontrolcode: u32,
                    lpvinbuffer: *const ::std::ffi::c_void,
                    cbinbuffer: u32,
                    lpvoutbuffer: *mut ::std::ffi::c_void,
                    cboutbuffer: u32,
                    lpcbbytesreturned: *mut u32,
                    lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                    lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>,
                ) -> i32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "ws2_32")]
                        extern "system" {
                            fn WSAIoctl(
                                s: SOCKET,
                                dwiocontrolcode: u32,
                                lpvinbuffer: *const ::std::ffi::c_void,
                                cbinbuffer: u32,
                                lpvoutbuffer: *mut ::std::ffi::c_void,
                                cboutbuffer: u32,
                                lpcbbytesreturned: *mut u32,
                                lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                                lpcompletionroutine: ::windows::RawPtr,
                            ) -> i32;
                        }
                        ::std::mem::transmute(WSAIoctl(
                            s.into_param().abi(),
                            ::std::mem::transmute(dwiocontrolcode),
                            ::std::mem::transmute(lpvinbuffer),
                            ::std::mem::transmute(cbinbuffer),
                            ::std::mem::transmute(lpvoutbuffer),
                            ::std::mem::transmute(cboutbuffer),
                            ::std::mem::transmute(lpcbbytesreturned),
                            ::std::mem::transmute(lpoverlapped),
                            ::std::mem::transmute(lpcompletionroutine),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn WSARecv<'a>(
                    s: impl ::windows::IntoParam<'a, SOCKET>,
                    lpbuffers: *const WSABUF,
                    dwbuffercount: u32,
                    lpnumberofbytesrecvd: *mut u32,
                    lpflags: *mut u32,
                    lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                    lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>,
                ) -> i32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "ws2_32")]
                        extern "system" {
                            fn WSARecv(
                                s: SOCKET,
                                lpbuffers: *const WSABUF,
                                dwbuffercount: u32,
                                lpnumberofbytesrecvd: *mut u32,
                                lpflags: *mut u32,
                                lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                                lpcompletionroutine: ::windows::RawPtr,
                            ) -> i32;
                        }
                        ::std::mem::transmute(WSARecv(
                            s.into_param().abi(),
                            ::std::mem::transmute(lpbuffers),
                            ::std::mem::transmute(dwbuffercount),
                            ::std::mem::transmute(lpnumberofbytesrecvd),
                            ::std::mem::transmute(lpflags),
                            ::std::mem::transmute(lpoverlapped),
                            ::std::mem::transmute(lpcompletionroutine),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn WSARecvFrom<'a>(
                    s: impl ::windows::IntoParam<'a, SOCKET>,
                    lpbuffers: *const WSABUF,
                    dwbuffercount: u32,
                    lpnumberofbytesrecvd: *mut u32,
                    lpflags: *mut u32,
                    lpfrom: *mut SOCKADDR,
                    lpfromlen: *mut i32,
                    lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                    lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>,
                ) -> i32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "ws2_32")]
                        extern "system" {
                            fn WSARecvFrom(
                                s: SOCKET,
                                lpbuffers: *const WSABUF,
                                dwbuffercount: u32,
                                lpnumberofbytesrecvd: *mut u32,
                                lpflags: *mut u32,
                                lpfrom: *mut SOCKADDR,
                                lpfromlen: *mut i32,
                                lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                                lpcompletionroutine: ::windows::RawPtr,
                            ) -> i32;
                        }
                        ::std::mem::transmute(WSARecvFrom(
                            s.into_param().abi(),
                            ::std::mem::transmute(lpbuffers),
                            ::std::mem::transmute(dwbuffercount),
                            ::std::mem::transmute(lpnumberofbytesrecvd),
                            ::std::mem::transmute(lpflags),
                            ::std::mem::transmute(lpfrom),
                            ::std::mem::transmute(lpfromlen),
                            ::std::mem::transmute(lpoverlapped),
                            ::std::mem::transmute(lpcompletionroutine),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn WSASend<'a>(
                    s: impl ::windows::IntoParam<'a, SOCKET>,
                    lpbuffers: *const WSABUF,
                    dwbuffercount: u32,
                    lpnumberofbytessent: *mut u32,
                    dwflags: u32,
                    lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                    lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>,
                ) -> i32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "ws2_32")]
                        extern "system" {
                            fn WSASend(
                                s: SOCKET,
                                lpbuffers: *const WSABUF,
                                dwbuffercount: u32,
                                lpnumberofbytessent: *mut u32,
                                dwflags: u32,
                                lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                                lpcompletionroutine: ::windows::RawPtr,
                            ) -> i32;
                        }
                        ::std::mem::transmute(WSASend(
                            s.into_param().abi(),
                            ::std::mem::transmute(lpbuffers),
                            ::std::mem::transmute(dwbuffercount),
                            ::std::mem::transmute(lpnumberofbytessent),
                            ::std::mem::transmute(dwflags),
                            ::std::mem::transmute(lpoverlapped),
                            ::std::mem::transmute(lpcompletionroutine),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn WSASendTo<'a>(
                    s: impl ::windows::IntoParam<'a, SOCKET>,
                    lpbuffers: *const WSABUF,
                    dwbuffercount: u32,
                    lpnumberofbytessent: *mut u32,
                    dwflags: u32,
                    lpto: *const SOCKADDR,
                    itolen: i32,
                    lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                    lpcompletionroutine: ::std::option::Option<LPWSAOVERLAPPED_COMPLETION_ROUTINE>,
                ) -> i32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "ws2_32")]
                        extern "system" {
                            fn WSASendTo(
                                s: SOCKET,
                                lpbuffers: *const WSABUF,
                                dwbuffercount: u32,
                                lpnumberofbytessent: *mut u32,
                                dwflags: u32,
                                lpto: *const SOCKADDR,
                                itolen: i32,
                                lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                                lpcompletionroutine: ::windows::RawPtr,
                            ) -> i32;
                        }
                        ::std::mem::transmute(WSASendTo(
                            s.into_param().abi(),
                            ::std::mem::transmute(lpbuffers),
                            ::std::mem::transmute(dwbuffercount),
                            ::std::mem::transmute(lpnumberofbytessent),
                            ::std::mem::transmute(dwflags),
                            ::std::mem::transmute(lpto),
                            ::std::mem::transmute(itolen),
                            ::std::mem::transmute(lpoverlapped),
                            ::std::mem::transmute(lpcompletionroutine),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct WSA_ERROR(pub i32);
                pub const WSA_IO_PENDING: WSA_ERROR = WSA_ERROR(997i32);
                pub const WSA_IO_INCOMPLETE: WSA_ERROR = WSA_ERROR(996i32);
                pub const WSA_INVALID_HANDLE: WSA_ERROR = WSA_ERROR(6i32);
                pub const WSA_INVALID_PARAMETER: WSA_ERROR = WSA_ERROR(87i32);
                pub const WSA_NOT_ENOUGH_MEMORY: WSA_ERROR = WSA_ERROR(8i32);
                pub const WSA_OPERATION_ABORTED: WSA_ERROR = WSA_ERROR(995i32);
                pub const WSABASEERR: WSA_ERROR = WSA_ERROR(10000i32);
                pub const WSAEINTR: WSA_ERROR = WSA_ERROR(10004i32);
                pub const WSAEBADF: WSA_ERROR = WSA_ERROR(10009i32);
                pub const WSAEACCES: WSA_ERROR = WSA_ERROR(10013i32);
                pub const WSAEFAULT: WSA_ERROR = WSA_ERROR(10014i32);
                pub const WSAEINVAL: WSA_ERROR = WSA_ERROR(10022i32);
                pub const WSAEMFILE: WSA_ERROR = WSA_ERROR(10024i32);
                pub const WSAEWOULDBLOCK: WSA_ERROR = WSA_ERROR(10035i32);
                pub const WSAEINPROGRESS: WSA_ERROR = WSA_ERROR(10036i32);
                pub const WSAEALREADY: WSA_ERROR = WSA_ERROR(10037i32);
                pub const WSAENOTSOCK: WSA_ERROR = WSA_ERROR(10038i32);
                pub const WSAEDESTADDRREQ: WSA_ERROR = WSA_ERROR(10039i32);
                pub const WSAEMSGSIZE: WSA_ERROR = WSA_ERROR(10040i32);
                pub const WSAEPROTOTYPE: WSA_ERROR = WSA_ERROR(10041i32);
                pub const WSAENOPROTOOPT: WSA_ERROR = WSA_ERROR(10042i32);
                pub const WSAEPROTONOSUPPORT: WSA_ERROR = WSA_ERROR(10043i32);
                pub const WSAESOCKTNOSUPPORT: WSA_ERROR = WSA_ERROR(10044i32);
                pub const WSAEOPNOTSUPP: WSA_ERROR = WSA_ERROR(10045i32);
                pub const WSAEPFNOSUPPORT: WSA_ERROR = WSA_ERROR(10046i32);
                pub const WSAEAFNOSUPPORT: WSA_ERROR = WSA_ERROR(10047i32);
                pub const WSAEADDRINUSE: WSA_ERROR = WSA_ERROR(10048i32);
                pub const WSAEADDRNOTAVAIL: WSA_ERROR = WSA_ERROR(10049i32);
                pub const WSAENETDOWN: WSA_ERROR = WSA_ERROR(10050i32);
                pub const WSAENETUNREACH: WSA_ERROR = WSA_ERROR(10051i32);
                pub const WSAENETRESET: WSA_ERROR = WSA_ERROR(10052i32);
                pub const WSAECONNABORTED: WSA_ERROR = WSA_ERROR(10053i32);
                pub const WSAECONNRESET: WSA_ERROR = WSA_ERROR(10054i32);
                pub const WSAENOBUFS: WSA_ERROR = WSA_ERROR(10055i32);
                pub const WSAEISCONN: WSA_ERROR = WSA_ERROR(10056i32);
                pub const WSAENOTCONN: WSA_ERROR = WSA_ERROR(10057i32);
                pub const WSAESHUTDOWN: WSA_ERROR = WSA_ERROR(10058i32);
                pub const WSAETOOMANYREFS: WSA_ERROR = WSA_ERROR(10059i32);
                pub const WSAETIMEDOUT: WSA_ERROR = WSA_ERROR(10060i32);
                pub const WSAECONNREFUSED: WSA_ERROR = WSA_ERROR(10061i32);
                pub const WSAELOOP: WSA_ERROR = WSA_ERROR(10062i32);
                pub const WSAENAMETOOLONG: WSA_ERROR = WSA_ERROR(10063i32);
                pub const WSAEHOSTDOWN: WSA_ERROR = WSA_ERROR(10064i32);
                pub const WSAEHOSTUNREACH: WSA_ERROR = WSA_ERROR(10065i32);
                pub const WSAENOTEMPTY: WSA_ERROR = WSA_ERROR(10066i32);
                pub const WSAEPROCLIM: WSA_ERROR = WSA_ERROR(10067i32);
                pub const WSAEUSERS: WSA_ERROR = WSA_ERROR(10068i32);
                pub const WSAEDQUOT: WSA_ERROR = WSA_ERROR(10069i32);
                pub const WSAESTALE: WSA_ERROR = WSA_ERROR(10070i32);
                pub const WSAEREMOTE: WSA_ERROR = WSA_ERROR(10071i32);
                pub const WSASYSNOTREADY: WSA_ERROR = WSA_ERROR(10091i32);
                pub const WSAVERNOTSUPPORTED: WSA_ERROR = WSA_ERROR(10092i32);
                pub const WSANOTINITIALISED: WSA_ERROR = WSA_ERROR(10093i32);
                pub const WSAEDISCON: WSA_ERROR = WSA_ERROR(10101i32);
                pub const WSAENOMORE: WSA_ERROR = WSA_ERROR(10102i32);
                pub const WSAECANCELLED: WSA_ERROR = WSA_ERROR(10103i32);
                pub const WSAEINVALIDPROCTABLE: WSA_ERROR = WSA_ERROR(10104i32);
                pub const WSAEINVALIDPROVIDER: WSA_ERROR = WSA_ERROR(10105i32);
                pub const WSAEPROVIDERFAILEDINIT: WSA_ERROR = WSA_ERROR(10106i32);
                pub const WSASYSCALLFAILURE: WSA_ERROR = WSA_ERROR(10107i32);
                pub const WSASERVICE_NOT_FOUND: WSA_ERROR = WSA_ERROR(10108i32);
                pub const WSATYPE_NOT_FOUND: WSA_ERROR = WSA_ERROR(10109i32);
                pub const WSA_E_NO_MORE: WSA_ERROR = WSA_ERROR(10110i32);
                pub const WSA_E_CANCELLED: WSA_ERROR = WSA_ERROR(10111i32);
                pub const WSAEREFUSED: WSA_ERROR = WSA_ERROR(10112i32);
                pub const WSAHOST_NOT_FOUND: WSA_ERROR = WSA_ERROR(11001i32);
                pub const WSATRY_AGAIN: WSA_ERROR = WSA_ERROR(11002i32);
                pub const WSANO_RECOVERY: WSA_ERROR = WSA_ERROR(11003i32);
                pub const WSANO_DATA: WSA_ERROR = WSA_ERROR(11004i32);
                pub const WSA_QOS_RECEIVERS: WSA_ERROR = WSA_ERROR(11005i32);
                pub const WSA_QOS_SENDERS: WSA_ERROR = WSA_ERROR(11006i32);
                pub const WSA_QOS_NO_SENDERS: WSA_ERROR = WSA_ERROR(11007i32);
                pub const WSA_QOS_NO_RECEIVERS: WSA_ERROR = WSA_ERROR(11008i32);
                pub const WSA_QOS_REQUEST_CONFIRMED: WSA_ERROR = WSA_ERROR(11009i32);
                pub const WSA_QOS_ADMISSION_FAILURE: WSA_ERROR = WSA_ERROR(11010i32);
                pub const WSA_QOS_POLICY_FAILURE: WSA_ERROR = WSA_ERROR(11011i32);
                pub const WSA_QOS_BAD_STYLE: WSA_ERROR = WSA_ERROR(11012i32);
                pub const WSA_QOS_BAD_OBJECT: WSA_ERROR = WSA_ERROR(11013i32);
                pub const WSA_QOS_TRAFFIC_CTRL_ERROR: WSA_ERROR = WSA_ERROR(11014i32);
                pub const WSA_QOS_GENERIC_ERROR: WSA_ERROR = WSA_ERROR(11015i32);
                pub const WSA_QOS_ESERVICETYPE: WSA_ERROR = WSA_ERROR(11016i32);
                pub const WSA_QOS_EFLOWSPEC: WSA_ERROR = WSA_ERROR(11017i32);
                pub const WSA_QOS_EPROVSPECBUF: WSA_ERROR = WSA_ERROR(11018i32);
                pub const WSA_QOS_EFILTERSTYLE: WSA_ERROR = WSA_ERROR(11019i32);
                pub const WSA_QOS_EFILTERTYPE: WSA_ERROR = WSA_ERROR(11020i32);
                pub const WSA_QOS_EFILTERCOUNT: WSA_ERROR = WSA_ERROR(11021i32);
                pub const WSA_QOS_EOBJLENGTH: WSA_ERROR = WSA_ERROR(11022i32);
                pub const WSA_QOS_EFLOWCOUNT: WSA_ERROR = WSA_ERROR(11023i32);
                pub const WSA_QOS_EUNKOWNPSOBJ: WSA_ERROR = WSA_ERROR(11024i32);
                pub const WSA_QOS_EPOLICYOBJ: WSA_ERROR = WSA_ERROR(11025i32);
                pub const WSA_QOS_EFLOWDESC: WSA_ERROR = WSA_ERROR(11026i32);
                pub const WSA_QOS_EPSFLOWSPEC: WSA_ERROR = WSA_ERROR(11027i32);
                pub const WSA_QOS_EPSFILTERSPEC: WSA_ERROR = WSA_ERROR(11028i32);
                pub const WSA_QOS_ESDMODEOBJ: WSA_ERROR = WSA_ERROR(11029i32);
                pub const WSA_QOS_ESHAPERATEOBJ: WSA_ERROR = WSA_ERROR(11030i32);
                pub const WSA_QOS_RESERVED_PETYPE: WSA_ERROR = WSA_ERROR(11031i32);
                pub const WSA_SECURE_HOST_NOT_FOUND: WSA_ERROR = WSA_ERROR(11032i32);
                pub const WSA_IPSEC_NAME_POLICY_ERROR: WSA_ERROR = WSA_ERROR(11033i32);
                impl ::std::convert::From<i32> for WSA_ERROR {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for WSA_ERROR {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                pub unsafe fn setsockopt<'a>(
                    s: impl ::windows::IntoParam<'a, SOCKET>,
                    level: i32,
                    optname: i32,
                    optval: impl ::windows::IntoParam<'a, super::super::Foundation::PSTR>,
                    optlen: i32,
                ) -> i32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "ws2_32")]
                        extern "system" {
                            fn setsockopt(
                                s: SOCKET,
                                level: i32,
                                optname: i32,
                                optval: super::super::Foundation::PSTR,
                                optlen: i32,
                            ) -> i32;
                        }
                        ::std::mem::transmute(setsockopt(
                            s.into_param().abi(),
                            ::std::mem::transmute(level),
                            ::std::mem::transmute(optname),
                            optval.into_param().abi(),
                            ::std::mem::transmute(optlen),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Security {
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(C)]
            pub struct SECURITY_ATTRIBUTES {
                pub nLength: u32,
                pub lpSecurityDescriptor: *mut ::std::ffi::c_void,
                pub bInheritHandle: super::Foundation::BOOL,
            }
            impl SECURITY_ATTRIBUTES {}
            impl ::std::default::Default for SECURITY_ATTRIBUTES {
                fn default() -> Self {
                    unsafe { ::std::mem::zeroed() }
                }
            }
            impl ::std::fmt::Debug for SECURITY_ATTRIBUTES {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("SECURITY_ATTRIBUTES")
                        .field("nLength", &self.nLength)
                        .field("lpSecurityDescriptor", &self.lpSecurityDescriptor)
                        .field("bInheritHandle", &self.bInheritHandle)
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for SECURITY_ATTRIBUTES {
                fn eq(&self, other: &Self) -> bool {
                    self.nLength == other.nLength
                        && self.lpSecurityDescriptor == other.lpSecurityDescriptor
                        && self.bInheritHandle == other.bInheritHandle
                }
            }
            impl ::std::cmp::Eq for SECURITY_ATTRIBUTES {}
            unsafe impl ::windows::Abi for SECURITY_ATTRIBUTES {
                type Abi = Self;
                type DefaultType = Self;
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Storage {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod FileSystem {
                pub unsafe fn CreateIoCompletionPort<'a>(
                    filehandle: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                    existingcompletionport: impl ::windows::IntoParam<
                        'a,
                        super::super::Foundation::HANDLE,
                    >,
                    completionkey: usize,
                    numberofconcurrentthreads: u32,
                ) -> super::super::Foundation::HANDLE {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn CreateIoCompletionPort(
                                filehandle: super::super::Foundation::HANDLE,
                                existingcompletionport: super::super::Foundation::HANDLE,
                                completionkey: usize,
                                numberofconcurrentthreads: u32,
                            ) -> super::super::Foundation::HANDLE;
                        }
                        ::std::mem::transmute(CreateIoCompletionPort(
                            filehandle.into_param().abi(),
                            existingcompletionport.into_param().abi(),
                            ::std::mem::transmute(completionkey),
                            ::std::mem::transmute(numberofconcurrentthreads),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct FILE_FLAGS_AND_ATTRIBUTES(pub u32);
                pub const FILE_ATTRIBUTE_READONLY: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(1u32);
                pub const FILE_ATTRIBUTE_HIDDEN: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(2u32);
                pub const FILE_ATTRIBUTE_SYSTEM: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(4u32);
                pub const FILE_ATTRIBUTE_DIRECTORY: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(16u32);
                pub const FILE_ATTRIBUTE_ARCHIVE: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(32u32);
                pub const FILE_ATTRIBUTE_DEVICE: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(64u32);
                pub const FILE_ATTRIBUTE_NORMAL: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(128u32);
                pub const FILE_ATTRIBUTE_TEMPORARY: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(256u32);
                pub const FILE_ATTRIBUTE_SPARSE_FILE: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(512u32);
                pub const FILE_ATTRIBUTE_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(1024u32);
                pub const FILE_ATTRIBUTE_COMPRESSED: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(2048u32);
                pub const FILE_ATTRIBUTE_OFFLINE: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(4096u32);
                pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(8192u32);
                pub const FILE_ATTRIBUTE_ENCRYPTED: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(16384u32);
                pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(32768u32);
                pub const FILE_ATTRIBUTE_VIRTUAL: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(65536u32);
                pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(131072u32);
                pub const FILE_ATTRIBUTE_EA: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(262144u32);
                pub const FILE_ATTRIBUTE_PINNED: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(524288u32);
                pub const FILE_ATTRIBUTE_UNPINNED: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
                pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(262144u32);
                pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(4194304u32);
                pub const FILE_FLAG_WRITE_THROUGH: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(2147483648u32);
                pub const FILE_FLAG_OVERLAPPED: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(1073741824u32);
                pub const FILE_FLAG_NO_BUFFERING: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(536870912u32);
                pub const FILE_FLAG_RANDOM_ACCESS: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(268435456u32);
                pub const FILE_FLAG_SEQUENTIAL_SCAN: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(134217728u32);
                pub const FILE_FLAG_DELETE_ON_CLOSE: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(67108864u32);
                pub const FILE_FLAG_BACKUP_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(33554432u32);
                pub const FILE_FLAG_POSIX_SEMANTICS: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(16777216u32);
                pub const FILE_FLAG_SESSION_AWARE: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(8388608u32);
                pub const FILE_FLAG_OPEN_REPARSE_POINT: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(2097152u32);
                pub const FILE_FLAG_OPEN_NO_RECALL: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
                pub const FILE_FLAG_FIRST_PIPE_INSTANCE: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(524288u32);
                pub const SECURITY_ANONYMOUS: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(0u32);
                pub const SECURITY_IDENTIFICATION: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(65536u32);
                pub const SECURITY_IMPERSONATION: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(131072u32);
                pub const SECURITY_DELEGATION: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(196608u32);
                pub const SECURITY_CONTEXT_TRACKING: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(262144u32);
                pub const SECURITY_EFFECTIVE_ONLY: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(524288u32);
                pub const SECURITY_SQOS_PRESENT: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(1048576u32);
                pub const SECURITY_VALID_SQOS_FLAGS: FILE_FLAGS_AND_ATTRIBUTES =
                    FILE_FLAGS_AND_ATTRIBUTES(2031616u32);
                impl ::std::convert::From<u32> for FILE_FLAGS_AND_ATTRIBUTES {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for FILE_FLAGS_AND_ATTRIBUTES {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for FILE_FLAGS_AND_ATTRIBUTES {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for FILE_FLAGS_AND_ATTRIBUTES {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for FILE_FLAGS_AND_ATTRIBUTES {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for FILE_FLAGS_AND_ATTRIBUTES {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::std::ops::Not for FILE_FLAGS_AND_ATTRIBUTES {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub unsafe fn FlushFileBuffers<'a>(
                    hfile: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn FlushFileBuffers(
                                hfile: super::super::Foundation::HANDLE,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(FlushFileBuffers(hfile.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn GetQueuedCompletionStatus<'a>(
                    completionport: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                    lpnumberofbytestransferred: *mut u32,
                    lpcompletionkey: *mut usize,
                    lpoverlapped: *mut *mut super::super::System::SystemServices::OVERLAPPED,
                    dwmilliseconds: u32,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn GetQueuedCompletionStatus(
                                completionport: super::super::Foundation::HANDLE,
                                lpnumberofbytestransferred: *mut u32,
                                lpcompletionkey: *mut usize,
                                lpoverlapped : * mut * mut super::super::System::SystemServices:: OVERLAPPED,
                                dwmilliseconds: u32,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(GetQueuedCompletionStatus(
                            completionport.into_param().abi(),
                            ::std::mem::transmute(lpnumberofbytestransferred),
                            ::std::mem::transmute(lpcompletionkey),
                            ::std::mem::transmute(lpoverlapped),
                            ::std::mem::transmute(dwmilliseconds),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn GetQueuedCompletionStatusEx<'a>(
                    completionport: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                    lpcompletionportentries: *mut OVERLAPPED_ENTRY,
                    ulcount: u32,
                    ulnumentriesremoved: *mut u32,
                    dwmilliseconds: u32,
                    falertable: impl ::windows::IntoParam<'a, super::super::Foundation::BOOL>,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn GetQueuedCompletionStatusEx(
                                completionport: super::super::Foundation::HANDLE,
                                lpcompletionportentries: *mut OVERLAPPED_ENTRY,
                                ulcount: u32,
                                ulnumentriesremoved: *mut u32,
                                dwmilliseconds: u32,
                                falertable: super::super::Foundation::BOOL,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(GetQueuedCompletionStatusEx(
                            completionport.into_param().abi(),
                            ::std::mem::transmute(lpcompletionportentries),
                            ::std::mem::transmute(ulcount),
                            ::std::mem::transmute(ulnumentriesremoved),
                            ::std::mem::transmute(dwmilliseconds),
                            falertable.into_param().abi(),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct OVERLAPPED_ENTRY {
                    pub lpCompletionKey: usize,
                    pub lpOverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                    pub Internal: usize,
                    pub dwNumberOfBytesTransferred: u32,
                }
                impl OVERLAPPED_ENTRY {}
                impl ::std::default::Default for OVERLAPPED_ENTRY {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for OVERLAPPED_ENTRY {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("OVERLAPPED_ENTRY")
                            .field("lpCompletionKey", &self.lpCompletionKey)
                            .field("lpOverlapped", &self.lpOverlapped)
                            .field("Internal", &self.Internal)
                            .field(
                                "dwNumberOfBytesTransferred",
                                &self.dwNumberOfBytesTransferred,
                            )
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for OVERLAPPED_ENTRY {
                    fn eq(&self, other: &Self) -> bool {
                        self.lpCompletionKey == other.lpCompletionKey
                            && self.lpOverlapped == other.lpOverlapped
                            && self.Internal == other.Internal
                            && self.dwNumberOfBytesTransferred == other.dwNumberOfBytesTransferred
                    }
                }
                impl ::std::cmp::Eq for OVERLAPPED_ENTRY {}
                unsafe impl ::windows::Abi for OVERLAPPED_ENTRY {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                pub unsafe fn PostQueuedCompletionStatus<'a>(
                    completionport: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                    dwnumberofbytestransferred: u32,
                    dwcompletionkey: usize,
                    lpoverlapped: *const super::super::System::SystemServices::OVERLAPPED,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn PostQueuedCompletionStatus(
                                completionport: super::super::Foundation::HANDLE,
                                dwnumberofbytestransferred: u32,
                                dwcompletionkey: usize,
                                lpoverlapped : * const super::super::System::SystemServices:: OVERLAPPED,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(PostQueuedCompletionStatus(
                            completionport.into_param().abi(),
                            ::std::mem::transmute(dwnumberofbytestransferred),
                            ::std::mem::transmute(dwcompletionkey),
                            ::std::mem::transmute(lpoverlapped),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn ReadFile<'a>(
                    hfile: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                    lpbuffer: *mut ::std::ffi::c_void,
                    nnumberofbytestoread: u32,
                    lpnumberofbytesread: *mut u32,
                    lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn ReadFile(
                                hfile: super::super::Foundation::HANDLE,
                                lpbuffer: *mut ::std::ffi::c_void,
                                nnumberofbytestoread: u32,
                                lpnumberofbytesread: *mut u32,
                                lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(ReadFile(
                            hfile.into_param().abi(),
                            ::std::mem::transmute(lpbuffer),
                            ::std::mem::transmute(nnumberofbytestoread),
                            ::std::mem::transmute(lpnumberofbytesread),
                            ::std::mem::transmute(lpoverlapped),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn WriteFile<'a>(
                    hfile: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                    lpbuffer: *const ::std::ffi::c_void,
                    nnumberofbytestowrite: u32,
                    lpnumberofbyteswritten: *mut u32,
                    lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn WriteFile(
                                hfile: super::super::Foundation::HANDLE,
                                lpbuffer: *const ::std::ffi::c_void,
                                nnumberofbytestowrite: u32,
                                lpnumberofbyteswritten: *mut u32,
                                lpoverlapped: *mut super::super::System::SystemServices::OVERLAPPED,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(WriteFile(
                            hfile.into_param().abi(),
                            ::std::mem::transmute(lpbuffer),
                            ::std::mem::transmute(nnumberofbytestowrite),
                            ::std::mem::transmute(lpnumberofbyteswritten),
                            ::std::mem::transmute(lpoverlapped),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod System {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Diagnostics {
                #[allow(
                    unused_variables,
                    non_upper_case_globals,
                    non_snake_case,
                    unused_unsafe,
                    non_camel_case_types,
                    dead_code,
                    clippy::all
                )]
                pub mod Debug {
                    #[derive(
                        :: std :: cmp :: PartialEq,
                        :: std :: cmp :: Eq,
                        :: std :: marker :: Copy,
                        :: std :: clone :: Clone,
                        :: std :: default :: Default,
                        :: std :: fmt :: Debug,
                    )]
                    #[repr(transparent)]
                    pub struct WIN32_ERROR(pub u32);
                    pub const NO_ERROR: WIN32_ERROR = WIN32_ERROR(0u32);
                    pub const ERROR_SUCCESS: WIN32_ERROR = WIN32_ERROR(0u32);
                    pub const ERROR_INVALID_FUNCTION: WIN32_ERROR = WIN32_ERROR(1u32);
                    pub const ERROR_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2u32);
                    pub const ERROR_PATH_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3u32);
                    pub const ERROR_TOO_MANY_OPEN_FILES: WIN32_ERROR = WIN32_ERROR(4u32);
                    pub const ERROR_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(5u32);
                    pub const ERROR_INVALID_HANDLE: WIN32_ERROR = WIN32_ERROR(6u32);
                    pub const ERROR_ARENA_TRASHED: WIN32_ERROR = WIN32_ERROR(7u32);
                    pub const ERROR_NOT_ENOUGH_MEMORY: WIN32_ERROR = WIN32_ERROR(8u32);
                    pub const ERROR_INVALID_BLOCK: WIN32_ERROR = WIN32_ERROR(9u32);
                    pub const ERROR_BAD_ENVIRONMENT: WIN32_ERROR = WIN32_ERROR(10u32);
                    pub const ERROR_BAD_FORMAT: WIN32_ERROR = WIN32_ERROR(11u32);
                    pub const ERROR_INVALID_ACCESS: WIN32_ERROR = WIN32_ERROR(12u32);
                    pub const ERROR_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(13u32);
                    pub const ERROR_OUTOFMEMORY: WIN32_ERROR = WIN32_ERROR(14u32);
                    pub const ERROR_INVALID_DRIVE: WIN32_ERROR = WIN32_ERROR(15u32);
                    pub const ERROR_CURRENT_DIRECTORY: WIN32_ERROR = WIN32_ERROR(16u32);
                    pub const ERROR_NOT_SAME_DEVICE: WIN32_ERROR = WIN32_ERROR(17u32);
                    pub const ERROR_NO_MORE_FILES: WIN32_ERROR = WIN32_ERROR(18u32);
                    pub const ERROR_WRITE_PROTECT: WIN32_ERROR = WIN32_ERROR(19u32);
                    pub const ERROR_BAD_UNIT: WIN32_ERROR = WIN32_ERROR(20u32);
                    pub const ERROR_NOT_READY: WIN32_ERROR = WIN32_ERROR(21u32);
                    pub const ERROR_BAD_COMMAND: WIN32_ERROR = WIN32_ERROR(22u32);
                    pub const ERROR_CRC: WIN32_ERROR = WIN32_ERROR(23u32);
                    pub const ERROR_BAD_LENGTH: WIN32_ERROR = WIN32_ERROR(24u32);
                    pub const ERROR_SEEK: WIN32_ERROR = WIN32_ERROR(25u32);
                    pub const ERROR_NOT_DOS_DISK: WIN32_ERROR = WIN32_ERROR(26u32);
                    pub const ERROR_SECTOR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(27u32);
                    pub const ERROR_OUT_OF_PAPER: WIN32_ERROR = WIN32_ERROR(28u32);
                    pub const ERROR_WRITE_FAULT: WIN32_ERROR = WIN32_ERROR(29u32);
                    pub const ERROR_READ_FAULT: WIN32_ERROR = WIN32_ERROR(30u32);
                    pub const ERROR_GEN_FAILURE: WIN32_ERROR = WIN32_ERROR(31u32);
                    pub const ERROR_SHARING_VIOLATION: WIN32_ERROR = WIN32_ERROR(32u32);
                    pub const ERROR_LOCK_VIOLATION: WIN32_ERROR = WIN32_ERROR(33u32);
                    pub const ERROR_WRONG_DISK: WIN32_ERROR = WIN32_ERROR(34u32);
                    pub const ERROR_SHARING_BUFFER_EXCEEDED: WIN32_ERROR = WIN32_ERROR(36u32);
                    pub const ERROR_HANDLE_EOF: WIN32_ERROR = WIN32_ERROR(38u32);
                    pub const ERROR_HANDLE_DISK_FULL: WIN32_ERROR = WIN32_ERROR(39u32);
                    pub const ERROR_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(50u32);
                    pub const ERROR_REM_NOT_LIST: WIN32_ERROR = WIN32_ERROR(51u32);
                    pub const ERROR_DUP_NAME: WIN32_ERROR = WIN32_ERROR(52u32);
                    pub const ERROR_BAD_NETPATH: WIN32_ERROR = WIN32_ERROR(53u32);
                    pub const ERROR_NETWORK_BUSY: WIN32_ERROR = WIN32_ERROR(54u32);
                    pub const ERROR_DEV_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(55u32);
                    pub const ERROR_TOO_MANY_CMDS: WIN32_ERROR = WIN32_ERROR(56u32);
                    pub const ERROR_ADAP_HDW_ERR: WIN32_ERROR = WIN32_ERROR(57u32);
                    pub const ERROR_BAD_NET_RESP: WIN32_ERROR = WIN32_ERROR(58u32);
                    pub const ERROR_UNEXP_NET_ERR: WIN32_ERROR = WIN32_ERROR(59u32);
                    pub const ERROR_BAD_REM_ADAP: WIN32_ERROR = WIN32_ERROR(60u32);
                    pub const ERROR_PRINTQ_FULL: WIN32_ERROR = WIN32_ERROR(61u32);
                    pub const ERROR_NO_SPOOL_SPACE: WIN32_ERROR = WIN32_ERROR(62u32);
                    pub const ERROR_PRINT_CANCELLED: WIN32_ERROR = WIN32_ERROR(63u32);
                    pub const ERROR_NETNAME_DELETED: WIN32_ERROR = WIN32_ERROR(64u32);
                    pub const ERROR_NETWORK_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(65u32);
                    pub const ERROR_BAD_DEV_TYPE: WIN32_ERROR = WIN32_ERROR(66u32);
                    pub const ERROR_BAD_NET_NAME: WIN32_ERROR = WIN32_ERROR(67u32);
                    pub const ERROR_TOO_MANY_NAMES: WIN32_ERROR = WIN32_ERROR(68u32);
                    pub const ERROR_TOO_MANY_SESS: WIN32_ERROR = WIN32_ERROR(69u32);
                    pub const ERROR_SHARING_PAUSED: WIN32_ERROR = WIN32_ERROR(70u32);
                    pub const ERROR_REQ_NOT_ACCEP: WIN32_ERROR = WIN32_ERROR(71u32);
                    pub const ERROR_REDIR_PAUSED: WIN32_ERROR = WIN32_ERROR(72u32);
                    pub const ERROR_FILE_EXISTS: WIN32_ERROR = WIN32_ERROR(80u32);
                    pub const ERROR_CANNOT_MAKE: WIN32_ERROR = WIN32_ERROR(82u32);
                    pub const ERROR_FAIL_I24: WIN32_ERROR = WIN32_ERROR(83u32);
                    pub const ERROR_OUT_OF_STRUCTURES: WIN32_ERROR = WIN32_ERROR(84u32);
                    pub const ERROR_ALREADY_ASSIGNED: WIN32_ERROR = WIN32_ERROR(85u32);
                    pub const ERROR_INVALID_PASSWORD: WIN32_ERROR = WIN32_ERROR(86u32);
                    pub const ERROR_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(87u32);
                    pub const ERROR_NET_WRITE_FAULT: WIN32_ERROR = WIN32_ERROR(88u32);
                    pub const ERROR_NO_PROC_SLOTS: WIN32_ERROR = WIN32_ERROR(89u32);
                    pub const ERROR_TOO_MANY_SEMAPHORES: WIN32_ERROR = WIN32_ERROR(100u32);
                    pub const ERROR_EXCL_SEM_ALREADY_OWNED: WIN32_ERROR = WIN32_ERROR(101u32);
                    pub const ERROR_SEM_IS_SET: WIN32_ERROR = WIN32_ERROR(102u32);
                    pub const ERROR_TOO_MANY_SEM_REQUESTS: WIN32_ERROR = WIN32_ERROR(103u32);
                    pub const ERROR_INVALID_AT_INTERRUPT_TIME: WIN32_ERROR = WIN32_ERROR(104u32);
                    pub const ERROR_SEM_OWNER_DIED: WIN32_ERROR = WIN32_ERROR(105u32);
                    pub const ERROR_SEM_USER_LIMIT: WIN32_ERROR = WIN32_ERROR(106u32);
                    pub const ERROR_DISK_CHANGE: WIN32_ERROR = WIN32_ERROR(107u32);
                    pub const ERROR_DRIVE_LOCKED: WIN32_ERROR = WIN32_ERROR(108u32);
                    pub const ERROR_BROKEN_PIPE: WIN32_ERROR = WIN32_ERROR(109u32);
                    pub const ERROR_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(110u32);
                    pub const ERROR_BUFFER_OVERFLOW: WIN32_ERROR = WIN32_ERROR(111u32);
                    pub const ERROR_DISK_FULL: WIN32_ERROR = WIN32_ERROR(112u32);
                    pub const ERROR_NO_MORE_SEARCH_HANDLES: WIN32_ERROR = WIN32_ERROR(113u32);
                    pub const ERROR_INVALID_TARGET_HANDLE: WIN32_ERROR = WIN32_ERROR(114u32);
                    pub const ERROR_INVALID_CATEGORY: WIN32_ERROR = WIN32_ERROR(117u32);
                    pub const ERROR_INVALID_VERIFY_SWITCH: WIN32_ERROR = WIN32_ERROR(118u32);
                    pub const ERROR_BAD_DRIVER_LEVEL: WIN32_ERROR = WIN32_ERROR(119u32);
                    pub const ERROR_CALL_NOT_IMPLEMENTED: WIN32_ERROR = WIN32_ERROR(120u32);
                    pub const ERROR_SEM_TIMEOUT: WIN32_ERROR = WIN32_ERROR(121u32);
                    pub const ERROR_INSUFFICIENT_BUFFER: WIN32_ERROR = WIN32_ERROR(122u32);
                    pub const ERROR_INVALID_NAME: WIN32_ERROR = WIN32_ERROR(123u32);
                    pub const ERROR_INVALID_LEVEL: WIN32_ERROR = WIN32_ERROR(124u32);
                    pub const ERROR_NO_VOLUME_LABEL: WIN32_ERROR = WIN32_ERROR(125u32);
                    pub const ERROR_MOD_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(126u32);
                    pub const ERROR_PROC_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(127u32);
                    pub const ERROR_WAIT_NO_CHILDREN: WIN32_ERROR = WIN32_ERROR(128u32);
                    pub const ERROR_CHILD_NOT_COMPLETE: WIN32_ERROR = WIN32_ERROR(129u32);
                    pub const ERROR_DIRECT_ACCESS_HANDLE: WIN32_ERROR = WIN32_ERROR(130u32);
                    pub const ERROR_NEGATIVE_SEEK: WIN32_ERROR = WIN32_ERROR(131u32);
                    pub const ERROR_SEEK_ON_DEVICE: WIN32_ERROR = WIN32_ERROR(132u32);
                    pub const ERROR_IS_JOIN_TARGET: WIN32_ERROR = WIN32_ERROR(133u32);
                    pub const ERROR_IS_JOINED: WIN32_ERROR = WIN32_ERROR(134u32);
                    pub const ERROR_IS_SUBSTED: WIN32_ERROR = WIN32_ERROR(135u32);
                    pub const ERROR_NOT_JOINED: WIN32_ERROR = WIN32_ERROR(136u32);
                    pub const ERROR_NOT_SUBSTED: WIN32_ERROR = WIN32_ERROR(137u32);
                    pub const ERROR_JOIN_TO_JOIN: WIN32_ERROR = WIN32_ERROR(138u32);
                    pub const ERROR_SUBST_TO_SUBST: WIN32_ERROR = WIN32_ERROR(139u32);
                    pub const ERROR_JOIN_TO_SUBST: WIN32_ERROR = WIN32_ERROR(140u32);
                    pub const ERROR_SUBST_TO_JOIN: WIN32_ERROR = WIN32_ERROR(141u32);
                    pub const ERROR_BUSY_DRIVE: WIN32_ERROR = WIN32_ERROR(142u32);
                    pub const ERROR_SAME_DRIVE: WIN32_ERROR = WIN32_ERROR(143u32);
                    pub const ERROR_DIR_NOT_ROOT: WIN32_ERROR = WIN32_ERROR(144u32);
                    pub const ERROR_DIR_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(145u32);
                    pub const ERROR_IS_SUBST_PATH: WIN32_ERROR = WIN32_ERROR(146u32);
                    pub const ERROR_IS_JOIN_PATH: WIN32_ERROR = WIN32_ERROR(147u32);
                    pub const ERROR_PATH_BUSY: WIN32_ERROR = WIN32_ERROR(148u32);
                    pub const ERROR_IS_SUBST_TARGET: WIN32_ERROR = WIN32_ERROR(149u32);
                    pub const ERROR_SYSTEM_TRACE: WIN32_ERROR = WIN32_ERROR(150u32);
                    pub const ERROR_INVALID_EVENT_COUNT: WIN32_ERROR = WIN32_ERROR(151u32);
                    pub const ERROR_TOO_MANY_MUXWAITERS: WIN32_ERROR = WIN32_ERROR(152u32);
                    pub const ERROR_INVALID_LIST_FORMAT: WIN32_ERROR = WIN32_ERROR(153u32);
                    pub const ERROR_LABEL_TOO_LONG: WIN32_ERROR = WIN32_ERROR(154u32);
                    pub const ERROR_TOO_MANY_TCBS: WIN32_ERROR = WIN32_ERROR(155u32);
                    pub const ERROR_SIGNAL_REFUSED: WIN32_ERROR = WIN32_ERROR(156u32);
                    pub const ERROR_DISCARDED: WIN32_ERROR = WIN32_ERROR(157u32);
                    pub const ERROR_NOT_LOCKED: WIN32_ERROR = WIN32_ERROR(158u32);
                    pub const ERROR_BAD_THREADID_ADDR: WIN32_ERROR = WIN32_ERROR(159u32);
                    pub const ERROR_BAD_ARGUMENTS: WIN32_ERROR = WIN32_ERROR(160u32);
                    pub const ERROR_BAD_PATHNAME: WIN32_ERROR = WIN32_ERROR(161u32);
                    pub const ERROR_SIGNAL_PENDING: WIN32_ERROR = WIN32_ERROR(162u32);
                    pub const ERROR_MAX_THRDS_REACHED: WIN32_ERROR = WIN32_ERROR(164u32);
                    pub const ERROR_LOCK_FAILED: WIN32_ERROR = WIN32_ERROR(167u32);
                    pub const ERROR_BUSY: WIN32_ERROR = WIN32_ERROR(170u32);
                    pub const ERROR_DEVICE_SUPPORT_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(171u32);
                    pub const ERROR_CANCEL_VIOLATION: WIN32_ERROR = WIN32_ERROR(173u32);
                    pub const ERROR_ATOMIC_LOCKS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(174u32);
                    pub const ERROR_INVALID_SEGMENT_NUMBER: WIN32_ERROR = WIN32_ERROR(180u32);
                    pub const ERROR_INVALID_ORDINAL: WIN32_ERROR = WIN32_ERROR(182u32);
                    pub const ERROR_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(183u32);
                    pub const ERROR_INVALID_FLAG_NUMBER: WIN32_ERROR = WIN32_ERROR(186u32);
                    pub const ERROR_SEM_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(187u32);
                    pub const ERROR_INVALID_STARTING_CODESEG: WIN32_ERROR = WIN32_ERROR(188u32);
                    pub const ERROR_INVALID_STACKSEG: WIN32_ERROR = WIN32_ERROR(189u32);
                    pub const ERROR_INVALID_MODULETYPE: WIN32_ERROR = WIN32_ERROR(190u32);
                    pub const ERROR_INVALID_EXE_SIGNATURE: WIN32_ERROR = WIN32_ERROR(191u32);
                    pub const ERROR_EXE_MARKED_INVALID: WIN32_ERROR = WIN32_ERROR(192u32);
                    pub const ERROR_BAD_EXE_FORMAT: WIN32_ERROR = WIN32_ERROR(193u32);
                    pub const ERROR_ITERATED_DATA_EXCEEDS_64k: WIN32_ERROR = WIN32_ERROR(194u32);
                    pub const ERROR_INVALID_MINALLOCSIZE: WIN32_ERROR = WIN32_ERROR(195u32);
                    pub const ERROR_DYNLINK_FROM_INVALID_RING: WIN32_ERROR = WIN32_ERROR(196u32);
                    pub const ERROR_IOPL_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(197u32);
                    pub const ERROR_INVALID_SEGDPL: WIN32_ERROR = WIN32_ERROR(198u32);
                    pub const ERROR_AUTODATASEG_EXCEEDS_64k: WIN32_ERROR = WIN32_ERROR(199u32);
                    pub const ERROR_RING2SEG_MUST_BE_MOVABLE: WIN32_ERROR = WIN32_ERROR(200u32);
                    pub const ERROR_RELOC_CHAIN_XEEDS_SEGLIM: WIN32_ERROR = WIN32_ERROR(201u32);
                    pub const ERROR_INFLOOP_IN_RELOC_CHAIN: WIN32_ERROR = WIN32_ERROR(202u32);
                    pub const ERROR_ENVVAR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(203u32);
                    pub const ERROR_NO_SIGNAL_SENT: WIN32_ERROR = WIN32_ERROR(205u32);
                    pub const ERROR_FILENAME_EXCED_RANGE: WIN32_ERROR = WIN32_ERROR(206u32);
                    pub const ERROR_RING2_STACK_IN_USE: WIN32_ERROR = WIN32_ERROR(207u32);
                    pub const ERROR_META_EXPANSION_TOO_LONG: WIN32_ERROR = WIN32_ERROR(208u32);
                    pub const ERROR_INVALID_SIGNAL_NUMBER: WIN32_ERROR = WIN32_ERROR(209u32);
                    pub const ERROR_THREAD_1_INACTIVE: WIN32_ERROR = WIN32_ERROR(210u32);
                    pub const ERROR_LOCKED: WIN32_ERROR = WIN32_ERROR(212u32);
                    pub const ERROR_TOO_MANY_MODULES: WIN32_ERROR = WIN32_ERROR(214u32);
                    pub const ERROR_NESTING_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(215u32);
                    pub const ERROR_EXE_MACHINE_TYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(216u32);
                    pub const ERROR_EXE_CANNOT_MODIFY_SIGNED_BINARY: WIN32_ERROR =
                        WIN32_ERROR(217u32);
                    pub const ERROR_EXE_CANNOT_MODIFY_STRONG_SIGNED_BINARY: WIN32_ERROR =
                        WIN32_ERROR(218u32);
                    pub const ERROR_FILE_CHECKED_OUT: WIN32_ERROR = WIN32_ERROR(220u32);
                    pub const ERROR_CHECKOUT_REQUIRED: WIN32_ERROR = WIN32_ERROR(221u32);
                    pub const ERROR_BAD_FILE_TYPE: WIN32_ERROR = WIN32_ERROR(222u32);
                    pub const ERROR_FILE_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(223u32);
                    pub const ERROR_FORMS_AUTH_REQUIRED: WIN32_ERROR = WIN32_ERROR(224u32);
                    pub const ERROR_VIRUS_INFECTED: WIN32_ERROR = WIN32_ERROR(225u32);
                    pub const ERROR_VIRUS_DELETED: WIN32_ERROR = WIN32_ERROR(226u32);
                    pub const ERROR_PIPE_LOCAL: WIN32_ERROR = WIN32_ERROR(229u32);
                    pub const ERROR_BAD_PIPE: WIN32_ERROR = WIN32_ERROR(230u32);
                    pub const ERROR_PIPE_BUSY: WIN32_ERROR = WIN32_ERROR(231u32);
                    pub const ERROR_NO_DATA: WIN32_ERROR = WIN32_ERROR(232u32);
                    pub const ERROR_PIPE_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(233u32);
                    pub const ERROR_MORE_DATA: WIN32_ERROR = WIN32_ERROR(234u32);
                    pub const ERROR_NO_WORK_DONE: WIN32_ERROR = WIN32_ERROR(235u32);
                    pub const ERROR_VC_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(240u32);
                    pub const ERROR_INVALID_EA_NAME: WIN32_ERROR = WIN32_ERROR(254u32);
                    pub const ERROR_EA_LIST_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(255u32);
                    pub const ERROR_NO_MORE_ITEMS: WIN32_ERROR = WIN32_ERROR(259u32);
                    pub const ERROR_CANNOT_COPY: WIN32_ERROR = WIN32_ERROR(266u32);
                    pub const ERROR_DIRECTORY: WIN32_ERROR = WIN32_ERROR(267u32);
                    pub const ERROR_EAS_DIDNT_FIT: WIN32_ERROR = WIN32_ERROR(275u32);
                    pub const ERROR_EA_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(276u32);
                    pub const ERROR_EA_TABLE_FULL: WIN32_ERROR = WIN32_ERROR(277u32);
                    pub const ERROR_INVALID_EA_HANDLE: WIN32_ERROR = WIN32_ERROR(278u32);
                    pub const ERROR_EAS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(282u32);
                    pub const ERROR_NOT_OWNER: WIN32_ERROR = WIN32_ERROR(288u32);
                    pub const ERROR_TOO_MANY_POSTS: WIN32_ERROR = WIN32_ERROR(298u32);
                    pub const ERROR_PARTIAL_COPY: WIN32_ERROR = WIN32_ERROR(299u32);
                    pub const ERROR_OPLOCK_NOT_GRANTED: WIN32_ERROR = WIN32_ERROR(300u32);
                    pub const ERROR_INVALID_OPLOCK_PROTOCOL: WIN32_ERROR = WIN32_ERROR(301u32);
                    pub const ERROR_DISK_TOO_FRAGMENTED: WIN32_ERROR = WIN32_ERROR(302u32);
                    pub const ERROR_DELETE_PENDING: WIN32_ERROR = WIN32_ERROR(303u32);
                    pub const ERROR_INCOMPATIBLE_WITH_GLOBAL_SHORT_NAME_REGISTRY_SETTING:
                        WIN32_ERROR = WIN32_ERROR(304u32);
                    pub const ERROR_SHORT_NAMES_NOT_ENABLED_ON_VOLUME: WIN32_ERROR =
                        WIN32_ERROR(305u32);
                    pub const ERROR_SECURITY_STREAM_IS_INCONSISTENT: WIN32_ERROR =
                        WIN32_ERROR(306u32);
                    pub const ERROR_INVALID_LOCK_RANGE: WIN32_ERROR = WIN32_ERROR(307u32);
                    pub const ERROR_IMAGE_SUBSYSTEM_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(308u32);
                    pub const ERROR_NOTIFICATION_GUID_ALREADY_DEFINED: WIN32_ERROR =
                        WIN32_ERROR(309u32);
                    pub const ERROR_INVALID_EXCEPTION_HANDLER: WIN32_ERROR = WIN32_ERROR(310u32);
                    pub const ERROR_DUPLICATE_PRIVILEGES: WIN32_ERROR = WIN32_ERROR(311u32);
                    pub const ERROR_NO_RANGES_PROCESSED: WIN32_ERROR = WIN32_ERROR(312u32);
                    pub const ERROR_NOT_ALLOWED_ON_SYSTEM_FILE: WIN32_ERROR = WIN32_ERROR(313u32);
                    pub const ERROR_DISK_RESOURCES_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(314u32);
                    pub const ERROR_INVALID_TOKEN: WIN32_ERROR = WIN32_ERROR(315u32);
                    pub const ERROR_DEVICE_FEATURE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(316u32);
                    pub const ERROR_MR_MID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(317u32);
                    pub const ERROR_SCOPE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(318u32);
                    pub const ERROR_UNDEFINED_SCOPE: WIN32_ERROR = WIN32_ERROR(319u32);
                    pub const ERROR_INVALID_CAP: WIN32_ERROR = WIN32_ERROR(320u32);
                    pub const ERROR_DEVICE_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(321u32);
                    pub const ERROR_DEVICE_NO_RESOURCES: WIN32_ERROR = WIN32_ERROR(322u32);
                    pub const ERROR_DATA_CHECKSUM_ERROR: WIN32_ERROR = WIN32_ERROR(323u32);
                    pub const ERROR_INTERMIXED_KERNEL_EA_OPERATION: WIN32_ERROR =
                        WIN32_ERROR(324u32);
                    pub const ERROR_FILE_LEVEL_TRIM_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(326u32);
                    pub const ERROR_OFFSET_ALIGNMENT_VIOLATION: WIN32_ERROR = WIN32_ERROR(327u32);
                    pub const ERROR_INVALID_FIELD_IN_PARAMETER_LIST: WIN32_ERROR =
                        WIN32_ERROR(328u32);
                    pub const ERROR_OPERATION_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(329u32);
                    pub const ERROR_BAD_DEVICE_PATH: WIN32_ERROR = WIN32_ERROR(330u32);
                    pub const ERROR_TOO_MANY_DESCRIPTORS: WIN32_ERROR = WIN32_ERROR(331u32);
                    pub const ERROR_SCRUB_DATA_DISABLED: WIN32_ERROR = WIN32_ERROR(332u32);
                    pub const ERROR_NOT_REDUNDANT_STORAGE: WIN32_ERROR = WIN32_ERROR(333u32);
                    pub const ERROR_RESIDENT_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(334u32);
                    pub const ERROR_COMPRESSED_FILE_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(335u32);
                    pub const ERROR_DIRECTORY_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(336u32);
                    pub const ERROR_NOT_READ_FROM_COPY: WIN32_ERROR = WIN32_ERROR(337u32);
                    pub const ERROR_FT_WRITE_FAILURE: WIN32_ERROR = WIN32_ERROR(338u32);
                    pub const ERROR_FT_DI_SCAN_REQUIRED: WIN32_ERROR = WIN32_ERROR(339u32);
                    pub const ERROR_INVALID_KERNEL_INFO_VERSION: WIN32_ERROR = WIN32_ERROR(340u32);
                    pub const ERROR_INVALID_PEP_INFO_VERSION: WIN32_ERROR = WIN32_ERROR(341u32);
                    pub const ERROR_OBJECT_NOT_EXTERNALLY_BACKED: WIN32_ERROR = WIN32_ERROR(342u32);
                    pub const ERROR_EXTERNAL_BACKING_PROVIDER_UNKNOWN: WIN32_ERROR =
                        WIN32_ERROR(343u32);
                    pub const ERROR_COMPRESSION_NOT_BENEFICIAL: WIN32_ERROR = WIN32_ERROR(344u32);
                    pub const ERROR_STORAGE_TOPOLOGY_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(345u32);
                    pub const ERROR_BLOCKED_BY_PARENTAL_CONTROLS: WIN32_ERROR = WIN32_ERROR(346u32);
                    pub const ERROR_BLOCK_TOO_MANY_REFERENCES: WIN32_ERROR = WIN32_ERROR(347u32);
                    pub const ERROR_MARKED_TO_DISALLOW_WRITES: WIN32_ERROR = WIN32_ERROR(348u32);
                    pub const ERROR_ENCLAVE_FAILURE: WIN32_ERROR = WIN32_ERROR(349u32);
                    pub const ERROR_FAIL_NOACTION_REBOOT: WIN32_ERROR = WIN32_ERROR(350u32);
                    pub const ERROR_FAIL_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(351u32);
                    pub const ERROR_FAIL_RESTART: WIN32_ERROR = WIN32_ERROR(352u32);
                    pub const ERROR_MAX_SESSIONS_REACHED: WIN32_ERROR = WIN32_ERROR(353u32);
                    pub const ERROR_NETWORK_ACCESS_DENIED_EDP: WIN32_ERROR = WIN32_ERROR(354u32);
                    pub const ERROR_DEVICE_HINT_NAME_BUFFER_TOO_SMALL: WIN32_ERROR =
                        WIN32_ERROR(355u32);
                    pub const ERROR_EDP_POLICY_DENIES_OPERATION: WIN32_ERROR = WIN32_ERROR(356u32);
                    pub const ERROR_EDP_DPL_POLICY_CANT_BE_SATISFIED: WIN32_ERROR =
                        WIN32_ERROR(357u32);
                    pub const ERROR_CLOUD_FILE_SYNC_ROOT_METADATA_CORRUPT: WIN32_ERROR =
                        WIN32_ERROR(358u32);
                    pub const ERROR_DEVICE_IN_MAINTENANCE: WIN32_ERROR = WIN32_ERROR(359u32);
                    pub const ERROR_NOT_SUPPORTED_ON_DAX: WIN32_ERROR = WIN32_ERROR(360u32);
                    pub const ERROR_DAX_MAPPING_EXISTS: WIN32_ERROR = WIN32_ERROR(361u32);
                    pub const ERROR_CLOUD_FILE_PROVIDER_NOT_RUNNING: WIN32_ERROR =
                        WIN32_ERROR(362u32);
                    pub const ERROR_CLOUD_FILE_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(363u32);
                    pub const ERROR_CLOUD_FILE_METADATA_TOO_LARGE: WIN32_ERROR =
                        WIN32_ERROR(364u32);
                    pub const ERROR_CLOUD_FILE_PROPERTY_BLOB_TOO_LARGE: WIN32_ERROR =
                        WIN32_ERROR(365u32);
                    pub const ERROR_CLOUD_FILE_PROPERTY_BLOB_CHECKSUM_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(366u32);
                    pub const ERROR_CHILD_PROCESS_BLOCKED: WIN32_ERROR = WIN32_ERROR(367u32);
                    pub const ERROR_STORAGE_LOST_DATA_PERSISTENCE: WIN32_ERROR =
                        WIN32_ERROR(368u32);
                    pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_UNAVAILABLE: WIN32_ERROR =
                        WIN32_ERROR(369u32);
                    pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_METADATA_CORRUPT: WIN32_ERROR =
                        WIN32_ERROR(370u32);
                    pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_BUSY: WIN32_ERROR =
                        WIN32_ERROR(371u32);
                    pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_PROVIDER_UNKNOWN: WIN32_ERROR =
                        WIN32_ERROR(372u32);
                    pub const ERROR_GDI_HANDLE_LEAK: WIN32_ERROR = WIN32_ERROR(373u32);
                    pub const ERROR_CLOUD_FILE_TOO_MANY_PROPERTY_BLOBS: WIN32_ERROR =
                        WIN32_ERROR(374u32);
                    pub const ERROR_CLOUD_FILE_PROPERTY_VERSION_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(375u32);
                    pub const ERROR_NOT_A_CLOUD_FILE: WIN32_ERROR = WIN32_ERROR(376u32);
                    pub const ERROR_CLOUD_FILE_NOT_IN_SYNC: WIN32_ERROR = WIN32_ERROR(377u32);
                    pub const ERROR_CLOUD_FILE_ALREADY_CONNECTED: WIN32_ERROR = WIN32_ERROR(378u32);
                    pub const ERROR_CLOUD_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(379u32);
                    pub const ERROR_CLOUD_FILE_INVALID_REQUEST: WIN32_ERROR = WIN32_ERROR(380u32);
                    pub const ERROR_CLOUD_FILE_READ_ONLY_VOLUME: WIN32_ERROR = WIN32_ERROR(381u32);
                    pub const ERROR_CLOUD_FILE_CONNECTED_PROVIDER_ONLY: WIN32_ERROR =
                        WIN32_ERROR(382u32);
                    pub const ERROR_CLOUD_FILE_VALIDATION_FAILED: WIN32_ERROR = WIN32_ERROR(383u32);
                    pub const ERROR_SMB1_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(384u32);
                    pub const ERROR_FILE_SYSTEM_VIRTUALIZATION_INVALID_OPERATION: WIN32_ERROR =
                        WIN32_ERROR(385u32);
                    pub const ERROR_CLOUD_FILE_AUTHENTICATION_FAILED: WIN32_ERROR =
                        WIN32_ERROR(386u32);
                    pub const ERROR_CLOUD_FILE_INSUFFICIENT_RESOURCES: WIN32_ERROR =
                        WIN32_ERROR(387u32);
                    pub const ERROR_CLOUD_FILE_NETWORK_UNAVAILABLE: WIN32_ERROR =
                        WIN32_ERROR(388u32);
                    pub const ERROR_CLOUD_FILE_UNSUCCESSFUL: WIN32_ERROR = WIN32_ERROR(389u32);
                    pub const ERROR_CLOUD_FILE_NOT_UNDER_SYNC_ROOT: WIN32_ERROR =
                        WIN32_ERROR(390u32);
                    pub const ERROR_CLOUD_FILE_IN_USE: WIN32_ERROR = WIN32_ERROR(391u32);
                    pub const ERROR_CLOUD_FILE_PINNED: WIN32_ERROR = WIN32_ERROR(392u32);
                    pub const ERROR_CLOUD_FILE_REQUEST_ABORTED: WIN32_ERROR = WIN32_ERROR(393u32);
                    pub const ERROR_CLOUD_FILE_PROPERTY_CORRUPT: WIN32_ERROR = WIN32_ERROR(394u32);
                    pub const ERROR_CLOUD_FILE_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(395u32);
                    pub const ERROR_CLOUD_FILE_INCOMPATIBLE_HARDLINKS: WIN32_ERROR =
                        WIN32_ERROR(396u32);
                    pub const ERROR_CLOUD_FILE_PROPERTY_LOCK_CONFLICT: WIN32_ERROR =
                        WIN32_ERROR(397u32);
                    pub const ERROR_CLOUD_FILE_REQUEST_CANCELED: WIN32_ERROR = WIN32_ERROR(398u32);
                    pub const ERROR_EXTERNAL_SYSKEY_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(399u32);
                    pub const ERROR_THREAD_MODE_ALREADY_BACKGROUND: WIN32_ERROR =
                        WIN32_ERROR(400u32);
                    pub const ERROR_THREAD_MODE_NOT_BACKGROUND: WIN32_ERROR = WIN32_ERROR(401u32);
                    pub const ERROR_PROCESS_MODE_ALREADY_BACKGROUND: WIN32_ERROR =
                        WIN32_ERROR(402u32);
                    pub const ERROR_PROCESS_MODE_NOT_BACKGROUND: WIN32_ERROR = WIN32_ERROR(403u32);
                    pub const ERROR_CLOUD_FILE_PROVIDER_TERMINATED: WIN32_ERROR =
                        WIN32_ERROR(404u32);
                    pub const ERROR_NOT_A_CLOUD_SYNC_ROOT: WIN32_ERROR = WIN32_ERROR(405u32);
                    pub const ERROR_FILE_PROTECTED_UNDER_DPL: WIN32_ERROR = WIN32_ERROR(406u32);
                    pub const ERROR_VOLUME_NOT_CLUSTER_ALIGNED: WIN32_ERROR = WIN32_ERROR(407u32);
                    pub const ERROR_NO_PHYSICALLY_ALIGNED_FREE_SPACE_FOUND: WIN32_ERROR =
                        WIN32_ERROR(408u32);
                    pub const ERROR_APPX_FILE_NOT_ENCRYPTED: WIN32_ERROR = WIN32_ERROR(409u32);
                    pub const ERROR_RWRAW_ENCRYPTED_FILE_NOT_ENCRYPTED: WIN32_ERROR =
                        WIN32_ERROR(410u32);
                    pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILEOFFSET: WIN32_ERROR =
                        WIN32_ERROR(411u32);
                    pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_FILERANGE: WIN32_ERROR =
                        WIN32_ERROR(412u32);
                    pub const ERROR_RWRAW_ENCRYPTED_INVALID_EDATAINFO_PARAMETER: WIN32_ERROR =
                        WIN32_ERROR(413u32);
                    pub const ERROR_LINUX_SUBSYSTEM_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(414u32);
                    pub const ERROR_FT_READ_FAILURE: WIN32_ERROR = WIN32_ERROR(415u32);
                    pub const ERROR_STORAGE_RESERVE_ID_INVALID: WIN32_ERROR = WIN32_ERROR(416u32);
                    pub const ERROR_STORAGE_RESERVE_DOES_NOT_EXIST: WIN32_ERROR =
                        WIN32_ERROR(417u32);
                    pub const ERROR_STORAGE_RESERVE_ALREADY_EXISTS: WIN32_ERROR =
                        WIN32_ERROR(418u32);
                    pub const ERROR_STORAGE_RESERVE_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(419u32);
                    pub const ERROR_NOT_A_DAX_VOLUME: WIN32_ERROR = WIN32_ERROR(420u32);
                    pub const ERROR_NOT_DAX_MAPPABLE: WIN32_ERROR = WIN32_ERROR(421u32);
                    pub const ERROR_TIME_SENSITIVE_THREAD: WIN32_ERROR = WIN32_ERROR(422u32);
                    pub const ERROR_DPL_NOT_SUPPORTED_FOR_USER: WIN32_ERROR = WIN32_ERROR(423u32);
                    pub const ERROR_CASE_DIFFERING_NAMES_IN_DIR: WIN32_ERROR = WIN32_ERROR(424u32);
                    pub const ERROR_FILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(425u32);
                    pub const ERROR_CLOUD_FILE_REQUEST_TIMEOUT: WIN32_ERROR = WIN32_ERROR(426u32);
                    pub const ERROR_NO_TASK_QUEUE: WIN32_ERROR = WIN32_ERROR(427u32);
                    pub const ERROR_SRC_SRV_DLL_LOAD_FAILED: WIN32_ERROR = WIN32_ERROR(428u32);
                    pub const ERROR_NOT_SUPPORTED_WITH_BTT: WIN32_ERROR = WIN32_ERROR(429u32);
                    pub const ERROR_ENCRYPTION_DISABLED: WIN32_ERROR = WIN32_ERROR(430u32);
                    pub const ERROR_ENCRYPTING_METADATA_DISALLOWED: WIN32_ERROR =
                        WIN32_ERROR(431u32);
                    pub const ERROR_CANT_CLEAR_ENCRYPTION_FLAG: WIN32_ERROR = WIN32_ERROR(432u32);
                    pub const ERROR_NO_SUCH_DEVICE: WIN32_ERROR = WIN32_ERROR(433u32);
                    pub const ERROR_CLOUD_FILE_DEHYDRATION_DISALLOWED: WIN32_ERROR =
                        WIN32_ERROR(434u32);
                    pub const ERROR_FILE_SNAP_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(435u32);
                    pub const ERROR_FILE_SNAP_USER_SECTION_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(436u32);
                    pub const ERROR_FILE_SNAP_MODIFY_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(437u32);
                    pub const ERROR_FILE_SNAP_IO_NOT_COORDINATED: WIN32_ERROR = WIN32_ERROR(438u32);
                    pub const ERROR_FILE_SNAP_UNEXPECTED_ERROR: WIN32_ERROR = WIN32_ERROR(439u32);
                    pub const ERROR_FILE_SNAP_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(440u32);
                    pub const ERROR_UNSATISFIED_DEPENDENCIES: WIN32_ERROR = WIN32_ERROR(441u32);
                    pub const ERROR_CASE_SENSITIVE_PATH: WIN32_ERROR = WIN32_ERROR(442u32);
                    pub const ERROR_UNEXPECTED_NTCACHEMANAGER_ERROR: WIN32_ERROR =
                        WIN32_ERROR(443u32);
                    pub const ERROR_CAPAUTHZ_NOT_DEVUNLOCKED: WIN32_ERROR = WIN32_ERROR(450u32);
                    pub const ERROR_CAPAUTHZ_CHANGE_TYPE: WIN32_ERROR = WIN32_ERROR(451u32);
                    pub const ERROR_CAPAUTHZ_NOT_PROVISIONED: WIN32_ERROR = WIN32_ERROR(452u32);
                    pub const ERROR_CAPAUTHZ_NOT_AUTHORIZED: WIN32_ERROR = WIN32_ERROR(453u32);
                    pub const ERROR_CAPAUTHZ_NO_POLICY: WIN32_ERROR = WIN32_ERROR(454u32);
                    pub const ERROR_CAPAUTHZ_DB_CORRUPTED: WIN32_ERROR = WIN32_ERROR(455u32);
                    pub const ERROR_CAPAUTHZ_SCCD_INVALID_CATALOG: WIN32_ERROR =
                        WIN32_ERROR(456u32);
                    pub const ERROR_CAPAUTHZ_SCCD_NO_AUTH_ENTITY: WIN32_ERROR = WIN32_ERROR(457u32);
                    pub const ERROR_CAPAUTHZ_SCCD_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(458u32);
                    pub const ERROR_CAPAUTHZ_SCCD_DEV_MODE_REQUIRED: WIN32_ERROR =
                        WIN32_ERROR(459u32);
                    pub const ERROR_CAPAUTHZ_SCCD_NO_CAPABILITY_MATCH: WIN32_ERROR =
                        WIN32_ERROR(460u32);
                    pub const ERROR_CIMFS_IMAGE_CORRUPT: WIN32_ERROR = WIN32_ERROR(470u32);
                    pub const ERROR_PNP_QUERY_REMOVE_DEVICE_TIMEOUT: WIN32_ERROR =
                        WIN32_ERROR(480u32);
                    pub const ERROR_PNP_QUERY_REMOVE_RELATED_DEVICE_TIMEOUT: WIN32_ERROR =
                        WIN32_ERROR(481u32);
                    pub const ERROR_PNP_QUERY_REMOVE_UNRELATED_DEVICE_TIMEOUT: WIN32_ERROR =
                        WIN32_ERROR(482u32);
                    pub const ERROR_DEVICE_HARDWARE_ERROR: WIN32_ERROR = WIN32_ERROR(483u32);
                    pub const ERROR_INVALID_ADDRESS: WIN32_ERROR = WIN32_ERROR(487u32);
                    pub const ERROR_VRF_CFG_AND_IO_ENABLED: WIN32_ERROR = WIN32_ERROR(1183u32);
                    pub const ERROR_PARTITION_TERMINATING: WIN32_ERROR = WIN32_ERROR(1184u32);
                    pub const ERROR_USER_PROFILE_LOAD: WIN32_ERROR = WIN32_ERROR(500u32);
                    pub const ERROR_ARITHMETIC_OVERFLOW: WIN32_ERROR = WIN32_ERROR(534u32);
                    pub const ERROR_PIPE_CONNECTED: WIN32_ERROR = WIN32_ERROR(535u32);
                    pub const ERROR_PIPE_LISTENING: WIN32_ERROR = WIN32_ERROR(536u32);
                    pub const ERROR_VERIFIER_STOP: WIN32_ERROR = WIN32_ERROR(537u32);
                    pub const ERROR_ABIOS_ERROR: WIN32_ERROR = WIN32_ERROR(538u32);
                    pub const ERROR_WX86_WARNING: WIN32_ERROR = WIN32_ERROR(539u32);
                    pub const ERROR_WX86_ERROR: WIN32_ERROR = WIN32_ERROR(540u32);
                    pub const ERROR_TIMER_NOT_CANCELED: WIN32_ERROR = WIN32_ERROR(541u32);
                    pub const ERROR_UNWIND: WIN32_ERROR = WIN32_ERROR(542u32);
                    pub const ERROR_BAD_STACK: WIN32_ERROR = WIN32_ERROR(543u32);
                    pub const ERROR_INVALID_UNWIND_TARGET: WIN32_ERROR = WIN32_ERROR(544u32);
                    pub const ERROR_INVALID_PORT_ATTRIBUTES: WIN32_ERROR = WIN32_ERROR(545u32);
                    pub const ERROR_PORT_MESSAGE_TOO_LONG: WIN32_ERROR = WIN32_ERROR(546u32);
                    pub const ERROR_INVALID_QUOTA_LOWER: WIN32_ERROR = WIN32_ERROR(547u32);
                    pub const ERROR_DEVICE_ALREADY_ATTACHED: WIN32_ERROR = WIN32_ERROR(548u32);
                    pub const ERROR_INSTRUCTION_MISALIGNMENT: WIN32_ERROR = WIN32_ERROR(549u32);
                    pub const ERROR_PROFILING_NOT_STARTED: WIN32_ERROR = WIN32_ERROR(550u32);
                    pub const ERROR_PROFILING_NOT_STOPPED: WIN32_ERROR = WIN32_ERROR(551u32);
                    pub const ERROR_COULD_NOT_INTERPRET: WIN32_ERROR = WIN32_ERROR(552u32);
                    pub const ERROR_PROFILING_AT_LIMIT: WIN32_ERROR = WIN32_ERROR(553u32);
                    pub const ERROR_CANT_WAIT: WIN32_ERROR = WIN32_ERROR(554u32);
                    pub const ERROR_CANT_TERMINATE_SELF: WIN32_ERROR = WIN32_ERROR(555u32);
                    pub const ERROR_UNEXPECTED_MM_CREATE_ERR: WIN32_ERROR = WIN32_ERROR(556u32);
                    pub const ERROR_UNEXPECTED_MM_MAP_ERROR: WIN32_ERROR = WIN32_ERROR(557u32);
                    pub const ERROR_UNEXPECTED_MM_EXTEND_ERR: WIN32_ERROR = WIN32_ERROR(558u32);
                    pub const ERROR_BAD_FUNCTION_TABLE: WIN32_ERROR = WIN32_ERROR(559u32);
                    pub const ERROR_NO_GUID_TRANSLATION: WIN32_ERROR = WIN32_ERROR(560u32);
                    pub const ERROR_INVALID_LDT_SIZE: WIN32_ERROR = WIN32_ERROR(561u32);
                    pub const ERROR_INVALID_LDT_OFFSET: WIN32_ERROR = WIN32_ERROR(563u32);
                    pub const ERROR_INVALID_LDT_DESCRIPTOR: WIN32_ERROR = WIN32_ERROR(564u32);
                    pub const ERROR_TOO_MANY_THREADS: WIN32_ERROR = WIN32_ERROR(565u32);
                    pub const ERROR_THREAD_NOT_IN_PROCESS: WIN32_ERROR = WIN32_ERROR(566u32);
                    pub const ERROR_PAGEFILE_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(567u32);
                    pub const ERROR_LOGON_SERVER_CONFLICT: WIN32_ERROR = WIN32_ERROR(568u32);
                    pub const ERROR_SYNCHRONIZATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(569u32);
                    pub const ERROR_NET_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(570u32);
                    pub const ERROR_IO_PRIVILEGE_FAILED: WIN32_ERROR = WIN32_ERROR(571u32);
                    pub const ERROR_CONTROL_C_EXIT: WIN32_ERROR = WIN32_ERROR(572u32);
                    pub const ERROR_MISSING_SYSTEMFILE: WIN32_ERROR = WIN32_ERROR(573u32);
                    pub const ERROR_UNHANDLED_EXCEPTION: WIN32_ERROR = WIN32_ERROR(574u32);
                    pub const ERROR_APP_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(575u32);
                    pub const ERROR_PAGEFILE_CREATE_FAILED: WIN32_ERROR = WIN32_ERROR(576u32);
                    pub const ERROR_INVALID_IMAGE_HASH: WIN32_ERROR = WIN32_ERROR(577u32);
                    pub const ERROR_NO_PAGEFILE: WIN32_ERROR = WIN32_ERROR(578u32);
                    pub const ERROR_ILLEGAL_FLOAT_CONTEXT: WIN32_ERROR = WIN32_ERROR(579u32);
                    pub const ERROR_NO_EVENT_PAIR: WIN32_ERROR = WIN32_ERROR(580u32);
                    pub const ERROR_DOMAIN_CTRLR_CONFIG_ERROR: WIN32_ERROR = WIN32_ERROR(581u32);
                    pub const ERROR_ILLEGAL_CHARACTER: WIN32_ERROR = WIN32_ERROR(582u32);
                    pub const ERROR_UNDEFINED_CHARACTER: WIN32_ERROR = WIN32_ERROR(583u32);
                    pub const ERROR_FLOPPY_VOLUME: WIN32_ERROR = WIN32_ERROR(584u32);
                    pub const ERROR_BIOS_FAILED_TO_CONNECT_INTERRUPT: WIN32_ERROR =
                        WIN32_ERROR(585u32);
                    pub const ERROR_BACKUP_CONTROLLER: WIN32_ERROR = WIN32_ERROR(586u32);
                    pub const ERROR_MUTANT_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(587u32);
                    pub const ERROR_FS_DRIVER_REQUIRED: WIN32_ERROR = WIN32_ERROR(588u32);
                    pub const ERROR_CANNOT_LOAD_REGISTRY_FILE: WIN32_ERROR = WIN32_ERROR(589u32);
                    pub const ERROR_DEBUG_ATTACH_FAILED: WIN32_ERROR = WIN32_ERROR(590u32);
                    pub const ERROR_SYSTEM_PROCESS_TERMINATED: WIN32_ERROR = WIN32_ERROR(591u32);
                    pub const ERROR_DATA_NOT_ACCEPTED: WIN32_ERROR = WIN32_ERROR(592u32);
                    pub const ERROR_VDM_HARD_ERROR: WIN32_ERROR = WIN32_ERROR(593u32);
                    pub const ERROR_DRIVER_CANCEL_TIMEOUT: WIN32_ERROR = WIN32_ERROR(594u32);
                    pub const ERROR_REPLY_MESSAGE_MISMATCH: WIN32_ERROR = WIN32_ERROR(595u32);
                    pub const ERROR_LOST_WRITEBEHIND_DATA: WIN32_ERROR = WIN32_ERROR(596u32);
                    pub const ERROR_CLIENT_SERVER_PARAMETERS_INVALID: WIN32_ERROR =
                        WIN32_ERROR(597u32);
                    pub const ERROR_NOT_TINY_STREAM: WIN32_ERROR = WIN32_ERROR(598u32);
                    pub const ERROR_STACK_OVERFLOW_READ: WIN32_ERROR = WIN32_ERROR(599u32);
                    pub const ERROR_CONVERT_TO_LARGE: WIN32_ERROR = WIN32_ERROR(600u32);
                    pub const ERROR_FOUND_OUT_OF_SCOPE: WIN32_ERROR = WIN32_ERROR(601u32);
                    pub const ERROR_ALLOCATE_BUCKET: WIN32_ERROR = WIN32_ERROR(602u32);
                    pub const ERROR_MARSHALL_OVERFLOW: WIN32_ERROR = WIN32_ERROR(603u32);
                    pub const ERROR_INVALID_VARIANT: WIN32_ERROR = WIN32_ERROR(604u32);
                    pub const ERROR_BAD_COMPRESSION_BUFFER: WIN32_ERROR = WIN32_ERROR(605u32);
                    pub const ERROR_AUDIT_FAILED: WIN32_ERROR = WIN32_ERROR(606u32);
                    pub const ERROR_TIMER_RESOLUTION_NOT_SET: WIN32_ERROR = WIN32_ERROR(607u32);
                    pub const ERROR_INSUFFICIENT_LOGON_INFO: WIN32_ERROR = WIN32_ERROR(608u32);
                    pub const ERROR_BAD_DLL_ENTRYPOINT: WIN32_ERROR = WIN32_ERROR(609u32);
                    pub const ERROR_BAD_SERVICE_ENTRYPOINT: WIN32_ERROR = WIN32_ERROR(610u32);
                    pub const ERROR_IP_ADDRESS_CONFLICT1: WIN32_ERROR = WIN32_ERROR(611u32);
                    pub const ERROR_IP_ADDRESS_CONFLICT2: WIN32_ERROR = WIN32_ERROR(612u32);
                    pub const ERROR_REGISTRY_QUOTA_LIMIT: WIN32_ERROR = WIN32_ERROR(613u32);
                    pub const ERROR_NO_CALLBACK_ACTIVE: WIN32_ERROR = WIN32_ERROR(614u32);
                    pub const ERROR_PWD_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(615u32);
                    pub const ERROR_PWD_TOO_RECENT: WIN32_ERROR = WIN32_ERROR(616u32);
                    pub const ERROR_PWD_HISTORY_CONFLICT: WIN32_ERROR = WIN32_ERROR(617u32);
                    pub const ERROR_UNSUPPORTED_COMPRESSION: WIN32_ERROR = WIN32_ERROR(618u32);
                    pub const ERROR_INVALID_HW_PROFILE: WIN32_ERROR = WIN32_ERROR(619u32);
                    pub const ERROR_INVALID_PLUGPLAY_DEVICE_PATH: WIN32_ERROR = WIN32_ERROR(620u32);
                    pub const ERROR_QUOTA_LIST_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(621u32);
                    pub const ERROR_EVALUATION_EXPIRATION: WIN32_ERROR = WIN32_ERROR(622u32);
                    pub const ERROR_ILLEGAL_DLL_RELOCATION: WIN32_ERROR = WIN32_ERROR(623u32);
                    pub const ERROR_DLL_INIT_FAILED_LOGOFF: WIN32_ERROR = WIN32_ERROR(624u32);
                    pub const ERROR_VALIDATE_CONTINUE: WIN32_ERROR = WIN32_ERROR(625u32);
                    pub const ERROR_NO_MORE_MATCHES: WIN32_ERROR = WIN32_ERROR(626u32);
                    pub const ERROR_RANGE_LIST_CONFLICT: WIN32_ERROR = WIN32_ERROR(627u32);
                    pub const ERROR_SERVER_SID_MISMATCH: WIN32_ERROR = WIN32_ERROR(628u32);
                    pub const ERROR_CANT_ENABLE_DENY_ONLY: WIN32_ERROR = WIN32_ERROR(629u32);
                    pub const ERROR_FLOAT_MULTIPLE_FAULTS: WIN32_ERROR = WIN32_ERROR(630u32);
                    pub const ERROR_FLOAT_MULTIPLE_TRAPS: WIN32_ERROR = WIN32_ERROR(631u32);
                    pub const ERROR_NOINTERFACE: WIN32_ERROR = WIN32_ERROR(632u32);
                    pub const ERROR_DRIVER_FAILED_SLEEP: WIN32_ERROR = WIN32_ERROR(633u32);
                    pub const ERROR_CORRUPT_SYSTEM_FILE: WIN32_ERROR = WIN32_ERROR(634u32);
                    pub const ERROR_COMMITMENT_MINIMUM: WIN32_ERROR = WIN32_ERROR(635u32);
                    pub const ERROR_PNP_RESTART_ENUMERATION: WIN32_ERROR = WIN32_ERROR(636u32);
                    pub const ERROR_SYSTEM_IMAGE_BAD_SIGNATURE: WIN32_ERROR = WIN32_ERROR(637u32);
                    pub const ERROR_PNP_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(638u32);
                    pub const ERROR_INSUFFICIENT_POWER: WIN32_ERROR = WIN32_ERROR(639u32);
                    pub const ERROR_MULTIPLE_FAULT_VIOLATION: WIN32_ERROR = WIN32_ERROR(640u32);
                    pub const ERROR_SYSTEM_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(641u32);
                    pub const ERROR_PORT_NOT_SET: WIN32_ERROR = WIN32_ERROR(642u32);
                    pub const ERROR_DS_VERSION_CHECK_FAILURE: WIN32_ERROR = WIN32_ERROR(643u32);
                    pub const ERROR_RANGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(644u32);
                    pub const ERROR_NOT_SAFE_MODE_DRIVER: WIN32_ERROR = WIN32_ERROR(646u32);
                    pub const ERROR_FAILED_DRIVER_ENTRY: WIN32_ERROR = WIN32_ERROR(647u32);
                    pub const ERROR_DEVICE_ENUMERATION_ERROR: WIN32_ERROR = WIN32_ERROR(648u32);
                    pub const ERROR_MOUNT_POINT_NOT_RESOLVED: WIN32_ERROR = WIN32_ERROR(649u32);
                    pub const ERROR_INVALID_DEVICE_OBJECT_PARAMETER: WIN32_ERROR =
                        WIN32_ERROR(650u32);
                    pub const ERROR_MCA_OCCURED: WIN32_ERROR = WIN32_ERROR(651u32);
                    pub const ERROR_DRIVER_DATABASE_ERROR: WIN32_ERROR = WIN32_ERROR(652u32);
                    pub const ERROR_SYSTEM_HIVE_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(653u32);
                    pub const ERROR_DRIVER_FAILED_PRIOR_UNLOAD: WIN32_ERROR = WIN32_ERROR(654u32);
                    pub const ERROR_VOLSNAP_PREPARE_HIBERNATE: WIN32_ERROR = WIN32_ERROR(655u32);
                    pub const ERROR_HIBERNATION_FAILURE: WIN32_ERROR = WIN32_ERROR(656u32);
                    pub const ERROR_PWD_TOO_LONG: WIN32_ERROR = WIN32_ERROR(657u32);
                    pub const ERROR_FILE_SYSTEM_LIMITATION: WIN32_ERROR = WIN32_ERROR(665u32);
                    pub const ERROR_ASSERTION_FAILURE: WIN32_ERROR = WIN32_ERROR(668u32);
                    pub const ERROR_ACPI_ERROR: WIN32_ERROR = WIN32_ERROR(669u32);
                    pub const ERROR_WOW_ASSERTION: WIN32_ERROR = WIN32_ERROR(670u32);
                    pub const ERROR_PNP_BAD_MPS_TABLE: WIN32_ERROR = WIN32_ERROR(671u32);
                    pub const ERROR_PNP_TRANSLATION_FAILED: WIN32_ERROR = WIN32_ERROR(672u32);
                    pub const ERROR_PNP_IRQ_TRANSLATION_FAILED: WIN32_ERROR = WIN32_ERROR(673u32);
                    pub const ERROR_PNP_INVALID_ID: WIN32_ERROR = WIN32_ERROR(674u32);
                    pub const ERROR_WAKE_SYSTEM_DEBUGGER: WIN32_ERROR = WIN32_ERROR(675u32);
                    pub const ERROR_HANDLES_CLOSED: WIN32_ERROR = WIN32_ERROR(676u32);
                    pub const ERROR_EXTRANEOUS_INFORMATION: WIN32_ERROR = WIN32_ERROR(677u32);
                    pub const ERROR_RXACT_COMMIT_NECESSARY: WIN32_ERROR = WIN32_ERROR(678u32);
                    pub const ERROR_MEDIA_CHECK: WIN32_ERROR = WIN32_ERROR(679u32);
                    pub const ERROR_GUID_SUBSTITUTION_MADE: WIN32_ERROR = WIN32_ERROR(680u32);
                    pub const ERROR_STOPPED_ON_SYMLINK: WIN32_ERROR = WIN32_ERROR(681u32);
                    pub const ERROR_LONGJUMP: WIN32_ERROR = WIN32_ERROR(682u32);
                    pub const ERROR_PLUGPLAY_QUERY_VETOED: WIN32_ERROR = WIN32_ERROR(683u32);
                    pub const ERROR_UNWIND_CONSOLIDATE: WIN32_ERROR = WIN32_ERROR(684u32);
                    pub const ERROR_REGISTRY_HIVE_RECOVERED: WIN32_ERROR = WIN32_ERROR(685u32);
                    pub const ERROR_DLL_MIGHT_BE_INSECURE: WIN32_ERROR = WIN32_ERROR(686u32);
                    pub const ERROR_DLL_MIGHT_BE_INCOMPATIBLE: WIN32_ERROR = WIN32_ERROR(687u32);
                    pub const ERROR_DBG_EXCEPTION_NOT_HANDLED: WIN32_ERROR = WIN32_ERROR(688u32);
                    pub const ERROR_DBG_REPLY_LATER: WIN32_ERROR = WIN32_ERROR(689u32);
                    pub const ERROR_DBG_UNABLE_TO_PROVIDE_HANDLE: WIN32_ERROR = WIN32_ERROR(690u32);
                    pub const ERROR_DBG_TERMINATE_THREAD: WIN32_ERROR = WIN32_ERROR(691u32);
                    pub const ERROR_DBG_TERMINATE_PROCESS: WIN32_ERROR = WIN32_ERROR(692u32);
                    pub const ERROR_DBG_CONTROL_C: WIN32_ERROR = WIN32_ERROR(693u32);
                    pub const ERROR_DBG_PRINTEXCEPTION_C: WIN32_ERROR = WIN32_ERROR(694u32);
                    pub const ERROR_DBG_RIPEXCEPTION: WIN32_ERROR = WIN32_ERROR(695u32);
                    pub const ERROR_DBG_CONTROL_BREAK: WIN32_ERROR = WIN32_ERROR(696u32);
                    pub const ERROR_DBG_COMMAND_EXCEPTION: WIN32_ERROR = WIN32_ERROR(697u32);
                    pub const ERROR_OBJECT_NAME_EXISTS: WIN32_ERROR = WIN32_ERROR(698u32);
                    pub const ERROR_THREAD_WAS_SUSPENDED: WIN32_ERROR = WIN32_ERROR(699u32);
                    pub const ERROR_IMAGE_NOT_AT_BASE: WIN32_ERROR = WIN32_ERROR(700u32);
                    pub const ERROR_RXACT_STATE_CREATED: WIN32_ERROR = WIN32_ERROR(701u32);
                    pub const ERROR_SEGMENT_NOTIFICATION: WIN32_ERROR = WIN32_ERROR(702u32);
                    pub const ERROR_BAD_CURRENT_DIRECTORY: WIN32_ERROR = WIN32_ERROR(703u32);
                    pub const ERROR_FT_READ_RECOVERY_FROM_BACKUP: WIN32_ERROR = WIN32_ERROR(704u32);
                    pub const ERROR_FT_WRITE_RECOVERY: WIN32_ERROR = WIN32_ERROR(705u32);
                    pub const ERROR_IMAGE_MACHINE_TYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(706u32);
                    pub const ERROR_RECEIVE_PARTIAL: WIN32_ERROR = WIN32_ERROR(707u32);
                    pub const ERROR_RECEIVE_EXPEDITED: WIN32_ERROR = WIN32_ERROR(708u32);
                    pub const ERROR_RECEIVE_PARTIAL_EXPEDITED: WIN32_ERROR = WIN32_ERROR(709u32);
                    pub const ERROR_EVENT_DONE: WIN32_ERROR = WIN32_ERROR(710u32);
                    pub const ERROR_EVENT_PENDING: WIN32_ERROR = WIN32_ERROR(711u32);
                    pub const ERROR_CHECKING_FILE_SYSTEM: WIN32_ERROR = WIN32_ERROR(712u32);
                    pub const ERROR_FATAL_APP_EXIT: WIN32_ERROR = WIN32_ERROR(713u32);
                    pub const ERROR_PREDEFINED_HANDLE: WIN32_ERROR = WIN32_ERROR(714u32);
                    pub const ERROR_WAS_UNLOCKED: WIN32_ERROR = WIN32_ERROR(715u32);
                    pub const ERROR_SERVICE_NOTIFICATION: WIN32_ERROR = WIN32_ERROR(716u32);
                    pub const ERROR_WAS_LOCKED: WIN32_ERROR = WIN32_ERROR(717u32);
                    pub const ERROR_LOG_HARD_ERROR: WIN32_ERROR = WIN32_ERROR(718u32);
                    pub const ERROR_ALREADY_WIN32: WIN32_ERROR = WIN32_ERROR(719u32);
                    pub const ERROR_IMAGE_MACHINE_TYPE_MISMATCH_EXE: WIN32_ERROR =
                        WIN32_ERROR(720u32);
                    pub const ERROR_NO_YIELD_PERFORMED: WIN32_ERROR = WIN32_ERROR(721u32);
                    pub const ERROR_TIMER_RESUME_IGNORED: WIN32_ERROR = WIN32_ERROR(722u32);
                    pub const ERROR_ARBITRATION_UNHANDLED: WIN32_ERROR = WIN32_ERROR(723u32);
                    pub const ERROR_CARDBUS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(724u32);
                    pub const ERROR_MP_PROCESSOR_MISMATCH: WIN32_ERROR = WIN32_ERROR(725u32);
                    pub const ERROR_HIBERNATED: WIN32_ERROR = WIN32_ERROR(726u32);
                    pub const ERROR_RESUME_HIBERNATION: WIN32_ERROR = WIN32_ERROR(727u32);
                    pub const ERROR_FIRMWARE_UPDATED: WIN32_ERROR = WIN32_ERROR(728u32);
                    pub const ERROR_DRIVERS_LEAKING_LOCKED_PAGES: WIN32_ERROR = WIN32_ERROR(729u32);
                    pub const ERROR_WAKE_SYSTEM: WIN32_ERROR = WIN32_ERROR(730u32);
                    pub const ERROR_WAIT_1: WIN32_ERROR = WIN32_ERROR(731u32);
                    pub const ERROR_WAIT_2: WIN32_ERROR = WIN32_ERROR(732u32);
                    pub const ERROR_WAIT_3: WIN32_ERROR = WIN32_ERROR(733u32);
                    pub const ERROR_WAIT_63: WIN32_ERROR = WIN32_ERROR(734u32);
                    pub const ERROR_ABANDONED_WAIT_0: WIN32_ERROR = WIN32_ERROR(735u32);
                    pub const ERROR_ABANDONED_WAIT_63: WIN32_ERROR = WIN32_ERROR(736u32);
                    pub const ERROR_USER_APC: WIN32_ERROR = WIN32_ERROR(737u32);
                    pub const ERROR_KERNEL_APC: WIN32_ERROR = WIN32_ERROR(738u32);
                    pub const ERROR_ALERTED: WIN32_ERROR = WIN32_ERROR(739u32);
                    pub const ERROR_ELEVATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(740u32);
                    pub const ERROR_REPARSE: WIN32_ERROR = WIN32_ERROR(741u32);
                    pub const ERROR_OPLOCK_BREAK_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(742u32);
                    pub const ERROR_VOLUME_MOUNTED: WIN32_ERROR = WIN32_ERROR(743u32);
                    pub const ERROR_RXACT_COMMITTED: WIN32_ERROR = WIN32_ERROR(744u32);
                    pub const ERROR_NOTIFY_CLEANUP: WIN32_ERROR = WIN32_ERROR(745u32);
                    pub const ERROR_PRIMARY_TRANSPORT_CONNECT_FAILED: WIN32_ERROR =
                        WIN32_ERROR(746u32);
                    pub const ERROR_PAGE_FAULT_TRANSITION: WIN32_ERROR = WIN32_ERROR(747u32);
                    pub const ERROR_PAGE_FAULT_DEMAND_ZERO: WIN32_ERROR = WIN32_ERROR(748u32);
                    pub const ERROR_PAGE_FAULT_COPY_ON_WRITE: WIN32_ERROR = WIN32_ERROR(749u32);
                    pub const ERROR_PAGE_FAULT_GUARD_PAGE: WIN32_ERROR = WIN32_ERROR(750u32);
                    pub const ERROR_PAGE_FAULT_PAGING_FILE: WIN32_ERROR = WIN32_ERROR(751u32);
                    pub const ERROR_CACHE_PAGE_LOCKED: WIN32_ERROR = WIN32_ERROR(752u32);
                    pub const ERROR_CRASH_DUMP: WIN32_ERROR = WIN32_ERROR(753u32);
                    pub const ERROR_BUFFER_ALL_ZEROS: WIN32_ERROR = WIN32_ERROR(754u32);
                    pub const ERROR_REPARSE_OBJECT: WIN32_ERROR = WIN32_ERROR(755u32);
                    pub const ERROR_RESOURCE_REQUIREMENTS_CHANGED: WIN32_ERROR =
                        WIN32_ERROR(756u32);
                    pub const ERROR_TRANSLATION_COMPLETE: WIN32_ERROR = WIN32_ERROR(757u32);
                    pub const ERROR_NOTHING_TO_TERMINATE: WIN32_ERROR = WIN32_ERROR(758u32);
                    pub const ERROR_PROCESS_NOT_IN_JOB: WIN32_ERROR = WIN32_ERROR(759u32);
                    pub const ERROR_PROCESS_IN_JOB: WIN32_ERROR = WIN32_ERROR(760u32);
                    pub const ERROR_VOLSNAP_HIBERNATE_READY: WIN32_ERROR = WIN32_ERROR(761u32);
                    pub const ERROR_FSFILTER_OP_COMPLETED_SUCCESSFULLY: WIN32_ERROR =
                        WIN32_ERROR(762u32);
                    pub const ERROR_INTERRUPT_VECTOR_ALREADY_CONNECTED: WIN32_ERROR =
                        WIN32_ERROR(763u32);
                    pub const ERROR_INTERRUPT_STILL_CONNECTED: WIN32_ERROR = WIN32_ERROR(764u32);
                    pub const ERROR_WAIT_FOR_OPLOCK: WIN32_ERROR = WIN32_ERROR(765u32);
                    pub const ERROR_DBG_EXCEPTION_HANDLED: WIN32_ERROR = WIN32_ERROR(766u32);
                    pub const ERROR_DBG_CONTINUE: WIN32_ERROR = WIN32_ERROR(767u32);
                    pub const ERROR_CALLBACK_POP_STACK: WIN32_ERROR = WIN32_ERROR(768u32);
                    pub const ERROR_COMPRESSION_DISABLED: WIN32_ERROR = WIN32_ERROR(769u32);
                    pub const ERROR_CANTFETCHBACKWARDS: WIN32_ERROR = WIN32_ERROR(770u32);
                    pub const ERROR_CANTSCROLLBACKWARDS: WIN32_ERROR = WIN32_ERROR(771u32);
                    pub const ERROR_ROWSNOTRELEASED: WIN32_ERROR = WIN32_ERROR(772u32);
                    pub const ERROR_BAD_ACCESSOR_FLAGS: WIN32_ERROR = WIN32_ERROR(773u32);
                    pub const ERROR_ERRORS_ENCOUNTERED: WIN32_ERROR = WIN32_ERROR(774u32);
                    pub const ERROR_NOT_CAPABLE: WIN32_ERROR = WIN32_ERROR(775u32);
                    pub const ERROR_REQUEST_OUT_OF_SEQUENCE: WIN32_ERROR = WIN32_ERROR(776u32);
                    pub const ERROR_VERSION_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(777u32);
                    pub const ERROR_BADSTARTPOSITION: WIN32_ERROR = WIN32_ERROR(778u32);
                    pub const ERROR_MEMORY_HARDWARE: WIN32_ERROR = WIN32_ERROR(779u32);
                    pub const ERROR_DISK_REPAIR_DISABLED: WIN32_ERROR = WIN32_ERROR(780u32);
                    pub const ERROR_INSUFFICIENT_RESOURCE_FOR_SPECIFIED_SHARED_SECTION_SIZE:
                        WIN32_ERROR = WIN32_ERROR(781u32);
                    pub const ERROR_SYSTEM_POWERSTATE_TRANSITION: WIN32_ERROR = WIN32_ERROR(782u32);
                    pub const ERROR_SYSTEM_POWERSTATE_COMPLEX_TRANSITION: WIN32_ERROR =
                        WIN32_ERROR(783u32);
                    pub const ERROR_MCA_EXCEPTION: WIN32_ERROR = WIN32_ERROR(784u32);
                    pub const ERROR_ACCESS_AUDIT_BY_POLICY: WIN32_ERROR = WIN32_ERROR(785u32);
                    pub const ERROR_ACCESS_DISABLED_NO_SAFER_UI_BY_POLICY: WIN32_ERROR =
                        WIN32_ERROR(786u32);
                    pub const ERROR_ABANDON_HIBERFILE: WIN32_ERROR = WIN32_ERROR(787u32);
                    pub const ERROR_LOST_WRITEBEHIND_DATA_NETWORK_DISCONNECTED: WIN32_ERROR =
                        WIN32_ERROR(788u32);
                    pub const ERROR_LOST_WRITEBEHIND_DATA_NETWORK_SERVER_ERROR: WIN32_ERROR =
                        WIN32_ERROR(789u32);
                    pub const ERROR_LOST_WRITEBEHIND_DATA_LOCAL_DISK_ERROR: WIN32_ERROR =
                        WIN32_ERROR(790u32);
                    pub const ERROR_BAD_MCFG_TABLE: WIN32_ERROR = WIN32_ERROR(791u32);
                    pub const ERROR_DISK_REPAIR_REDIRECTED: WIN32_ERROR = WIN32_ERROR(792u32);
                    pub const ERROR_DISK_REPAIR_UNSUCCESSFUL: WIN32_ERROR = WIN32_ERROR(793u32);
                    pub const ERROR_CORRUPT_LOG_OVERFULL: WIN32_ERROR = WIN32_ERROR(794u32);
                    pub const ERROR_CORRUPT_LOG_CORRUPTED: WIN32_ERROR = WIN32_ERROR(795u32);
                    pub const ERROR_CORRUPT_LOG_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(796u32);
                    pub const ERROR_CORRUPT_LOG_DELETED_FULL: WIN32_ERROR = WIN32_ERROR(797u32);
                    pub const ERROR_CORRUPT_LOG_CLEARED: WIN32_ERROR = WIN32_ERROR(798u32);
                    pub const ERROR_ORPHAN_NAME_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(799u32);
                    pub const ERROR_OPLOCK_SWITCHED_TO_NEW_HANDLE: WIN32_ERROR =
                        WIN32_ERROR(800u32);
                    pub const ERROR_CANNOT_GRANT_REQUESTED_OPLOCK: WIN32_ERROR =
                        WIN32_ERROR(801u32);
                    pub const ERROR_CANNOT_BREAK_OPLOCK: WIN32_ERROR = WIN32_ERROR(802u32);
                    pub const ERROR_OPLOCK_HANDLE_CLOSED: WIN32_ERROR = WIN32_ERROR(803u32);
                    pub const ERROR_NO_ACE_CONDITION: WIN32_ERROR = WIN32_ERROR(804u32);
                    pub const ERROR_INVALID_ACE_CONDITION: WIN32_ERROR = WIN32_ERROR(805u32);
                    pub const ERROR_FILE_HANDLE_REVOKED: WIN32_ERROR = WIN32_ERROR(806u32);
                    pub const ERROR_IMAGE_AT_DIFFERENT_BASE: WIN32_ERROR = WIN32_ERROR(807u32);
                    pub const ERROR_ENCRYPTED_IO_NOT_POSSIBLE: WIN32_ERROR = WIN32_ERROR(808u32);
                    pub const ERROR_FILE_METADATA_OPTIMIZATION_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(809u32);
                    pub const ERROR_QUOTA_ACTIVITY: WIN32_ERROR = WIN32_ERROR(810u32);
                    pub const ERROR_HANDLE_REVOKED: WIN32_ERROR = WIN32_ERROR(811u32);
                    pub const ERROR_CALLBACK_INVOKE_INLINE: WIN32_ERROR = WIN32_ERROR(812u32);
                    pub const ERROR_CPU_SET_INVALID: WIN32_ERROR = WIN32_ERROR(813u32);
                    pub const ERROR_ENCLAVE_NOT_TERMINATED: WIN32_ERROR = WIN32_ERROR(814u32);
                    pub const ERROR_ENCLAVE_VIOLATION: WIN32_ERROR = WIN32_ERROR(815u32);
                    pub const ERROR_EA_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(994u32);
                    pub const ERROR_OPERATION_ABORTED: WIN32_ERROR = WIN32_ERROR(995u32);
                    pub const ERROR_IO_INCOMPLETE: WIN32_ERROR = WIN32_ERROR(996u32);
                    pub const ERROR_IO_PENDING: WIN32_ERROR = WIN32_ERROR(997u32);
                    pub const ERROR_NOACCESS: WIN32_ERROR = WIN32_ERROR(998u32);
                    pub const ERROR_SWAPERROR: WIN32_ERROR = WIN32_ERROR(999u32);
                    pub const ERROR_STACK_OVERFLOW: WIN32_ERROR = WIN32_ERROR(1001u32);
                    pub const ERROR_INVALID_MESSAGE: WIN32_ERROR = WIN32_ERROR(1002u32);
                    pub const ERROR_CAN_NOT_COMPLETE: WIN32_ERROR = WIN32_ERROR(1003u32);
                    pub const ERROR_INVALID_FLAGS: WIN32_ERROR = WIN32_ERROR(1004u32);
                    pub const ERROR_UNRECOGNIZED_VOLUME: WIN32_ERROR = WIN32_ERROR(1005u32);
                    pub const ERROR_FILE_INVALID: WIN32_ERROR = WIN32_ERROR(1006u32);
                    pub const ERROR_FULLSCREEN_MODE: WIN32_ERROR = WIN32_ERROR(1007u32);
                    pub const ERROR_NO_TOKEN: WIN32_ERROR = WIN32_ERROR(1008u32);
                    pub const ERROR_BADDB: WIN32_ERROR = WIN32_ERROR(1009u32);
                    pub const ERROR_BADKEY: WIN32_ERROR = WIN32_ERROR(1010u32);
                    pub const ERROR_CANTOPEN: WIN32_ERROR = WIN32_ERROR(1011u32);
                    pub const ERROR_CANTREAD: WIN32_ERROR = WIN32_ERROR(1012u32);
                    pub const ERROR_CANTWRITE: WIN32_ERROR = WIN32_ERROR(1013u32);
                    pub const ERROR_REGISTRY_RECOVERED: WIN32_ERROR = WIN32_ERROR(1014u32);
                    pub const ERROR_REGISTRY_CORRUPT: WIN32_ERROR = WIN32_ERROR(1015u32);
                    pub const ERROR_REGISTRY_IO_FAILED: WIN32_ERROR = WIN32_ERROR(1016u32);
                    pub const ERROR_NOT_REGISTRY_FILE: WIN32_ERROR = WIN32_ERROR(1017u32);
                    pub const ERROR_KEY_DELETED: WIN32_ERROR = WIN32_ERROR(1018u32);
                    pub const ERROR_NO_LOG_SPACE: WIN32_ERROR = WIN32_ERROR(1019u32);
                    pub const ERROR_KEY_HAS_CHILDREN: WIN32_ERROR = WIN32_ERROR(1020u32);
                    pub const ERROR_CHILD_MUST_BE_VOLATILE: WIN32_ERROR = WIN32_ERROR(1021u32);
                    pub const ERROR_NOTIFY_ENUM_DIR: WIN32_ERROR = WIN32_ERROR(1022u32);
                    pub const ERROR_DEPENDENT_SERVICES_RUNNING: WIN32_ERROR = WIN32_ERROR(1051u32);
                    pub const ERROR_INVALID_SERVICE_CONTROL: WIN32_ERROR = WIN32_ERROR(1052u32);
                    pub const ERROR_SERVICE_REQUEST_TIMEOUT: WIN32_ERROR = WIN32_ERROR(1053u32);
                    pub const ERROR_SERVICE_NO_THREAD: WIN32_ERROR = WIN32_ERROR(1054u32);
                    pub const ERROR_SERVICE_DATABASE_LOCKED: WIN32_ERROR = WIN32_ERROR(1055u32);
                    pub const ERROR_SERVICE_ALREADY_RUNNING: WIN32_ERROR = WIN32_ERROR(1056u32);
                    pub const ERROR_INVALID_SERVICE_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1057u32);
                    pub const ERROR_SERVICE_DISABLED: WIN32_ERROR = WIN32_ERROR(1058u32);
                    pub const ERROR_CIRCULAR_DEPENDENCY: WIN32_ERROR = WIN32_ERROR(1059u32);
                    pub const ERROR_SERVICE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(1060u32);
                    pub const ERROR_SERVICE_CANNOT_ACCEPT_CTRL: WIN32_ERROR = WIN32_ERROR(1061u32);
                    pub const ERROR_SERVICE_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(1062u32);
                    pub const ERROR_FAILED_SERVICE_CONTROLLER_CONNECT: WIN32_ERROR =
                        WIN32_ERROR(1063u32);
                    pub const ERROR_EXCEPTION_IN_SERVICE: WIN32_ERROR = WIN32_ERROR(1064u32);
                    pub const ERROR_DATABASE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(1065u32);
                    pub const ERROR_SERVICE_SPECIFIC_ERROR: WIN32_ERROR = WIN32_ERROR(1066u32);
                    pub const ERROR_PROCESS_ABORTED: WIN32_ERROR = WIN32_ERROR(1067u32);
                    pub const ERROR_SERVICE_DEPENDENCY_FAIL: WIN32_ERROR = WIN32_ERROR(1068u32);
                    pub const ERROR_SERVICE_LOGON_FAILED: WIN32_ERROR = WIN32_ERROR(1069u32);
                    pub const ERROR_SERVICE_START_HANG: WIN32_ERROR = WIN32_ERROR(1070u32);
                    pub const ERROR_INVALID_SERVICE_LOCK: WIN32_ERROR = WIN32_ERROR(1071u32);
                    pub const ERROR_SERVICE_MARKED_FOR_DELETE: WIN32_ERROR = WIN32_ERROR(1072u32);
                    pub const ERROR_SERVICE_EXISTS: WIN32_ERROR = WIN32_ERROR(1073u32);
                    pub const ERROR_ALREADY_RUNNING_LKG: WIN32_ERROR = WIN32_ERROR(1074u32);
                    pub const ERROR_SERVICE_DEPENDENCY_DELETED: WIN32_ERROR = WIN32_ERROR(1075u32);
                    pub const ERROR_BOOT_ALREADY_ACCEPTED: WIN32_ERROR = WIN32_ERROR(1076u32);
                    pub const ERROR_SERVICE_NEVER_STARTED: WIN32_ERROR = WIN32_ERROR(1077u32);
                    pub const ERROR_DUPLICATE_SERVICE_NAME: WIN32_ERROR = WIN32_ERROR(1078u32);
                    pub const ERROR_DIFFERENT_SERVICE_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1079u32);
                    pub const ERROR_CANNOT_DETECT_DRIVER_FAILURE: WIN32_ERROR =
                        WIN32_ERROR(1080u32);
                    pub const ERROR_CANNOT_DETECT_PROCESS_ABORT: WIN32_ERROR = WIN32_ERROR(1081u32);
                    pub const ERROR_NO_RECOVERY_PROGRAM: WIN32_ERROR = WIN32_ERROR(1082u32);
                    pub const ERROR_SERVICE_NOT_IN_EXE: WIN32_ERROR = WIN32_ERROR(1083u32);
                    pub const ERROR_NOT_SAFEBOOT_SERVICE: WIN32_ERROR = WIN32_ERROR(1084u32);
                    pub const ERROR_END_OF_MEDIA: WIN32_ERROR = WIN32_ERROR(1100u32);
                    pub const ERROR_FILEMARK_DETECTED: WIN32_ERROR = WIN32_ERROR(1101u32);
                    pub const ERROR_BEGINNING_OF_MEDIA: WIN32_ERROR = WIN32_ERROR(1102u32);
                    pub const ERROR_SETMARK_DETECTED: WIN32_ERROR = WIN32_ERROR(1103u32);
                    pub const ERROR_NO_DATA_DETECTED: WIN32_ERROR = WIN32_ERROR(1104u32);
                    pub const ERROR_PARTITION_FAILURE: WIN32_ERROR = WIN32_ERROR(1105u32);
                    pub const ERROR_INVALID_BLOCK_LENGTH: WIN32_ERROR = WIN32_ERROR(1106u32);
                    pub const ERROR_DEVICE_NOT_PARTITIONED: WIN32_ERROR = WIN32_ERROR(1107u32);
                    pub const ERROR_UNABLE_TO_LOCK_MEDIA: WIN32_ERROR = WIN32_ERROR(1108u32);
                    pub const ERROR_UNABLE_TO_UNLOAD_MEDIA: WIN32_ERROR = WIN32_ERROR(1109u32);
                    pub const ERROR_MEDIA_CHANGED: WIN32_ERROR = WIN32_ERROR(1110u32);
                    pub const ERROR_BUS_RESET: WIN32_ERROR = WIN32_ERROR(1111u32);
                    pub const ERROR_NO_MEDIA_IN_DRIVE: WIN32_ERROR = WIN32_ERROR(1112u32);
                    pub const ERROR_NO_UNICODE_TRANSLATION: WIN32_ERROR = WIN32_ERROR(1113u32);
                    pub const ERROR_DLL_INIT_FAILED: WIN32_ERROR = WIN32_ERROR(1114u32);
                    pub const ERROR_SHUTDOWN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1115u32);
                    pub const ERROR_NO_SHUTDOWN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1116u32);
                    pub const ERROR_IO_DEVICE: WIN32_ERROR = WIN32_ERROR(1117u32);
                    pub const ERROR_SERIAL_NO_DEVICE: WIN32_ERROR = WIN32_ERROR(1118u32);
                    pub const ERROR_IRQ_BUSY: WIN32_ERROR = WIN32_ERROR(1119u32);
                    pub const ERROR_MORE_WRITES: WIN32_ERROR = WIN32_ERROR(1120u32);
                    pub const ERROR_COUNTER_TIMEOUT: WIN32_ERROR = WIN32_ERROR(1121u32);
                    pub const ERROR_FLOPPY_ID_MARK_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1122u32);
                    pub const ERROR_FLOPPY_WRONG_CYLINDER: WIN32_ERROR = WIN32_ERROR(1123u32);
                    pub const ERROR_FLOPPY_UNKNOWN_ERROR: WIN32_ERROR = WIN32_ERROR(1124u32);
                    pub const ERROR_FLOPPY_BAD_REGISTERS: WIN32_ERROR = WIN32_ERROR(1125u32);
                    pub const ERROR_DISK_RECALIBRATE_FAILED: WIN32_ERROR = WIN32_ERROR(1126u32);
                    pub const ERROR_DISK_OPERATION_FAILED: WIN32_ERROR = WIN32_ERROR(1127u32);
                    pub const ERROR_DISK_RESET_FAILED: WIN32_ERROR = WIN32_ERROR(1128u32);
                    pub const ERROR_EOM_OVERFLOW: WIN32_ERROR = WIN32_ERROR(1129u32);
                    pub const ERROR_NOT_ENOUGH_SERVER_MEMORY: WIN32_ERROR = WIN32_ERROR(1130u32);
                    pub const ERROR_POSSIBLE_DEADLOCK: WIN32_ERROR = WIN32_ERROR(1131u32);
                    pub const ERROR_MAPPED_ALIGNMENT: WIN32_ERROR = WIN32_ERROR(1132u32);
                    pub const ERROR_SET_POWER_STATE_VETOED: WIN32_ERROR = WIN32_ERROR(1140u32);
                    pub const ERROR_SET_POWER_STATE_FAILED: WIN32_ERROR = WIN32_ERROR(1141u32);
                    pub const ERROR_TOO_MANY_LINKS: WIN32_ERROR = WIN32_ERROR(1142u32);
                    pub const ERROR_OLD_WIN_VERSION: WIN32_ERROR = WIN32_ERROR(1150u32);
                    pub const ERROR_APP_WRONG_OS: WIN32_ERROR = WIN32_ERROR(1151u32);
                    pub const ERROR_SINGLE_INSTANCE_APP: WIN32_ERROR = WIN32_ERROR(1152u32);
                    pub const ERROR_RMODE_APP: WIN32_ERROR = WIN32_ERROR(1153u32);
                    pub const ERROR_INVALID_DLL: WIN32_ERROR = WIN32_ERROR(1154u32);
                    pub const ERROR_NO_ASSOCIATION: WIN32_ERROR = WIN32_ERROR(1155u32);
                    pub const ERROR_DDE_FAIL: WIN32_ERROR = WIN32_ERROR(1156u32);
                    pub const ERROR_DLL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1157u32);
                    pub const ERROR_NO_MORE_USER_HANDLES: WIN32_ERROR = WIN32_ERROR(1158u32);
                    pub const ERROR_MESSAGE_SYNC_ONLY: WIN32_ERROR = WIN32_ERROR(1159u32);
                    pub const ERROR_SOURCE_ELEMENT_EMPTY: WIN32_ERROR = WIN32_ERROR(1160u32);
                    pub const ERROR_DESTINATION_ELEMENT_FULL: WIN32_ERROR = WIN32_ERROR(1161u32);
                    pub const ERROR_ILLEGAL_ELEMENT_ADDRESS: WIN32_ERROR = WIN32_ERROR(1162u32);
                    pub const ERROR_MAGAZINE_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(1163u32);
                    pub const ERROR_DEVICE_REINITIALIZATION_NEEDED: WIN32_ERROR =
                        WIN32_ERROR(1164u32);
                    pub const ERROR_DEVICE_REQUIRES_CLEANING: WIN32_ERROR = WIN32_ERROR(1165u32);
                    pub const ERROR_DEVICE_DOOR_OPEN: WIN32_ERROR = WIN32_ERROR(1166u32);
                    pub const ERROR_DEVICE_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(1167u32);
                    pub const ERROR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1168u32);
                    pub const ERROR_NO_MATCH: WIN32_ERROR = WIN32_ERROR(1169u32);
                    pub const ERROR_SET_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1170u32);
                    pub const ERROR_POINT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1171u32);
                    pub const ERROR_NO_TRACKING_SERVICE: WIN32_ERROR = WIN32_ERROR(1172u32);
                    pub const ERROR_NO_VOLUME_ID: WIN32_ERROR = WIN32_ERROR(1173u32);
                    pub const ERROR_UNABLE_TO_REMOVE_REPLACED: WIN32_ERROR = WIN32_ERROR(1175u32);
                    pub const ERROR_UNABLE_TO_MOVE_REPLACEMENT: WIN32_ERROR = WIN32_ERROR(1176u32);
                    pub const ERROR_UNABLE_TO_MOVE_REPLACEMENT_2: WIN32_ERROR =
                        WIN32_ERROR(1177u32);
                    pub const ERROR_JOURNAL_DELETE_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1178u32);
                    pub const ERROR_JOURNAL_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(1179u32);
                    pub const ERROR_POTENTIAL_FILE_FOUND: WIN32_ERROR = WIN32_ERROR(1180u32);
                    pub const ERROR_JOURNAL_ENTRY_DELETED: WIN32_ERROR = WIN32_ERROR(1181u32);
                    pub const ERROR_SHUTDOWN_IS_SCHEDULED: WIN32_ERROR = WIN32_ERROR(1190u32);
                    pub const ERROR_SHUTDOWN_USERS_LOGGED_ON: WIN32_ERROR = WIN32_ERROR(1191u32);
                    pub const ERROR_BAD_DEVICE: WIN32_ERROR = WIN32_ERROR(1200u32);
                    pub const ERROR_CONNECTION_UNAVAIL: WIN32_ERROR = WIN32_ERROR(1201u32);
                    pub const ERROR_DEVICE_ALREADY_REMEMBERED: WIN32_ERROR = WIN32_ERROR(1202u32);
                    pub const ERROR_NO_NET_OR_BAD_PATH: WIN32_ERROR = WIN32_ERROR(1203u32);
                    pub const ERROR_BAD_PROVIDER: WIN32_ERROR = WIN32_ERROR(1204u32);
                    pub const ERROR_CANNOT_OPEN_PROFILE: WIN32_ERROR = WIN32_ERROR(1205u32);
                    pub const ERROR_BAD_PROFILE: WIN32_ERROR = WIN32_ERROR(1206u32);
                    pub const ERROR_NOT_CONTAINER: WIN32_ERROR = WIN32_ERROR(1207u32);
                    pub const ERROR_EXTENDED_ERROR: WIN32_ERROR = WIN32_ERROR(1208u32);
                    pub const ERROR_INVALID_GROUPNAME: WIN32_ERROR = WIN32_ERROR(1209u32);
                    pub const ERROR_INVALID_COMPUTERNAME: WIN32_ERROR = WIN32_ERROR(1210u32);
                    pub const ERROR_INVALID_EVENTNAME: WIN32_ERROR = WIN32_ERROR(1211u32);
                    pub const ERROR_INVALID_DOMAINNAME: WIN32_ERROR = WIN32_ERROR(1212u32);
                    pub const ERROR_INVALID_SERVICENAME: WIN32_ERROR = WIN32_ERROR(1213u32);
                    pub const ERROR_INVALID_NETNAME: WIN32_ERROR = WIN32_ERROR(1214u32);
                    pub const ERROR_INVALID_SHARENAME: WIN32_ERROR = WIN32_ERROR(1215u32);
                    pub const ERROR_INVALID_PASSWORDNAME: WIN32_ERROR = WIN32_ERROR(1216u32);
                    pub const ERROR_INVALID_MESSAGENAME: WIN32_ERROR = WIN32_ERROR(1217u32);
                    pub const ERROR_INVALID_MESSAGEDEST: WIN32_ERROR = WIN32_ERROR(1218u32);
                    pub const ERROR_SESSION_CREDENTIAL_CONFLICT: WIN32_ERROR = WIN32_ERROR(1219u32);
                    pub const ERROR_REMOTE_SESSION_LIMIT_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(1220u32);
                    pub const ERROR_DUP_DOMAINNAME: WIN32_ERROR = WIN32_ERROR(1221u32);
                    pub const ERROR_NO_NETWORK: WIN32_ERROR = WIN32_ERROR(1222u32);
                    pub const ERROR_CANCELLED: WIN32_ERROR = WIN32_ERROR(1223u32);
                    pub const ERROR_USER_MAPPED_FILE: WIN32_ERROR = WIN32_ERROR(1224u32);
                    pub const ERROR_CONNECTION_REFUSED: WIN32_ERROR = WIN32_ERROR(1225u32);
                    pub const ERROR_GRACEFUL_DISCONNECT: WIN32_ERROR = WIN32_ERROR(1226u32);
                    pub const ERROR_ADDRESS_ALREADY_ASSOCIATED: WIN32_ERROR = WIN32_ERROR(1227u32);
                    pub const ERROR_ADDRESS_NOT_ASSOCIATED: WIN32_ERROR = WIN32_ERROR(1228u32);
                    pub const ERROR_CONNECTION_INVALID: WIN32_ERROR = WIN32_ERROR(1229u32);
                    pub const ERROR_CONNECTION_ACTIVE: WIN32_ERROR = WIN32_ERROR(1230u32);
                    pub const ERROR_NETWORK_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1231u32);
                    pub const ERROR_HOST_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1232u32);
                    pub const ERROR_PROTOCOL_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1233u32);
                    pub const ERROR_PORT_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(1234u32);
                    pub const ERROR_REQUEST_ABORTED: WIN32_ERROR = WIN32_ERROR(1235u32);
                    pub const ERROR_CONNECTION_ABORTED: WIN32_ERROR = WIN32_ERROR(1236u32);
                    pub const ERROR_RETRY: WIN32_ERROR = WIN32_ERROR(1237u32);
                    pub const ERROR_CONNECTION_COUNT_LIMIT: WIN32_ERROR = WIN32_ERROR(1238u32);
                    pub const ERROR_LOGIN_TIME_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1239u32);
                    pub const ERROR_LOGIN_WKSTA_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1240u32);
                    pub const ERROR_INCORRECT_ADDRESS: WIN32_ERROR = WIN32_ERROR(1241u32);
                    pub const ERROR_ALREADY_REGISTERED: WIN32_ERROR = WIN32_ERROR(1242u32);
                    pub const ERROR_SERVICE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1243u32);
                    pub const ERROR_NOT_AUTHENTICATED: WIN32_ERROR = WIN32_ERROR(1244u32);
                    pub const ERROR_NOT_LOGGED_ON: WIN32_ERROR = WIN32_ERROR(1245u32);
                    pub const ERROR_CONTINUE: WIN32_ERROR = WIN32_ERROR(1246u32);
                    pub const ERROR_ALREADY_INITIALIZED: WIN32_ERROR = WIN32_ERROR(1247u32);
                    pub const ERROR_NO_MORE_DEVICES: WIN32_ERROR = WIN32_ERROR(1248u32);
                    pub const ERROR_NO_SUCH_SITE: WIN32_ERROR = WIN32_ERROR(1249u32);
                    pub const ERROR_DOMAIN_CONTROLLER_EXISTS: WIN32_ERROR = WIN32_ERROR(1250u32);
                    pub const ERROR_ONLY_IF_CONNECTED: WIN32_ERROR = WIN32_ERROR(1251u32);
                    pub const ERROR_OVERRIDE_NOCHANGES: WIN32_ERROR = WIN32_ERROR(1252u32);
                    pub const ERROR_BAD_USER_PROFILE: WIN32_ERROR = WIN32_ERROR(1253u32);
                    pub const ERROR_NOT_SUPPORTED_ON_SBS: WIN32_ERROR = WIN32_ERROR(1254u32);
                    pub const ERROR_SERVER_SHUTDOWN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(1255u32);
                    pub const ERROR_HOST_DOWN: WIN32_ERROR = WIN32_ERROR(1256u32);
                    pub const ERROR_NON_ACCOUNT_SID: WIN32_ERROR = WIN32_ERROR(1257u32);
                    pub const ERROR_NON_DOMAIN_SID: WIN32_ERROR = WIN32_ERROR(1258u32);
                    pub const ERROR_APPHELP_BLOCK: WIN32_ERROR = WIN32_ERROR(1259u32);
                    pub const ERROR_ACCESS_DISABLED_BY_POLICY: WIN32_ERROR = WIN32_ERROR(1260u32);
                    pub const ERROR_REG_NAT_CONSUMPTION: WIN32_ERROR = WIN32_ERROR(1261u32);
                    pub const ERROR_CSCSHARE_OFFLINE: WIN32_ERROR = WIN32_ERROR(1262u32);
                    pub const ERROR_PKINIT_FAILURE: WIN32_ERROR = WIN32_ERROR(1263u32);
                    pub const ERROR_SMARTCARD_SUBSYSTEM_FAILURE: WIN32_ERROR = WIN32_ERROR(1264u32);
                    pub const ERROR_DOWNGRADE_DETECTED: WIN32_ERROR = WIN32_ERROR(1265u32);
                    pub const ERROR_MACHINE_LOCKED: WIN32_ERROR = WIN32_ERROR(1271u32);
                    pub const ERROR_SMB_GUEST_LOGON_BLOCKED: WIN32_ERROR = WIN32_ERROR(1272u32);
                    pub const ERROR_CALLBACK_SUPPLIED_INVALID_DATA: WIN32_ERROR =
                        WIN32_ERROR(1273u32);
                    pub const ERROR_SYNC_FOREGROUND_REFRESH_REQUIRED: WIN32_ERROR =
                        WIN32_ERROR(1274u32);
                    pub const ERROR_DRIVER_BLOCKED: WIN32_ERROR = WIN32_ERROR(1275u32);
                    pub const ERROR_INVALID_IMPORT_OF_NON_DLL: WIN32_ERROR = WIN32_ERROR(1276u32);
                    pub const ERROR_ACCESS_DISABLED_WEBBLADE: WIN32_ERROR = WIN32_ERROR(1277u32);
                    pub const ERROR_ACCESS_DISABLED_WEBBLADE_TAMPER: WIN32_ERROR =
                        WIN32_ERROR(1278u32);
                    pub const ERROR_RECOVERY_FAILURE: WIN32_ERROR = WIN32_ERROR(1279u32);
                    pub const ERROR_ALREADY_FIBER: WIN32_ERROR = WIN32_ERROR(1280u32);
                    pub const ERROR_ALREADY_THREAD: WIN32_ERROR = WIN32_ERROR(1281u32);
                    pub const ERROR_STACK_BUFFER_OVERRUN: WIN32_ERROR = WIN32_ERROR(1282u32);
                    pub const ERROR_PARAMETER_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1283u32);
                    pub const ERROR_DEBUGGER_INACTIVE: WIN32_ERROR = WIN32_ERROR(1284u32);
                    pub const ERROR_DELAY_LOAD_FAILED: WIN32_ERROR = WIN32_ERROR(1285u32);
                    pub const ERROR_VDM_DISALLOWED: WIN32_ERROR = WIN32_ERROR(1286u32);
                    pub const ERROR_UNIDENTIFIED_ERROR: WIN32_ERROR = WIN32_ERROR(1287u32);
                    pub const ERROR_INVALID_CRUNTIME_PARAMETER: WIN32_ERROR = WIN32_ERROR(1288u32);
                    pub const ERROR_BEYOND_VDL: WIN32_ERROR = WIN32_ERROR(1289u32);
                    pub const ERROR_INCOMPATIBLE_SERVICE_SID_TYPE: WIN32_ERROR =
                        WIN32_ERROR(1290u32);
                    pub const ERROR_DRIVER_PROCESS_TERMINATED: WIN32_ERROR = WIN32_ERROR(1291u32);
                    pub const ERROR_IMPLEMENTATION_LIMIT: WIN32_ERROR = WIN32_ERROR(1292u32);
                    pub const ERROR_PROCESS_IS_PROTECTED: WIN32_ERROR = WIN32_ERROR(1293u32);
                    pub const ERROR_SERVICE_NOTIFY_CLIENT_LAGGING: WIN32_ERROR =
                        WIN32_ERROR(1294u32);
                    pub const ERROR_DISK_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1295u32);
                    pub const ERROR_CONTENT_BLOCKED: WIN32_ERROR = WIN32_ERROR(1296u32);
                    pub const ERROR_INCOMPATIBLE_SERVICE_PRIVILEGE: WIN32_ERROR =
                        WIN32_ERROR(1297u32);
                    pub const ERROR_APP_HANG: WIN32_ERROR = WIN32_ERROR(1298u32);
                    pub const ERROR_INVALID_LABEL: WIN32_ERROR = WIN32_ERROR(1299u32);
                    pub const ERROR_NOT_ALL_ASSIGNED: WIN32_ERROR = WIN32_ERROR(1300u32);
                    pub const ERROR_SOME_NOT_MAPPED: WIN32_ERROR = WIN32_ERROR(1301u32);
                    pub const ERROR_NO_QUOTAS_FOR_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1302u32);
                    pub const ERROR_LOCAL_USER_SESSION_KEY: WIN32_ERROR = WIN32_ERROR(1303u32);
                    pub const ERROR_NULL_LM_PASSWORD: WIN32_ERROR = WIN32_ERROR(1304u32);
                    pub const ERROR_UNKNOWN_REVISION: WIN32_ERROR = WIN32_ERROR(1305u32);
                    pub const ERROR_REVISION_MISMATCH: WIN32_ERROR = WIN32_ERROR(1306u32);
                    pub const ERROR_INVALID_OWNER: WIN32_ERROR = WIN32_ERROR(1307u32);
                    pub const ERROR_INVALID_PRIMARY_GROUP: WIN32_ERROR = WIN32_ERROR(1308u32);
                    pub const ERROR_NO_IMPERSONATION_TOKEN: WIN32_ERROR = WIN32_ERROR(1309u32);
                    pub const ERROR_CANT_DISABLE_MANDATORY: WIN32_ERROR = WIN32_ERROR(1310u32);
                    pub const ERROR_NO_LOGON_SERVERS: WIN32_ERROR = WIN32_ERROR(1311u32);
                    pub const ERROR_NO_SUCH_LOGON_SESSION: WIN32_ERROR = WIN32_ERROR(1312u32);
                    pub const ERROR_NO_SUCH_PRIVILEGE: WIN32_ERROR = WIN32_ERROR(1313u32);
                    pub const ERROR_PRIVILEGE_NOT_HELD: WIN32_ERROR = WIN32_ERROR(1314u32);
                    pub const ERROR_INVALID_ACCOUNT_NAME: WIN32_ERROR = WIN32_ERROR(1315u32);
                    pub const ERROR_USER_EXISTS: WIN32_ERROR = WIN32_ERROR(1316u32);
                    pub const ERROR_NO_SUCH_USER: WIN32_ERROR = WIN32_ERROR(1317u32);
                    pub const ERROR_GROUP_EXISTS: WIN32_ERROR = WIN32_ERROR(1318u32);
                    pub const ERROR_NO_SUCH_GROUP: WIN32_ERROR = WIN32_ERROR(1319u32);
                    pub const ERROR_MEMBER_IN_GROUP: WIN32_ERROR = WIN32_ERROR(1320u32);
                    pub const ERROR_MEMBER_NOT_IN_GROUP: WIN32_ERROR = WIN32_ERROR(1321u32);
                    pub const ERROR_LAST_ADMIN: WIN32_ERROR = WIN32_ERROR(1322u32);
                    pub const ERROR_WRONG_PASSWORD: WIN32_ERROR = WIN32_ERROR(1323u32);
                    pub const ERROR_ILL_FORMED_PASSWORD: WIN32_ERROR = WIN32_ERROR(1324u32);
                    pub const ERROR_PASSWORD_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1325u32);
                    pub const ERROR_LOGON_FAILURE: WIN32_ERROR = WIN32_ERROR(1326u32);
                    pub const ERROR_ACCOUNT_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1327u32);
                    pub const ERROR_INVALID_LOGON_HOURS: WIN32_ERROR = WIN32_ERROR(1328u32);
                    pub const ERROR_INVALID_WORKSTATION: WIN32_ERROR = WIN32_ERROR(1329u32);
                    pub const ERROR_PASSWORD_EXPIRED: WIN32_ERROR = WIN32_ERROR(1330u32);
                    pub const ERROR_ACCOUNT_DISABLED: WIN32_ERROR = WIN32_ERROR(1331u32);
                    pub const ERROR_NONE_MAPPED: WIN32_ERROR = WIN32_ERROR(1332u32);
                    pub const ERROR_TOO_MANY_LUIDS_REQUESTED: WIN32_ERROR = WIN32_ERROR(1333u32);
                    pub const ERROR_LUIDS_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(1334u32);
                    pub const ERROR_INVALID_SUB_AUTHORITY: WIN32_ERROR = WIN32_ERROR(1335u32);
                    pub const ERROR_INVALID_ACL: WIN32_ERROR = WIN32_ERROR(1336u32);
                    pub const ERROR_INVALID_SID: WIN32_ERROR = WIN32_ERROR(1337u32);
                    pub const ERROR_INVALID_SECURITY_DESCR: WIN32_ERROR = WIN32_ERROR(1338u32);
                    pub const ERROR_BAD_INHERITANCE_ACL: WIN32_ERROR = WIN32_ERROR(1340u32);
                    pub const ERROR_SERVER_DISABLED: WIN32_ERROR = WIN32_ERROR(1341u32);
                    pub const ERROR_SERVER_NOT_DISABLED: WIN32_ERROR = WIN32_ERROR(1342u32);
                    pub const ERROR_INVALID_ID_AUTHORITY: WIN32_ERROR = WIN32_ERROR(1343u32);
                    pub const ERROR_ALLOTTED_SPACE_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1344u32);
                    pub const ERROR_INVALID_GROUP_ATTRIBUTES: WIN32_ERROR = WIN32_ERROR(1345u32);
                    pub const ERROR_BAD_IMPERSONATION_LEVEL: WIN32_ERROR = WIN32_ERROR(1346u32);
                    pub const ERROR_CANT_OPEN_ANONYMOUS: WIN32_ERROR = WIN32_ERROR(1347u32);
                    pub const ERROR_BAD_VALIDATION_CLASS: WIN32_ERROR = WIN32_ERROR(1348u32);
                    pub const ERROR_BAD_TOKEN_TYPE: WIN32_ERROR = WIN32_ERROR(1349u32);
                    pub const ERROR_NO_SECURITY_ON_OBJECT: WIN32_ERROR = WIN32_ERROR(1350u32);
                    pub const ERROR_CANT_ACCESS_DOMAIN_INFO: WIN32_ERROR = WIN32_ERROR(1351u32);
                    pub const ERROR_INVALID_SERVER_STATE: WIN32_ERROR = WIN32_ERROR(1352u32);
                    pub const ERROR_INVALID_DOMAIN_STATE: WIN32_ERROR = WIN32_ERROR(1353u32);
                    pub const ERROR_INVALID_DOMAIN_ROLE: WIN32_ERROR = WIN32_ERROR(1354u32);
                    pub const ERROR_NO_SUCH_DOMAIN: WIN32_ERROR = WIN32_ERROR(1355u32);
                    pub const ERROR_DOMAIN_EXISTS: WIN32_ERROR = WIN32_ERROR(1356u32);
                    pub const ERROR_DOMAIN_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1357u32);
                    pub const ERROR_INTERNAL_DB_CORRUPTION: WIN32_ERROR = WIN32_ERROR(1358u32);
                    pub const ERROR_INTERNAL_ERROR: WIN32_ERROR = WIN32_ERROR(1359u32);
                    pub const ERROR_GENERIC_NOT_MAPPED: WIN32_ERROR = WIN32_ERROR(1360u32);
                    pub const ERROR_BAD_DESCRIPTOR_FORMAT: WIN32_ERROR = WIN32_ERROR(1361u32);
                    pub const ERROR_NOT_LOGON_PROCESS: WIN32_ERROR = WIN32_ERROR(1362u32);
                    pub const ERROR_LOGON_SESSION_EXISTS: WIN32_ERROR = WIN32_ERROR(1363u32);
                    pub const ERROR_NO_SUCH_PACKAGE: WIN32_ERROR = WIN32_ERROR(1364u32);
                    pub const ERROR_BAD_LOGON_SESSION_STATE: WIN32_ERROR = WIN32_ERROR(1365u32);
                    pub const ERROR_LOGON_SESSION_COLLISION: WIN32_ERROR = WIN32_ERROR(1366u32);
                    pub const ERROR_INVALID_LOGON_TYPE: WIN32_ERROR = WIN32_ERROR(1367u32);
                    pub const ERROR_CANNOT_IMPERSONATE: WIN32_ERROR = WIN32_ERROR(1368u32);
                    pub const ERROR_RXACT_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(1369u32);
                    pub const ERROR_RXACT_COMMIT_FAILURE: WIN32_ERROR = WIN32_ERROR(1370u32);
                    pub const ERROR_SPECIAL_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1371u32);
                    pub const ERROR_SPECIAL_GROUP: WIN32_ERROR = WIN32_ERROR(1372u32);
                    pub const ERROR_SPECIAL_USER: WIN32_ERROR = WIN32_ERROR(1373u32);
                    pub const ERROR_MEMBERS_PRIMARY_GROUP: WIN32_ERROR = WIN32_ERROR(1374u32);
                    pub const ERROR_TOKEN_ALREADY_IN_USE: WIN32_ERROR = WIN32_ERROR(1375u32);
                    pub const ERROR_NO_SUCH_ALIAS: WIN32_ERROR = WIN32_ERROR(1376u32);
                    pub const ERROR_MEMBER_NOT_IN_ALIAS: WIN32_ERROR = WIN32_ERROR(1377u32);
                    pub const ERROR_MEMBER_IN_ALIAS: WIN32_ERROR = WIN32_ERROR(1378u32);
                    pub const ERROR_ALIAS_EXISTS: WIN32_ERROR = WIN32_ERROR(1379u32);
                    pub const ERROR_LOGON_NOT_GRANTED: WIN32_ERROR = WIN32_ERROR(1380u32);
                    pub const ERROR_TOO_MANY_SECRETS: WIN32_ERROR = WIN32_ERROR(1381u32);
                    pub const ERROR_SECRET_TOO_LONG: WIN32_ERROR = WIN32_ERROR(1382u32);
                    pub const ERROR_INTERNAL_DB_ERROR: WIN32_ERROR = WIN32_ERROR(1383u32);
                    pub const ERROR_TOO_MANY_CONTEXT_IDS: WIN32_ERROR = WIN32_ERROR(1384u32);
                    pub const ERROR_LOGON_TYPE_NOT_GRANTED: WIN32_ERROR = WIN32_ERROR(1385u32);
                    pub const ERROR_NT_CROSS_ENCRYPTION_REQUIRED: WIN32_ERROR =
                        WIN32_ERROR(1386u32);
                    pub const ERROR_NO_SUCH_MEMBER: WIN32_ERROR = WIN32_ERROR(1387u32);
                    pub const ERROR_INVALID_MEMBER: WIN32_ERROR = WIN32_ERROR(1388u32);
                    pub const ERROR_TOO_MANY_SIDS: WIN32_ERROR = WIN32_ERROR(1389u32);
                    pub const ERROR_LM_CROSS_ENCRYPTION_REQUIRED: WIN32_ERROR =
                        WIN32_ERROR(1390u32);
                    pub const ERROR_NO_INHERITANCE: WIN32_ERROR = WIN32_ERROR(1391u32);
                    pub const ERROR_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(1392u32);
                    pub const ERROR_DISK_CORRUPT: WIN32_ERROR = WIN32_ERROR(1393u32);
                    pub const ERROR_NO_USER_SESSION_KEY: WIN32_ERROR = WIN32_ERROR(1394u32);
                    pub const ERROR_LICENSE_QUOTA_EXCEEDED: WIN32_ERROR = WIN32_ERROR(1395u32);
                    pub const ERROR_WRONG_TARGET_NAME: WIN32_ERROR = WIN32_ERROR(1396u32);
                    pub const ERROR_MUTUAL_AUTH_FAILED: WIN32_ERROR = WIN32_ERROR(1397u32);
                    pub const ERROR_TIME_SKEW: WIN32_ERROR = WIN32_ERROR(1398u32);
                    pub const ERROR_CURRENT_DOMAIN_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(1399u32);
                    pub const ERROR_INVALID_WINDOW_HANDLE: WIN32_ERROR = WIN32_ERROR(1400u32);
                    pub const ERROR_INVALID_MENU_HANDLE: WIN32_ERROR = WIN32_ERROR(1401u32);
                    pub const ERROR_INVALID_CURSOR_HANDLE: WIN32_ERROR = WIN32_ERROR(1402u32);
                    pub const ERROR_INVALID_ACCEL_HANDLE: WIN32_ERROR = WIN32_ERROR(1403u32);
                    pub const ERROR_INVALID_HOOK_HANDLE: WIN32_ERROR = WIN32_ERROR(1404u32);
                    pub const ERROR_INVALID_DWP_HANDLE: WIN32_ERROR = WIN32_ERROR(1405u32);
                    pub const ERROR_TLW_WITH_WSCHILD: WIN32_ERROR = WIN32_ERROR(1406u32);
                    pub const ERROR_CANNOT_FIND_WND_CLASS: WIN32_ERROR = WIN32_ERROR(1407u32);
                    pub const ERROR_WINDOW_OF_OTHER_THREAD: WIN32_ERROR = WIN32_ERROR(1408u32);
                    pub const ERROR_HOTKEY_ALREADY_REGISTERED: WIN32_ERROR = WIN32_ERROR(1409u32);
                    pub const ERROR_CLASS_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(1410u32);
                    pub const ERROR_CLASS_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(1411u32);
                    pub const ERROR_CLASS_HAS_WINDOWS: WIN32_ERROR = WIN32_ERROR(1412u32);
                    pub const ERROR_INVALID_INDEX: WIN32_ERROR = WIN32_ERROR(1413u32);
                    pub const ERROR_INVALID_ICON_HANDLE: WIN32_ERROR = WIN32_ERROR(1414u32);
                    pub const ERROR_PRIVATE_DIALOG_INDEX: WIN32_ERROR = WIN32_ERROR(1415u32);
                    pub const ERROR_LISTBOX_ID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1416u32);
                    pub const ERROR_NO_WILDCARD_CHARACTERS: WIN32_ERROR = WIN32_ERROR(1417u32);
                    pub const ERROR_CLIPBOARD_NOT_OPEN: WIN32_ERROR = WIN32_ERROR(1418u32);
                    pub const ERROR_HOTKEY_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(1419u32);
                    pub const ERROR_WINDOW_NOT_DIALOG: WIN32_ERROR = WIN32_ERROR(1420u32);
                    pub const ERROR_CONTROL_ID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1421u32);
                    pub const ERROR_INVALID_COMBOBOX_MESSAGE: WIN32_ERROR = WIN32_ERROR(1422u32);
                    pub const ERROR_WINDOW_NOT_COMBOBOX: WIN32_ERROR = WIN32_ERROR(1423u32);
                    pub const ERROR_INVALID_EDIT_HEIGHT: WIN32_ERROR = WIN32_ERROR(1424u32);
                    pub const ERROR_DC_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1425u32);
                    pub const ERROR_INVALID_HOOK_FILTER: WIN32_ERROR = WIN32_ERROR(1426u32);
                    pub const ERROR_INVALID_FILTER_PROC: WIN32_ERROR = WIN32_ERROR(1427u32);
                    pub const ERROR_HOOK_NEEDS_HMOD: WIN32_ERROR = WIN32_ERROR(1428u32);
                    pub const ERROR_GLOBAL_ONLY_HOOK: WIN32_ERROR = WIN32_ERROR(1429u32);
                    pub const ERROR_JOURNAL_HOOK_SET: WIN32_ERROR = WIN32_ERROR(1430u32);
                    pub const ERROR_HOOK_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(1431u32);
                    pub const ERROR_INVALID_LB_MESSAGE: WIN32_ERROR = WIN32_ERROR(1432u32);
                    pub const ERROR_SETCOUNT_ON_BAD_LB: WIN32_ERROR = WIN32_ERROR(1433u32);
                    pub const ERROR_LB_WITHOUT_TABSTOPS: WIN32_ERROR = WIN32_ERROR(1434u32);
                    pub const ERROR_DESTROY_OBJECT_OF_OTHER_THREAD: WIN32_ERROR =
                        WIN32_ERROR(1435u32);
                    pub const ERROR_CHILD_WINDOW_MENU: WIN32_ERROR = WIN32_ERROR(1436u32);
                    pub const ERROR_NO_SYSTEM_MENU: WIN32_ERROR = WIN32_ERROR(1437u32);
                    pub const ERROR_INVALID_MSGBOX_STYLE: WIN32_ERROR = WIN32_ERROR(1438u32);
                    pub const ERROR_INVALID_SPI_VALUE: WIN32_ERROR = WIN32_ERROR(1439u32);
                    pub const ERROR_SCREEN_ALREADY_LOCKED: WIN32_ERROR = WIN32_ERROR(1440u32);
                    pub const ERROR_HWNDS_HAVE_DIFF_PARENT: WIN32_ERROR = WIN32_ERROR(1441u32);
                    pub const ERROR_NOT_CHILD_WINDOW: WIN32_ERROR = WIN32_ERROR(1442u32);
                    pub const ERROR_INVALID_GW_COMMAND: WIN32_ERROR = WIN32_ERROR(1443u32);
                    pub const ERROR_INVALID_THREAD_ID: WIN32_ERROR = WIN32_ERROR(1444u32);
                    pub const ERROR_NON_MDICHILD_WINDOW: WIN32_ERROR = WIN32_ERROR(1445u32);
                    pub const ERROR_POPUP_ALREADY_ACTIVE: WIN32_ERROR = WIN32_ERROR(1446u32);
                    pub const ERROR_NO_SCROLLBARS: WIN32_ERROR = WIN32_ERROR(1447u32);
                    pub const ERROR_INVALID_SCROLLBAR_RANGE: WIN32_ERROR = WIN32_ERROR(1448u32);
                    pub const ERROR_INVALID_SHOWWIN_COMMAND: WIN32_ERROR = WIN32_ERROR(1449u32);
                    pub const ERROR_NO_SYSTEM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1450u32);
                    pub const ERROR_NONPAGED_SYSTEM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1451u32);
                    pub const ERROR_PAGED_SYSTEM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1452u32);
                    pub const ERROR_WORKING_SET_QUOTA: WIN32_ERROR = WIN32_ERROR(1453u32);
                    pub const ERROR_PAGEFILE_QUOTA: WIN32_ERROR = WIN32_ERROR(1454u32);
                    pub const ERROR_COMMITMENT_LIMIT: WIN32_ERROR = WIN32_ERROR(1455u32);
                    pub const ERROR_MENU_ITEM_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1456u32);
                    pub const ERROR_INVALID_KEYBOARD_HANDLE: WIN32_ERROR = WIN32_ERROR(1457u32);
                    pub const ERROR_HOOK_TYPE_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(1458u32);
                    pub const ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION: WIN32_ERROR =
                        WIN32_ERROR(1459u32);
                    pub const ERROR_TIMEOUT: WIN32_ERROR = WIN32_ERROR(1460u32);
                    pub const ERROR_INVALID_MONITOR_HANDLE: WIN32_ERROR = WIN32_ERROR(1461u32);
                    pub const ERROR_INCORRECT_SIZE: WIN32_ERROR = WIN32_ERROR(1462u32);
                    pub const ERROR_SYMLINK_CLASS_DISABLED: WIN32_ERROR = WIN32_ERROR(1463u32);
                    pub const ERROR_SYMLINK_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(1464u32);
                    pub const ERROR_XML_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(1465u32);
                    pub const ERROR_XMLDSIG_ERROR: WIN32_ERROR = WIN32_ERROR(1466u32);
                    pub const ERROR_RESTART_APPLICATION: WIN32_ERROR = WIN32_ERROR(1467u32);
                    pub const ERROR_WRONG_COMPARTMENT: WIN32_ERROR = WIN32_ERROR(1468u32);
                    pub const ERROR_AUTHIP_FAILURE: WIN32_ERROR = WIN32_ERROR(1469u32);
                    pub const ERROR_NO_NVRAM_RESOURCES: WIN32_ERROR = WIN32_ERROR(1470u32);
                    pub const ERROR_NOT_GUI_PROCESS: WIN32_ERROR = WIN32_ERROR(1471u32);
                    pub const ERROR_EVENTLOG_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(1500u32);
                    pub const ERROR_EVENTLOG_CANT_START: WIN32_ERROR = WIN32_ERROR(1501u32);
                    pub const ERROR_LOG_FILE_FULL: WIN32_ERROR = WIN32_ERROR(1502u32);
                    pub const ERROR_EVENTLOG_FILE_CHANGED: WIN32_ERROR = WIN32_ERROR(1503u32);
                    pub const ERROR_CONTAINER_ASSIGNED: WIN32_ERROR = WIN32_ERROR(1504u32);
                    pub const ERROR_JOB_NO_CONTAINER: WIN32_ERROR = WIN32_ERROR(1505u32);
                    pub const ERROR_INVALID_TASK_NAME: WIN32_ERROR = WIN32_ERROR(1550u32);
                    pub const ERROR_INVALID_TASK_INDEX: WIN32_ERROR = WIN32_ERROR(1551u32);
                    pub const ERROR_THREAD_ALREADY_IN_TASK: WIN32_ERROR = WIN32_ERROR(1552u32);
                    pub const ERROR_INSTALL_SERVICE_FAILURE: WIN32_ERROR = WIN32_ERROR(1601u32);
                    pub const ERROR_INSTALL_USEREXIT: WIN32_ERROR = WIN32_ERROR(1602u32);
                    pub const ERROR_INSTALL_FAILURE: WIN32_ERROR = WIN32_ERROR(1603u32);
                    pub const ERROR_INSTALL_SUSPEND: WIN32_ERROR = WIN32_ERROR(1604u32);
                    pub const ERROR_UNKNOWN_PRODUCT: WIN32_ERROR = WIN32_ERROR(1605u32);
                    pub const ERROR_UNKNOWN_FEATURE: WIN32_ERROR = WIN32_ERROR(1606u32);
                    pub const ERROR_UNKNOWN_COMPONENT: WIN32_ERROR = WIN32_ERROR(1607u32);
                    pub const ERROR_UNKNOWN_PROPERTY: WIN32_ERROR = WIN32_ERROR(1608u32);
                    pub const ERROR_INVALID_HANDLE_STATE: WIN32_ERROR = WIN32_ERROR(1609u32);
                    pub const ERROR_BAD_CONFIGURATION: WIN32_ERROR = WIN32_ERROR(1610u32);
                    pub const ERROR_INDEX_ABSENT: WIN32_ERROR = WIN32_ERROR(1611u32);
                    pub const ERROR_INSTALL_SOURCE_ABSENT: WIN32_ERROR = WIN32_ERROR(1612u32);
                    pub const ERROR_INSTALL_PACKAGE_VERSION: WIN32_ERROR = WIN32_ERROR(1613u32);
                    pub const ERROR_PRODUCT_UNINSTALLED: WIN32_ERROR = WIN32_ERROR(1614u32);
                    pub const ERROR_BAD_QUERY_SYNTAX: WIN32_ERROR = WIN32_ERROR(1615u32);
                    pub const ERROR_INVALID_FIELD: WIN32_ERROR = WIN32_ERROR(1616u32);
                    pub const ERROR_DEVICE_REMOVED: WIN32_ERROR = WIN32_ERROR(1617u32);
                    pub const ERROR_INSTALL_ALREADY_RUNNING: WIN32_ERROR = WIN32_ERROR(1618u32);
                    pub const ERROR_INSTALL_PACKAGE_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(1619u32);
                    pub const ERROR_INSTALL_PACKAGE_INVALID: WIN32_ERROR = WIN32_ERROR(1620u32);
                    pub const ERROR_INSTALL_UI_FAILURE: WIN32_ERROR = WIN32_ERROR(1621u32);
                    pub const ERROR_INSTALL_LOG_FAILURE: WIN32_ERROR = WIN32_ERROR(1622u32);
                    pub const ERROR_INSTALL_LANGUAGE_UNSUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(1623u32);
                    pub const ERROR_INSTALL_TRANSFORM_FAILURE: WIN32_ERROR = WIN32_ERROR(1624u32);
                    pub const ERROR_INSTALL_PACKAGE_REJECTED: WIN32_ERROR = WIN32_ERROR(1625u32);
                    pub const ERROR_FUNCTION_NOT_CALLED: WIN32_ERROR = WIN32_ERROR(1626u32);
                    pub const ERROR_FUNCTION_FAILED: WIN32_ERROR = WIN32_ERROR(1627u32);
                    pub const ERROR_INVALID_TABLE: WIN32_ERROR = WIN32_ERROR(1628u32);
                    pub const ERROR_DATATYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(1629u32);
                    pub const ERROR_UNSUPPORTED_TYPE: WIN32_ERROR = WIN32_ERROR(1630u32);
                    pub const ERROR_CREATE_FAILED: WIN32_ERROR = WIN32_ERROR(1631u32);
                    pub const ERROR_INSTALL_TEMP_UNWRITABLE: WIN32_ERROR = WIN32_ERROR(1632u32);
                    pub const ERROR_INSTALL_PLATFORM_UNSUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(1633u32);
                    pub const ERROR_INSTALL_NOTUSED: WIN32_ERROR = WIN32_ERROR(1634u32);
                    pub const ERROR_PATCH_PACKAGE_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(1635u32);
                    pub const ERROR_PATCH_PACKAGE_INVALID: WIN32_ERROR = WIN32_ERROR(1636u32);
                    pub const ERROR_PATCH_PACKAGE_UNSUPPORTED: WIN32_ERROR = WIN32_ERROR(1637u32);
                    pub const ERROR_PRODUCT_VERSION: WIN32_ERROR = WIN32_ERROR(1638u32);
                    pub const ERROR_INVALID_COMMAND_LINE: WIN32_ERROR = WIN32_ERROR(1639u32);
                    pub const ERROR_INSTALL_REMOTE_DISALLOWED: WIN32_ERROR = WIN32_ERROR(1640u32);
                    pub const ERROR_SUCCESS_REBOOT_INITIATED: WIN32_ERROR = WIN32_ERROR(1641u32);
                    pub const ERROR_PATCH_TARGET_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1642u32);
                    pub const ERROR_PATCH_PACKAGE_REJECTED: WIN32_ERROR = WIN32_ERROR(1643u32);
                    pub const ERROR_INSTALL_TRANSFORM_REJECTED: WIN32_ERROR = WIN32_ERROR(1644u32);
                    pub const ERROR_INSTALL_REMOTE_PROHIBITED: WIN32_ERROR = WIN32_ERROR(1645u32);
                    pub const ERROR_PATCH_REMOVAL_UNSUPPORTED: WIN32_ERROR = WIN32_ERROR(1646u32);
                    pub const ERROR_UNKNOWN_PATCH: WIN32_ERROR = WIN32_ERROR(1647u32);
                    pub const ERROR_PATCH_NO_SEQUENCE: WIN32_ERROR = WIN32_ERROR(1648u32);
                    pub const ERROR_PATCH_REMOVAL_DISALLOWED: WIN32_ERROR = WIN32_ERROR(1649u32);
                    pub const ERROR_INVALID_PATCH_XML: WIN32_ERROR = WIN32_ERROR(1650u32);
                    pub const ERROR_PATCH_MANAGED_ADVERTISED_PRODUCT: WIN32_ERROR =
                        WIN32_ERROR(1651u32);
                    pub const ERROR_INSTALL_SERVICE_SAFEBOOT: WIN32_ERROR = WIN32_ERROR(1652u32);
                    pub const ERROR_FAIL_FAST_EXCEPTION: WIN32_ERROR = WIN32_ERROR(1653u32);
                    pub const ERROR_INSTALL_REJECTED: WIN32_ERROR = WIN32_ERROR(1654u32);
                    pub const ERROR_DYNAMIC_CODE_BLOCKED: WIN32_ERROR = WIN32_ERROR(1655u32);
                    pub const ERROR_NOT_SAME_OBJECT: WIN32_ERROR = WIN32_ERROR(1656u32);
                    pub const ERROR_STRICT_CFG_VIOLATION: WIN32_ERROR = WIN32_ERROR(1657u32);
                    pub const ERROR_SET_CONTEXT_DENIED: WIN32_ERROR = WIN32_ERROR(1660u32);
                    pub const ERROR_CROSS_PARTITION_VIOLATION: WIN32_ERROR = WIN32_ERROR(1661u32);
                    pub const ERROR_RETURN_ADDRESS_HIJACK_ATTEMPT: WIN32_ERROR =
                        WIN32_ERROR(1662u32);
                    pub const ERROR_INVALID_USER_BUFFER: WIN32_ERROR = WIN32_ERROR(1784u32);
                    pub const ERROR_UNRECOGNIZED_MEDIA: WIN32_ERROR = WIN32_ERROR(1785u32);
                    pub const ERROR_NO_TRUST_LSA_SECRET: WIN32_ERROR = WIN32_ERROR(1786u32);
                    pub const ERROR_NO_TRUST_SAM_ACCOUNT: WIN32_ERROR = WIN32_ERROR(1787u32);
                    pub const ERROR_TRUSTED_DOMAIN_FAILURE: WIN32_ERROR = WIN32_ERROR(1788u32);
                    pub const ERROR_TRUSTED_RELATIONSHIP_FAILURE: WIN32_ERROR =
                        WIN32_ERROR(1789u32);
                    pub const ERROR_TRUST_FAILURE: WIN32_ERROR = WIN32_ERROR(1790u32);
                    pub const ERROR_NETLOGON_NOT_STARTED: WIN32_ERROR = WIN32_ERROR(1792u32);
                    pub const ERROR_ACCOUNT_EXPIRED: WIN32_ERROR = WIN32_ERROR(1793u32);
                    pub const ERROR_REDIRECTOR_HAS_OPEN_HANDLES: WIN32_ERROR = WIN32_ERROR(1794u32);
                    pub const ERROR_PRINTER_DRIVER_ALREADY_INSTALLED: WIN32_ERROR =
                        WIN32_ERROR(1795u32);
                    pub const ERROR_UNKNOWN_PORT: WIN32_ERROR = WIN32_ERROR(1796u32);
                    pub const ERROR_UNKNOWN_PRINTER_DRIVER: WIN32_ERROR = WIN32_ERROR(1797u32);
                    pub const ERROR_UNKNOWN_PRINTPROCESSOR: WIN32_ERROR = WIN32_ERROR(1798u32);
                    pub const ERROR_INVALID_SEPARATOR_FILE: WIN32_ERROR = WIN32_ERROR(1799u32);
                    pub const ERROR_INVALID_PRIORITY: WIN32_ERROR = WIN32_ERROR(1800u32);
                    pub const ERROR_INVALID_PRINTER_NAME: WIN32_ERROR = WIN32_ERROR(1801u32);
                    pub const ERROR_PRINTER_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(1802u32);
                    pub const ERROR_INVALID_PRINTER_COMMAND: WIN32_ERROR = WIN32_ERROR(1803u32);
                    pub const ERROR_INVALID_DATATYPE: WIN32_ERROR = WIN32_ERROR(1804u32);
                    pub const ERROR_INVALID_ENVIRONMENT: WIN32_ERROR = WIN32_ERROR(1805u32);
                    pub const ERROR_NOLOGON_INTERDOMAIN_TRUST_ACCOUNT: WIN32_ERROR =
                        WIN32_ERROR(1807u32);
                    pub const ERROR_NOLOGON_WORKSTATION_TRUST_ACCOUNT: WIN32_ERROR =
                        WIN32_ERROR(1808u32);
                    pub const ERROR_NOLOGON_SERVER_TRUST_ACCOUNT: WIN32_ERROR =
                        WIN32_ERROR(1809u32);
                    pub const ERROR_DOMAIN_TRUST_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(1810u32);
                    pub const ERROR_SERVER_HAS_OPEN_HANDLES: WIN32_ERROR = WIN32_ERROR(1811u32);
                    pub const ERROR_RESOURCE_DATA_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1812u32);
                    pub const ERROR_RESOURCE_TYPE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1813u32);
                    pub const ERROR_RESOURCE_NAME_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1814u32);
                    pub const ERROR_RESOURCE_LANG_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1815u32);
                    pub const ERROR_NOT_ENOUGH_QUOTA: WIN32_ERROR = WIN32_ERROR(1816u32);
                    pub const ERROR_INVALID_TIME: WIN32_ERROR = WIN32_ERROR(1901u32);
                    pub const ERROR_INVALID_FORM_NAME: WIN32_ERROR = WIN32_ERROR(1902u32);
                    pub const ERROR_INVALID_FORM_SIZE: WIN32_ERROR = WIN32_ERROR(1903u32);
                    pub const ERROR_ALREADY_WAITING: WIN32_ERROR = WIN32_ERROR(1904u32);
                    pub const ERROR_PRINTER_DELETED: WIN32_ERROR = WIN32_ERROR(1905u32);
                    pub const ERROR_INVALID_PRINTER_STATE: WIN32_ERROR = WIN32_ERROR(1906u32);
                    pub const ERROR_PASSWORD_MUST_CHANGE: WIN32_ERROR = WIN32_ERROR(1907u32);
                    pub const ERROR_DOMAIN_CONTROLLER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1908u32);
                    pub const ERROR_ACCOUNT_LOCKED_OUT: WIN32_ERROR = WIN32_ERROR(1909u32);
                    pub const ERROR_NO_SITENAME: WIN32_ERROR = WIN32_ERROR(1919u32);
                    pub const ERROR_CANT_ACCESS_FILE: WIN32_ERROR = WIN32_ERROR(1920u32);
                    pub const ERROR_CANT_RESOLVE_FILENAME: WIN32_ERROR = WIN32_ERROR(1921u32);
                    pub const ERROR_KM_DRIVER_BLOCKED: WIN32_ERROR = WIN32_ERROR(1930u32);
                    pub const ERROR_CONTEXT_EXPIRED: WIN32_ERROR = WIN32_ERROR(1931u32);
                    pub const ERROR_PER_USER_TRUST_QUOTA_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(1932u32);
                    pub const ERROR_ALL_USER_TRUST_QUOTA_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(1933u32);
                    pub const ERROR_USER_DELETE_TRUST_QUOTA_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(1934u32);
                    pub const ERROR_AUTHENTICATION_FIREWALL_FAILED: WIN32_ERROR =
                        WIN32_ERROR(1935u32);
                    pub const ERROR_REMOTE_PRINT_CONNECTIONS_BLOCKED: WIN32_ERROR =
                        WIN32_ERROR(1936u32);
                    pub const ERROR_NTLM_BLOCKED: WIN32_ERROR = WIN32_ERROR(1937u32);
                    pub const ERROR_PASSWORD_CHANGE_REQUIRED: WIN32_ERROR = WIN32_ERROR(1938u32);
                    pub const ERROR_LOST_MODE_LOGON_RESTRICTION: WIN32_ERROR = WIN32_ERROR(1939u32);
                    pub const ERROR_INVALID_PIXEL_FORMAT: WIN32_ERROR = WIN32_ERROR(2000u32);
                    pub const ERROR_BAD_DRIVER: WIN32_ERROR = WIN32_ERROR(2001u32);
                    pub const ERROR_INVALID_WINDOW_STYLE: WIN32_ERROR = WIN32_ERROR(2002u32);
                    pub const ERROR_METAFILE_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2003u32);
                    pub const ERROR_TRANSFORM_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2004u32);
                    pub const ERROR_CLIPPING_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2005u32);
                    pub const ERROR_INVALID_CMM: WIN32_ERROR = WIN32_ERROR(2010u32);
                    pub const ERROR_INVALID_PROFILE: WIN32_ERROR = WIN32_ERROR(2011u32);
                    pub const ERROR_TAG_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2012u32);
                    pub const ERROR_TAG_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(2013u32);
                    pub const ERROR_DUPLICATE_TAG: WIN32_ERROR = WIN32_ERROR(2014u32);
                    pub const ERROR_PROFILE_NOT_ASSOCIATED_WITH_DEVICE: WIN32_ERROR =
                        WIN32_ERROR(2015u32);
                    pub const ERROR_PROFILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2016u32);
                    pub const ERROR_INVALID_COLORSPACE: WIN32_ERROR = WIN32_ERROR(2017u32);
                    pub const ERROR_ICM_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(2018u32);
                    pub const ERROR_DELETING_ICM_XFORM: WIN32_ERROR = WIN32_ERROR(2019u32);
                    pub const ERROR_INVALID_TRANSFORM: WIN32_ERROR = WIN32_ERROR(2020u32);
                    pub const ERROR_COLORSPACE_MISMATCH: WIN32_ERROR = WIN32_ERROR(2021u32);
                    pub const ERROR_INVALID_COLORINDEX: WIN32_ERROR = WIN32_ERROR(2022u32);
                    pub const ERROR_PROFILE_DOES_NOT_MATCH_DEVICE: WIN32_ERROR =
                        WIN32_ERROR(2023u32);
                    pub const ERROR_CONNECTED_OTHER_PASSWORD: WIN32_ERROR = WIN32_ERROR(2108u32);
                    pub const ERROR_CONNECTED_OTHER_PASSWORD_DEFAULT: WIN32_ERROR =
                        WIN32_ERROR(2109u32);
                    pub const ERROR_BAD_USERNAME: WIN32_ERROR = WIN32_ERROR(2202u32);
                    pub const ERROR_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(2250u32);
                    pub const ERROR_OPEN_FILES: WIN32_ERROR = WIN32_ERROR(2401u32);
                    pub const ERROR_ACTIVE_CONNECTIONS: WIN32_ERROR = WIN32_ERROR(2402u32);
                    pub const ERROR_DEVICE_IN_USE: WIN32_ERROR = WIN32_ERROR(2404u32);
                    pub const ERROR_UNKNOWN_PRINT_MONITOR: WIN32_ERROR = WIN32_ERROR(3000u32);
                    pub const ERROR_PRINTER_DRIVER_IN_USE: WIN32_ERROR = WIN32_ERROR(3001u32);
                    pub const ERROR_SPOOL_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3002u32);
                    pub const ERROR_SPL_NO_STARTDOC: WIN32_ERROR = WIN32_ERROR(3003u32);
                    pub const ERROR_SPL_NO_ADDJOB: WIN32_ERROR = WIN32_ERROR(3004u32);
                    pub const ERROR_PRINT_PROCESSOR_ALREADY_INSTALLED: WIN32_ERROR =
                        WIN32_ERROR(3005u32);
                    pub const ERROR_PRINT_MONITOR_ALREADY_INSTALLED: WIN32_ERROR =
                        WIN32_ERROR(3006u32);
                    pub const ERROR_INVALID_PRINT_MONITOR: WIN32_ERROR = WIN32_ERROR(3007u32);
                    pub const ERROR_PRINT_MONITOR_IN_USE: WIN32_ERROR = WIN32_ERROR(3008u32);
                    pub const ERROR_PRINTER_HAS_JOBS_QUEUED: WIN32_ERROR = WIN32_ERROR(3009u32);
                    pub const ERROR_SUCCESS_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(3010u32);
                    pub const ERROR_SUCCESS_RESTART_REQUIRED: WIN32_ERROR = WIN32_ERROR(3011u32);
                    pub const ERROR_PRINTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(3012u32);
                    pub const ERROR_PRINTER_DRIVER_WARNED: WIN32_ERROR = WIN32_ERROR(3013u32);
                    pub const ERROR_PRINTER_DRIVER_BLOCKED: WIN32_ERROR = WIN32_ERROR(3014u32);
                    pub const ERROR_PRINTER_DRIVER_PACKAGE_IN_USE: WIN32_ERROR =
                        WIN32_ERROR(3015u32);
                    pub const ERROR_CORE_DRIVER_PACKAGE_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(3016u32);
                    pub const ERROR_FAIL_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(3017u32);
                    pub const ERROR_FAIL_REBOOT_INITIATED: WIN32_ERROR = WIN32_ERROR(3018u32);
                    pub const ERROR_PRINTER_DRIVER_DOWNLOAD_NEEDED: WIN32_ERROR =
                        WIN32_ERROR(3019u32);
                    pub const ERROR_PRINT_JOB_RESTART_REQUIRED: WIN32_ERROR = WIN32_ERROR(3020u32);
                    pub const ERROR_INVALID_PRINTER_DRIVER_MANIFEST: WIN32_ERROR =
                        WIN32_ERROR(3021u32);
                    pub const ERROR_PRINTER_NOT_SHAREABLE: WIN32_ERROR = WIN32_ERROR(3022u32);
                    pub const ERROR_REQUEST_PAUSED: WIN32_ERROR = WIN32_ERROR(3050u32);
                    pub const ERROR_APPEXEC_CONDITION_NOT_SATISFIED: WIN32_ERROR =
                        WIN32_ERROR(3060u32);
                    pub const ERROR_APPEXEC_HANDLE_INVALIDATED: WIN32_ERROR = WIN32_ERROR(3061u32);
                    pub const ERROR_APPEXEC_INVALID_HOST_GENERATION: WIN32_ERROR =
                        WIN32_ERROR(3062u32);
                    pub const ERROR_APPEXEC_UNEXPECTED_PROCESS_REGISTRATION: WIN32_ERROR =
                        WIN32_ERROR(3063u32);
                    pub const ERROR_APPEXEC_INVALID_HOST_STATE: WIN32_ERROR = WIN32_ERROR(3064u32);
                    pub const ERROR_APPEXEC_NO_DONOR: WIN32_ERROR = WIN32_ERROR(3065u32);
                    pub const ERROR_APPEXEC_HOST_ID_MISMATCH: WIN32_ERROR = WIN32_ERROR(3066u32);
                    pub const ERROR_APPEXEC_UNKNOWN_USER: WIN32_ERROR = WIN32_ERROR(3067u32);
                    pub const ERROR_IO_REISSUE_AS_CACHED: WIN32_ERROR = WIN32_ERROR(3950u32);
                    pub const ERROR_WINS_INTERNAL: WIN32_ERROR = WIN32_ERROR(4000u32);
                    pub const ERROR_CAN_NOT_DEL_LOCAL_WINS: WIN32_ERROR = WIN32_ERROR(4001u32);
                    pub const ERROR_STATIC_INIT: WIN32_ERROR = WIN32_ERROR(4002u32);
                    pub const ERROR_INC_BACKUP: WIN32_ERROR = WIN32_ERROR(4003u32);
                    pub const ERROR_FULL_BACKUP: WIN32_ERROR = WIN32_ERROR(4004u32);
                    pub const ERROR_REC_NON_EXISTENT: WIN32_ERROR = WIN32_ERROR(4005u32);
                    pub const ERROR_RPL_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(4006u32);
                    pub const ERROR_DHCP_ADDRESS_CONFLICT: WIN32_ERROR = WIN32_ERROR(4100u32);
                    pub const ERROR_WMI_GUID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4200u32);
                    pub const ERROR_WMI_INSTANCE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4201u32);
                    pub const ERROR_WMI_ITEMID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4202u32);
                    pub const ERROR_WMI_TRY_AGAIN: WIN32_ERROR = WIN32_ERROR(4203u32);
                    pub const ERROR_WMI_DP_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4204u32);
                    pub const ERROR_WMI_UNRESOLVED_INSTANCE_REF: WIN32_ERROR = WIN32_ERROR(4205u32);
                    pub const ERROR_WMI_ALREADY_ENABLED: WIN32_ERROR = WIN32_ERROR(4206u32);
                    pub const ERROR_WMI_GUID_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(4207u32);
                    pub const ERROR_WMI_SERVER_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(4208u32);
                    pub const ERROR_WMI_DP_FAILED: WIN32_ERROR = WIN32_ERROR(4209u32);
                    pub const ERROR_WMI_INVALID_MOF: WIN32_ERROR = WIN32_ERROR(4210u32);
                    pub const ERROR_WMI_INVALID_REGINFO: WIN32_ERROR = WIN32_ERROR(4211u32);
                    pub const ERROR_WMI_ALREADY_DISABLED: WIN32_ERROR = WIN32_ERROR(4212u32);
                    pub const ERROR_WMI_READ_ONLY: WIN32_ERROR = WIN32_ERROR(4213u32);
                    pub const ERROR_WMI_SET_FAILURE: WIN32_ERROR = WIN32_ERROR(4214u32);
                    pub const ERROR_NOT_APPCONTAINER: WIN32_ERROR = WIN32_ERROR(4250u32);
                    pub const ERROR_APPCONTAINER_REQUIRED: WIN32_ERROR = WIN32_ERROR(4251u32);
                    pub const ERROR_NOT_SUPPORTED_IN_APPCONTAINER: WIN32_ERROR =
                        WIN32_ERROR(4252u32);
                    pub const ERROR_INVALID_PACKAGE_SID_LENGTH: WIN32_ERROR = WIN32_ERROR(4253u32);
                    pub const ERROR_INVALID_MEDIA: WIN32_ERROR = WIN32_ERROR(4300u32);
                    pub const ERROR_INVALID_LIBRARY: WIN32_ERROR = WIN32_ERROR(4301u32);
                    pub const ERROR_INVALID_MEDIA_POOL: WIN32_ERROR = WIN32_ERROR(4302u32);
                    pub const ERROR_DRIVE_MEDIA_MISMATCH: WIN32_ERROR = WIN32_ERROR(4303u32);
                    pub const ERROR_MEDIA_OFFLINE: WIN32_ERROR = WIN32_ERROR(4304u32);
                    pub const ERROR_LIBRARY_OFFLINE: WIN32_ERROR = WIN32_ERROR(4305u32);
                    pub const ERROR_EMPTY: WIN32_ERROR = WIN32_ERROR(4306u32);
                    pub const ERROR_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(4307u32);
                    pub const ERROR_MEDIA_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(4308u32);
                    pub const ERROR_RESOURCE_DISABLED: WIN32_ERROR = WIN32_ERROR(4309u32);
                    pub const ERROR_INVALID_CLEANER: WIN32_ERROR = WIN32_ERROR(4310u32);
                    pub const ERROR_UNABLE_TO_CLEAN: WIN32_ERROR = WIN32_ERROR(4311u32);
                    pub const ERROR_OBJECT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4312u32);
                    pub const ERROR_DATABASE_FAILURE: WIN32_ERROR = WIN32_ERROR(4313u32);
                    pub const ERROR_DATABASE_FULL: WIN32_ERROR = WIN32_ERROR(4314u32);
                    pub const ERROR_MEDIA_INCOMPATIBLE: WIN32_ERROR = WIN32_ERROR(4315u32);
                    pub const ERROR_RESOURCE_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(4316u32);
                    pub const ERROR_INVALID_OPERATION: WIN32_ERROR = WIN32_ERROR(4317u32);
                    pub const ERROR_MEDIA_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(4318u32);
                    pub const ERROR_DEVICE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(4319u32);
                    pub const ERROR_REQUEST_REFUSED: WIN32_ERROR = WIN32_ERROR(4320u32);
                    pub const ERROR_INVALID_DRIVE_OBJECT: WIN32_ERROR = WIN32_ERROR(4321u32);
                    pub const ERROR_LIBRARY_FULL: WIN32_ERROR = WIN32_ERROR(4322u32);
                    pub const ERROR_MEDIUM_NOT_ACCESSIBLE: WIN32_ERROR = WIN32_ERROR(4323u32);
                    pub const ERROR_UNABLE_TO_LOAD_MEDIUM: WIN32_ERROR = WIN32_ERROR(4324u32);
                    pub const ERROR_UNABLE_TO_INVENTORY_DRIVE: WIN32_ERROR = WIN32_ERROR(4325u32);
                    pub const ERROR_UNABLE_TO_INVENTORY_SLOT: WIN32_ERROR = WIN32_ERROR(4326u32);
                    pub const ERROR_UNABLE_TO_INVENTORY_TRANSPORT: WIN32_ERROR =
                        WIN32_ERROR(4327u32);
                    pub const ERROR_TRANSPORT_FULL: WIN32_ERROR = WIN32_ERROR(4328u32);
                    pub const ERROR_CONTROLLING_IEPORT: WIN32_ERROR = WIN32_ERROR(4329u32);
                    pub const ERROR_UNABLE_TO_EJECT_MOUNTED_MEDIA: WIN32_ERROR =
                        WIN32_ERROR(4330u32);
                    pub const ERROR_CLEANER_SLOT_SET: WIN32_ERROR = WIN32_ERROR(4331u32);
                    pub const ERROR_CLEANER_SLOT_NOT_SET: WIN32_ERROR = WIN32_ERROR(4332u32);
                    pub const ERROR_CLEANER_CARTRIDGE_SPENT: WIN32_ERROR = WIN32_ERROR(4333u32);
                    pub const ERROR_UNEXPECTED_OMID: WIN32_ERROR = WIN32_ERROR(4334u32);
                    pub const ERROR_CANT_DELETE_LAST_ITEM: WIN32_ERROR = WIN32_ERROR(4335u32);
                    pub const ERROR_MESSAGE_EXCEEDS_MAX_SIZE: WIN32_ERROR = WIN32_ERROR(4336u32);
                    pub const ERROR_VOLUME_CONTAINS_SYS_FILES: WIN32_ERROR = WIN32_ERROR(4337u32);
                    pub const ERROR_INDIGENOUS_TYPE: WIN32_ERROR = WIN32_ERROR(4338u32);
                    pub const ERROR_NO_SUPPORTING_DRIVES: WIN32_ERROR = WIN32_ERROR(4339u32);
                    pub const ERROR_CLEANER_CARTRIDGE_INSTALLED: WIN32_ERROR = WIN32_ERROR(4340u32);
                    pub const ERROR_IEPORT_FULL: WIN32_ERROR = WIN32_ERROR(4341u32);
                    pub const ERROR_FILE_OFFLINE: WIN32_ERROR = WIN32_ERROR(4350u32);
                    pub const ERROR_REMOTE_STORAGE_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(4351u32);
                    pub const ERROR_REMOTE_STORAGE_MEDIA_ERROR: WIN32_ERROR = WIN32_ERROR(4352u32);
                    pub const ERROR_NOT_A_REPARSE_POINT: WIN32_ERROR = WIN32_ERROR(4390u32);
                    pub const ERROR_REPARSE_ATTRIBUTE_CONFLICT: WIN32_ERROR = WIN32_ERROR(4391u32);
                    pub const ERROR_INVALID_REPARSE_DATA: WIN32_ERROR = WIN32_ERROR(4392u32);
                    pub const ERROR_REPARSE_TAG_INVALID: WIN32_ERROR = WIN32_ERROR(4393u32);
                    pub const ERROR_REPARSE_TAG_MISMATCH: WIN32_ERROR = WIN32_ERROR(4394u32);
                    pub const ERROR_REPARSE_POINT_ENCOUNTERED: WIN32_ERROR = WIN32_ERROR(4395u32);
                    pub const ERROR_APP_DATA_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(4400u32);
                    pub const ERROR_APP_DATA_EXPIRED: WIN32_ERROR = WIN32_ERROR(4401u32);
                    pub const ERROR_APP_DATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(4402u32);
                    pub const ERROR_APP_DATA_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(4403u32);
                    pub const ERROR_APP_DATA_REBOOT_REQUIRED: WIN32_ERROR = WIN32_ERROR(4404u32);
                    pub const ERROR_SECUREBOOT_ROLLBACK_DETECTED: WIN32_ERROR =
                        WIN32_ERROR(4420u32);
                    pub const ERROR_SECUREBOOT_POLICY_VIOLATION: WIN32_ERROR = WIN32_ERROR(4421u32);
                    pub const ERROR_SECUREBOOT_INVALID_POLICY: WIN32_ERROR = WIN32_ERROR(4422u32);
                    pub const ERROR_SECUREBOOT_POLICY_PUBLISHER_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(4423u32);
                    pub const ERROR_SECUREBOOT_POLICY_NOT_SIGNED: WIN32_ERROR =
                        WIN32_ERROR(4424u32);
                    pub const ERROR_SECUREBOOT_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(4425u32);
                    pub const ERROR_SECUREBOOT_FILE_REPLACED: WIN32_ERROR = WIN32_ERROR(4426u32);
                    pub const ERROR_SECUREBOOT_POLICY_NOT_AUTHORIZED: WIN32_ERROR =
                        WIN32_ERROR(4427u32);
                    pub const ERROR_SECUREBOOT_POLICY_UNKNOWN: WIN32_ERROR = WIN32_ERROR(4428u32);
                    pub const ERROR_SECUREBOOT_POLICY_MISSING_ANTIROLLBACKVERSION: WIN32_ERROR =
                        WIN32_ERROR(4429u32);
                    pub const ERROR_SECUREBOOT_PLATFORM_ID_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(4430u32);
                    pub const ERROR_SECUREBOOT_POLICY_ROLLBACK_DETECTED: WIN32_ERROR =
                        WIN32_ERROR(4431u32);
                    pub const ERROR_SECUREBOOT_POLICY_UPGRADE_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(4432u32);
                    pub const ERROR_SECUREBOOT_REQUIRED_POLICY_FILE_MISSING: WIN32_ERROR =
                        WIN32_ERROR(4433u32);
                    pub const ERROR_SECUREBOOT_NOT_BASE_POLICY: WIN32_ERROR = WIN32_ERROR(4434u32);
                    pub const ERROR_SECUREBOOT_NOT_SUPPLEMENTAL_POLICY: WIN32_ERROR =
                        WIN32_ERROR(4435u32);
                    pub const ERROR_OFFLOAD_READ_FLT_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(4440u32);
                    pub const ERROR_OFFLOAD_WRITE_FLT_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(4441u32);
                    pub const ERROR_OFFLOAD_READ_FILE_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(4442u32);
                    pub const ERROR_OFFLOAD_WRITE_FILE_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(4443u32);
                    pub const ERROR_ALREADY_HAS_STREAM_ID: WIN32_ERROR = WIN32_ERROR(4444u32);
                    pub const ERROR_SMR_GARBAGE_COLLECTION_REQUIRED: WIN32_ERROR =
                        WIN32_ERROR(4445u32);
                    pub const ERROR_WOF_WIM_HEADER_CORRUPT: WIN32_ERROR = WIN32_ERROR(4446u32);
                    pub const ERROR_WOF_WIM_RESOURCE_TABLE_CORRUPT: WIN32_ERROR =
                        WIN32_ERROR(4447u32);
                    pub const ERROR_WOF_FILE_RESOURCE_TABLE_CORRUPT: WIN32_ERROR =
                        WIN32_ERROR(4448u32);
                    pub const ERROR_VOLUME_NOT_SIS_ENABLED: WIN32_ERROR = WIN32_ERROR(4500u32);
                    pub const ERROR_SYSTEM_INTEGRITY_ROLLBACK_DETECTED: WIN32_ERROR =
                        WIN32_ERROR(4550u32);
                    pub const ERROR_SYSTEM_INTEGRITY_POLICY_VIOLATION: WIN32_ERROR =
                        WIN32_ERROR(4551u32);
                    pub const ERROR_SYSTEM_INTEGRITY_INVALID_POLICY: WIN32_ERROR =
                        WIN32_ERROR(4552u32);
                    pub const ERROR_SYSTEM_INTEGRITY_POLICY_NOT_SIGNED: WIN32_ERROR =
                        WIN32_ERROR(4553u32);
                    pub const ERROR_SYSTEM_INTEGRITY_TOO_MANY_POLICIES: WIN32_ERROR =
                        WIN32_ERROR(4554u32);
                    pub const ERROR_SYSTEM_INTEGRITY_SUPPLEMENTAL_POLICY_NOT_AUTHORIZED:
                        WIN32_ERROR = WIN32_ERROR(4555u32);
                    pub const ERROR_VSM_NOT_INITIALIZED: WIN32_ERROR = WIN32_ERROR(4560u32);
                    pub const ERROR_VSM_DMA_PROTECTION_NOT_IN_USE: WIN32_ERROR =
                        WIN32_ERROR(4561u32);
                    pub const ERROR_PLATFORM_MANIFEST_NOT_AUTHORIZED: WIN32_ERROR =
                        WIN32_ERROR(4570u32);
                    pub const ERROR_PLATFORM_MANIFEST_INVALID: WIN32_ERROR = WIN32_ERROR(4571u32);
                    pub const ERROR_PLATFORM_MANIFEST_FILE_NOT_AUTHORIZED: WIN32_ERROR =
                        WIN32_ERROR(4572u32);
                    pub const ERROR_PLATFORM_MANIFEST_CATALOG_NOT_AUTHORIZED: WIN32_ERROR =
                        WIN32_ERROR(4573u32);
                    pub const ERROR_PLATFORM_MANIFEST_BINARY_ID_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(4574u32);
                    pub const ERROR_PLATFORM_MANIFEST_NOT_ACTIVE: WIN32_ERROR =
                        WIN32_ERROR(4575u32);
                    pub const ERROR_PLATFORM_MANIFEST_NOT_SIGNED: WIN32_ERROR =
                        WIN32_ERROR(4576u32);
                    pub const ERROR_DEPENDENT_RESOURCE_EXISTS: WIN32_ERROR = WIN32_ERROR(5001u32);
                    pub const ERROR_DEPENDENCY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5002u32);
                    pub const ERROR_DEPENDENCY_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(5003u32);
                    pub const ERROR_RESOURCE_NOT_ONLINE: WIN32_ERROR = WIN32_ERROR(5004u32);
                    pub const ERROR_HOST_NODE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5005u32);
                    pub const ERROR_RESOURCE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5006u32);
                    pub const ERROR_RESOURCE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5007u32);
                    pub const ERROR_SHUTDOWN_CLUSTER: WIN32_ERROR = WIN32_ERROR(5008u32);
                    pub const ERROR_CANT_EVICT_ACTIVE_NODE: WIN32_ERROR = WIN32_ERROR(5009u32);
                    pub const ERROR_OBJECT_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(5010u32);
                    pub const ERROR_OBJECT_IN_LIST: WIN32_ERROR = WIN32_ERROR(5011u32);
                    pub const ERROR_GROUP_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5012u32);
                    pub const ERROR_GROUP_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5013u32);
                    pub const ERROR_GROUP_NOT_ONLINE: WIN32_ERROR = WIN32_ERROR(5014u32);
                    pub const ERROR_HOST_NODE_NOT_RESOURCE_OWNER: WIN32_ERROR =
                        WIN32_ERROR(5015u32);
                    pub const ERROR_HOST_NODE_NOT_GROUP_OWNER: WIN32_ERROR = WIN32_ERROR(5016u32);
                    pub const ERROR_RESMON_CREATE_FAILED: WIN32_ERROR = WIN32_ERROR(5017u32);
                    pub const ERROR_RESMON_ONLINE_FAILED: WIN32_ERROR = WIN32_ERROR(5018u32);
                    pub const ERROR_RESOURCE_ONLINE: WIN32_ERROR = WIN32_ERROR(5019u32);
                    pub const ERROR_QUORUM_RESOURCE: WIN32_ERROR = WIN32_ERROR(5020u32);
                    pub const ERROR_NOT_QUORUM_CAPABLE: WIN32_ERROR = WIN32_ERROR(5021u32);
                    pub const ERROR_CLUSTER_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(5022u32);
                    pub const ERROR_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(5023u32);
                    pub const ERROR_RESOURCE_PROPERTIES_STORED: WIN32_ERROR = WIN32_ERROR(5024u32);
                    pub const ERROR_NOT_QUORUM_CLASS: WIN32_ERROR = WIN32_ERROR(5025u32);
                    pub const ERROR_CORE_RESOURCE: WIN32_ERROR = WIN32_ERROR(5026u32);
                    pub const ERROR_QUORUM_RESOURCE_ONLINE_FAILED: WIN32_ERROR =
                        WIN32_ERROR(5027u32);
                    pub const ERROR_QUORUMLOG_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(5028u32);
                    pub const ERROR_CLUSTERLOG_CORRUPT: WIN32_ERROR = WIN32_ERROR(5029u32);
                    pub const ERROR_CLUSTERLOG_RECORD_EXCEEDS_MAXSIZE: WIN32_ERROR =
                        WIN32_ERROR(5030u32);
                    pub const ERROR_CLUSTERLOG_EXCEEDS_MAXSIZE: WIN32_ERROR = WIN32_ERROR(5031u32);
                    pub const ERROR_CLUSTERLOG_CHKPOINT_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(5032u32);
                    pub const ERROR_CLUSTERLOG_NOT_ENOUGH_SPACE: WIN32_ERROR = WIN32_ERROR(5033u32);
                    pub const ERROR_QUORUM_OWNER_ALIVE: WIN32_ERROR = WIN32_ERROR(5034u32);
                    pub const ERROR_NETWORK_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5035u32);
                    pub const ERROR_NODE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5036u32);
                    pub const ERROR_ALL_NODES_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5037u32);
                    pub const ERROR_RESOURCE_FAILED: WIN32_ERROR = WIN32_ERROR(5038u32);
                    pub const ERROR_CLUSTER_INVALID_NODE: WIN32_ERROR = WIN32_ERROR(5039u32);
                    pub const ERROR_CLUSTER_NODE_EXISTS: WIN32_ERROR = WIN32_ERROR(5040u32);
                    pub const ERROR_CLUSTER_JOIN_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5041u32);
                    pub const ERROR_CLUSTER_NODE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5042u32);
                    pub const ERROR_CLUSTER_LOCAL_NODE_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(5043u32);
                    pub const ERROR_CLUSTER_NETWORK_EXISTS: WIN32_ERROR = WIN32_ERROR(5044u32);
                    pub const ERROR_CLUSTER_NETWORK_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5045u32);
                    pub const ERROR_CLUSTER_NETINTERFACE_EXISTS: WIN32_ERROR = WIN32_ERROR(5046u32);
                    pub const ERROR_CLUSTER_NETINTERFACE_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(5047u32);
                    pub const ERROR_CLUSTER_INVALID_REQUEST: WIN32_ERROR = WIN32_ERROR(5048u32);
                    pub const ERROR_CLUSTER_INVALID_NETWORK_PROVIDER: WIN32_ERROR =
                        WIN32_ERROR(5049u32);
                    pub const ERROR_CLUSTER_NODE_DOWN: WIN32_ERROR = WIN32_ERROR(5050u32);
                    pub const ERROR_CLUSTER_NODE_UNREACHABLE: WIN32_ERROR = WIN32_ERROR(5051u32);
                    pub const ERROR_CLUSTER_NODE_NOT_MEMBER: WIN32_ERROR = WIN32_ERROR(5052u32);
                    pub const ERROR_CLUSTER_JOIN_NOT_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(5053u32);
                    pub const ERROR_CLUSTER_INVALID_NETWORK: WIN32_ERROR = WIN32_ERROR(5054u32);
                    pub const ERROR_CLUSTER_NODE_UP: WIN32_ERROR = WIN32_ERROR(5056u32);
                    pub const ERROR_CLUSTER_IPADDR_IN_USE: WIN32_ERROR = WIN32_ERROR(5057u32);
                    pub const ERROR_CLUSTER_NODE_NOT_PAUSED: WIN32_ERROR = WIN32_ERROR(5058u32);
                    pub const ERROR_CLUSTER_NO_SECURITY_CONTEXT: WIN32_ERROR = WIN32_ERROR(5059u32);
                    pub const ERROR_CLUSTER_NETWORK_NOT_INTERNAL: WIN32_ERROR =
                        WIN32_ERROR(5060u32);
                    pub const ERROR_CLUSTER_NODE_ALREADY_UP: WIN32_ERROR = WIN32_ERROR(5061u32);
                    pub const ERROR_CLUSTER_NODE_ALREADY_DOWN: WIN32_ERROR = WIN32_ERROR(5062u32);
                    pub const ERROR_CLUSTER_NETWORK_ALREADY_ONLINE: WIN32_ERROR =
                        WIN32_ERROR(5063u32);
                    pub const ERROR_CLUSTER_NETWORK_ALREADY_OFFLINE: WIN32_ERROR =
                        WIN32_ERROR(5064u32);
                    pub const ERROR_CLUSTER_NODE_ALREADY_MEMBER: WIN32_ERROR = WIN32_ERROR(5065u32);
                    pub const ERROR_CLUSTER_LAST_INTERNAL_NETWORK: WIN32_ERROR =
                        WIN32_ERROR(5066u32);
                    pub const ERROR_CLUSTER_NETWORK_HAS_DEPENDENTS: WIN32_ERROR =
                        WIN32_ERROR(5067u32);
                    pub const ERROR_INVALID_OPERATION_ON_QUORUM: WIN32_ERROR = WIN32_ERROR(5068u32);
                    pub const ERROR_DEPENDENCY_NOT_ALLOWED: WIN32_ERROR = WIN32_ERROR(5069u32);
                    pub const ERROR_CLUSTER_NODE_PAUSED: WIN32_ERROR = WIN32_ERROR(5070u32);
                    pub const ERROR_NODE_CANT_HOST_RESOURCE: WIN32_ERROR = WIN32_ERROR(5071u32);
                    pub const ERROR_CLUSTER_NODE_NOT_READY: WIN32_ERROR = WIN32_ERROR(5072u32);
                    pub const ERROR_CLUSTER_NODE_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(5073u32);
                    pub const ERROR_CLUSTER_JOIN_ABORTED: WIN32_ERROR = WIN32_ERROR(5074u32);
                    pub const ERROR_CLUSTER_INCOMPATIBLE_VERSIONS: WIN32_ERROR =
                        WIN32_ERROR(5075u32);
                    pub const ERROR_CLUSTER_MAXNUM_OF_RESOURCES_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(5076u32);
                    pub const ERROR_CLUSTER_SYSTEM_CONFIG_CHANGED: WIN32_ERROR =
                        WIN32_ERROR(5077u32);
                    pub const ERROR_CLUSTER_RESOURCE_TYPE_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(5078u32);
                    pub const ERROR_CLUSTER_RESTYPE_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(5079u32);
                    pub const ERROR_CLUSTER_RESNAME_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5080u32);
                    pub const ERROR_CLUSTER_NO_RPC_PACKAGES_REGISTERED: WIN32_ERROR =
                        WIN32_ERROR(5081u32);
                    pub const ERROR_CLUSTER_OWNER_NOT_IN_PREFLIST: WIN32_ERROR =
                        WIN32_ERROR(5082u32);
                    pub const ERROR_CLUSTER_DATABASE_SEQMISMATCH: WIN32_ERROR =
                        WIN32_ERROR(5083u32);
                    pub const ERROR_RESMON_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(5084u32);
                    pub const ERROR_CLUSTER_GUM_NOT_LOCKER: WIN32_ERROR = WIN32_ERROR(5085u32);
                    pub const ERROR_QUORUM_DISK_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5086u32);
                    pub const ERROR_DATABASE_BACKUP_CORRUPT: WIN32_ERROR = WIN32_ERROR(5087u32);
                    pub const ERROR_CLUSTER_NODE_ALREADY_HAS_DFS_ROOT: WIN32_ERROR =
                        WIN32_ERROR(5088u32);
                    pub const ERROR_RESOURCE_PROPERTY_UNCHANGEABLE: WIN32_ERROR =
                        WIN32_ERROR(5089u32);
                    pub const ERROR_NO_ADMIN_ACCESS_POINT: WIN32_ERROR = WIN32_ERROR(5090u32);
                    pub const ERROR_CLUSTER_MEMBERSHIP_INVALID_STATE: WIN32_ERROR =
                        WIN32_ERROR(5890u32);
                    pub const ERROR_CLUSTER_QUORUMLOG_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5891u32);
                    pub const ERROR_CLUSTER_MEMBERSHIP_HALT: WIN32_ERROR = WIN32_ERROR(5892u32);
                    pub const ERROR_CLUSTER_INSTANCE_ID_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(5893u32);
                    pub const ERROR_CLUSTER_NETWORK_NOT_FOUND_FOR_IP: WIN32_ERROR =
                        WIN32_ERROR(5894u32);
                    pub const ERROR_CLUSTER_PROPERTY_DATA_TYPE_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(5895u32);
                    pub const ERROR_CLUSTER_EVICT_WITHOUT_CLEANUP: WIN32_ERROR =
                        WIN32_ERROR(5896u32);
                    pub const ERROR_CLUSTER_PARAMETER_MISMATCH: WIN32_ERROR = WIN32_ERROR(5897u32);
                    pub const ERROR_NODE_CANNOT_BE_CLUSTERED: WIN32_ERROR = WIN32_ERROR(5898u32);
                    pub const ERROR_CLUSTER_WRONG_OS_VERSION: WIN32_ERROR = WIN32_ERROR(5899u32);
                    pub const ERROR_CLUSTER_CANT_CREATE_DUP_CLUSTER_NAME: WIN32_ERROR =
                        WIN32_ERROR(5900u32);
                    pub const ERROR_CLUSCFG_ALREADY_COMMITTED: WIN32_ERROR = WIN32_ERROR(5901u32);
                    pub const ERROR_CLUSCFG_ROLLBACK_FAILED: WIN32_ERROR = WIN32_ERROR(5902u32);
                    pub const ERROR_CLUSCFG_SYSTEM_DISK_DRIVE_LETTER_CONFLICT: WIN32_ERROR =
                        WIN32_ERROR(5903u32);
                    pub const ERROR_CLUSTER_OLD_VERSION: WIN32_ERROR = WIN32_ERROR(5904u32);
                    pub const ERROR_CLUSTER_MISMATCHED_COMPUTER_ACCT_NAME: WIN32_ERROR =
                        WIN32_ERROR(5905u32);
                    pub const ERROR_CLUSTER_NO_NET_ADAPTERS: WIN32_ERROR = WIN32_ERROR(5906u32);
                    pub const ERROR_CLUSTER_POISONED: WIN32_ERROR = WIN32_ERROR(5907u32);
                    pub const ERROR_CLUSTER_GROUP_MOVING: WIN32_ERROR = WIN32_ERROR(5908u32);
                    pub const ERROR_CLUSTER_RESOURCE_TYPE_BUSY: WIN32_ERROR = WIN32_ERROR(5909u32);
                    pub const ERROR_RESOURCE_CALL_TIMED_OUT: WIN32_ERROR = WIN32_ERROR(5910u32);
                    pub const ERROR_INVALID_CLUSTER_IPV6_ADDRESS: WIN32_ERROR =
                        WIN32_ERROR(5911u32);
                    pub const ERROR_CLUSTER_INTERNAL_INVALID_FUNCTION: WIN32_ERROR =
                        WIN32_ERROR(5912u32);
                    pub const ERROR_CLUSTER_PARAMETER_OUT_OF_BOUNDS: WIN32_ERROR =
                        WIN32_ERROR(5913u32);
                    pub const ERROR_CLUSTER_PARTIAL_SEND: WIN32_ERROR = WIN32_ERROR(5914u32);
                    pub const ERROR_CLUSTER_REGISTRY_INVALID_FUNCTION: WIN32_ERROR =
                        WIN32_ERROR(5915u32);
                    pub const ERROR_CLUSTER_INVALID_STRING_TERMINATION: WIN32_ERROR =
                        WIN32_ERROR(5916u32);
                    pub const ERROR_CLUSTER_INVALID_STRING_FORMAT: WIN32_ERROR =
                        WIN32_ERROR(5917u32);
                    pub const ERROR_CLUSTER_DATABASE_TRANSACTION_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(5918u32);
                    pub const ERROR_CLUSTER_DATABASE_TRANSACTION_NOT_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(5919u32);
                    pub const ERROR_CLUSTER_NULL_DATA: WIN32_ERROR = WIN32_ERROR(5920u32);
                    pub const ERROR_CLUSTER_PARTIAL_READ: WIN32_ERROR = WIN32_ERROR(5921u32);
                    pub const ERROR_CLUSTER_PARTIAL_WRITE: WIN32_ERROR = WIN32_ERROR(5922u32);
                    pub const ERROR_CLUSTER_CANT_DESERIALIZE_DATA: WIN32_ERROR =
                        WIN32_ERROR(5923u32);
                    pub const ERROR_DEPENDENT_RESOURCE_PROPERTY_CONFLICT: WIN32_ERROR =
                        WIN32_ERROR(5924u32);
                    pub const ERROR_CLUSTER_NO_QUORUM: WIN32_ERROR = WIN32_ERROR(5925u32);
                    pub const ERROR_CLUSTER_INVALID_IPV6_NETWORK: WIN32_ERROR =
                        WIN32_ERROR(5926u32);
                    pub const ERROR_CLUSTER_INVALID_IPV6_TUNNEL_NETWORK: WIN32_ERROR =
                        WIN32_ERROR(5927u32);
                    pub const ERROR_QUORUM_NOT_ALLOWED_IN_THIS_GROUP: WIN32_ERROR =
                        WIN32_ERROR(5928u32);
                    pub const ERROR_DEPENDENCY_TREE_TOO_COMPLEX: WIN32_ERROR = WIN32_ERROR(5929u32);
                    pub const ERROR_EXCEPTION_IN_RESOURCE_CALL: WIN32_ERROR = WIN32_ERROR(5930u32);
                    pub const ERROR_CLUSTER_RHS_FAILED_INITIALIZATION: WIN32_ERROR =
                        WIN32_ERROR(5931u32);
                    pub const ERROR_CLUSTER_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(5932u32);
                    pub const ERROR_CLUSTER_RESOURCES_MUST_BE_ONLINE_ON_THE_SAME_NODE: WIN32_ERROR =
                        WIN32_ERROR(5933u32);
                    pub const ERROR_CLUSTER_MAX_NODES_IN_CLUSTER: WIN32_ERROR =
                        WIN32_ERROR(5934u32);
                    pub const ERROR_CLUSTER_TOO_MANY_NODES: WIN32_ERROR = WIN32_ERROR(5935u32);
                    pub const ERROR_CLUSTER_OBJECT_ALREADY_USED: WIN32_ERROR = WIN32_ERROR(5936u32);
                    pub const ERROR_NONCORE_GROUPS_FOUND: WIN32_ERROR = WIN32_ERROR(5937u32);
                    pub const ERROR_FILE_SHARE_RESOURCE_CONFLICT: WIN32_ERROR =
                        WIN32_ERROR(5938u32);
                    pub const ERROR_CLUSTER_EVICT_INVALID_REQUEST: WIN32_ERROR =
                        WIN32_ERROR(5939u32);
                    pub const ERROR_CLUSTER_SINGLETON_RESOURCE: WIN32_ERROR = WIN32_ERROR(5940u32);
                    pub const ERROR_CLUSTER_GROUP_SINGLETON_RESOURCE: WIN32_ERROR =
                        WIN32_ERROR(5941u32);
                    pub const ERROR_CLUSTER_RESOURCE_PROVIDER_FAILED: WIN32_ERROR =
                        WIN32_ERROR(5942u32);
                    pub const ERROR_CLUSTER_RESOURCE_CONFIGURATION_ERROR: WIN32_ERROR =
                        WIN32_ERROR(5943u32);
                    pub const ERROR_CLUSTER_GROUP_BUSY: WIN32_ERROR = WIN32_ERROR(5944u32);
                    pub const ERROR_CLUSTER_NOT_SHARED_VOLUME: WIN32_ERROR = WIN32_ERROR(5945u32);
                    pub const ERROR_CLUSTER_INVALID_SECURITY_DESCRIPTOR: WIN32_ERROR =
                        WIN32_ERROR(5946u32);
                    pub const ERROR_CLUSTER_SHARED_VOLUMES_IN_USE: WIN32_ERROR =
                        WIN32_ERROR(5947u32);
                    pub const ERROR_CLUSTER_USE_SHARED_VOLUMES_API: WIN32_ERROR =
                        WIN32_ERROR(5948u32);
                    pub const ERROR_CLUSTER_BACKUP_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5949u32);
                    pub const ERROR_NON_CSV_PATH: WIN32_ERROR = WIN32_ERROR(5950u32);
                    pub const ERROR_CSV_VOLUME_NOT_LOCAL: WIN32_ERROR = WIN32_ERROR(5951u32);
                    pub const ERROR_CLUSTER_WATCHDOG_TERMINATING: WIN32_ERROR =
                        WIN32_ERROR(5952u32);
                    pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_INCOMPATIBLE_NODES: WIN32_ERROR =
                        WIN32_ERROR(5953u32);
                    pub const ERROR_CLUSTER_INVALID_NODE_WEIGHT: WIN32_ERROR = WIN32_ERROR(5954u32);
                    pub const ERROR_CLUSTER_RESOURCE_VETOED_CALL: WIN32_ERROR =
                        WIN32_ERROR(5955u32);
                    pub const ERROR_RESMON_SYSTEM_RESOURCES_LACKING: WIN32_ERROR =
                        WIN32_ERROR(5956u32);
                    pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_DESTINATION : WIN32_ERROR = WIN32_ERROR ( 5957u32 ) ;
                    pub const ERROR_CLUSTER_RESOURCE_VETOED_MOVE_NOT_ENOUGH_RESOURCES_ON_SOURCE:
                        WIN32_ERROR = WIN32_ERROR(5958u32);
                    pub const ERROR_CLUSTER_GROUP_QUEUED: WIN32_ERROR = WIN32_ERROR(5959u32);
                    pub const ERROR_CLUSTER_RESOURCE_LOCKED_STATUS: WIN32_ERROR =
                        WIN32_ERROR(5960u32);
                    pub const ERROR_CLUSTER_SHARED_VOLUME_FAILOVER_NOT_ALLOWED: WIN32_ERROR =
                        WIN32_ERROR(5961u32);
                    pub const ERROR_CLUSTER_NODE_DRAIN_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(5962u32);
                    pub const ERROR_CLUSTER_DISK_NOT_CONNECTED: WIN32_ERROR = WIN32_ERROR(5963u32);
                    pub const ERROR_DISK_NOT_CSV_CAPABLE: WIN32_ERROR = WIN32_ERROR(5964u32);
                    pub const ERROR_RESOURCE_NOT_IN_AVAILABLE_STORAGE: WIN32_ERROR =
                        WIN32_ERROR(5965u32);
                    pub const ERROR_CLUSTER_SHARED_VOLUME_REDIRECTED: WIN32_ERROR =
                        WIN32_ERROR(5966u32);
                    pub const ERROR_CLUSTER_SHARED_VOLUME_NOT_REDIRECTED: WIN32_ERROR =
                        WIN32_ERROR(5967u32);
                    pub const ERROR_CLUSTER_CANNOT_RETURN_PROPERTIES: WIN32_ERROR =
                        WIN32_ERROR(5968u32);
                    pub const ERROR_CLUSTER_RESOURCE_CONTAINS_UNSUPPORTED_DIFF_AREA_FOR_SHARED_VOLUMES : WIN32_ERROR = WIN32_ERROR ( 5969u32 ) ;
                    pub const ERROR_CLUSTER_RESOURCE_IS_IN_MAINTENANCE_MODE: WIN32_ERROR =
                        WIN32_ERROR(5970u32);
                    pub const ERROR_CLUSTER_AFFINITY_CONFLICT: WIN32_ERROR = WIN32_ERROR(5971u32);
                    pub const ERROR_CLUSTER_RESOURCE_IS_REPLICA_VIRTUAL_MACHINE: WIN32_ERROR =
                        WIN32_ERROR(5972u32);
                    pub const ERROR_CLUSTER_UPGRADE_INCOMPATIBLE_VERSIONS: WIN32_ERROR =
                        WIN32_ERROR(5973u32);
                    pub const ERROR_CLUSTER_UPGRADE_FIX_QUORUM_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(5974u32);
                    pub const ERROR_CLUSTER_UPGRADE_RESTART_REQUIRED: WIN32_ERROR =
                        WIN32_ERROR(5975u32);
                    pub const ERROR_CLUSTER_UPGRADE_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(5976u32);
                    pub const ERROR_CLUSTER_UPGRADE_INCOMPLETE: WIN32_ERROR = WIN32_ERROR(5977u32);
                    pub const ERROR_CLUSTER_NODE_IN_GRACE_PERIOD: WIN32_ERROR =
                        WIN32_ERROR(5978u32);
                    pub const ERROR_CLUSTER_CSV_IO_PAUSE_TIMEOUT: WIN32_ERROR =
                        WIN32_ERROR(5979u32);
                    pub const ERROR_NODE_NOT_ACTIVE_CLUSTER_MEMBER: WIN32_ERROR =
                        WIN32_ERROR(5980u32);
                    pub const ERROR_CLUSTER_RESOURCE_NOT_MONITORED: WIN32_ERROR =
                        WIN32_ERROR(5981u32);
                    pub const ERROR_CLUSTER_RESOURCE_DOES_NOT_SUPPORT_UNMONITORED: WIN32_ERROR =
                        WIN32_ERROR(5982u32);
                    pub const ERROR_CLUSTER_RESOURCE_IS_REPLICATED: WIN32_ERROR =
                        WIN32_ERROR(5983u32);
                    pub const ERROR_CLUSTER_NODE_ISOLATED: WIN32_ERROR = WIN32_ERROR(5984u32);
                    pub const ERROR_CLUSTER_NODE_QUARANTINED: WIN32_ERROR = WIN32_ERROR(5985u32);
                    pub const ERROR_CLUSTER_DATABASE_UPDATE_CONDITION_FAILED: WIN32_ERROR =
                        WIN32_ERROR(5986u32);
                    pub const ERROR_CLUSTER_SPACE_DEGRADED: WIN32_ERROR = WIN32_ERROR(5987u32);
                    pub const ERROR_CLUSTER_TOKEN_DELEGATION_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(5988u32);
                    pub const ERROR_CLUSTER_CSV_INVALID_HANDLE: WIN32_ERROR = WIN32_ERROR(5989u32);
                    pub const ERROR_CLUSTER_CSV_SUPPORTED_ONLY_ON_COORDINATOR: WIN32_ERROR =
                        WIN32_ERROR(5990u32);
                    pub const ERROR_GROUPSET_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(5991u32);
                    pub const ERROR_GROUPSET_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(5992u32);
                    pub const ERROR_GROUPSET_CANT_PROVIDE: WIN32_ERROR = WIN32_ERROR(5993u32);
                    pub const ERROR_CLUSTER_FAULT_DOMAIN_PARENT_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(5994u32);
                    pub const ERROR_CLUSTER_FAULT_DOMAIN_INVALID_HIERARCHY: WIN32_ERROR =
                        WIN32_ERROR(5995u32);
                    pub const ERROR_CLUSTER_FAULT_DOMAIN_FAILED_S2D_VALIDATION: WIN32_ERROR =
                        WIN32_ERROR(5996u32);
                    pub const ERROR_CLUSTER_FAULT_DOMAIN_S2D_CONNECTIVITY_LOSS: WIN32_ERROR =
                        WIN32_ERROR(5997u32);
                    pub const ERROR_CLUSTER_INVALID_INFRASTRUCTURE_FILESERVER_NAME: WIN32_ERROR =
                        WIN32_ERROR(5998u32);
                    pub const ERROR_CLUSTERSET_MANAGEMENT_CLUSTER_UNREACHABLE: WIN32_ERROR =
                        WIN32_ERROR(5999u32);
                    pub const ERROR_ENCRYPTION_FAILED: WIN32_ERROR = WIN32_ERROR(6000u32);
                    pub const ERROR_DECRYPTION_FAILED: WIN32_ERROR = WIN32_ERROR(6001u32);
                    pub const ERROR_FILE_ENCRYPTED: WIN32_ERROR = WIN32_ERROR(6002u32);
                    pub const ERROR_NO_RECOVERY_POLICY: WIN32_ERROR = WIN32_ERROR(6003u32);
                    pub const ERROR_NO_EFS: WIN32_ERROR = WIN32_ERROR(6004u32);
                    pub const ERROR_WRONG_EFS: WIN32_ERROR = WIN32_ERROR(6005u32);
                    pub const ERROR_NO_USER_KEYS: WIN32_ERROR = WIN32_ERROR(6006u32);
                    pub const ERROR_FILE_NOT_ENCRYPTED: WIN32_ERROR = WIN32_ERROR(6007u32);
                    pub const ERROR_NOT_EXPORT_FORMAT: WIN32_ERROR = WIN32_ERROR(6008u32);
                    pub const ERROR_FILE_READ_ONLY: WIN32_ERROR = WIN32_ERROR(6009u32);
                    pub const ERROR_DIR_EFS_DISALLOWED: WIN32_ERROR = WIN32_ERROR(6010u32);
                    pub const ERROR_EFS_SERVER_NOT_TRUSTED: WIN32_ERROR = WIN32_ERROR(6011u32);
                    pub const ERROR_BAD_RECOVERY_POLICY: WIN32_ERROR = WIN32_ERROR(6012u32);
                    pub const ERROR_EFS_ALG_BLOB_TOO_BIG: WIN32_ERROR = WIN32_ERROR(6013u32);
                    pub const ERROR_VOLUME_NOT_SUPPORT_EFS: WIN32_ERROR = WIN32_ERROR(6014u32);
                    pub const ERROR_EFS_DISABLED: WIN32_ERROR = WIN32_ERROR(6015u32);
                    pub const ERROR_EFS_VERSION_NOT_SUPPORT: WIN32_ERROR = WIN32_ERROR(6016u32);
                    pub const ERROR_CS_ENCRYPTION_INVALID_SERVER_RESPONSE: WIN32_ERROR =
                        WIN32_ERROR(6017u32);
                    pub const ERROR_CS_ENCRYPTION_UNSUPPORTED_SERVER: WIN32_ERROR =
                        WIN32_ERROR(6018u32);
                    pub const ERROR_CS_ENCRYPTION_EXISTING_ENCRYPTED_FILE: WIN32_ERROR =
                        WIN32_ERROR(6019u32);
                    pub const ERROR_CS_ENCRYPTION_NEW_ENCRYPTED_FILE: WIN32_ERROR =
                        WIN32_ERROR(6020u32);
                    pub const ERROR_CS_ENCRYPTION_FILE_NOT_CSE: WIN32_ERROR = WIN32_ERROR(6021u32);
                    pub const ERROR_ENCRYPTION_POLICY_DENIES_OPERATION: WIN32_ERROR =
                        WIN32_ERROR(6022u32);
                    pub const ERROR_WIP_ENCRYPTION_FAILED: WIN32_ERROR = WIN32_ERROR(6023u32);
                    pub const ERROR_NO_BROWSER_SERVERS_FOUND: WIN32_ERROR = WIN32_ERROR(6118u32);
                    pub const ERROR_CLUSTER_OBJECT_IS_CLUSTER_SET_VM: WIN32_ERROR =
                        WIN32_ERROR(6250u32);
                    pub const ERROR_LOG_SECTOR_INVALID: WIN32_ERROR = WIN32_ERROR(6600u32);
                    pub const ERROR_LOG_SECTOR_PARITY_INVALID: WIN32_ERROR = WIN32_ERROR(6601u32);
                    pub const ERROR_LOG_SECTOR_REMAPPED: WIN32_ERROR = WIN32_ERROR(6602u32);
                    pub const ERROR_LOG_BLOCK_INCOMPLETE: WIN32_ERROR = WIN32_ERROR(6603u32);
                    pub const ERROR_LOG_INVALID_RANGE: WIN32_ERROR = WIN32_ERROR(6604u32);
                    pub const ERROR_LOG_BLOCKS_EXHAUSTED: WIN32_ERROR = WIN32_ERROR(6605u32);
                    pub const ERROR_LOG_READ_CONTEXT_INVALID: WIN32_ERROR = WIN32_ERROR(6606u32);
                    pub const ERROR_LOG_RESTART_INVALID: WIN32_ERROR = WIN32_ERROR(6607u32);
                    pub const ERROR_LOG_BLOCK_VERSION: WIN32_ERROR = WIN32_ERROR(6608u32);
                    pub const ERROR_LOG_BLOCK_INVALID: WIN32_ERROR = WIN32_ERROR(6609u32);
                    pub const ERROR_LOG_READ_MODE_INVALID: WIN32_ERROR = WIN32_ERROR(6610u32);
                    pub const ERROR_LOG_NO_RESTART: WIN32_ERROR = WIN32_ERROR(6611u32);
                    pub const ERROR_LOG_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(6612u32);
                    pub const ERROR_LOG_METADATA_INVALID: WIN32_ERROR = WIN32_ERROR(6613u32);
                    pub const ERROR_LOG_METADATA_INCONSISTENT: WIN32_ERROR = WIN32_ERROR(6614u32);
                    pub const ERROR_LOG_RESERVATION_INVALID: WIN32_ERROR = WIN32_ERROR(6615u32);
                    pub const ERROR_LOG_CANT_DELETE: WIN32_ERROR = WIN32_ERROR(6616u32);
                    pub const ERROR_LOG_CONTAINER_LIMIT_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(6617u32);
                    pub const ERROR_LOG_START_OF_LOG: WIN32_ERROR = WIN32_ERROR(6618u32);
                    pub const ERROR_LOG_POLICY_ALREADY_INSTALLED: WIN32_ERROR =
                        WIN32_ERROR(6619u32);
                    pub const ERROR_LOG_POLICY_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(6620u32);
                    pub const ERROR_LOG_POLICY_INVALID: WIN32_ERROR = WIN32_ERROR(6621u32);
                    pub const ERROR_LOG_POLICY_CONFLICT: WIN32_ERROR = WIN32_ERROR(6622u32);
                    pub const ERROR_LOG_PINNED_ARCHIVE_TAIL: WIN32_ERROR = WIN32_ERROR(6623u32);
                    pub const ERROR_LOG_RECORD_NONEXISTENT: WIN32_ERROR = WIN32_ERROR(6624u32);
                    pub const ERROR_LOG_RECORDS_RESERVED_INVALID: WIN32_ERROR =
                        WIN32_ERROR(6625u32);
                    pub const ERROR_LOG_SPACE_RESERVED_INVALID: WIN32_ERROR = WIN32_ERROR(6626u32);
                    pub const ERROR_LOG_TAIL_INVALID: WIN32_ERROR = WIN32_ERROR(6627u32);
                    pub const ERROR_LOG_FULL: WIN32_ERROR = WIN32_ERROR(6628u32);
                    pub const ERROR_COULD_NOT_RESIZE_LOG: WIN32_ERROR = WIN32_ERROR(6629u32);
                    pub const ERROR_LOG_MULTIPLEXED: WIN32_ERROR = WIN32_ERROR(6630u32);
                    pub const ERROR_LOG_DEDICATED: WIN32_ERROR = WIN32_ERROR(6631u32);
                    pub const ERROR_LOG_ARCHIVE_NOT_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(6632u32);
                    pub const ERROR_LOG_ARCHIVE_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(6633u32);
                    pub const ERROR_LOG_EPHEMERAL: WIN32_ERROR = WIN32_ERROR(6634u32);
                    pub const ERROR_LOG_NOT_ENOUGH_CONTAINERS: WIN32_ERROR = WIN32_ERROR(6635u32);
                    pub const ERROR_LOG_CLIENT_ALREADY_REGISTERED: WIN32_ERROR =
                        WIN32_ERROR(6636u32);
                    pub const ERROR_LOG_CLIENT_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(6637u32);
                    pub const ERROR_LOG_FULL_HANDLER_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(6638u32);
                    pub const ERROR_LOG_CONTAINER_READ_FAILED: WIN32_ERROR = WIN32_ERROR(6639u32);
                    pub const ERROR_LOG_CONTAINER_WRITE_FAILED: WIN32_ERROR = WIN32_ERROR(6640u32);
                    pub const ERROR_LOG_CONTAINER_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(6641u32);
                    pub const ERROR_LOG_CONTAINER_STATE_INVALID: WIN32_ERROR = WIN32_ERROR(6642u32);
                    pub const ERROR_LOG_STATE_INVALID: WIN32_ERROR = WIN32_ERROR(6643u32);
                    pub const ERROR_LOG_PINNED: WIN32_ERROR = WIN32_ERROR(6644u32);
                    pub const ERROR_LOG_METADATA_FLUSH_FAILED: WIN32_ERROR = WIN32_ERROR(6645u32);
                    pub const ERROR_LOG_INCONSISTENT_SECURITY: WIN32_ERROR = WIN32_ERROR(6646u32);
                    pub const ERROR_LOG_APPENDED_FLUSH_FAILED: WIN32_ERROR = WIN32_ERROR(6647u32);
                    pub const ERROR_LOG_PINNED_RESERVATION: WIN32_ERROR = WIN32_ERROR(6648u32);
                    pub const ERROR_INVALID_TRANSACTION: WIN32_ERROR = WIN32_ERROR(6700u32);
                    pub const ERROR_TRANSACTION_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(6701u32);
                    pub const ERROR_TRANSACTION_REQUEST_NOT_VALID: WIN32_ERROR =
                        WIN32_ERROR(6702u32);
                    pub const ERROR_TRANSACTION_NOT_REQUESTED: WIN32_ERROR = WIN32_ERROR(6703u32);
                    pub const ERROR_TRANSACTION_ALREADY_ABORTED: WIN32_ERROR = WIN32_ERROR(6704u32);
                    pub const ERROR_TRANSACTION_ALREADY_COMMITTED: WIN32_ERROR =
                        WIN32_ERROR(6705u32);
                    pub const ERROR_TM_INITIALIZATION_FAILED: WIN32_ERROR = WIN32_ERROR(6706u32);
                    pub const ERROR_RESOURCEMANAGER_READ_ONLY: WIN32_ERROR = WIN32_ERROR(6707u32);
                    pub const ERROR_TRANSACTION_NOT_JOINED: WIN32_ERROR = WIN32_ERROR(6708u32);
                    pub const ERROR_TRANSACTION_SUPERIOR_EXISTS: WIN32_ERROR = WIN32_ERROR(6709u32);
                    pub const ERROR_CRM_PROTOCOL_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(6710u32);
                    pub const ERROR_TRANSACTION_PROPAGATION_FAILED: WIN32_ERROR =
                        WIN32_ERROR(6711u32);
                    pub const ERROR_CRM_PROTOCOL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6712u32);
                    pub const ERROR_TRANSACTION_INVALID_MARSHALL_BUFFER: WIN32_ERROR =
                        WIN32_ERROR(6713u32);
                    pub const ERROR_CURRENT_TRANSACTION_NOT_VALID: WIN32_ERROR =
                        WIN32_ERROR(6714u32);
                    pub const ERROR_TRANSACTION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6715u32);
                    pub const ERROR_RESOURCEMANAGER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6716u32);
                    pub const ERROR_ENLISTMENT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(6717u32);
                    pub const ERROR_TRANSACTIONMANAGER_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(6718u32);
                    pub const ERROR_TRANSACTIONMANAGER_NOT_ONLINE: WIN32_ERROR =
                        WIN32_ERROR(6719u32);
                    pub const ERROR_TRANSACTIONMANAGER_RECOVERY_NAME_COLLISION: WIN32_ERROR =
                        WIN32_ERROR(6720u32);
                    pub const ERROR_TRANSACTION_NOT_ROOT: WIN32_ERROR = WIN32_ERROR(6721u32);
                    pub const ERROR_TRANSACTION_OBJECT_EXPIRED: WIN32_ERROR = WIN32_ERROR(6722u32);
                    pub const ERROR_TRANSACTION_RESPONSE_NOT_ENLISTED: WIN32_ERROR =
                        WIN32_ERROR(6723u32);
                    pub const ERROR_TRANSACTION_RECORD_TOO_LONG: WIN32_ERROR = WIN32_ERROR(6724u32);
                    pub const ERROR_IMPLICIT_TRANSACTION_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(6725u32);
                    pub const ERROR_TRANSACTION_INTEGRITY_VIOLATED: WIN32_ERROR =
                        WIN32_ERROR(6726u32);
                    pub const ERROR_TRANSACTIONMANAGER_IDENTITY_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(6727u32);
                    pub const ERROR_RM_CANNOT_BE_FROZEN_FOR_SNAPSHOT: WIN32_ERROR =
                        WIN32_ERROR(6728u32);
                    pub const ERROR_TRANSACTION_MUST_WRITETHROUGH: WIN32_ERROR =
                        WIN32_ERROR(6729u32);
                    pub const ERROR_TRANSACTION_NO_SUPERIOR: WIN32_ERROR = WIN32_ERROR(6730u32);
                    pub const ERROR_HEURISTIC_DAMAGE_POSSIBLE: WIN32_ERROR = WIN32_ERROR(6731u32);
                    pub const ERROR_TRANSACTIONAL_CONFLICT: WIN32_ERROR = WIN32_ERROR(6800u32);
                    pub const ERROR_RM_NOT_ACTIVE: WIN32_ERROR = WIN32_ERROR(6801u32);
                    pub const ERROR_RM_METADATA_CORRUPT: WIN32_ERROR = WIN32_ERROR(6802u32);
                    pub const ERROR_DIRECTORY_NOT_RM: WIN32_ERROR = WIN32_ERROR(6803u32);
                    pub const ERROR_TRANSACTIONS_UNSUPPORTED_REMOTE: WIN32_ERROR =
                        WIN32_ERROR(6805u32);
                    pub const ERROR_LOG_RESIZE_INVALID_SIZE: WIN32_ERROR = WIN32_ERROR(6806u32);
                    pub const ERROR_OBJECT_NO_LONGER_EXISTS: WIN32_ERROR = WIN32_ERROR(6807u32);
                    pub const ERROR_STREAM_MINIVERSION_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(6808u32);
                    pub const ERROR_STREAM_MINIVERSION_NOT_VALID: WIN32_ERROR =
                        WIN32_ERROR(6809u32);
                    pub const ERROR_MINIVERSION_INACCESSIBLE_FROM_SPECIFIED_TRANSACTION:
                        WIN32_ERROR = WIN32_ERROR(6810u32);
                    pub const ERROR_CANT_OPEN_MINIVERSION_WITH_MODIFY_INTENT: WIN32_ERROR =
                        WIN32_ERROR(6811u32);
                    pub const ERROR_CANT_CREATE_MORE_STREAM_MINIVERSIONS: WIN32_ERROR =
                        WIN32_ERROR(6812u32);
                    pub const ERROR_REMOTE_FILE_VERSION_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(6814u32);
                    pub const ERROR_HANDLE_NO_LONGER_VALID: WIN32_ERROR = WIN32_ERROR(6815u32);
                    pub const ERROR_NO_TXF_METADATA: WIN32_ERROR = WIN32_ERROR(6816u32);
                    pub const ERROR_LOG_CORRUPTION_DETECTED: WIN32_ERROR = WIN32_ERROR(6817u32);
                    pub const ERROR_CANT_RECOVER_WITH_HANDLE_OPEN: WIN32_ERROR =
                        WIN32_ERROR(6818u32);
                    pub const ERROR_RM_DISCONNECTED: WIN32_ERROR = WIN32_ERROR(6819u32);
                    pub const ERROR_ENLISTMENT_NOT_SUPERIOR: WIN32_ERROR = WIN32_ERROR(6820u32);
                    pub const ERROR_RECOVERY_NOT_NEEDED: WIN32_ERROR = WIN32_ERROR(6821u32);
                    pub const ERROR_RM_ALREADY_STARTED: WIN32_ERROR = WIN32_ERROR(6822u32);
                    pub const ERROR_FILE_IDENTITY_NOT_PERSISTENT: WIN32_ERROR =
                        WIN32_ERROR(6823u32);
                    pub const ERROR_CANT_BREAK_TRANSACTIONAL_DEPENDENCY: WIN32_ERROR =
                        WIN32_ERROR(6824u32);
                    pub const ERROR_CANT_CROSS_RM_BOUNDARY: WIN32_ERROR = WIN32_ERROR(6825u32);
                    pub const ERROR_TXF_DIR_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(6826u32);
                    pub const ERROR_INDOUBT_TRANSACTIONS_EXIST: WIN32_ERROR = WIN32_ERROR(6827u32);
                    pub const ERROR_TM_VOLATILE: WIN32_ERROR = WIN32_ERROR(6828u32);
                    pub const ERROR_ROLLBACK_TIMER_EXPIRED: WIN32_ERROR = WIN32_ERROR(6829u32);
                    pub const ERROR_TXF_ATTRIBUTE_CORRUPT: WIN32_ERROR = WIN32_ERROR(6830u32);
                    pub const ERROR_EFS_NOT_ALLOWED_IN_TRANSACTION: WIN32_ERROR =
                        WIN32_ERROR(6831u32);
                    pub const ERROR_TRANSACTIONAL_OPEN_NOT_ALLOWED: WIN32_ERROR =
                        WIN32_ERROR(6832u32);
                    pub const ERROR_LOG_GROWTH_FAILED: WIN32_ERROR = WIN32_ERROR(6833u32);
                    pub const ERROR_TRANSACTED_MAPPING_UNSUPPORTED_REMOTE: WIN32_ERROR =
                        WIN32_ERROR(6834u32);
                    pub const ERROR_TXF_METADATA_ALREADY_PRESENT: WIN32_ERROR =
                        WIN32_ERROR(6835u32);
                    pub const ERROR_TRANSACTION_SCOPE_CALLBACKS_NOT_SET: WIN32_ERROR =
                        WIN32_ERROR(6836u32);
                    pub const ERROR_TRANSACTION_REQUIRED_PROMOTION: WIN32_ERROR =
                        WIN32_ERROR(6837u32);
                    pub const ERROR_CANNOT_EXECUTE_FILE_IN_TRANSACTION: WIN32_ERROR =
                        WIN32_ERROR(6838u32);
                    pub const ERROR_TRANSACTIONS_NOT_FROZEN: WIN32_ERROR = WIN32_ERROR(6839u32);
                    pub const ERROR_TRANSACTION_FREEZE_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(6840u32);
                    pub const ERROR_NOT_SNAPSHOT_VOLUME: WIN32_ERROR = WIN32_ERROR(6841u32);
                    pub const ERROR_NO_SAVEPOINT_WITH_OPEN_FILES: WIN32_ERROR =
                        WIN32_ERROR(6842u32);
                    pub const ERROR_DATA_LOST_REPAIR: WIN32_ERROR = WIN32_ERROR(6843u32);
                    pub const ERROR_SPARSE_NOT_ALLOWED_IN_TRANSACTION: WIN32_ERROR =
                        WIN32_ERROR(6844u32);
                    pub const ERROR_TM_IDENTITY_MISMATCH: WIN32_ERROR = WIN32_ERROR(6845u32);
                    pub const ERROR_FLOATED_SECTION: WIN32_ERROR = WIN32_ERROR(6846u32);
                    pub const ERROR_CANNOT_ACCEPT_TRANSACTED_WORK: WIN32_ERROR =
                        WIN32_ERROR(6847u32);
                    pub const ERROR_CANNOT_ABORT_TRANSACTIONS: WIN32_ERROR = WIN32_ERROR(6848u32);
                    pub const ERROR_BAD_CLUSTERS: WIN32_ERROR = WIN32_ERROR(6849u32);
                    pub const ERROR_COMPRESSION_NOT_ALLOWED_IN_TRANSACTION: WIN32_ERROR =
                        WIN32_ERROR(6850u32);
                    pub const ERROR_VOLUME_DIRTY: WIN32_ERROR = WIN32_ERROR(6851u32);
                    pub const ERROR_NO_LINK_TRACKING_IN_TRANSACTION: WIN32_ERROR =
                        WIN32_ERROR(6852u32);
                    pub const ERROR_OPERATION_NOT_SUPPORTED_IN_TRANSACTION: WIN32_ERROR =
                        WIN32_ERROR(6853u32);
                    pub const ERROR_EXPIRED_HANDLE: WIN32_ERROR = WIN32_ERROR(6854u32);
                    pub const ERROR_TRANSACTION_NOT_ENLISTED: WIN32_ERROR = WIN32_ERROR(6855u32);
                    pub const ERROR_CTX_WINSTATION_NAME_INVALID: WIN32_ERROR = WIN32_ERROR(7001u32);
                    pub const ERROR_CTX_INVALID_PD: WIN32_ERROR = WIN32_ERROR(7002u32);
                    pub const ERROR_CTX_PD_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7003u32);
                    pub const ERROR_CTX_WD_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7004u32);
                    pub const ERROR_CTX_CANNOT_MAKE_EVENTLOG_ENTRY: WIN32_ERROR =
                        WIN32_ERROR(7005u32);
                    pub const ERROR_CTX_SERVICE_NAME_COLLISION: WIN32_ERROR = WIN32_ERROR(7006u32);
                    pub const ERROR_CTX_CLOSE_PENDING: WIN32_ERROR = WIN32_ERROR(7007u32);
                    pub const ERROR_CTX_NO_OUTBUF: WIN32_ERROR = WIN32_ERROR(7008u32);
                    pub const ERROR_CTX_MODEM_INF_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7009u32);
                    pub const ERROR_CTX_INVALID_MODEMNAME: WIN32_ERROR = WIN32_ERROR(7010u32);
                    pub const ERROR_CTX_MODEM_RESPONSE_ERROR: WIN32_ERROR = WIN32_ERROR(7011u32);
                    pub const ERROR_CTX_MODEM_RESPONSE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(7012u32);
                    pub const ERROR_CTX_MODEM_RESPONSE_NO_CARRIER: WIN32_ERROR =
                        WIN32_ERROR(7013u32);
                    pub const ERROR_CTX_MODEM_RESPONSE_NO_DIALTONE: WIN32_ERROR =
                        WIN32_ERROR(7014u32);
                    pub const ERROR_CTX_MODEM_RESPONSE_BUSY: WIN32_ERROR = WIN32_ERROR(7015u32);
                    pub const ERROR_CTX_MODEM_RESPONSE_VOICE: WIN32_ERROR = WIN32_ERROR(7016u32);
                    pub const ERROR_CTX_TD_ERROR: WIN32_ERROR = WIN32_ERROR(7017u32);
                    pub const ERROR_CTX_WINSTATION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(7022u32);
                    pub const ERROR_CTX_WINSTATION_ALREADY_EXISTS: WIN32_ERROR =
                        WIN32_ERROR(7023u32);
                    pub const ERROR_CTX_WINSTATION_BUSY: WIN32_ERROR = WIN32_ERROR(7024u32);
                    pub const ERROR_CTX_BAD_VIDEO_MODE: WIN32_ERROR = WIN32_ERROR(7025u32);
                    pub const ERROR_CTX_GRAPHICS_INVALID: WIN32_ERROR = WIN32_ERROR(7035u32);
                    pub const ERROR_CTX_LOGON_DISABLED: WIN32_ERROR = WIN32_ERROR(7037u32);
                    pub const ERROR_CTX_NOT_CONSOLE: WIN32_ERROR = WIN32_ERROR(7038u32);
                    pub const ERROR_CTX_CLIENT_QUERY_TIMEOUT: WIN32_ERROR = WIN32_ERROR(7040u32);
                    pub const ERROR_CTX_CONSOLE_DISCONNECT: WIN32_ERROR = WIN32_ERROR(7041u32);
                    pub const ERROR_CTX_CONSOLE_CONNECT: WIN32_ERROR = WIN32_ERROR(7042u32);
                    pub const ERROR_CTX_SHADOW_DENIED: WIN32_ERROR = WIN32_ERROR(7044u32);
                    pub const ERROR_CTX_WINSTATION_ACCESS_DENIED: WIN32_ERROR =
                        WIN32_ERROR(7045u32);
                    pub const ERROR_CTX_INVALID_WD: WIN32_ERROR = WIN32_ERROR(7049u32);
                    pub const ERROR_CTX_SHADOW_INVALID: WIN32_ERROR = WIN32_ERROR(7050u32);
                    pub const ERROR_CTX_SHADOW_DISABLED: WIN32_ERROR = WIN32_ERROR(7051u32);
                    pub const ERROR_CTX_CLIENT_LICENSE_IN_USE: WIN32_ERROR = WIN32_ERROR(7052u32);
                    pub const ERROR_CTX_CLIENT_LICENSE_NOT_SET: WIN32_ERROR = WIN32_ERROR(7053u32);
                    pub const ERROR_CTX_LICENSE_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(7054u32);
                    pub const ERROR_CTX_LICENSE_CLIENT_INVALID: WIN32_ERROR = WIN32_ERROR(7055u32);
                    pub const ERROR_CTX_LICENSE_EXPIRED: WIN32_ERROR = WIN32_ERROR(7056u32);
                    pub const ERROR_CTX_SHADOW_NOT_RUNNING: WIN32_ERROR = WIN32_ERROR(7057u32);
                    pub const ERROR_CTX_SHADOW_ENDED_BY_MODE_CHANGE: WIN32_ERROR =
                        WIN32_ERROR(7058u32);
                    pub const ERROR_ACTIVATION_COUNT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(7059u32);
                    pub const ERROR_CTX_WINSTATIONS_DISABLED: WIN32_ERROR = WIN32_ERROR(7060u32);
                    pub const ERROR_CTX_ENCRYPTION_LEVEL_REQUIRED: WIN32_ERROR =
                        WIN32_ERROR(7061u32);
                    pub const ERROR_CTX_SESSION_IN_USE: WIN32_ERROR = WIN32_ERROR(7062u32);
                    pub const ERROR_CTX_NO_FORCE_LOGOFF: WIN32_ERROR = WIN32_ERROR(7063u32);
                    pub const ERROR_CTX_ACCOUNT_RESTRICTION: WIN32_ERROR = WIN32_ERROR(7064u32);
                    pub const ERROR_RDP_PROTOCOL_ERROR: WIN32_ERROR = WIN32_ERROR(7065u32);
                    pub const ERROR_CTX_CDM_CONNECT: WIN32_ERROR = WIN32_ERROR(7066u32);
                    pub const ERROR_CTX_CDM_DISCONNECT: WIN32_ERROR = WIN32_ERROR(7067u32);
                    pub const ERROR_CTX_SECURITY_LAYER_ERROR: WIN32_ERROR = WIN32_ERROR(7068u32);
                    pub const ERROR_TS_INCOMPATIBLE_SESSIONS: WIN32_ERROR = WIN32_ERROR(7069u32);
                    pub const ERROR_TS_VIDEO_SUBSYSTEM_ERROR: WIN32_ERROR = WIN32_ERROR(7070u32);
                    pub const ERROR_DS_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(8200u32);
                    pub const ERROR_DS_MEMBERSHIP_EVALUATED_LOCALLY: WIN32_ERROR =
                        WIN32_ERROR(8201u32);
                    pub const ERROR_DS_NO_ATTRIBUTE_OR_VALUE: WIN32_ERROR = WIN32_ERROR(8202u32);
                    pub const ERROR_DS_INVALID_ATTRIBUTE_SYNTAX: WIN32_ERROR = WIN32_ERROR(8203u32);
                    pub const ERROR_DS_ATTRIBUTE_TYPE_UNDEFINED: WIN32_ERROR = WIN32_ERROR(8204u32);
                    pub const ERROR_DS_ATTRIBUTE_OR_VALUE_EXISTS: WIN32_ERROR =
                        WIN32_ERROR(8205u32);
                    pub const ERROR_DS_BUSY: WIN32_ERROR = WIN32_ERROR(8206u32);
                    pub const ERROR_DS_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(8207u32);
                    pub const ERROR_DS_NO_RIDS_ALLOCATED: WIN32_ERROR = WIN32_ERROR(8208u32);
                    pub const ERROR_DS_NO_MORE_RIDS: WIN32_ERROR = WIN32_ERROR(8209u32);
                    pub const ERROR_DS_INCORRECT_ROLE_OWNER: WIN32_ERROR = WIN32_ERROR(8210u32);
                    pub const ERROR_DS_RIDMGR_INIT_ERROR: WIN32_ERROR = WIN32_ERROR(8211u32);
                    pub const ERROR_DS_OBJ_CLASS_VIOLATION: WIN32_ERROR = WIN32_ERROR(8212u32);
                    pub const ERROR_DS_CANT_ON_NON_LEAF: WIN32_ERROR = WIN32_ERROR(8213u32);
                    pub const ERROR_DS_CANT_ON_RDN: WIN32_ERROR = WIN32_ERROR(8214u32);
                    pub const ERROR_DS_CANT_MOD_OBJ_CLASS: WIN32_ERROR = WIN32_ERROR(8215u32);
                    pub const ERROR_DS_CROSS_DOM_MOVE_ERROR: WIN32_ERROR = WIN32_ERROR(8216u32);
                    pub const ERROR_DS_GC_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(8217u32);
                    pub const ERROR_SHARED_POLICY: WIN32_ERROR = WIN32_ERROR(8218u32);
                    pub const ERROR_POLICY_OBJECT_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8219u32);
                    pub const ERROR_POLICY_ONLY_IN_DS: WIN32_ERROR = WIN32_ERROR(8220u32);
                    pub const ERROR_PROMOTION_ACTIVE: WIN32_ERROR = WIN32_ERROR(8221u32);
                    pub const ERROR_NO_PROMOTION_ACTIVE: WIN32_ERROR = WIN32_ERROR(8222u32);
                    pub const ERROR_DS_OPERATIONS_ERROR: WIN32_ERROR = WIN32_ERROR(8224u32);
                    pub const ERROR_DS_PROTOCOL_ERROR: WIN32_ERROR = WIN32_ERROR(8225u32);
                    pub const ERROR_DS_TIMELIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8226u32);
                    pub const ERROR_DS_SIZELIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8227u32);
                    pub const ERROR_DS_ADMIN_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8228u32);
                    pub const ERROR_DS_COMPARE_FALSE: WIN32_ERROR = WIN32_ERROR(8229u32);
                    pub const ERROR_DS_COMPARE_TRUE: WIN32_ERROR = WIN32_ERROR(8230u32);
                    pub const ERROR_DS_AUTH_METHOD_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(8231u32);
                    pub const ERROR_DS_STRONG_AUTH_REQUIRED: WIN32_ERROR = WIN32_ERROR(8232u32);
                    pub const ERROR_DS_INAPPROPRIATE_AUTH: WIN32_ERROR = WIN32_ERROR(8233u32);
                    pub const ERROR_DS_AUTH_UNKNOWN: WIN32_ERROR = WIN32_ERROR(8234u32);
                    pub const ERROR_DS_REFERRAL: WIN32_ERROR = WIN32_ERROR(8235u32);
                    pub const ERROR_DS_UNAVAILABLE_CRIT_EXTENSION: WIN32_ERROR =
                        WIN32_ERROR(8236u32);
                    pub const ERROR_DS_CONFIDENTIALITY_REQUIRED: WIN32_ERROR = WIN32_ERROR(8237u32);
                    pub const ERROR_DS_INAPPROPRIATE_MATCHING: WIN32_ERROR = WIN32_ERROR(8238u32);
                    pub const ERROR_DS_CONSTRAINT_VIOLATION: WIN32_ERROR = WIN32_ERROR(8239u32);
                    pub const ERROR_DS_NO_SUCH_OBJECT: WIN32_ERROR = WIN32_ERROR(8240u32);
                    pub const ERROR_DS_ALIAS_PROBLEM: WIN32_ERROR = WIN32_ERROR(8241u32);
                    pub const ERROR_DS_INVALID_DN_SYNTAX: WIN32_ERROR = WIN32_ERROR(8242u32);
                    pub const ERROR_DS_IS_LEAF: WIN32_ERROR = WIN32_ERROR(8243u32);
                    pub const ERROR_DS_ALIAS_DEREF_PROBLEM: WIN32_ERROR = WIN32_ERROR(8244u32);
                    pub const ERROR_DS_UNWILLING_TO_PERFORM: WIN32_ERROR = WIN32_ERROR(8245u32);
                    pub const ERROR_DS_LOOP_DETECT: WIN32_ERROR = WIN32_ERROR(8246u32);
                    pub const ERROR_DS_NAMING_VIOLATION: WIN32_ERROR = WIN32_ERROR(8247u32);
                    pub const ERROR_DS_OBJECT_RESULTS_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(8248u32);
                    pub const ERROR_DS_AFFECTS_MULTIPLE_DSAS: WIN32_ERROR = WIN32_ERROR(8249u32);
                    pub const ERROR_DS_SERVER_DOWN: WIN32_ERROR = WIN32_ERROR(8250u32);
                    pub const ERROR_DS_LOCAL_ERROR: WIN32_ERROR = WIN32_ERROR(8251u32);
                    pub const ERROR_DS_ENCODING_ERROR: WIN32_ERROR = WIN32_ERROR(8252u32);
                    pub const ERROR_DS_DECODING_ERROR: WIN32_ERROR = WIN32_ERROR(8253u32);
                    pub const ERROR_DS_FILTER_UNKNOWN: WIN32_ERROR = WIN32_ERROR(8254u32);
                    pub const ERROR_DS_PARAM_ERROR: WIN32_ERROR = WIN32_ERROR(8255u32);
                    pub const ERROR_DS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(8256u32);
                    pub const ERROR_DS_NO_RESULTS_RETURNED: WIN32_ERROR = WIN32_ERROR(8257u32);
                    pub const ERROR_DS_CONTROL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8258u32);
                    pub const ERROR_DS_CLIENT_LOOP: WIN32_ERROR = WIN32_ERROR(8259u32);
                    pub const ERROR_DS_REFERRAL_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8260u32);
                    pub const ERROR_DS_SORT_CONTROL_MISSING: WIN32_ERROR = WIN32_ERROR(8261u32);
                    pub const ERROR_DS_OFFSET_RANGE_ERROR: WIN32_ERROR = WIN32_ERROR(8262u32);
                    pub const ERROR_DS_RIDMGR_DISABLED: WIN32_ERROR = WIN32_ERROR(8263u32);
                    pub const ERROR_DS_ROOT_MUST_BE_NC: WIN32_ERROR = WIN32_ERROR(8301u32);
                    pub const ERROR_DS_ADD_REPLICA_INHIBITED: WIN32_ERROR = WIN32_ERROR(8302u32);
                    pub const ERROR_DS_ATT_NOT_DEF_IN_SCHEMA: WIN32_ERROR = WIN32_ERROR(8303u32);
                    pub const ERROR_DS_MAX_OBJ_SIZE_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8304u32);
                    pub const ERROR_DS_OBJ_STRING_NAME_EXISTS: WIN32_ERROR = WIN32_ERROR(8305u32);
                    pub const ERROR_DS_NO_RDN_DEFINED_IN_SCHEMA: WIN32_ERROR = WIN32_ERROR(8306u32);
                    pub const ERROR_DS_RDN_DOESNT_MATCH_SCHEMA: WIN32_ERROR = WIN32_ERROR(8307u32);
                    pub const ERROR_DS_NO_REQUESTED_ATTS_FOUND: WIN32_ERROR = WIN32_ERROR(8308u32);
                    pub const ERROR_DS_USER_BUFFER_TO_SMALL: WIN32_ERROR = WIN32_ERROR(8309u32);
                    pub const ERROR_DS_ATT_IS_NOT_ON_OBJ: WIN32_ERROR = WIN32_ERROR(8310u32);
                    pub const ERROR_DS_ILLEGAL_MOD_OPERATION: WIN32_ERROR = WIN32_ERROR(8311u32);
                    pub const ERROR_DS_OBJ_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(8312u32);
                    pub const ERROR_DS_BAD_INSTANCE_TYPE: WIN32_ERROR = WIN32_ERROR(8313u32);
                    pub const ERROR_DS_MASTERDSA_REQUIRED: WIN32_ERROR = WIN32_ERROR(8314u32);
                    pub const ERROR_DS_OBJECT_CLASS_REQUIRED: WIN32_ERROR = WIN32_ERROR(8315u32);
                    pub const ERROR_DS_MISSING_REQUIRED_ATT: WIN32_ERROR = WIN32_ERROR(8316u32);
                    pub const ERROR_DS_ATT_NOT_DEF_FOR_CLASS: WIN32_ERROR = WIN32_ERROR(8317u32);
                    pub const ERROR_DS_ATT_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(8318u32);
                    pub const ERROR_DS_CANT_ADD_ATT_VALUES: WIN32_ERROR = WIN32_ERROR(8320u32);
                    pub const ERROR_DS_SINGLE_VALUE_CONSTRAINT: WIN32_ERROR = WIN32_ERROR(8321u32);
                    pub const ERROR_DS_RANGE_CONSTRAINT: WIN32_ERROR = WIN32_ERROR(8322u32);
                    pub const ERROR_DS_ATT_VAL_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(8323u32);
                    pub const ERROR_DS_CANT_REM_MISSING_ATT: WIN32_ERROR = WIN32_ERROR(8324u32);
                    pub const ERROR_DS_CANT_REM_MISSING_ATT_VAL: WIN32_ERROR = WIN32_ERROR(8325u32);
                    pub const ERROR_DS_ROOT_CANT_BE_SUBREF: WIN32_ERROR = WIN32_ERROR(8326u32);
                    pub const ERROR_DS_NO_CHAINING: WIN32_ERROR = WIN32_ERROR(8327u32);
                    pub const ERROR_DS_NO_CHAINED_EVAL: WIN32_ERROR = WIN32_ERROR(8328u32);
                    pub const ERROR_DS_NO_PARENT_OBJECT: WIN32_ERROR = WIN32_ERROR(8329u32);
                    pub const ERROR_DS_PARENT_IS_AN_ALIAS: WIN32_ERROR = WIN32_ERROR(8330u32);
                    pub const ERROR_DS_CANT_MIX_MASTER_AND_REPS: WIN32_ERROR = WIN32_ERROR(8331u32);
                    pub const ERROR_DS_CHILDREN_EXIST: WIN32_ERROR = WIN32_ERROR(8332u32);
                    pub const ERROR_DS_OBJ_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8333u32);
                    pub const ERROR_DS_ALIASED_OBJ_MISSING: WIN32_ERROR = WIN32_ERROR(8334u32);
                    pub const ERROR_DS_BAD_NAME_SYNTAX: WIN32_ERROR = WIN32_ERROR(8335u32);
                    pub const ERROR_DS_ALIAS_POINTS_TO_ALIAS: WIN32_ERROR = WIN32_ERROR(8336u32);
                    pub const ERROR_DS_CANT_DEREF_ALIAS: WIN32_ERROR = WIN32_ERROR(8337u32);
                    pub const ERROR_DS_OUT_OF_SCOPE: WIN32_ERROR = WIN32_ERROR(8338u32);
                    pub const ERROR_DS_OBJECT_BEING_REMOVED: WIN32_ERROR = WIN32_ERROR(8339u32);
                    pub const ERROR_DS_CANT_DELETE_DSA_OBJ: WIN32_ERROR = WIN32_ERROR(8340u32);
                    pub const ERROR_DS_GENERIC_ERROR: WIN32_ERROR = WIN32_ERROR(8341u32);
                    pub const ERROR_DS_DSA_MUST_BE_INT_MASTER: WIN32_ERROR = WIN32_ERROR(8342u32);
                    pub const ERROR_DS_CLASS_NOT_DSA: WIN32_ERROR = WIN32_ERROR(8343u32);
                    pub const ERROR_DS_INSUFF_ACCESS_RIGHTS: WIN32_ERROR = WIN32_ERROR(8344u32);
                    pub const ERROR_DS_ILLEGAL_SUPERIOR: WIN32_ERROR = WIN32_ERROR(8345u32);
                    pub const ERROR_DS_ATTRIBUTE_OWNED_BY_SAM: WIN32_ERROR = WIN32_ERROR(8346u32);
                    pub const ERROR_DS_NAME_TOO_MANY_PARTS: WIN32_ERROR = WIN32_ERROR(8347u32);
                    pub const ERROR_DS_NAME_TOO_LONG: WIN32_ERROR = WIN32_ERROR(8348u32);
                    pub const ERROR_DS_NAME_VALUE_TOO_LONG: WIN32_ERROR = WIN32_ERROR(8349u32);
                    pub const ERROR_DS_NAME_UNPARSEABLE: WIN32_ERROR = WIN32_ERROR(8350u32);
                    pub const ERROR_DS_NAME_TYPE_UNKNOWN: WIN32_ERROR = WIN32_ERROR(8351u32);
                    pub const ERROR_DS_NOT_AN_OBJECT: WIN32_ERROR = WIN32_ERROR(8352u32);
                    pub const ERROR_DS_SEC_DESC_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(8353u32);
                    pub const ERROR_DS_SEC_DESC_INVALID: WIN32_ERROR = WIN32_ERROR(8354u32);
                    pub const ERROR_DS_NO_DELETED_NAME: WIN32_ERROR = WIN32_ERROR(8355u32);
                    pub const ERROR_DS_SUBREF_MUST_HAVE_PARENT: WIN32_ERROR = WIN32_ERROR(8356u32);
                    pub const ERROR_DS_NCNAME_MUST_BE_NC: WIN32_ERROR = WIN32_ERROR(8357u32);
                    pub const ERROR_DS_CANT_ADD_SYSTEM_ONLY: WIN32_ERROR = WIN32_ERROR(8358u32);
                    pub const ERROR_DS_CLASS_MUST_BE_CONCRETE: WIN32_ERROR = WIN32_ERROR(8359u32);
                    pub const ERROR_DS_INVALID_DMD: WIN32_ERROR = WIN32_ERROR(8360u32);
                    pub const ERROR_DS_OBJ_GUID_EXISTS: WIN32_ERROR = WIN32_ERROR(8361u32);
                    pub const ERROR_DS_NOT_ON_BACKLINK: WIN32_ERROR = WIN32_ERROR(8362u32);
                    pub const ERROR_DS_NO_CROSSREF_FOR_NC: WIN32_ERROR = WIN32_ERROR(8363u32);
                    pub const ERROR_DS_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(8364u32);
                    pub const ERROR_DS_UNKNOWN_OPERATION: WIN32_ERROR = WIN32_ERROR(8365u32);
                    pub const ERROR_DS_INVALID_ROLE_OWNER: WIN32_ERROR = WIN32_ERROR(8366u32);
                    pub const ERROR_DS_COULDNT_CONTACT_FSMO: WIN32_ERROR = WIN32_ERROR(8367u32);
                    pub const ERROR_DS_CROSS_NC_DN_RENAME: WIN32_ERROR = WIN32_ERROR(8368u32);
                    pub const ERROR_DS_CANT_MOD_SYSTEM_ONLY: WIN32_ERROR = WIN32_ERROR(8369u32);
                    pub const ERROR_DS_REPLICATOR_ONLY: WIN32_ERROR = WIN32_ERROR(8370u32);
                    pub const ERROR_DS_OBJ_CLASS_NOT_DEFINED: WIN32_ERROR = WIN32_ERROR(8371u32);
                    pub const ERROR_DS_OBJ_CLASS_NOT_SUBCLASS: WIN32_ERROR = WIN32_ERROR(8372u32);
                    pub const ERROR_DS_NAME_REFERENCE_INVALID: WIN32_ERROR = WIN32_ERROR(8373u32);
                    pub const ERROR_DS_CROSS_REF_EXISTS: WIN32_ERROR = WIN32_ERROR(8374u32);
                    pub const ERROR_DS_CANT_DEL_MASTER_CROSSREF: WIN32_ERROR = WIN32_ERROR(8375u32);
                    pub const ERROR_DS_SUBTREE_NOTIFY_NOT_NC_HEAD: WIN32_ERROR =
                        WIN32_ERROR(8376u32);
                    pub const ERROR_DS_NOTIFY_FILTER_TOO_COMPLEX: WIN32_ERROR =
                        WIN32_ERROR(8377u32);
                    pub const ERROR_DS_DUP_RDN: WIN32_ERROR = WIN32_ERROR(8378u32);
                    pub const ERROR_DS_DUP_OID: WIN32_ERROR = WIN32_ERROR(8379u32);
                    pub const ERROR_DS_DUP_MAPI_ID: WIN32_ERROR = WIN32_ERROR(8380u32);
                    pub const ERROR_DS_DUP_SCHEMA_ID_GUID: WIN32_ERROR = WIN32_ERROR(8381u32);
                    pub const ERROR_DS_DUP_LDAP_DISPLAY_NAME: WIN32_ERROR = WIN32_ERROR(8382u32);
                    pub const ERROR_DS_SEMANTIC_ATT_TEST: WIN32_ERROR = WIN32_ERROR(8383u32);
                    pub const ERROR_DS_SYNTAX_MISMATCH: WIN32_ERROR = WIN32_ERROR(8384u32);
                    pub const ERROR_DS_EXISTS_IN_MUST_HAVE: WIN32_ERROR = WIN32_ERROR(8385u32);
                    pub const ERROR_DS_EXISTS_IN_MAY_HAVE: WIN32_ERROR = WIN32_ERROR(8386u32);
                    pub const ERROR_DS_NONEXISTENT_MAY_HAVE: WIN32_ERROR = WIN32_ERROR(8387u32);
                    pub const ERROR_DS_NONEXISTENT_MUST_HAVE: WIN32_ERROR = WIN32_ERROR(8388u32);
                    pub const ERROR_DS_AUX_CLS_TEST_FAIL: WIN32_ERROR = WIN32_ERROR(8389u32);
                    pub const ERROR_DS_NONEXISTENT_POSS_SUP: WIN32_ERROR = WIN32_ERROR(8390u32);
                    pub const ERROR_DS_SUB_CLS_TEST_FAIL: WIN32_ERROR = WIN32_ERROR(8391u32);
                    pub const ERROR_DS_BAD_RDN_ATT_ID_SYNTAX: WIN32_ERROR = WIN32_ERROR(8392u32);
                    pub const ERROR_DS_EXISTS_IN_AUX_CLS: WIN32_ERROR = WIN32_ERROR(8393u32);
                    pub const ERROR_DS_EXISTS_IN_SUB_CLS: WIN32_ERROR = WIN32_ERROR(8394u32);
                    pub const ERROR_DS_EXISTS_IN_POSS_SUP: WIN32_ERROR = WIN32_ERROR(8395u32);
                    pub const ERROR_DS_RECALCSCHEMA_FAILED: WIN32_ERROR = WIN32_ERROR(8396u32);
                    pub const ERROR_DS_TREE_DELETE_NOT_FINISHED: WIN32_ERROR = WIN32_ERROR(8397u32);
                    pub const ERROR_DS_CANT_DELETE: WIN32_ERROR = WIN32_ERROR(8398u32);
                    pub const ERROR_DS_ATT_SCHEMA_REQ_ID: WIN32_ERROR = WIN32_ERROR(8399u32);
                    pub const ERROR_DS_BAD_ATT_SCHEMA_SYNTAX: WIN32_ERROR = WIN32_ERROR(8400u32);
                    pub const ERROR_DS_CANT_CACHE_ATT: WIN32_ERROR = WIN32_ERROR(8401u32);
                    pub const ERROR_DS_CANT_CACHE_CLASS: WIN32_ERROR = WIN32_ERROR(8402u32);
                    pub const ERROR_DS_CANT_REMOVE_ATT_CACHE: WIN32_ERROR = WIN32_ERROR(8403u32);
                    pub const ERROR_DS_CANT_REMOVE_CLASS_CACHE: WIN32_ERROR = WIN32_ERROR(8404u32);
                    pub const ERROR_DS_CANT_RETRIEVE_DN: WIN32_ERROR = WIN32_ERROR(8405u32);
                    pub const ERROR_DS_MISSING_SUPREF: WIN32_ERROR = WIN32_ERROR(8406u32);
                    pub const ERROR_DS_CANT_RETRIEVE_INSTANCE: WIN32_ERROR = WIN32_ERROR(8407u32);
                    pub const ERROR_DS_CODE_INCONSISTENCY: WIN32_ERROR = WIN32_ERROR(8408u32);
                    pub const ERROR_DS_DATABASE_ERROR: WIN32_ERROR = WIN32_ERROR(8409u32);
                    pub const ERROR_DS_GOVERNSID_MISSING: WIN32_ERROR = WIN32_ERROR(8410u32);
                    pub const ERROR_DS_MISSING_EXPECTED_ATT: WIN32_ERROR = WIN32_ERROR(8411u32);
                    pub const ERROR_DS_NCNAME_MISSING_CR_REF: WIN32_ERROR = WIN32_ERROR(8412u32);
                    pub const ERROR_DS_SECURITY_CHECKING_ERROR: WIN32_ERROR = WIN32_ERROR(8413u32);
                    pub const ERROR_DS_SCHEMA_NOT_LOADED: WIN32_ERROR = WIN32_ERROR(8414u32);
                    pub const ERROR_DS_SCHEMA_ALLOC_FAILED: WIN32_ERROR = WIN32_ERROR(8415u32);
                    pub const ERROR_DS_ATT_SCHEMA_REQ_SYNTAX: WIN32_ERROR = WIN32_ERROR(8416u32);
                    pub const ERROR_DS_GCVERIFY_ERROR: WIN32_ERROR = WIN32_ERROR(8417u32);
                    pub const ERROR_DS_DRA_SCHEMA_MISMATCH: WIN32_ERROR = WIN32_ERROR(8418u32);
                    pub const ERROR_DS_CANT_FIND_DSA_OBJ: WIN32_ERROR = WIN32_ERROR(8419u32);
                    pub const ERROR_DS_CANT_FIND_EXPECTED_NC: WIN32_ERROR = WIN32_ERROR(8420u32);
                    pub const ERROR_DS_CANT_FIND_NC_IN_CACHE: WIN32_ERROR = WIN32_ERROR(8421u32);
                    pub const ERROR_DS_CANT_RETRIEVE_CHILD: WIN32_ERROR = WIN32_ERROR(8422u32);
                    pub const ERROR_DS_SECURITY_ILLEGAL_MODIFY: WIN32_ERROR = WIN32_ERROR(8423u32);
                    pub const ERROR_DS_CANT_REPLACE_HIDDEN_REC: WIN32_ERROR = WIN32_ERROR(8424u32);
                    pub const ERROR_DS_BAD_HIERARCHY_FILE: WIN32_ERROR = WIN32_ERROR(8425u32);
                    pub const ERROR_DS_BUILD_HIERARCHY_TABLE_FAILED: WIN32_ERROR =
                        WIN32_ERROR(8426u32);
                    pub const ERROR_DS_CONFIG_PARAM_MISSING: WIN32_ERROR = WIN32_ERROR(8427u32);
                    pub const ERROR_DS_COUNTING_AB_INDICES_FAILED: WIN32_ERROR =
                        WIN32_ERROR(8428u32);
                    pub const ERROR_DS_HIERARCHY_TABLE_MALLOC_FAILED: WIN32_ERROR =
                        WIN32_ERROR(8429u32);
                    pub const ERROR_DS_INTERNAL_FAILURE: WIN32_ERROR = WIN32_ERROR(8430u32);
                    pub const ERROR_DS_UNKNOWN_ERROR: WIN32_ERROR = WIN32_ERROR(8431u32);
                    pub const ERROR_DS_ROOT_REQUIRES_CLASS_TOP: WIN32_ERROR = WIN32_ERROR(8432u32);
                    pub const ERROR_DS_REFUSING_FSMO_ROLES: WIN32_ERROR = WIN32_ERROR(8433u32);
                    pub const ERROR_DS_MISSING_FSMO_SETTINGS: WIN32_ERROR = WIN32_ERROR(8434u32);
                    pub const ERROR_DS_UNABLE_TO_SURRENDER_ROLES: WIN32_ERROR =
                        WIN32_ERROR(8435u32);
                    pub const ERROR_DS_DRA_GENERIC: WIN32_ERROR = WIN32_ERROR(8436u32);
                    pub const ERROR_DS_DRA_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(8437u32);
                    pub const ERROR_DS_DRA_BUSY: WIN32_ERROR = WIN32_ERROR(8438u32);
                    pub const ERROR_DS_DRA_BAD_DN: WIN32_ERROR = WIN32_ERROR(8439u32);
                    pub const ERROR_DS_DRA_BAD_NC: WIN32_ERROR = WIN32_ERROR(8440u32);
                    pub const ERROR_DS_DRA_DN_EXISTS: WIN32_ERROR = WIN32_ERROR(8441u32);
                    pub const ERROR_DS_DRA_INTERNAL_ERROR: WIN32_ERROR = WIN32_ERROR(8442u32);
                    pub const ERROR_DS_DRA_INCONSISTENT_DIT: WIN32_ERROR = WIN32_ERROR(8443u32);
                    pub const ERROR_DS_DRA_CONNECTION_FAILED: WIN32_ERROR = WIN32_ERROR(8444u32);
                    pub const ERROR_DS_DRA_BAD_INSTANCE_TYPE: WIN32_ERROR = WIN32_ERROR(8445u32);
                    pub const ERROR_DS_DRA_OUT_OF_MEM: WIN32_ERROR = WIN32_ERROR(8446u32);
                    pub const ERROR_DS_DRA_MAIL_PROBLEM: WIN32_ERROR = WIN32_ERROR(8447u32);
                    pub const ERROR_DS_DRA_REF_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(8448u32);
                    pub const ERROR_DS_DRA_REF_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8449u32);
                    pub const ERROR_DS_DRA_OBJ_IS_REP_SOURCE: WIN32_ERROR = WIN32_ERROR(8450u32);
                    pub const ERROR_DS_DRA_DB_ERROR: WIN32_ERROR = WIN32_ERROR(8451u32);
                    pub const ERROR_DS_DRA_NO_REPLICA: WIN32_ERROR = WIN32_ERROR(8452u32);
                    pub const ERROR_DS_DRA_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(8453u32);
                    pub const ERROR_DS_DRA_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(8454u32);
                    pub const ERROR_DS_DRA_RPC_CANCELLED: WIN32_ERROR = WIN32_ERROR(8455u32);
                    pub const ERROR_DS_DRA_SOURCE_DISABLED: WIN32_ERROR = WIN32_ERROR(8456u32);
                    pub const ERROR_DS_DRA_SINK_DISABLED: WIN32_ERROR = WIN32_ERROR(8457u32);
                    pub const ERROR_DS_DRA_NAME_COLLISION: WIN32_ERROR = WIN32_ERROR(8458u32);
                    pub const ERROR_DS_DRA_SOURCE_REINSTALLED: WIN32_ERROR = WIN32_ERROR(8459u32);
                    pub const ERROR_DS_DRA_MISSING_PARENT: WIN32_ERROR = WIN32_ERROR(8460u32);
                    pub const ERROR_DS_DRA_PREEMPTED: WIN32_ERROR = WIN32_ERROR(8461u32);
                    pub const ERROR_DS_DRA_ABANDON_SYNC: WIN32_ERROR = WIN32_ERROR(8462u32);
                    pub const ERROR_DS_DRA_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(8463u32);
                    pub const ERROR_DS_DRA_INCOMPATIBLE_PARTIAL_SET: WIN32_ERROR =
                        WIN32_ERROR(8464u32);
                    pub const ERROR_DS_DRA_SOURCE_IS_PARTIAL_REPLICA: WIN32_ERROR =
                        WIN32_ERROR(8465u32);
                    pub const ERROR_DS_DRA_EXTN_CONNECTION_FAILED: WIN32_ERROR =
                        WIN32_ERROR(8466u32);
                    pub const ERROR_DS_INSTALL_SCHEMA_MISMATCH: WIN32_ERROR = WIN32_ERROR(8467u32);
                    pub const ERROR_DS_DUP_LINK_ID: WIN32_ERROR = WIN32_ERROR(8468u32);
                    pub const ERROR_DS_NAME_ERROR_RESOLVING: WIN32_ERROR = WIN32_ERROR(8469u32);
                    pub const ERROR_DS_NAME_ERROR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8470u32);
                    pub const ERROR_DS_NAME_ERROR_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8471u32);
                    pub const ERROR_DS_NAME_ERROR_NO_MAPPING: WIN32_ERROR = WIN32_ERROR(8472u32);
                    pub const ERROR_DS_NAME_ERROR_DOMAIN_ONLY: WIN32_ERROR = WIN32_ERROR(8473u32);
                    pub const ERROR_DS_NAME_ERROR_NO_SYNTACTICAL_MAPPING: WIN32_ERROR =
                        WIN32_ERROR(8474u32);
                    pub const ERROR_DS_CONSTRUCTED_ATT_MOD: WIN32_ERROR = WIN32_ERROR(8475u32);
                    pub const ERROR_DS_WRONG_OM_OBJ_CLASS: WIN32_ERROR = WIN32_ERROR(8476u32);
                    pub const ERROR_DS_DRA_REPL_PENDING: WIN32_ERROR = WIN32_ERROR(8477u32);
                    pub const ERROR_DS_DS_REQUIRED: WIN32_ERROR = WIN32_ERROR(8478u32);
                    pub const ERROR_DS_INVALID_LDAP_DISPLAY_NAME: WIN32_ERROR =
                        WIN32_ERROR(8479u32);
                    pub const ERROR_DS_NON_BASE_SEARCH: WIN32_ERROR = WIN32_ERROR(8480u32);
                    pub const ERROR_DS_CANT_RETRIEVE_ATTS: WIN32_ERROR = WIN32_ERROR(8481u32);
                    pub const ERROR_DS_BACKLINK_WITHOUT_LINK: WIN32_ERROR = WIN32_ERROR(8482u32);
                    pub const ERROR_DS_EPOCH_MISMATCH: WIN32_ERROR = WIN32_ERROR(8483u32);
                    pub const ERROR_DS_SRC_NAME_MISMATCH: WIN32_ERROR = WIN32_ERROR(8484u32);
                    pub const ERROR_DS_SRC_AND_DST_NC_IDENTICAL: WIN32_ERROR = WIN32_ERROR(8485u32);
                    pub const ERROR_DS_DST_NC_MISMATCH: WIN32_ERROR = WIN32_ERROR(8486u32);
                    pub const ERROR_DS_NOT_AUTHORITIVE_FOR_DST_NC: WIN32_ERROR =
                        WIN32_ERROR(8487u32);
                    pub const ERROR_DS_SRC_GUID_MISMATCH: WIN32_ERROR = WIN32_ERROR(8488u32);
                    pub const ERROR_DS_CANT_MOVE_DELETED_OBJECT: WIN32_ERROR = WIN32_ERROR(8489u32);
                    pub const ERROR_DS_PDC_OPERATION_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(8490u32);
                    pub const ERROR_DS_CROSS_DOMAIN_CLEANUP_REQD: WIN32_ERROR =
                        WIN32_ERROR(8491u32);
                    pub const ERROR_DS_ILLEGAL_XDOM_MOVE_OPERATION: WIN32_ERROR =
                        WIN32_ERROR(8492u32);
                    pub const ERROR_DS_CANT_WITH_ACCT_GROUP_MEMBERSHPS: WIN32_ERROR =
                        WIN32_ERROR(8493u32);
                    pub const ERROR_DS_NC_MUST_HAVE_NC_PARENT: WIN32_ERROR = WIN32_ERROR(8494u32);
                    pub const ERROR_DS_CR_IMPOSSIBLE_TO_VALIDATE: WIN32_ERROR =
                        WIN32_ERROR(8495u32);
                    pub const ERROR_DS_DST_DOMAIN_NOT_NATIVE: WIN32_ERROR = WIN32_ERROR(8496u32);
                    pub const ERROR_DS_MISSING_INFRASTRUCTURE_CONTAINER: WIN32_ERROR =
                        WIN32_ERROR(8497u32);
                    pub const ERROR_DS_CANT_MOVE_ACCOUNT_GROUP: WIN32_ERROR = WIN32_ERROR(8498u32);
                    pub const ERROR_DS_CANT_MOVE_RESOURCE_GROUP: WIN32_ERROR = WIN32_ERROR(8499u32);
                    pub const ERROR_DS_INVALID_SEARCH_FLAG: WIN32_ERROR = WIN32_ERROR(8500u32);
                    pub const ERROR_DS_NO_TREE_DELETE_ABOVE_NC: WIN32_ERROR = WIN32_ERROR(8501u32);
                    pub const ERROR_DS_COULDNT_LOCK_TREE_FOR_DELETE: WIN32_ERROR =
                        WIN32_ERROR(8502u32);
                    pub const ERROR_DS_COULDNT_IDENTIFY_OBJECTS_FOR_TREE_DELETE: WIN32_ERROR =
                        WIN32_ERROR(8503u32);
                    pub const ERROR_DS_SAM_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8504u32);
                    pub const ERROR_DS_SENSITIVE_GROUP_VIOLATION: WIN32_ERROR =
                        WIN32_ERROR(8505u32);
                    pub const ERROR_DS_CANT_MOD_PRIMARYGROUPID: WIN32_ERROR = WIN32_ERROR(8506u32);
                    pub const ERROR_DS_ILLEGAL_BASE_SCHEMA_MOD: WIN32_ERROR = WIN32_ERROR(8507u32);
                    pub const ERROR_DS_NONSAFE_SCHEMA_CHANGE: WIN32_ERROR = WIN32_ERROR(8508u32);
                    pub const ERROR_DS_SCHEMA_UPDATE_DISALLOWED: WIN32_ERROR = WIN32_ERROR(8509u32);
                    pub const ERROR_DS_CANT_CREATE_UNDER_SCHEMA: WIN32_ERROR = WIN32_ERROR(8510u32);
                    pub const ERROR_DS_INSTALL_NO_SRC_SCH_VERSION: WIN32_ERROR =
                        WIN32_ERROR(8511u32);
                    pub const ERROR_DS_INSTALL_NO_SCH_VERSION_IN_INIFILE: WIN32_ERROR =
                        WIN32_ERROR(8512u32);
                    pub const ERROR_DS_INVALID_GROUP_TYPE: WIN32_ERROR = WIN32_ERROR(8513u32);
                    pub const ERROR_DS_NO_NEST_GLOBALGROUP_IN_MIXEDDOMAIN: WIN32_ERROR =
                        WIN32_ERROR(8514u32);
                    pub const ERROR_DS_NO_NEST_LOCALGROUP_IN_MIXEDDOMAIN: WIN32_ERROR =
                        WIN32_ERROR(8515u32);
                    pub const ERROR_DS_GLOBAL_CANT_HAVE_LOCAL_MEMBER: WIN32_ERROR =
                        WIN32_ERROR(8516u32);
                    pub const ERROR_DS_GLOBAL_CANT_HAVE_UNIVERSAL_MEMBER: WIN32_ERROR =
                        WIN32_ERROR(8517u32);
                    pub const ERROR_DS_UNIVERSAL_CANT_HAVE_LOCAL_MEMBER: WIN32_ERROR =
                        WIN32_ERROR(8518u32);
                    pub const ERROR_DS_GLOBAL_CANT_HAVE_CROSSDOMAIN_MEMBER: WIN32_ERROR =
                        WIN32_ERROR(8519u32);
                    pub const ERROR_DS_LOCAL_CANT_HAVE_CROSSDOMAIN_LOCAL_MEMBER: WIN32_ERROR =
                        WIN32_ERROR(8520u32);
                    pub const ERROR_DS_HAVE_PRIMARY_MEMBERS: WIN32_ERROR = WIN32_ERROR(8521u32);
                    pub const ERROR_DS_STRING_SD_CONVERSION_FAILED: WIN32_ERROR =
                        WIN32_ERROR(8522u32);
                    pub const ERROR_DS_NAMING_MASTER_GC: WIN32_ERROR = WIN32_ERROR(8523u32);
                    pub const ERROR_DS_DNS_LOOKUP_FAILURE: WIN32_ERROR = WIN32_ERROR(8524u32);
                    pub const ERROR_DS_COULDNT_UPDATE_SPNS: WIN32_ERROR = WIN32_ERROR(8525u32);
                    pub const ERROR_DS_CANT_RETRIEVE_SD: WIN32_ERROR = WIN32_ERROR(8526u32);
                    pub const ERROR_DS_KEY_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8527u32);
                    pub const ERROR_DS_WRONG_LINKED_ATT_SYNTAX: WIN32_ERROR = WIN32_ERROR(8528u32);
                    pub const ERROR_DS_SAM_NEED_BOOTKEY_PASSWORD: WIN32_ERROR =
                        WIN32_ERROR(8529u32);
                    pub const ERROR_DS_SAM_NEED_BOOTKEY_FLOPPY: WIN32_ERROR = WIN32_ERROR(8530u32);
                    pub const ERROR_DS_CANT_START: WIN32_ERROR = WIN32_ERROR(8531u32);
                    pub const ERROR_DS_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8532u32);
                    pub const ERROR_DS_NO_PKT_PRIVACY_ON_CONNECTION: WIN32_ERROR =
                        WIN32_ERROR(8533u32);
                    pub const ERROR_DS_SOURCE_DOMAIN_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8534u32);
                    pub const ERROR_DS_DESTINATION_DOMAIN_NOT_IN_FOREST: WIN32_ERROR =
                        WIN32_ERROR(8535u32);
                    pub const ERROR_DS_DESTINATION_AUDITING_NOT_ENABLED: WIN32_ERROR =
                        WIN32_ERROR(8536u32);
                    pub const ERROR_DS_CANT_FIND_DC_FOR_SRC_DOMAIN: WIN32_ERROR =
                        WIN32_ERROR(8537u32);
                    pub const ERROR_DS_SRC_OBJ_NOT_GROUP_OR_USER: WIN32_ERROR =
                        WIN32_ERROR(8538u32);
                    pub const ERROR_DS_SRC_SID_EXISTS_IN_FOREST: WIN32_ERROR = WIN32_ERROR(8539u32);
                    pub const ERROR_DS_SRC_AND_DST_OBJECT_CLASS_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(8540u32);
                    pub const ERROR_SAM_INIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8541u32);
                    pub const ERROR_DS_DRA_SCHEMA_INFO_SHIP: WIN32_ERROR = WIN32_ERROR(8542u32);
                    pub const ERROR_DS_DRA_SCHEMA_CONFLICT: WIN32_ERROR = WIN32_ERROR(8543u32);
                    pub const ERROR_DS_DRA_EARLIER_SCHEMA_CONFLICT: WIN32_ERROR =
                        WIN32_ERROR(8544u32);
                    pub const ERROR_DS_DRA_OBJ_NC_MISMATCH: WIN32_ERROR = WIN32_ERROR(8545u32);
                    pub const ERROR_DS_NC_STILL_HAS_DSAS: WIN32_ERROR = WIN32_ERROR(8546u32);
                    pub const ERROR_DS_GC_REQUIRED: WIN32_ERROR = WIN32_ERROR(8547u32);
                    pub const ERROR_DS_LOCAL_MEMBER_OF_LOCAL_ONLY: WIN32_ERROR =
                        WIN32_ERROR(8548u32);
                    pub const ERROR_DS_NO_FPO_IN_UNIVERSAL_GROUPS: WIN32_ERROR =
                        WIN32_ERROR(8549u32);
                    pub const ERROR_DS_CANT_ADD_TO_GC: WIN32_ERROR = WIN32_ERROR(8550u32);
                    pub const ERROR_DS_NO_CHECKPOINT_WITH_PDC: WIN32_ERROR = WIN32_ERROR(8551u32);
                    pub const ERROR_DS_SOURCE_AUDITING_NOT_ENABLED: WIN32_ERROR =
                        WIN32_ERROR(8552u32);
                    pub const ERROR_DS_CANT_CREATE_IN_NONDOMAIN_NC: WIN32_ERROR =
                        WIN32_ERROR(8553u32);
                    pub const ERROR_DS_INVALID_NAME_FOR_SPN: WIN32_ERROR = WIN32_ERROR(8554u32);
                    pub const ERROR_DS_FILTER_USES_CONTRUCTED_ATTRS: WIN32_ERROR =
                        WIN32_ERROR(8555u32);
                    pub const ERROR_DS_UNICODEPWD_NOT_IN_QUOTES: WIN32_ERROR = WIN32_ERROR(8556u32);
                    pub const ERROR_DS_MACHINE_ACCOUNT_QUOTA_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(8557u32);
                    pub const ERROR_DS_MUST_BE_RUN_ON_DST_DC: WIN32_ERROR = WIN32_ERROR(8558u32);
                    pub const ERROR_DS_SRC_DC_MUST_BE_SP4_OR_GREATER: WIN32_ERROR =
                        WIN32_ERROR(8559u32);
                    pub const ERROR_DS_CANT_TREE_DELETE_CRITICAL_OBJ: WIN32_ERROR =
                        WIN32_ERROR(8560u32);
                    pub const ERROR_DS_INIT_FAILURE_CONSOLE: WIN32_ERROR = WIN32_ERROR(8561u32);
                    pub const ERROR_DS_SAM_INIT_FAILURE_CONSOLE: WIN32_ERROR = WIN32_ERROR(8562u32);
                    pub const ERROR_DS_FOREST_VERSION_TOO_HIGH: WIN32_ERROR = WIN32_ERROR(8563u32);
                    pub const ERROR_DS_DOMAIN_VERSION_TOO_HIGH: WIN32_ERROR = WIN32_ERROR(8564u32);
                    pub const ERROR_DS_FOREST_VERSION_TOO_LOW: WIN32_ERROR = WIN32_ERROR(8565u32);
                    pub const ERROR_DS_DOMAIN_VERSION_TOO_LOW: WIN32_ERROR = WIN32_ERROR(8566u32);
                    pub const ERROR_DS_INCOMPATIBLE_VERSION: WIN32_ERROR = WIN32_ERROR(8567u32);
                    pub const ERROR_DS_LOW_DSA_VERSION: WIN32_ERROR = WIN32_ERROR(8568u32);
                    pub const ERROR_DS_NO_BEHAVIOR_VERSION_IN_MIXEDDOMAIN: WIN32_ERROR =
                        WIN32_ERROR(8569u32);
                    pub const ERROR_DS_NOT_SUPPORTED_SORT_ORDER: WIN32_ERROR = WIN32_ERROR(8570u32);
                    pub const ERROR_DS_NAME_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8571u32);
                    pub const ERROR_DS_MACHINE_ACCOUNT_CREATED_PRENT4: WIN32_ERROR =
                        WIN32_ERROR(8572u32);
                    pub const ERROR_DS_OUT_OF_VERSION_STORE: WIN32_ERROR = WIN32_ERROR(8573u32);
                    pub const ERROR_DS_INCOMPATIBLE_CONTROLS_USED: WIN32_ERROR =
                        WIN32_ERROR(8574u32);
                    pub const ERROR_DS_NO_REF_DOMAIN: WIN32_ERROR = WIN32_ERROR(8575u32);
                    pub const ERROR_DS_RESERVED_LINK_ID: WIN32_ERROR = WIN32_ERROR(8576u32);
                    pub const ERROR_DS_LINK_ID_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(8577u32);
                    pub const ERROR_DS_AG_CANT_HAVE_UNIVERSAL_MEMBER: WIN32_ERROR =
                        WIN32_ERROR(8578u32);
                    pub const ERROR_DS_MODIFYDN_DISALLOWED_BY_INSTANCE_TYPE: WIN32_ERROR =
                        WIN32_ERROR(8579u32);
                    pub const ERROR_DS_NO_OBJECT_MOVE_IN_SCHEMA_NC: WIN32_ERROR =
                        WIN32_ERROR(8580u32);
                    pub const ERROR_DS_MODIFYDN_DISALLOWED_BY_FLAG: WIN32_ERROR =
                        WIN32_ERROR(8581u32);
                    pub const ERROR_DS_MODIFYDN_WRONG_GRANDPARENT: WIN32_ERROR =
                        WIN32_ERROR(8582u32);
                    pub const ERROR_DS_NAME_ERROR_TRUST_REFERRAL: WIN32_ERROR =
                        WIN32_ERROR(8583u32);
                    pub const ERROR_NOT_SUPPORTED_ON_STANDARD_SERVER: WIN32_ERROR =
                        WIN32_ERROR(8584u32);
                    pub const ERROR_DS_CANT_ACCESS_REMOTE_PART_OF_AD: WIN32_ERROR =
                        WIN32_ERROR(8585u32);
                    pub const ERROR_DS_CR_IMPOSSIBLE_TO_VALIDATE_V2: WIN32_ERROR =
                        WIN32_ERROR(8586u32);
                    pub const ERROR_DS_THREAD_LIMIT_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8587u32);
                    pub const ERROR_DS_NOT_CLOSEST: WIN32_ERROR = WIN32_ERROR(8588u32);
                    pub const ERROR_DS_CANT_DERIVE_SPN_WITHOUT_SERVER_REF: WIN32_ERROR =
                        WIN32_ERROR(8589u32);
                    pub const ERROR_DS_SINGLE_USER_MODE_FAILED: WIN32_ERROR = WIN32_ERROR(8590u32);
                    pub const ERROR_DS_NTDSCRIPT_SYNTAX_ERROR: WIN32_ERROR = WIN32_ERROR(8591u32);
                    pub const ERROR_DS_NTDSCRIPT_PROCESS_ERROR: WIN32_ERROR = WIN32_ERROR(8592u32);
                    pub const ERROR_DS_DIFFERENT_REPL_EPOCHS: WIN32_ERROR = WIN32_ERROR(8593u32);
                    pub const ERROR_DS_DRS_EXTENSIONS_CHANGED: WIN32_ERROR = WIN32_ERROR(8594u32);
                    pub const ERROR_DS_REPLICA_SET_CHANGE_NOT_ALLOWED_ON_DISABLED_CR: WIN32_ERROR =
                        WIN32_ERROR(8595u32);
                    pub const ERROR_DS_NO_MSDS_INTID: WIN32_ERROR = WIN32_ERROR(8596u32);
                    pub const ERROR_DS_DUP_MSDS_INTID: WIN32_ERROR = WIN32_ERROR(8597u32);
                    pub const ERROR_DS_EXISTS_IN_RDNATTID: WIN32_ERROR = WIN32_ERROR(8598u32);
                    pub const ERROR_DS_AUTHORIZATION_FAILED: WIN32_ERROR = WIN32_ERROR(8599u32);
                    pub const ERROR_DS_INVALID_SCRIPT: WIN32_ERROR = WIN32_ERROR(8600u32);
                    pub const ERROR_DS_REMOTE_CROSSREF_OP_FAILED: WIN32_ERROR =
                        WIN32_ERROR(8601u32);
                    pub const ERROR_DS_CROSS_REF_BUSY: WIN32_ERROR = WIN32_ERROR(8602u32);
                    pub const ERROR_DS_CANT_DERIVE_SPN_FOR_DELETED_DOMAIN: WIN32_ERROR =
                        WIN32_ERROR(8603u32);
                    pub const ERROR_DS_CANT_DEMOTE_WITH_WRITEABLE_NC: WIN32_ERROR =
                        WIN32_ERROR(8604u32);
                    pub const ERROR_DS_DUPLICATE_ID_FOUND: WIN32_ERROR = WIN32_ERROR(8605u32);
                    pub const ERROR_DS_INSUFFICIENT_ATTR_TO_CREATE_OBJECT: WIN32_ERROR =
                        WIN32_ERROR(8606u32);
                    pub const ERROR_DS_GROUP_CONVERSION_ERROR: WIN32_ERROR = WIN32_ERROR(8607u32);
                    pub const ERROR_DS_CANT_MOVE_APP_BASIC_GROUP: WIN32_ERROR =
                        WIN32_ERROR(8608u32);
                    pub const ERROR_DS_CANT_MOVE_APP_QUERY_GROUP: WIN32_ERROR =
                        WIN32_ERROR(8609u32);
                    pub const ERROR_DS_ROLE_NOT_VERIFIED: WIN32_ERROR = WIN32_ERROR(8610u32);
                    pub const ERROR_DS_WKO_CONTAINER_CANNOT_BE_SPECIAL: WIN32_ERROR =
                        WIN32_ERROR(8611u32);
                    pub const ERROR_DS_DOMAIN_RENAME_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(8612u32);
                    pub const ERROR_DS_EXISTING_AD_CHILD_NC: WIN32_ERROR = WIN32_ERROR(8613u32);
                    pub const ERROR_DS_REPL_LIFETIME_EXCEEDED: WIN32_ERROR = WIN32_ERROR(8614u32);
                    pub const ERROR_DS_DISALLOWED_IN_SYSTEM_CONTAINER: WIN32_ERROR =
                        WIN32_ERROR(8615u32);
                    pub const ERROR_DS_LDAP_SEND_QUEUE_FULL: WIN32_ERROR = WIN32_ERROR(8616u32);
                    pub const ERROR_DS_DRA_OUT_SCHEDULE_WINDOW: WIN32_ERROR = WIN32_ERROR(8617u32);
                    pub const ERROR_DS_POLICY_NOT_KNOWN: WIN32_ERROR = WIN32_ERROR(8618u32);
                    pub const ERROR_NO_SITE_SETTINGS_OBJECT: WIN32_ERROR = WIN32_ERROR(8619u32);
                    pub const ERROR_NO_SECRETS: WIN32_ERROR = WIN32_ERROR(8620u32);
                    pub const ERROR_NO_WRITABLE_DC_FOUND: WIN32_ERROR = WIN32_ERROR(8621u32);
                    pub const ERROR_DS_NO_SERVER_OBJECT: WIN32_ERROR = WIN32_ERROR(8622u32);
                    pub const ERROR_DS_NO_NTDSA_OBJECT: WIN32_ERROR = WIN32_ERROR(8623u32);
                    pub const ERROR_DS_NON_ASQ_SEARCH: WIN32_ERROR = WIN32_ERROR(8624u32);
                    pub const ERROR_DS_AUDIT_FAILURE: WIN32_ERROR = WIN32_ERROR(8625u32);
                    pub const ERROR_DS_INVALID_SEARCH_FLAG_SUBTREE: WIN32_ERROR =
                        WIN32_ERROR(8626u32);
                    pub const ERROR_DS_INVALID_SEARCH_FLAG_TUPLE: WIN32_ERROR =
                        WIN32_ERROR(8627u32);
                    pub const ERROR_DS_HIERARCHY_TABLE_TOO_DEEP: WIN32_ERROR = WIN32_ERROR(8628u32);
                    pub const ERROR_DS_DRA_CORRUPT_UTD_VECTOR: WIN32_ERROR = WIN32_ERROR(8629u32);
                    pub const ERROR_DS_DRA_SECRETS_DENIED: WIN32_ERROR = WIN32_ERROR(8630u32);
                    pub const ERROR_DS_RESERVED_MAPI_ID: WIN32_ERROR = WIN32_ERROR(8631u32);
                    pub const ERROR_DS_MAPI_ID_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(8632u32);
                    pub const ERROR_DS_DRA_MISSING_KRBTGT_SECRET: WIN32_ERROR =
                        WIN32_ERROR(8633u32);
                    pub const ERROR_DS_DOMAIN_NAME_EXISTS_IN_FOREST: WIN32_ERROR =
                        WIN32_ERROR(8634u32);
                    pub const ERROR_DS_FLAT_NAME_EXISTS_IN_FOREST: WIN32_ERROR =
                        WIN32_ERROR(8635u32);
                    pub const ERROR_INVALID_USER_PRINCIPAL_NAME: WIN32_ERROR = WIN32_ERROR(8636u32);
                    pub const ERROR_DS_OID_MAPPED_GROUP_CANT_HAVE_MEMBERS: WIN32_ERROR =
                        WIN32_ERROR(8637u32);
                    pub const ERROR_DS_OID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(8638u32);
                    pub const ERROR_DS_DRA_RECYCLED_TARGET: WIN32_ERROR = WIN32_ERROR(8639u32);
                    pub const ERROR_DS_DISALLOWED_NC_REDIRECT: WIN32_ERROR = WIN32_ERROR(8640u32);
                    pub const ERROR_DS_HIGH_ADLDS_FFL: WIN32_ERROR = WIN32_ERROR(8641u32);
                    pub const ERROR_DS_HIGH_DSA_VERSION: WIN32_ERROR = WIN32_ERROR(8642u32);
                    pub const ERROR_DS_LOW_ADLDS_FFL: WIN32_ERROR = WIN32_ERROR(8643u32);
                    pub const ERROR_DOMAIN_SID_SAME_AS_LOCAL_WORKSTATION: WIN32_ERROR =
                        WIN32_ERROR(8644u32);
                    pub const ERROR_DS_UNDELETE_SAM_VALIDATION_FAILED: WIN32_ERROR =
                        WIN32_ERROR(8645u32);
                    pub const ERROR_INCORRECT_ACCOUNT_TYPE: WIN32_ERROR = WIN32_ERROR(8646u32);
                    pub const ERROR_DS_SPN_VALUE_NOT_UNIQUE_IN_FOREST: WIN32_ERROR =
                        WIN32_ERROR(8647u32);
                    pub const ERROR_DS_UPN_VALUE_NOT_UNIQUE_IN_FOREST: WIN32_ERROR =
                        WIN32_ERROR(8648u32);
                    pub const ERROR_DS_MISSING_FOREST_TRUST: WIN32_ERROR = WIN32_ERROR(8649u32);
                    pub const ERROR_DS_VALUE_KEY_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(8650u32);
                    pub const DNS_ERROR_RESPONSE_CODES_BASE: WIN32_ERROR = WIN32_ERROR(9000u32);
                    pub const DNS_ERROR_RCODE_NO_ERROR: WIN32_ERROR = WIN32_ERROR(0u32);
                    pub const DNS_ERROR_MASK: WIN32_ERROR = WIN32_ERROR(9000u32);
                    pub const DNS_ERROR_RCODE_FORMAT_ERROR: WIN32_ERROR = WIN32_ERROR(9001u32);
                    pub const DNS_ERROR_RCODE_SERVER_FAILURE: WIN32_ERROR = WIN32_ERROR(9002u32);
                    pub const DNS_ERROR_RCODE_NAME_ERROR: WIN32_ERROR = WIN32_ERROR(9003u32);
                    pub const DNS_ERROR_RCODE_NOT_IMPLEMENTED: WIN32_ERROR = WIN32_ERROR(9004u32);
                    pub const DNS_ERROR_RCODE_REFUSED: WIN32_ERROR = WIN32_ERROR(9005u32);
                    pub const DNS_ERROR_RCODE_YXDOMAIN: WIN32_ERROR = WIN32_ERROR(9006u32);
                    pub const DNS_ERROR_RCODE_YXRRSET: WIN32_ERROR = WIN32_ERROR(9007u32);
                    pub const DNS_ERROR_RCODE_NXRRSET: WIN32_ERROR = WIN32_ERROR(9008u32);
                    pub const DNS_ERROR_RCODE_NOTAUTH: WIN32_ERROR = WIN32_ERROR(9009u32);
                    pub const DNS_ERROR_RCODE_NOTZONE: WIN32_ERROR = WIN32_ERROR(9010u32);
                    pub const DNS_ERROR_RCODE_BADSIG: WIN32_ERROR = WIN32_ERROR(9016u32);
                    pub const DNS_ERROR_RCODE_BADKEY: WIN32_ERROR = WIN32_ERROR(9017u32);
                    pub const DNS_ERROR_RCODE_BADTIME: WIN32_ERROR = WIN32_ERROR(9018u32);
                    pub const DNS_ERROR_RCODE_LAST: WIN32_ERROR = WIN32_ERROR(9018u32);
                    pub const DNS_ERROR_DNSSEC_BASE: WIN32_ERROR = WIN32_ERROR(9100u32);
                    pub const DNS_ERROR_KEYMASTER_REQUIRED: WIN32_ERROR = WIN32_ERROR(9101u32);
                    pub const DNS_ERROR_NOT_ALLOWED_ON_SIGNED_ZONE: WIN32_ERROR =
                        WIN32_ERROR(9102u32);
                    pub const DNS_ERROR_NSEC3_INCOMPATIBLE_WITH_RSA_SHA1: WIN32_ERROR =
                        WIN32_ERROR(9103u32);
                    pub const DNS_ERROR_NOT_ENOUGH_SIGNING_KEY_DESCRIPTORS: WIN32_ERROR =
                        WIN32_ERROR(9104u32);
                    pub const DNS_ERROR_UNSUPPORTED_ALGORITHM: WIN32_ERROR = WIN32_ERROR(9105u32);
                    pub const DNS_ERROR_INVALID_KEY_SIZE: WIN32_ERROR = WIN32_ERROR(9106u32);
                    pub const DNS_ERROR_SIGNING_KEY_NOT_ACCESSIBLE: WIN32_ERROR =
                        WIN32_ERROR(9107u32);
                    pub const DNS_ERROR_KSP_DOES_NOT_SUPPORT_PROTECTION: WIN32_ERROR =
                        WIN32_ERROR(9108u32);
                    pub const DNS_ERROR_UNEXPECTED_DATA_PROTECTION_ERROR: WIN32_ERROR =
                        WIN32_ERROR(9109u32);
                    pub const DNS_ERROR_UNEXPECTED_CNG_ERROR: WIN32_ERROR = WIN32_ERROR(9110u32);
                    pub const DNS_ERROR_UNKNOWN_SIGNING_PARAMETER_VERSION: WIN32_ERROR =
                        WIN32_ERROR(9111u32);
                    pub const DNS_ERROR_KSP_NOT_ACCESSIBLE: WIN32_ERROR = WIN32_ERROR(9112u32);
                    pub const DNS_ERROR_TOO_MANY_SKDS: WIN32_ERROR = WIN32_ERROR(9113u32);
                    pub const DNS_ERROR_INVALID_ROLLOVER_PERIOD: WIN32_ERROR = WIN32_ERROR(9114u32);
                    pub const DNS_ERROR_INVALID_INITIAL_ROLLOVER_OFFSET: WIN32_ERROR =
                        WIN32_ERROR(9115u32);
                    pub const DNS_ERROR_ROLLOVER_IN_PROGRESS: WIN32_ERROR = WIN32_ERROR(9116u32);
                    pub const DNS_ERROR_STANDBY_KEY_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(9117u32);
                    pub const DNS_ERROR_NOT_ALLOWED_ON_ZSK: WIN32_ERROR = WIN32_ERROR(9118u32);
                    pub const DNS_ERROR_NOT_ALLOWED_ON_ACTIVE_SKD: WIN32_ERROR =
                        WIN32_ERROR(9119u32);
                    pub const DNS_ERROR_ROLLOVER_ALREADY_QUEUED: WIN32_ERROR = WIN32_ERROR(9120u32);
                    pub const DNS_ERROR_NOT_ALLOWED_ON_UNSIGNED_ZONE: WIN32_ERROR =
                        WIN32_ERROR(9121u32);
                    pub const DNS_ERROR_BAD_KEYMASTER: WIN32_ERROR = WIN32_ERROR(9122u32);
                    pub const DNS_ERROR_INVALID_SIGNATURE_VALIDITY_PERIOD: WIN32_ERROR =
                        WIN32_ERROR(9123u32);
                    pub const DNS_ERROR_INVALID_NSEC3_ITERATION_COUNT: WIN32_ERROR =
                        WIN32_ERROR(9124u32);
                    pub const DNS_ERROR_DNSSEC_IS_DISABLED: WIN32_ERROR = WIN32_ERROR(9125u32);
                    pub const DNS_ERROR_INVALID_XML: WIN32_ERROR = WIN32_ERROR(9126u32);
                    pub const DNS_ERROR_NO_VALID_TRUST_ANCHORS: WIN32_ERROR = WIN32_ERROR(9127u32);
                    pub const DNS_ERROR_ROLLOVER_NOT_POKEABLE: WIN32_ERROR = WIN32_ERROR(9128u32);
                    pub const DNS_ERROR_NSEC3_NAME_COLLISION: WIN32_ERROR = WIN32_ERROR(9129u32);
                    pub const DNS_ERROR_NSEC_INCOMPATIBLE_WITH_NSEC3_RSA_SHA1: WIN32_ERROR =
                        WIN32_ERROR(9130u32);
                    pub const DNS_ERROR_PACKET_FMT_BASE: WIN32_ERROR = WIN32_ERROR(9500u32);
                    pub const DNS_ERROR_BAD_PACKET: WIN32_ERROR = WIN32_ERROR(9502u32);
                    pub const DNS_ERROR_NO_PACKET: WIN32_ERROR = WIN32_ERROR(9503u32);
                    pub const DNS_ERROR_RCODE: WIN32_ERROR = WIN32_ERROR(9504u32);
                    pub const DNS_ERROR_UNSECURE_PACKET: WIN32_ERROR = WIN32_ERROR(9505u32);
                    pub const DNS_ERROR_NO_MEMORY: WIN32_ERROR = WIN32_ERROR(14u32);
                    pub const DNS_ERROR_INVALID_NAME: WIN32_ERROR = WIN32_ERROR(123u32);
                    pub const DNS_ERROR_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(13u32);
                    pub const DNS_ERROR_GENERAL_API_BASE: WIN32_ERROR = WIN32_ERROR(9550u32);
                    pub const DNS_ERROR_INVALID_TYPE: WIN32_ERROR = WIN32_ERROR(9551u32);
                    pub const DNS_ERROR_INVALID_IP_ADDRESS: WIN32_ERROR = WIN32_ERROR(9552u32);
                    pub const DNS_ERROR_INVALID_PROPERTY: WIN32_ERROR = WIN32_ERROR(9553u32);
                    pub const DNS_ERROR_TRY_AGAIN_LATER: WIN32_ERROR = WIN32_ERROR(9554u32);
                    pub const DNS_ERROR_NOT_UNIQUE: WIN32_ERROR = WIN32_ERROR(9555u32);
                    pub const DNS_ERROR_NON_RFC_NAME: WIN32_ERROR = WIN32_ERROR(9556u32);
                    pub const DNS_ERROR_INVALID_NAME_CHAR: WIN32_ERROR = WIN32_ERROR(9560u32);
                    pub const DNS_ERROR_NUMERIC_NAME: WIN32_ERROR = WIN32_ERROR(9561u32);
                    pub const DNS_ERROR_NOT_ALLOWED_ON_ROOT_SERVER: WIN32_ERROR =
                        WIN32_ERROR(9562u32);
                    pub const DNS_ERROR_NOT_ALLOWED_UNDER_DELEGATION: WIN32_ERROR =
                        WIN32_ERROR(9563u32);
                    pub const DNS_ERROR_CANNOT_FIND_ROOT_HINTS: WIN32_ERROR = WIN32_ERROR(9564u32);
                    pub const DNS_ERROR_INCONSISTENT_ROOT_HINTS: WIN32_ERROR = WIN32_ERROR(9565u32);
                    pub const DNS_ERROR_DWORD_VALUE_TOO_SMALL: WIN32_ERROR = WIN32_ERROR(9566u32);
                    pub const DNS_ERROR_DWORD_VALUE_TOO_LARGE: WIN32_ERROR = WIN32_ERROR(9567u32);
                    pub const DNS_ERROR_BACKGROUND_LOADING: WIN32_ERROR = WIN32_ERROR(9568u32);
                    pub const DNS_ERROR_NOT_ALLOWED_ON_RODC: WIN32_ERROR = WIN32_ERROR(9569u32);
                    pub const DNS_ERROR_NOT_ALLOWED_UNDER_DNAME: WIN32_ERROR = WIN32_ERROR(9570u32);
                    pub const DNS_ERROR_DELEGATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(9571u32);
                    pub const DNS_ERROR_INVALID_POLICY_TABLE: WIN32_ERROR = WIN32_ERROR(9572u32);
                    pub const DNS_ERROR_ADDRESS_REQUIRED: WIN32_ERROR = WIN32_ERROR(9573u32);
                    pub const DNS_ERROR_ZONE_BASE: WIN32_ERROR = WIN32_ERROR(9600u32);
                    pub const DNS_ERROR_ZONE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9601u32);
                    pub const DNS_ERROR_NO_ZONE_INFO: WIN32_ERROR = WIN32_ERROR(9602u32);
                    pub const DNS_ERROR_INVALID_ZONE_OPERATION: WIN32_ERROR = WIN32_ERROR(9603u32);
                    pub const DNS_ERROR_ZONE_CONFIGURATION_ERROR: WIN32_ERROR =
                        WIN32_ERROR(9604u32);
                    pub const DNS_ERROR_ZONE_HAS_NO_SOA_RECORD: WIN32_ERROR = WIN32_ERROR(9605u32);
                    pub const DNS_ERROR_ZONE_HAS_NO_NS_RECORDS: WIN32_ERROR = WIN32_ERROR(9606u32);
                    pub const DNS_ERROR_ZONE_LOCKED: WIN32_ERROR = WIN32_ERROR(9607u32);
                    pub const DNS_ERROR_ZONE_CREATION_FAILED: WIN32_ERROR = WIN32_ERROR(9608u32);
                    pub const DNS_ERROR_ZONE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9609u32);
                    pub const DNS_ERROR_AUTOZONE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9610u32);
                    pub const DNS_ERROR_INVALID_ZONE_TYPE: WIN32_ERROR = WIN32_ERROR(9611u32);
                    pub const DNS_ERROR_SECONDARY_REQUIRES_MASTER_IP: WIN32_ERROR =
                        WIN32_ERROR(9612u32);
                    pub const DNS_ERROR_ZONE_NOT_SECONDARY: WIN32_ERROR = WIN32_ERROR(9613u32);
                    pub const DNS_ERROR_NEED_SECONDARY_ADDRESSES: WIN32_ERROR =
                        WIN32_ERROR(9614u32);
                    pub const DNS_ERROR_WINS_INIT_FAILED: WIN32_ERROR = WIN32_ERROR(9615u32);
                    pub const DNS_ERROR_NEED_WINS_SERVERS: WIN32_ERROR = WIN32_ERROR(9616u32);
                    pub const DNS_ERROR_NBSTAT_INIT_FAILED: WIN32_ERROR = WIN32_ERROR(9617u32);
                    pub const DNS_ERROR_SOA_DELETE_INVALID: WIN32_ERROR = WIN32_ERROR(9618u32);
                    pub const DNS_ERROR_FORWARDER_ALREADY_EXISTS: WIN32_ERROR =
                        WIN32_ERROR(9619u32);
                    pub const DNS_ERROR_ZONE_REQUIRES_MASTER_IP: WIN32_ERROR = WIN32_ERROR(9620u32);
                    pub const DNS_ERROR_ZONE_IS_SHUTDOWN: WIN32_ERROR = WIN32_ERROR(9621u32);
                    pub const DNS_ERROR_ZONE_LOCKED_FOR_SIGNING: WIN32_ERROR = WIN32_ERROR(9622u32);
                    pub const DNS_ERROR_DATAFILE_BASE: WIN32_ERROR = WIN32_ERROR(9650u32);
                    pub const DNS_ERROR_PRIMARY_REQUIRES_DATAFILE: WIN32_ERROR =
                        WIN32_ERROR(9651u32);
                    pub const DNS_ERROR_INVALID_DATAFILE_NAME: WIN32_ERROR = WIN32_ERROR(9652u32);
                    pub const DNS_ERROR_DATAFILE_OPEN_FAILURE: WIN32_ERROR = WIN32_ERROR(9653u32);
                    pub const DNS_ERROR_FILE_WRITEBACK_FAILED: WIN32_ERROR = WIN32_ERROR(9654u32);
                    pub const DNS_ERROR_DATAFILE_PARSING: WIN32_ERROR = WIN32_ERROR(9655u32);
                    pub const DNS_ERROR_DATABASE_BASE: WIN32_ERROR = WIN32_ERROR(9700u32);
                    pub const DNS_ERROR_RECORD_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9701u32);
                    pub const DNS_ERROR_RECORD_FORMAT: WIN32_ERROR = WIN32_ERROR(9702u32);
                    pub const DNS_ERROR_NODE_CREATION_FAILED: WIN32_ERROR = WIN32_ERROR(9703u32);
                    pub const DNS_ERROR_UNKNOWN_RECORD_TYPE: WIN32_ERROR = WIN32_ERROR(9704u32);
                    pub const DNS_ERROR_RECORD_TIMED_OUT: WIN32_ERROR = WIN32_ERROR(9705u32);
                    pub const DNS_ERROR_NAME_NOT_IN_ZONE: WIN32_ERROR = WIN32_ERROR(9706u32);
                    pub const DNS_ERROR_CNAME_LOOP: WIN32_ERROR = WIN32_ERROR(9707u32);
                    pub const DNS_ERROR_NODE_IS_CNAME: WIN32_ERROR = WIN32_ERROR(9708u32);
                    pub const DNS_ERROR_CNAME_COLLISION: WIN32_ERROR = WIN32_ERROR(9709u32);
                    pub const DNS_ERROR_RECORD_ONLY_AT_ZONE_ROOT: WIN32_ERROR =
                        WIN32_ERROR(9710u32);
                    pub const DNS_ERROR_RECORD_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9711u32);
                    pub const DNS_ERROR_SECONDARY_DATA: WIN32_ERROR = WIN32_ERROR(9712u32);
                    pub const DNS_ERROR_NO_CREATE_CACHE_DATA: WIN32_ERROR = WIN32_ERROR(9713u32);
                    pub const DNS_ERROR_NAME_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9714u32);
                    pub const DNS_ERROR_DS_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(9717u32);
                    pub const DNS_ERROR_DS_ZONE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9718u32);
                    pub const DNS_ERROR_NO_BOOTFILE_IF_DS_ZONE: WIN32_ERROR = WIN32_ERROR(9719u32);
                    pub const DNS_ERROR_NODE_IS_DNAME: WIN32_ERROR = WIN32_ERROR(9720u32);
                    pub const DNS_ERROR_DNAME_COLLISION: WIN32_ERROR = WIN32_ERROR(9721u32);
                    pub const DNS_ERROR_ALIAS_LOOP: WIN32_ERROR = WIN32_ERROR(9722u32);
                    pub const DNS_ERROR_OPERATION_BASE: WIN32_ERROR = WIN32_ERROR(9750u32);
                    pub const DNS_ERROR_AXFR: WIN32_ERROR = WIN32_ERROR(9752u32);
                    pub const DNS_ERROR_SECURE_BASE: WIN32_ERROR = WIN32_ERROR(9800u32);
                    pub const DNS_ERROR_SETUP_BASE: WIN32_ERROR = WIN32_ERROR(9850u32);
                    pub const DNS_ERROR_NO_TCPIP: WIN32_ERROR = WIN32_ERROR(9851u32);
                    pub const DNS_ERROR_NO_DNS_SERVERS: WIN32_ERROR = WIN32_ERROR(9852u32);
                    pub const DNS_ERROR_DP_BASE: WIN32_ERROR = WIN32_ERROR(9900u32);
                    pub const DNS_ERROR_DP_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9901u32);
                    pub const DNS_ERROR_DP_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9902u32);
                    pub const DNS_ERROR_DP_NOT_ENLISTED: WIN32_ERROR = WIN32_ERROR(9903u32);
                    pub const DNS_ERROR_DP_ALREADY_ENLISTED: WIN32_ERROR = WIN32_ERROR(9904u32);
                    pub const DNS_ERROR_DP_NOT_AVAILABLE: WIN32_ERROR = WIN32_ERROR(9905u32);
                    pub const DNS_ERROR_DP_FSMO_ERROR: WIN32_ERROR = WIN32_ERROR(9906u32);
                    pub const DNS_ERROR_RRL_NOT_ENABLED: WIN32_ERROR = WIN32_ERROR(9911u32);
                    pub const DNS_ERROR_RRL_INVALID_WINDOW_SIZE: WIN32_ERROR = WIN32_ERROR(9912u32);
                    pub const DNS_ERROR_RRL_INVALID_IPV4_PREFIX: WIN32_ERROR = WIN32_ERROR(9913u32);
                    pub const DNS_ERROR_RRL_INVALID_IPV6_PREFIX: WIN32_ERROR = WIN32_ERROR(9914u32);
                    pub const DNS_ERROR_RRL_INVALID_TC_RATE: WIN32_ERROR = WIN32_ERROR(9915u32);
                    pub const DNS_ERROR_RRL_INVALID_LEAK_RATE: WIN32_ERROR = WIN32_ERROR(9916u32);
                    pub const DNS_ERROR_RRL_LEAK_RATE_LESSTHAN_TC_RATE: WIN32_ERROR =
                        WIN32_ERROR(9917u32);
                    pub const DNS_ERROR_VIRTUALIZATION_INSTANCE_ALREADY_EXISTS: WIN32_ERROR =
                        WIN32_ERROR(9921u32);
                    pub const DNS_ERROR_VIRTUALIZATION_INSTANCE_DOES_NOT_EXIST: WIN32_ERROR =
                        WIN32_ERROR(9922u32);
                    pub const DNS_ERROR_VIRTUALIZATION_TREE_LOCKED: WIN32_ERROR =
                        WIN32_ERROR(9923u32);
                    pub const DNS_ERROR_INVAILD_VIRTUALIZATION_INSTANCE_NAME: WIN32_ERROR =
                        WIN32_ERROR(9924u32);
                    pub const DNS_ERROR_DEFAULT_VIRTUALIZATION_INSTANCE: WIN32_ERROR =
                        WIN32_ERROR(9925u32);
                    pub const DNS_ERROR_ZONESCOPE_ALREADY_EXISTS: WIN32_ERROR =
                        WIN32_ERROR(9951u32);
                    pub const DNS_ERROR_ZONESCOPE_DOES_NOT_EXIST: WIN32_ERROR =
                        WIN32_ERROR(9952u32);
                    pub const DNS_ERROR_DEFAULT_ZONESCOPE: WIN32_ERROR = WIN32_ERROR(9953u32);
                    pub const DNS_ERROR_INVALID_ZONESCOPE_NAME: WIN32_ERROR = WIN32_ERROR(9954u32);
                    pub const DNS_ERROR_NOT_ALLOWED_WITH_ZONESCOPES: WIN32_ERROR =
                        WIN32_ERROR(9955u32);
                    pub const DNS_ERROR_LOAD_ZONESCOPE_FAILED: WIN32_ERROR = WIN32_ERROR(9956u32);
                    pub const DNS_ERROR_ZONESCOPE_FILE_WRITEBACK_FAILED: WIN32_ERROR =
                        WIN32_ERROR(9957u32);
                    pub const DNS_ERROR_INVALID_SCOPE_NAME: WIN32_ERROR = WIN32_ERROR(9958u32);
                    pub const DNS_ERROR_SCOPE_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9959u32);
                    pub const DNS_ERROR_DEFAULT_SCOPE: WIN32_ERROR = WIN32_ERROR(9960u32);
                    pub const DNS_ERROR_INVALID_SCOPE_OPERATION: WIN32_ERROR = WIN32_ERROR(9961u32);
                    pub const DNS_ERROR_SCOPE_LOCKED: WIN32_ERROR = WIN32_ERROR(9962u32);
                    pub const DNS_ERROR_SCOPE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9963u32);
                    pub const DNS_ERROR_POLICY_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9971u32);
                    pub const DNS_ERROR_POLICY_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9972u32);
                    pub const DNS_ERROR_POLICY_INVALID_CRITERIA: WIN32_ERROR = WIN32_ERROR(9973u32);
                    pub const DNS_ERROR_POLICY_INVALID_SETTINGS: WIN32_ERROR = WIN32_ERROR(9974u32);
                    pub const DNS_ERROR_CLIENT_SUBNET_IS_ACCESSED: WIN32_ERROR =
                        WIN32_ERROR(9975u32);
                    pub const DNS_ERROR_CLIENT_SUBNET_DOES_NOT_EXIST: WIN32_ERROR =
                        WIN32_ERROR(9976u32);
                    pub const DNS_ERROR_CLIENT_SUBNET_ALREADY_EXISTS: WIN32_ERROR =
                        WIN32_ERROR(9977u32);
                    pub const DNS_ERROR_SUBNET_DOES_NOT_EXIST: WIN32_ERROR = WIN32_ERROR(9978u32);
                    pub const DNS_ERROR_SUBNET_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(9979u32);
                    pub const DNS_ERROR_POLICY_LOCKED: WIN32_ERROR = WIN32_ERROR(9980u32);
                    pub const DNS_ERROR_POLICY_INVALID_WEIGHT: WIN32_ERROR = WIN32_ERROR(9981u32);
                    pub const DNS_ERROR_POLICY_INVALID_NAME: WIN32_ERROR = WIN32_ERROR(9982u32);
                    pub const DNS_ERROR_POLICY_MISSING_CRITERIA: WIN32_ERROR = WIN32_ERROR(9983u32);
                    pub const DNS_ERROR_INVALID_CLIENT_SUBNET_NAME: WIN32_ERROR =
                        WIN32_ERROR(9984u32);
                    pub const DNS_ERROR_POLICY_PROCESSING_ORDER_INVALID: WIN32_ERROR =
                        WIN32_ERROR(9985u32);
                    pub const DNS_ERROR_POLICY_SCOPE_MISSING: WIN32_ERROR = WIN32_ERROR(9986u32);
                    pub const DNS_ERROR_POLICY_SCOPE_NOT_ALLOWED: WIN32_ERROR =
                        WIN32_ERROR(9987u32);
                    pub const DNS_ERROR_SERVERSCOPE_IS_REFERENCED: WIN32_ERROR =
                        WIN32_ERROR(9988u32);
                    pub const DNS_ERROR_ZONESCOPE_IS_REFERENCED: WIN32_ERROR = WIN32_ERROR(9989u32);
                    pub const DNS_ERROR_POLICY_INVALID_CRITERIA_CLIENT_SUBNET: WIN32_ERROR =
                        WIN32_ERROR(9990u32);
                    pub const DNS_ERROR_POLICY_INVALID_CRITERIA_TRANSPORT_PROTOCOL: WIN32_ERROR =
                        WIN32_ERROR(9991u32);
                    pub const DNS_ERROR_POLICY_INVALID_CRITERIA_NETWORK_PROTOCOL: WIN32_ERROR =
                        WIN32_ERROR(9992u32);
                    pub const DNS_ERROR_POLICY_INVALID_CRITERIA_INTERFACE: WIN32_ERROR =
                        WIN32_ERROR(9993u32);
                    pub const DNS_ERROR_POLICY_INVALID_CRITERIA_FQDN: WIN32_ERROR =
                        WIN32_ERROR(9994u32);
                    pub const DNS_ERROR_POLICY_INVALID_CRITERIA_QUERY_TYPE: WIN32_ERROR =
                        WIN32_ERROR(9995u32);
                    pub const DNS_ERROR_POLICY_INVALID_CRITERIA_TIME_OF_DAY: WIN32_ERROR =
                        WIN32_ERROR(9996u32);
                    pub const ERROR_IPSEC_QM_POLICY_EXISTS: WIN32_ERROR = WIN32_ERROR(13000u32);
                    pub const ERROR_IPSEC_QM_POLICY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13001u32);
                    pub const ERROR_IPSEC_QM_POLICY_IN_USE: WIN32_ERROR = WIN32_ERROR(13002u32);
                    pub const ERROR_IPSEC_MM_POLICY_EXISTS: WIN32_ERROR = WIN32_ERROR(13003u32);
                    pub const ERROR_IPSEC_MM_POLICY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13004u32);
                    pub const ERROR_IPSEC_MM_POLICY_IN_USE: WIN32_ERROR = WIN32_ERROR(13005u32);
                    pub const ERROR_IPSEC_MM_FILTER_EXISTS: WIN32_ERROR = WIN32_ERROR(13006u32);
                    pub const ERROR_IPSEC_MM_FILTER_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13007u32);
                    pub const ERROR_IPSEC_TRANSPORT_FILTER_EXISTS: WIN32_ERROR =
                        WIN32_ERROR(13008u32);
                    pub const ERROR_IPSEC_TRANSPORT_FILTER_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(13009u32);
                    pub const ERROR_IPSEC_MM_AUTH_EXISTS: WIN32_ERROR = WIN32_ERROR(13010u32);
                    pub const ERROR_IPSEC_MM_AUTH_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(13011u32);
                    pub const ERROR_IPSEC_MM_AUTH_IN_USE: WIN32_ERROR = WIN32_ERROR(13012u32);
                    pub const ERROR_IPSEC_DEFAULT_MM_POLICY_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(13013u32);
                    pub const ERROR_IPSEC_DEFAULT_MM_AUTH_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(13014u32);
                    pub const ERROR_IPSEC_DEFAULT_QM_POLICY_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(13015u32);
                    pub const ERROR_IPSEC_TUNNEL_FILTER_EXISTS: WIN32_ERROR = WIN32_ERROR(13016u32);
                    pub const ERROR_IPSEC_TUNNEL_FILTER_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(13017u32);
                    pub const ERROR_IPSEC_MM_FILTER_PENDING_DELETION: WIN32_ERROR =
                        WIN32_ERROR(13018u32);
                    pub const ERROR_IPSEC_TRANSPORT_FILTER_PENDING_DELETION: WIN32_ERROR =
                        WIN32_ERROR(13019u32);
                    pub const ERROR_IPSEC_TUNNEL_FILTER_PENDING_DELETION: WIN32_ERROR =
                        WIN32_ERROR(13020u32);
                    pub const ERROR_IPSEC_MM_POLICY_PENDING_DELETION: WIN32_ERROR =
                        WIN32_ERROR(13021u32);
                    pub const ERROR_IPSEC_MM_AUTH_PENDING_DELETION: WIN32_ERROR =
                        WIN32_ERROR(13022u32);
                    pub const ERROR_IPSEC_QM_POLICY_PENDING_DELETION: WIN32_ERROR =
                        WIN32_ERROR(13023u32);
                    pub const ERROR_IPSEC_IKE_NEG_STATUS_BEGIN: WIN32_ERROR = WIN32_ERROR(13800u32);
                    pub const ERROR_IPSEC_IKE_AUTH_FAIL: WIN32_ERROR = WIN32_ERROR(13801u32);
                    pub const ERROR_IPSEC_IKE_ATTRIB_FAIL: WIN32_ERROR = WIN32_ERROR(13802u32);
                    pub const ERROR_IPSEC_IKE_NEGOTIATION_PENDING: WIN32_ERROR =
                        WIN32_ERROR(13803u32);
                    pub const ERROR_IPSEC_IKE_GENERAL_PROCESSING_ERROR: WIN32_ERROR =
                        WIN32_ERROR(13804u32);
                    pub const ERROR_IPSEC_IKE_TIMED_OUT: WIN32_ERROR = WIN32_ERROR(13805u32);
                    pub const ERROR_IPSEC_IKE_NO_CERT: WIN32_ERROR = WIN32_ERROR(13806u32);
                    pub const ERROR_IPSEC_IKE_SA_DELETED: WIN32_ERROR = WIN32_ERROR(13807u32);
                    pub const ERROR_IPSEC_IKE_SA_REAPED: WIN32_ERROR = WIN32_ERROR(13808u32);
                    pub const ERROR_IPSEC_IKE_MM_ACQUIRE_DROP: WIN32_ERROR = WIN32_ERROR(13809u32);
                    pub const ERROR_IPSEC_IKE_QM_ACQUIRE_DROP: WIN32_ERROR = WIN32_ERROR(13810u32);
                    pub const ERROR_IPSEC_IKE_QUEUE_DROP_MM: WIN32_ERROR = WIN32_ERROR(13811u32);
                    pub const ERROR_IPSEC_IKE_QUEUE_DROP_NO_MM: WIN32_ERROR = WIN32_ERROR(13812u32);
                    pub const ERROR_IPSEC_IKE_DROP_NO_RESPONSE: WIN32_ERROR = WIN32_ERROR(13813u32);
                    pub const ERROR_IPSEC_IKE_MM_DELAY_DROP: WIN32_ERROR = WIN32_ERROR(13814u32);
                    pub const ERROR_IPSEC_IKE_QM_DELAY_DROP: WIN32_ERROR = WIN32_ERROR(13815u32);
                    pub const ERROR_IPSEC_IKE_ERROR: WIN32_ERROR = WIN32_ERROR(13816u32);
                    pub const ERROR_IPSEC_IKE_CRL_FAILED: WIN32_ERROR = WIN32_ERROR(13817u32);
                    pub const ERROR_IPSEC_IKE_INVALID_KEY_USAGE: WIN32_ERROR =
                        WIN32_ERROR(13818u32);
                    pub const ERROR_IPSEC_IKE_INVALID_CERT_TYPE: WIN32_ERROR =
                        WIN32_ERROR(13819u32);
                    pub const ERROR_IPSEC_IKE_NO_PRIVATE_KEY: WIN32_ERROR = WIN32_ERROR(13820u32);
                    pub const ERROR_IPSEC_IKE_SIMULTANEOUS_REKEY: WIN32_ERROR =
                        WIN32_ERROR(13821u32);
                    pub const ERROR_IPSEC_IKE_DH_FAIL: WIN32_ERROR = WIN32_ERROR(13822u32);
                    pub const ERROR_IPSEC_IKE_CRITICAL_PAYLOAD_NOT_RECOGNIZED: WIN32_ERROR =
                        WIN32_ERROR(13823u32);
                    pub const ERROR_IPSEC_IKE_INVALID_HEADER: WIN32_ERROR = WIN32_ERROR(13824u32);
                    pub const ERROR_IPSEC_IKE_NO_POLICY: WIN32_ERROR = WIN32_ERROR(13825u32);
                    pub const ERROR_IPSEC_IKE_INVALID_SIGNATURE: WIN32_ERROR =
                        WIN32_ERROR(13826u32);
                    pub const ERROR_IPSEC_IKE_KERBEROS_ERROR: WIN32_ERROR = WIN32_ERROR(13827u32);
                    pub const ERROR_IPSEC_IKE_NO_PUBLIC_KEY: WIN32_ERROR = WIN32_ERROR(13828u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR: WIN32_ERROR = WIN32_ERROR(13829u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_SA: WIN32_ERROR = WIN32_ERROR(13830u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_PROP: WIN32_ERROR = WIN32_ERROR(13831u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_TRANS: WIN32_ERROR =
                        WIN32_ERROR(13832u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_KE: WIN32_ERROR = WIN32_ERROR(13833u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_ID: WIN32_ERROR = WIN32_ERROR(13834u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_CERT: WIN32_ERROR = WIN32_ERROR(13835u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_CERT_REQ: WIN32_ERROR =
                        WIN32_ERROR(13836u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_HASH: WIN32_ERROR = WIN32_ERROR(13837u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_SIG: WIN32_ERROR = WIN32_ERROR(13838u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_NONCE: WIN32_ERROR =
                        WIN32_ERROR(13839u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_NOTIFY: WIN32_ERROR =
                        WIN32_ERROR(13840u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_DELETE: WIN32_ERROR =
                        WIN32_ERROR(13841u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_VENDOR: WIN32_ERROR =
                        WIN32_ERROR(13842u32);
                    pub const ERROR_IPSEC_IKE_INVALID_PAYLOAD: WIN32_ERROR = WIN32_ERROR(13843u32);
                    pub const ERROR_IPSEC_IKE_LOAD_SOFT_SA: WIN32_ERROR = WIN32_ERROR(13844u32);
                    pub const ERROR_IPSEC_IKE_SOFT_SA_TORN_DOWN: WIN32_ERROR =
                        WIN32_ERROR(13845u32);
                    pub const ERROR_IPSEC_IKE_INVALID_COOKIE: WIN32_ERROR = WIN32_ERROR(13846u32);
                    pub const ERROR_IPSEC_IKE_NO_PEER_CERT: WIN32_ERROR = WIN32_ERROR(13847u32);
                    pub const ERROR_IPSEC_IKE_PEER_CRL_FAILED: WIN32_ERROR = WIN32_ERROR(13848u32);
                    pub const ERROR_IPSEC_IKE_POLICY_CHANGE: WIN32_ERROR = WIN32_ERROR(13849u32);
                    pub const ERROR_IPSEC_IKE_NO_MM_POLICY: WIN32_ERROR = WIN32_ERROR(13850u32);
                    pub const ERROR_IPSEC_IKE_NOTCBPRIV: WIN32_ERROR = WIN32_ERROR(13851u32);
                    pub const ERROR_IPSEC_IKE_SECLOADFAIL: WIN32_ERROR = WIN32_ERROR(13852u32);
                    pub const ERROR_IPSEC_IKE_FAILSSPINIT: WIN32_ERROR = WIN32_ERROR(13853u32);
                    pub const ERROR_IPSEC_IKE_FAILQUERYSSP: WIN32_ERROR = WIN32_ERROR(13854u32);
                    pub const ERROR_IPSEC_IKE_SRVACQFAIL: WIN32_ERROR = WIN32_ERROR(13855u32);
                    pub const ERROR_IPSEC_IKE_SRVQUERYCRED: WIN32_ERROR = WIN32_ERROR(13856u32);
                    pub const ERROR_IPSEC_IKE_GETSPIFAIL: WIN32_ERROR = WIN32_ERROR(13857u32);
                    pub const ERROR_IPSEC_IKE_INVALID_FILTER: WIN32_ERROR = WIN32_ERROR(13858u32);
                    pub const ERROR_IPSEC_IKE_OUT_OF_MEMORY: WIN32_ERROR = WIN32_ERROR(13859u32);
                    pub const ERROR_IPSEC_IKE_ADD_UPDATE_KEY_FAILED: WIN32_ERROR =
                        WIN32_ERROR(13860u32);
                    pub const ERROR_IPSEC_IKE_INVALID_POLICY: WIN32_ERROR = WIN32_ERROR(13861u32);
                    pub const ERROR_IPSEC_IKE_UNKNOWN_DOI: WIN32_ERROR = WIN32_ERROR(13862u32);
                    pub const ERROR_IPSEC_IKE_INVALID_SITUATION: WIN32_ERROR =
                        WIN32_ERROR(13863u32);
                    pub const ERROR_IPSEC_IKE_DH_FAILURE: WIN32_ERROR = WIN32_ERROR(13864u32);
                    pub const ERROR_IPSEC_IKE_INVALID_GROUP: WIN32_ERROR = WIN32_ERROR(13865u32);
                    pub const ERROR_IPSEC_IKE_ENCRYPT: WIN32_ERROR = WIN32_ERROR(13866u32);
                    pub const ERROR_IPSEC_IKE_DECRYPT: WIN32_ERROR = WIN32_ERROR(13867u32);
                    pub const ERROR_IPSEC_IKE_POLICY_MATCH: WIN32_ERROR = WIN32_ERROR(13868u32);
                    pub const ERROR_IPSEC_IKE_UNSUPPORTED_ID: WIN32_ERROR = WIN32_ERROR(13869u32);
                    pub const ERROR_IPSEC_IKE_INVALID_HASH: WIN32_ERROR = WIN32_ERROR(13870u32);
                    pub const ERROR_IPSEC_IKE_INVALID_HASH_ALG: WIN32_ERROR = WIN32_ERROR(13871u32);
                    pub const ERROR_IPSEC_IKE_INVALID_HASH_SIZE: WIN32_ERROR =
                        WIN32_ERROR(13872u32);
                    pub const ERROR_IPSEC_IKE_INVALID_ENCRYPT_ALG: WIN32_ERROR =
                        WIN32_ERROR(13873u32);
                    pub const ERROR_IPSEC_IKE_INVALID_AUTH_ALG: WIN32_ERROR = WIN32_ERROR(13874u32);
                    pub const ERROR_IPSEC_IKE_INVALID_SIG: WIN32_ERROR = WIN32_ERROR(13875u32);
                    pub const ERROR_IPSEC_IKE_LOAD_FAILED: WIN32_ERROR = WIN32_ERROR(13876u32);
                    pub const ERROR_IPSEC_IKE_RPC_DELETE: WIN32_ERROR = WIN32_ERROR(13877u32);
                    pub const ERROR_IPSEC_IKE_BENIGN_REINIT: WIN32_ERROR = WIN32_ERROR(13878u32);
                    pub const ERROR_IPSEC_IKE_INVALID_RESPONDER_LIFETIME_NOTIFY: WIN32_ERROR =
                        WIN32_ERROR(13879u32);
                    pub const ERROR_IPSEC_IKE_INVALID_MAJOR_VERSION: WIN32_ERROR =
                        WIN32_ERROR(13880u32);
                    pub const ERROR_IPSEC_IKE_INVALID_CERT_KEYLEN: WIN32_ERROR =
                        WIN32_ERROR(13881u32);
                    pub const ERROR_IPSEC_IKE_MM_LIMIT: WIN32_ERROR = WIN32_ERROR(13882u32);
                    pub const ERROR_IPSEC_IKE_NEGOTIATION_DISABLED: WIN32_ERROR =
                        WIN32_ERROR(13883u32);
                    pub const ERROR_IPSEC_IKE_QM_LIMIT: WIN32_ERROR = WIN32_ERROR(13884u32);
                    pub const ERROR_IPSEC_IKE_MM_EXPIRED: WIN32_ERROR = WIN32_ERROR(13885u32);
                    pub const ERROR_IPSEC_IKE_PEER_MM_ASSUMED_INVALID: WIN32_ERROR =
                        WIN32_ERROR(13886u32);
                    pub const ERROR_IPSEC_IKE_CERT_CHAIN_POLICY_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(13887u32);
                    pub const ERROR_IPSEC_IKE_UNEXPECTED_MESSAGE_ID: WIN32_ERROR =
                        WIN32_ERROR(13888u32);
                    pub const ERROR_IPSEC_IKE_INVALID_AUTH_PAYLOAD: WIN32_ERROR =
                        WIN32_ERROR(13889u32);
                    pub const ERROR_IPSEC_IKE_DOS_COOKIE_SENT: WIN32_ERROR = WIN32_ERROR(13890u32);
                    pub const ERROR_IPSEC_IKE_SHUTTING_DOWN: WIN32_ERROR = WIN32_ERROR(13891u32);
                    pub const ERROR_IPSEC_IKE_CGA_AUTH_FAILED: WIN32_ERROR = WIN32_ERROR(13892u32);
                    pub const ERROR_IPSEC_IKE_PROCESS_ERR_NATOA: WIN32_ERROR =
                        WIN32_ERROR(13893u32);
                    pub const ERROR_IPSEC_IKE_INVALID_MM_FOR_QM: WIN32_ERROR =
                        WIN32_ERROR(13894u32);
                    pub const ERROR_IPSEC_IKE_QM_EXPIRED: WIN32_ERROR = WIN32_ERROR(13895u32);
                    pub const ERROR_IPSEC_IKE_TOO_MANY_FILTERS: WIN32_ERROR = WIN32_ERROR(13896u32);
                    pub const ERROR_IPSEC_IKE_NEG_STATUS_END: WIN32_ERROR = WIN32_ERROR(13897u32);
                    pub const ERROR_IPSEC_IKE_KILL_DUMMY_NAP_TUNNEL: WIN32_ERROR =
                        WIN32_ERROR(13898u32);
                    pub const ERROR_IPSEC_IKE_INNER_IP_ASSIGNMENT_FAILURE: WIN32_ERROR =
                        WIN32_ERROR(13899u32);
                    pub const ERROR_IPSEC_IKE_REQUIRE_CP_PAYLOAD_MISSING: WIN32_ERROR =
                        WIN32_ERROR(13900u32);
                    pub const ERROR_IPSEC_KEY_MODULE_IMPERSONATION_NEGOTIATION_PENDING:
                        WIN32_ERROR = WIN32_ERROR(13901u32);
                    pub const ERROR_IPSEC_IKE_COEXISTENCE_SUPPRESS: WIN32_ERROR =
                        WIN32_ERROR(13902u32);
                    pub const ERROR_IPSEC_IKE_RATELIMIT_DROP: WIN32_ERROR = WIN32_ERROR(13903u32);
                    pub const ERROR_IPSEC_IKE_PEER_DOESNT_SUPPORT_MOBIKE: WIN32_ERROR =
                        WIN32_ERROR(13904u32);
                    pub const ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE: WIN32_ERROR =
                        WIN32_ERROR(13905u32);
                    pub const ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_FAILURE: WIN32_ERROR =
                        WIN32_ERROR(13906u32);
                    pub const ERROR_IPSEC_IKE_AUTHORIZATION_FAILURE_WITH_OPTIONAL_RETRY:
                        WIN32_ERROR = WIN32_ERROR(13907u32);
                    pub const ERROR_IPSEC_IKE_STRONG_CRED_AUTHORIZATION_AND_CERTMAP_FAILURE:
                        WIN32_ERROR = WIN32_ERROR(13908u32);
                    pub const ERROR_IPSEC_IKE_NEG_STATUS_EXTENDED_END: WIN32_ERROR =
                        WIN32_ERROR(13909u32);
                    pub const ERROR_IPSEC_BAD_SPI: WIN32_ERROR = WIN32_ERROR(13910u32);
                    pub const ERROR_IPSEC_SA_LIFETIME_EXPIRED: WIN32_ERROR = WIN32_ERROR(13911u32);
                    pub const ERROR_IPSEC_WRONG_SA: WIN32_ERROR = WIN32_ERROR(13912u32);
                    pub const ERROR_IPSEC_REPLAY_CHECK_FAILED: WIN32_ERROR = WIN32_ERROR(13913u32);
                    pub const ERROR_IPSEC_INVALID_PACKET: WIN32_ERROR = WIN32_ERROR(13914u32);
                    pub const ERROR_IPSEC_INTEGRITY_CHECK_FAILED: WIN32_ERROR =
                        WIN32_ERROR(13915u32);
                    pub const ERROR_IPSEC_CLEAR_TEXT_DROP: WIN32_ERROR = WIN32_ERROR(13916u32);
                    pub const ERROR_IPSEC_AUTH_FIREWALL_DROP: WIN32_ERROR = WIN32_ERROR(13917u32);
                    pub const ERROR_IPSEC_THROTTLE_DROP: WIN32_ERROR = WIN32_ERROR(13918u32);
                    pub const ERROR_IPSEC_DOSP_BLOCK: WIN32_ERROR = WIN32_ERROR(13925u32);
                    pub const ERROR_IPSEC_DOSP_RECEIVED_MULTICAST: WIN32_ERROR =
                        WIN32_ERROR(13926u32);
                    pub const ERROR_IPSEC_DOSP_INVALID_PACKET: WIN32_ERROR = WIN32_ERROR(13927u32);
                    pub const ERROR_IPSEC_DOSP_STATE_LOOKUP_FAILED: WIN32_ERROR =
                        WIN32_ERROR(13928u32);
                    pub const ERROR_IPSEC_DOSP_MAX_ENTRIES: WIN32_ERROR = WIN32_ERROR(13929u32);
                    pub const ERROR_IPSEC_DOSP_KEYMOD_NOT_ALLOWED: WIN32_ERROR =
                        WIN32_ERROR(13930u32);
                    pub const ERROR_IPSEC_DOSP_NOT_INSTALLED: WIN32_ERROR = WIN32_ERROR(13931u32);
                    pub const ERROR_IPSEC_DOSP_MAX_PER_IP_RATELIMIT_QUEUES: WIN32_ERROR =
                        WIN32_ERROR(13932u32);
                    pub const ERROR_SXS_SECTION_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(14000u32);
                    pub const ERROR_SXS_CANT_GEN_ACTCTX: WIN32_ERROR = WIN32_ERROR(14001u32);
                    pub const ERROR_SXS_INVALID_ACTCTXDATA_FORMAT: WIN32_ERROR =
                        WIN32_ERROR(14002u32);
                    pub const ERROR_SXS_ASSEMBLY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(14003u32);
                    pub const ERROR_SXS_MANIFEST_FORMAT_ERROR: WIN32_ERROR = WIN32_ERROR(14004u32);
                    pub const ERROR_SXS_MANIFEST_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(14005u32);
                    pub const ERROR_SXS_ACTIVATION_CONTEXT_DISABLED: WIN32_ERROR =
                        WIN32_ERROR(14006u32);
                    pub const ERROR_SXS_KEY_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(14007u32);
                    pub const ERROR_SXS_VERSION_CONFLICT: WIN32_ERROR = WIN32_ERROR(14008u32);
                    pub const ERROR_SXS_WRONG_SECTION_TYPE: WIN32_ERROR = WIN32_ERROR(14009u32);
                    pub const ERROR_SXS_THREAD_QUERIES_DISABLED: WIN32_ERROR =
                        WIN32_ERROR(14010u32);
                    pub const ERROR_SXS_PROCESS_DEFAULT_ALREADY_SET: WIN32_ERROR =
                        WIN32_ERROR(14011u32);
                    pub const ERROR_SXS_UNKNOWN_ENCODING_GROUP: WIN32_ERROR = WIN32_ERROR(14012u32);
                    pub const ERROR_SXS_UNKNOWN_ENCODING: WIN32_ERROR = WIN32_ERROR(14013u32);
                    pub const ERROR_SXS_INVALID_XML_NAMESPACE_URI: WIN32_ERROR =
                        WIN32_ERROR(14014u32);
                    pub const ERROR_SXS_ROOT_MANIFEST_DEPENDENCY_NOT_INSTALLED: WIN32_ERROR =
                        WIN32_ERROR(14015u32);
                    pub const ERROR_SXS_LEAF_MANIFEST_DEPENDENCY_NOT_INSTALLED: WIN32_ERROR =
                        WIN32_ERROR(14016u32);
                    pub const ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE: WIN32_ERROR =
                        WIN32_ERROR(14017u32);
                    pub const ERROR_SXS_MANIFEST_MISSING_REQUIRED_DEFAULT_NAMESPACE: WIN32_ERROR =
                        WIN32_ERROR(14018u32);
                    pub const ERROR_SXS_MANIFEST_INVALID_REQUIRED_DEFAULT_NAMESPACE: WIN32_ERROR =
                        WIN32_ERROR(14019u32);
                    pub const ERROR_SXS_PRIVATE_MANIFEST_CROSS_PATH_WITH_REPARSE_POINT:
                        WIN32_ERROR = WIN32_ERROR(14020u32);
                    pub const ERROR_SXS_DUPLICATE_DLL_NAME: WIN32_ERROR = WIN32_ERROR(14021u32);
                    pub const ERROR_SXS_DUPLICATE_WINDOWCLASS_NAME: WIN32_ERROR =
                        WIN32_ERROR(14022u32);
                    pub const ERROR_SXS_DUPLICATE_CLSID: WIN32_ERROR = WIN32_ERROR(14023u32);
                    pub const ERROR_SXS_DUPLICATE_IID: WIN32_ERROR = WIN32_ERROR(14024u32);
                    pub const ERROR_SXS_DUPLICATE_TLBID: WIN32_ERROR = WIN32_ERROR(14025u32);
                    pub const ERROR_SXS_DUPLICATE_PROGID: WIN32_ERROR = WIN32_ERROR(14026u32);
                    pub const ERROR_SXS_DUPLICATE_ASSEMBLY_NAME: WIN32_ERROR =
                        WIN32_ERROR(14027u32);
                    pub const ERROR_SXS_FILE_HASH_MISMATCH: WIN32_ERROR = WIN32_ERROR(14028u32);
                    pub const ERROR_SXS_POLICY_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(14029u32);
                    pub const ERROR_SXS_XML_E_MISSINGQUOTE: WIN32_ERROR = WIN32_ERROR(14030u32);
                    pub const ERROR_SXS_XML_E_COMMENTSYNTAX: WIN32_ERROR = WIN32_ERROR(14031u32);
                    pub const ERROR_SXS_XML_E_BADSTARTNAMECHAR: WIN32_ERROR = WIN32_ERROR(14032u32);
                    pub const ERROR_SXS_XML_E_BADNAMECHAR: WIN32_ERROR = WIN32_ERROR(14033u32);
                    pub const ERROR_SXS_XML_E_BADCHARINSTRING: WIN32_ERROR = WIN32_ERROR(14034u32);
                    pub const ERROR_SXS_XML_E_XMLDECLSYNTAX: WIN32_ERROR = WIN32_ERROR(14035u32);
                    pub const ERROR_SXS_XML_E_BADCHARDATA: WIN32_ERROR = WIN32_ERROR(14036u32);
                    pub const ERROR_SXS_XML_E_MISSINGWHITESPACE: WIN32_ERROR =
                        WIN32_ERROR(14037u32);
                    pub const ERROR_SXS_XML_E_EXPECTINGTAGEND: WIN32_ERROR = WIN32_ERROR(14038u32);
                    pub const ERROR_SXS_XML_E_MISSINGSEMICOLON: WIN32_ERROR = WIN32_ERROR(14039u32);
                    pub const ERROR_SXS_XML_E_UNBALANCEDPAREN: WIN32_ERROR = WIN32_ERROR(14040u32);
                    pub const ERROR_SXS_XML_E_INTERNALERROR: WIN32_ERROR = WIN32_ERROR(14041u32);
                    pub const ERROR_SXS_XML_E_UNEXPECTED_WHITESPACE: WIN32_ERROR =
                        WIN32_ERROR(14042u32);
                    pub const ERROR_SXS_XML_E_INCOMPLETE_ENCODING: WIN32_ERROR =
                        WIN32_ERROR(14043u32);
                    pub const ERROR_SXS_XML_E_MISSING_PAREN: WIN32_ERROR = WIN32_ERROR(14044u32);
                    pub const ERROR_SXS_XML_E_EXPECTINGCLOSEQUOTE: WIN32_ERROR =
                        WIN32_ERROR(14045u32);
                    pub const ERROR_SXS_XML_E_MULTIPLE_COLONS: WIN32_ERROR = WIN32_ERROR(14046u32);
                    pub const ERROR_SXS_XML_E_INVALID_DECIMAL: WIN32_ERROR = WIN32_ERROR(14047u32);
                    pub const ERROR_SXS_XML_E_INVALID_HEXIDECIMAL: WIN32_ERROR =
                        WIN32_ERROR(14048u32);
                    pub const ERROR_SXS_XML_E_INVALID_UNICODE: WIN32_ERROR = WIN32_ERROR(14049u32);
                    pub const ERROR_SXS_XML_E_WHITESPACEORQUESTIONMARK: WIN32_ERROR =
                        WIN32_ERROR(14050u32);
                    pub const ERROR_SXS_XML_E_UNEXPECTEDENDTAG: WIN32_ERROR = WIN32_ERROR(14051u32);
                    pub const ERROR_SXS_XML_E_UNCLOSEDTAG: WIN32_ERROR = WIN32_ERROR(14052u32);
                    pub const ERROR_SXS_XML_E_DUPLICATEATTRIBUTE: WIN32_ERROR =
                        WIN32_ERROR(14053u32);
                    pub const ERROR_SXS_XML_E_MULTIPLEROOTS: WIN32_ERROR = WIN32_ERROR(14054u32);
                    pub const ERROR_SXS_XML_E_INVALIDATROOTLEVEL: WIN32_ERROR =
                        WIN32_ERROR(14055u32);
                    pub const ERROR_SXS_XML_E_BADXMLDECL: WIN32_ERROR = WIN32_ERROR(14056u32);
                    pub const ERROR_SXS_XML_E_MISSINGROOT: WIN32_ERROR = WIN32_ERROR(14057u32);
                    pub const ERROR_SXS_XML_E_UNEXPECTEDEOF: WIN32_ERROR = WIN32_ERROR(14058u32);
                    pub const ERROR_SXS_XML_E_BADPEREFINSUBSET: WIN32_ERROR = WIN32_ERROR(14059u32);
                    pub const ERROR_SXS_XML_E_UNCLOSEDSTARTTAG: WIN32_ERROR = WIN32_ERROR(14060u32);
                    pub const ERROR_SXS_XML_E_UNCLOSEDENDTAG: WIN32_ERROR = WIN32_ERROR(14061u32);
                    pub const ERROR_SXS_XML_E_UNCLOSEDSTRING: WIN32_ERROR = WIN32_ERROR(14062u32);
                    pub const ERROR_SXS_XML_E_UNCLOSEDCOMMENT: WIN32_ERROR = WIN32_ERROR(14063u32);
                    pub const ERROR_SXS_XML_E_UNCLOSEDDECL: WIN32_ERROR = WIN32_ERROR(14064u32);
                    pub const ERROR_SXS_XML_E_UNCLOSEDCDATA: WIN32_ERROR = WIN32_ERROR(14065u32);
                    pub const ERROR_SXS_XML_E_RESERVEDNAMESPACE: WIN32_ERROR =
                        WIN32_ERROR(14066u32);
                    pub const ERROR_SXS_XML_E_INVALIDENCODING: WIN32_ERROR = WIN32_ERROR(14067u32);
                    pub const ERROR_SXS_XML_E_INVALIDSWITCH: WIN32_ERROR = WIN32_ERROR(14068u32);
                    pub const ERROR_SXS_XML_E_BADXMLCASE: WIN32_ERROR = WIN32_ERROR(14069u32);
                    pub const ERROR_SXS_XML_E_INVALID_STANDALONE: WIN32_ERROR =
                        WIN32_ERROR(14070u32);
                    pub const ERROR_SXS_XML_E_UNEXPECTED_STANDALONE: WIN32_ERROR =
                        WIN32_ERROR(14071u32);
                    pub const ERROR_SXS_XML_E_INVALID_VERSION: WIN32_ERROR = WIN32_ERROR(14072u32);
                    pub const ERROR_SXS_XML_E_MISSINGEQUALS: WIN32_ERROR = WIN32_ERROR(14073u32);
                    pub const ERROR_SXS_PROTECTION_RECOVERY_FAILED: WIN32_ERROR =
                        WIN32_ERROR(14074u32);
                    pub const ERROR_SXS_PROTECTION_PUBLIC_KEY_TOO_SHORT: WIN32_ERROR =
                        WIN32_ERROR(14075u32);
                    pub const ERROR_SXS_PROTECTION_CATALOG_NOT_VALID: WIN32_ERROR =
                        WIN32_ERROR(14076u32);
                    pub const ERROR_SXS_UNTRANSLATABLE_HRESULT: WIN32_ERROR = WIN32_ERROR(14077u32);
                    pub const ERROR_SXS_PROTECTION_CATALOG_FILE_MISSING: WIN32_ERROR =
                        WIN32_ERROR(14078u32);
                    pub const ERROR_SXS_MISSING_ASSEMBLY_IDENTITY_ATTRIBUTE: WIN32_ERROR =
                        WIN32_ERROR(14079u32);
                    pub const ERROR_SXS_INVALID_ASSEMBLY_IDENTITY_ATTRIBUTE_NAME: WIN32_ERROR =
                        WIN32_ERROR(14080u32);
                    pub const ERROR_SXS_ASSEMBLY_MISSING: WIN32_ERROR = WIN32_ERROR(14081u32);
                    pub const ERROR_SXS_CORRUPT_ACTIVATION_STACK: WIN32_ERROR =
                        WIN32_ERROR(14082u32);
                    pub const ERROR_SXS_CORRUPTION: WIN32_ERROR = WIN32_ERROR(14083u32);
                    pub const ERROR_SXS_EARLY_DEACTIVATION: WIN32_ERROR = WIN32_ERROR(14084u32);
                    pub const ERROR_SXS_INVALID_DEACTIVATION: WIN32_ERROR = WIN32_ERROR(14085u32);
                    pub const ERROR_SXS_MULTIPLE_DEACTIVATION: WIN32_ERROR = WIN32_ERROR(14086u32);
                    pub const ERROR_SXS_PROCESS_TERMINATION_REQUESTED: WIN32_ERROR =
                        WIN32_ERROR(14087u32);
                    pub const ERROR_SXS_RELEASE_ACTIVATION_CONTEXT: WIN32_ERROR =
                        WIN32_ERROR(14088u32);
                    pub const ERROR_SXS_SYSTEM_DEFAULT_ACTIVATION_CONTEXT_EMPTY: WIN32_ERROR =
                        WIN32_ERROR(14089u32);
                    pub const ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_VALUE: WIN32_ERROR =
                        WIN32_ERROR(14090u32);
                    pub const ERROR_SXS_INVALID_IDENTITY_ATTRIBUTE_NAME: WIN32_ERROR =
                        WIN32_ERROR(14091u32);
                    pub const ERROR_SXS_IDENTITY_DUPLICATE_ATTRIBUTE: WIN32_ERROR =
                        WIN32_ERROR(14092u32);
                    pub const ERROR_SXS_IDENTITY_PARSE_ERROR: WIN32_ERROR = WIN32_ERROR(14093u32);
                    pub const ERROR_MALFORMED_SUBSTITUTION_STRING: WIN32_ERROR =
                        WIN32_ERROR(14094u32);
                    pub const ERROR_SXS_INCORRECT_PUBLIC_KEY_TOKEN: WIN32_ERROR =
                        WIN32_ERROR(14095u32);
                    pub const ERROR_UNMAPPED_SUBSTITUTION_STRING: WIN32_ERROR =
                        WIN32_ERROR(14096u32);
                    pub const ERROR_SXS_ASSEMBLY_NOT_LOCKED: WIN32_ERROR = WIN32_ERROR(14097u32);
                    pub const ERROR_SXS_COMPONENT_STORE_CORRUPT: WIN32_ERROR =
                        WIN32_ERROR(14098u32);
                    pub const ERROR_ADVANCED_INSTALLER_FAILED: WIN32_ERROR = WIN32_ERROR(14099u32);
                    pub const ERROR_XML_ENCODING_MISMATCH: WIN32_ERROR = WIN32_ERROR(14100u32);
                    pub const ERROR_SXS_MANIFEST_IDENTITY_SAME_BUT_CONTENTS_DIFFERENT: WIN32_ERROR =
                        WIN32_ERROR(14101u32);
                    pub const ERROR_SXS_IDENTITIES_DIFFERENT: WIN32_ERROR = WIN32_ERROR(14102u32);
                    pub const ERROR_SXS_ASSEMBLY_IS_NOT_A_DEPLOYMENT: WIN32_ERROR =
                        WIN32_ERROR(14103u32);
                    pub const ERROR_SXS_FILE_NOT_PART_OF_ASSEMBLY: WIN32_ERROR =
                        WIN32_ERROR(14104u32);
                    pub const ERROR_SXS_MANIFEST_TOO_BIG: WIN32_ERROR = WIN32_ERROR(14105u32);
                    pub const ERROR_SXS_SETTING_NOT_REGISTERED: WIN32_ERROR = WIN32_ERROR(14106u32);
                    pub const ERROR_SXS_TRANSACTION_CLOSURE_INCOMPLETE: WIN32_ERROR =
                        WIN32_ERROR(14107u32);
                    pub const ERROR_SMI_PRIMITIVE_INSTALLER_FAILED: WIN32_ERROR =
                        WIN32_ERROR(14108u32);
                    pub const ERROR_GENERIC_COMMAND_FAILED: WIN32_ERROR = WIN32_ERROR(14109u32);
                    pub const ERROR_SXS_FILE_HASH_MISSING: WIN32_ERROR = WIN32_ERROR(14110u32);
                    pub const ERROR_SXS_DUPLICATE_ACTIVATABLE_CLASS: WIN32_ERROR =
                        WIN32_ERROR(14111u32);
                    pub const ERROR_EVT_INVALID_CHANNEL_PATH: WIN32_ERROR = WIN32_ERROR(15000u32);
                    pub const ERROR_EVT_INVALID_QUERY: WIN32_ERROR = WIN32_ERROR(15001u32);
                    pub const ERROR_EVT_PUBLISHER_METADATA_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(15002u32);
                    pub const ERROR_EVT_EVENT_TEMPLATE_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(15003u32);
                    pub const ERROR_EVT_INVALID_PUBLISHER_NAME: WIN32_ERROR = WIN32_ERROR(15004u32);
                    pub const ERROR_EVT_INVALID_EVENT_DATA: WIN32_ERROR = WIN32_ERROR(15005u32);
                    pub const ERROR_EVT_CHANNEL_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15007u32);
                    pub const ERROR_EVT_MALFORMED_XML_TEXT: WIN32_ERROR = WIN32_ERROR(15008u32);
                    pub const ERROR_EVT_SUBSCRIPTION_TO_DIRECT_CHANNEL: WIN32_ERROR =
                        WIN32_ERROR(15009u32);
                    pub const ERROR_EVT_CONFIGURATION_ERROR: WIN32_ERROR = WIN32_ERROR(15010u32);
                    pub const ERROR_EVT_QUERY_RESULT_STALE: WIN32_ERROR = WIN32_ERROR(15011u32);
                    pub const ERROR_EVT_QUERY_RESULT_INVALID_POSITION: WIN32_ERROR =
                        WIN32_ERROR(15012u32);
                    pub const ERROR_EVT_NON_VALIDATING_MSXML: WIN32_ERROR = WIN32_ERROR(15013u32);
                    pub const ERROR_EVT_FILTER_ALREADYSCOPED: WIN32_ERROR = WIN32_ERROR(15014u32);
                    pub const ERROR_EVT_FILTER_NOTELTSET: WIN32_ERROR = WIN32_ERROR(15015u32);
                    pub const ERROR_EVT_FILTER_INVARG: WIN32_ERROR = WIN32_ERROR(15016u32);
                    pub const ERROR_EVT_FILTER_INVTEST: WIN32_ERROR = WIN32_ERROR(15017u32);
                    pub const ERROR_EVT_FILTER_INVTYPE: WIN32_ERROR = WIN32_ERROR(15018u32);
                    pub const ERROR_EVT_FILTER_PARSEERR: WIN32_ERROR = WIN32_ERROR(15019u32);
                    pub const ERROR_EVT_FILTER_UNSUPPORTEDOP: WIN32_ERROR = WIN32_ERROR(15020u32);
                    pub const ERROR_EVT_FILTER_UNEXPECTEDTOKEN: WIN32_ERROR = WIN32_ERROR(15021u32);
                    pub const ERROR_EVT_INVALID_OPERATION_OVER_ENABLED_DIRECT_CHANNEL: WIN32_ERROR =
                        WIN32_ERROR(15022u32);
                    pub const ERROR_EVT_INVALID_CHANNEL_PROPERTY_VALUE: WIN32_ERROR =
                        WIN32_ERROR(15023u32);
                    pub const ERROR_EVT_INVALID_PUBLISHER_PROPERTY_VALUE: WIN32_ERROR =
                        WIN32_ERROR(15024u32);
                    pub const ERROR_EVT_CHANNEL_CANNOT_ACTIVATE: WIN32_ERROR =
                        WIN32_ERROR(15025u32);
                    pub const ERROR_EVT_FILTER_TOO_COMPLEX: WIN32_ERROR = WIN32_ERROR(15026u32);
                    pub const ERROR_EVT_MESSAGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15027u32);
                    pub const ERROR_EVT_MESSAGE_ID_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15028u32);
                    pub const ERROR_EVT_UNRESOLVED_VALUE_INSERT: WIN32_ERROR =
                        WIN32_ERROR(15029u32);
                    pub const ERROR_EVT_UNRESOLVED_PARAMETER_INSERT: WIN32_ERROR =
                        WIN32_ERROR(15030u32);
                    pub const ERROR_EVT_MAX_INSERTS_REACHED: WIN32_ERROR = WIN32_ERROR(15031u32);
                    pub const ERROR_EVT_EVENT_DEFINITION_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(15032u32);
                    pub const ERROR_EVT_MESSAGE_LOCALE_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(15033u32);
                    pub const ERROR_EVT_VERSION_TOO_OLD: WIN32_ERROR = WIN32_ERROR(15034u32);
                    pub const ERROR_EVT_VERSION_TOO_NEW: WIN32_ERROR = WIN32_ERROR(15035u32);
                    pub const ERROR_EVT_CANNOT_OPEN_CHANNEL_OF_QUERY: WIN32_ERROR =
                        WIN32_ERROR(15036u32);
                    pub const ERROR_EVT_PUBLISHER_DISABLED: WIN32_ERROR = WIN32_ERROR(15037u32);
                    pub const ERROR_EVT_FILTER_OUT_OF_RANGE: WIN32_ERROR = WIN32_ERROR(15038u32);
                    pub const ERROR_EC_SUBSCRIPTION_CANNOT_ACTIVATE: WIN32_ERROR =
                        WIN32_ERROR(15080u32);
                    pub const ERROR_EC_LOG_DISABLED: WIN32_ERROR = WIN32_ERROR(15081u32);
                    pub const ERROR_EC_CIRCULAR_FORWARDING: WIN32_ERROR = WIN32_ERROR(15082u32);
                    pub const ERROR_EC_CREDSTORE_FULL: WIN32_ERROR = WIN32_ERROR(15083u32);
                    pub const ERROR_EC_CRED_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15084u32);
                    pub const ERROR_EC_NO_ACTIVE_CHANNEL: WIN32_ERROR = WIN32_ERROR(15085u32);
                    pub const ERROR_MUI_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15100u32);
                    pub const ERROR_MUI_INVALID_FILE: WIN32_ERROR = WIN32_ERROR(15101u32);
                    pub const ERROR_MUI_INVALID_RC_CONFIG: WIN32_ERROR = WIN32_ERROR(15102u32);
                    pub const ERROR_MUI_INVALID_LOCALE_NAME: WIN32_ERROR = WIN32_ERROR(15103u32);
                    pub const ERROR_MUI_INVALID_ULTIMATEFALLBACK_NAME: WIN32_ERROR =
                        WIN32_ERROR(15104u32);
                    pub const ERROR_MUI_FILE_NOT_LOADED: WIN32_ERROR = WIN32_ERROR(15105u32);
                    pub const ERROR_RESOURCE_ENUM_USER_STOP: WIN32_ERROR = WIN32_ERROR(15106u32);
                    pub const ERROR_MUI_INTLSETTINGS_UILANG_NOT_INSTALLED: WIN32_ERROR =
                        WIN32_ERROR(15107u32);
                    pub const ERROR_MUI_INTLSETTINGS_INVALID_LOCALE_NAME: WIN32_ERROR =
                        WIN32_ERROR(15108u32);
                    pub const ERROR_MRM_RUNTIME_NO_DEFAULT_OR_NEUTRAL_RESOURCE: WIN32_ERROR =
                        WIN32_ERROR(15110u32);
                    pub const ERROR_MRM_INVALID_PRICONFIG: WIN32_ERROR = WIN32_ERROR(15111u32);
                    pub const ERROR_MRM_INVALID_FILE_TYPE: WIN32_ERROR = WIN32_ERROR(15112u32);
                    pub const ERROR_MRM_UNKNOWN_QUALIFIER: WIN32_ERROR = WIN32_ERROR(15113u32);
                    pub const ERROR_MRM_INVALID_QUALIFIER_VALUE: WIN32_ERROR =
                        WIN32_ERROR(15114u32);
                    pub const ERROR_MRM_NO_CANDIDATE: WIN32_ERROR = WIN32_ERROR(15115u32);
                    pub const ERROR_MRM_NO_MATCH_OR_DEFAULT_CANDIDATE: WIN32_ERROR =
                        WIN32_ERROR(15116u32);
                    pub const ERROR_MRM_RESOURCE_TYPE_MISMATCH: WIN32_ERROR = WIN32_ERROR(15117u32);
                    pub const ERROR_MRM_DUPLICATE_MAP_NAME: WIN32_ERROR = WIN32_ERROR(15118u32);
                    pub const ERROR_MRM_DUPLICATE_ENTRY: WIN32_ERROR = WIN32_ERROR(15119u32);
                    pub const ERROR_MRM_INVALID_RESOURCE_IDENTIFIER: WIN32_ERROR =
                        WIN32_ERROR(15120u32);
                    pub const ERROR_MRM_FILEPATH_TOO_LONG: WIN32_ERROR = WIN32_ERROR(15121u32);
                    pub const ERROR_MRM_UNSUPPORTED_DIRECTORY_TYPE: WIN32_ERROR =
                        WIN32_ERROR(15122u32);
                    pub const ERROR_MRM_INVALID_PRI_FILE: WIN32_ERROR = WIN32_ERROR(15126u32);
                    pub const ERROR_MRM_NAMED_RESOURCE_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(15127u32);
                    pub const ERROR_MRM_MAP_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15135u32);
                    pub const ERROR_MRM_UNSUPPORTED_PROFILE_TYPE: WIN32_ERROR =
                        WIN32_ERROR(15136u32);
                    pub const ERROR_MRM_INVALID_QUALIFIER_OPERATOR: WIN32_ERROR =
                        WIN32_ERROR(15137u32);
                    pub const ERROR_MRM_INDETERMINATE_QUALIFIER_VALUE: WIN32_ERROR =
                        WIN32_ERROR(15138u32);
                    pub const ERROR_MRM_AUTOMERGE_ENABLED: WIN32_ERROR = WIN32_ERROR(15139u32);
                    pub const ERROR_MRM_TOO_MANY_RESOURCES: WIN32_ERROR = WIN32_ERROR(15140u32);
                    pub const ERROR_MRM_UNSUPPORTED_FILE_TYPE_FOR_MERGE: WIN32_ERROR =
                        WIN32_ERROR(15141u32);
                    pub const ERROR_MRM_UNSUPPORTED_FILE_TYPE_FOR_LOAD_UNLOAD_PRI_FILE:
                        WIN32_ERROR = WIN32_ERROR(15142u32);
                    pub const ERROR_MRM_NO_CURRENT_VIEW_ON_THREAD: WIN32_ERROR =
                        WIN32_ERROR(15143u32);
                    pub const ERROR_DIFFERENT_PROFILE_RESOURCE_MANAGER_EXIST: WIN32_ERROR =
                        WIN32_ERROR(15144u32);
                    pub const ERROR_OPERATION_NOT_ALLOWED_FROM_SYSTEM_COMPONENT: WIN32_ERROR =
                        WIN32_ERROR(15145u32);
                    pub const ERROR_MRM_DIRECT_REF_TO_NON_DEFAULT_RESOURCE: WIN32_ERROR =
                        WIN32_ERROR(15146u32);
                    pub const ERROR_MRM_GENERATION_COUNT_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(15147u32);
                    pub const ERROR_PRI_MERGE_VERSION_MISMATCH: WIN32_ERROR = WIN32_ERROR(15148u32);
                    pub const ERROR_PRI_MERGE_MISSING_SCHEMA: WIN32_ERROR = WIN32_ERROR(15149u32);
                    pub const ERROR_PRI_MERGE_LOAD_FILE_FAILED: WIN32_ERROR = WIN32_ERROR(15150u32);
                    pub const ERROR_PRI_MERGE_ADD_FILE_FAILED: WIN32_ERROR = WIN32_ERROR(15151u32);
                    pub const ERROR_PRI_MERGE_WRITE_FILE_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15152u32);
                    pub const ERROR_PRI_MERGE_MULTIPLE_PACKAGE_FAMILIES_NOT_ALLOWED: WIN32_ERROR =
                        WIN32_ERROR(15153u32);
                    pub const ERROR_PRI_MERGE_MULTIPLE_MAIN_PACKAGES_NOT_ALLOWED: WIN32_ERROR =
                        WIN32_ERROR(15154u32);
                    pub const ERROR_PRI_MERGE_BUNDLE_PACKAGES_NOT_ALLOWED: WIN32_ERROR =
                        WIN32_ERROR(15155u32);
                    pub const ERROR_PRI_MERGE_MAIN_PACKAGE_REQUIRED: WIN32_ERROR =
                        WIN32_ERROR(15156u32);
                    pub const ERROR_PRI_MERGE_RESOURCE_PACKAGE_REQUIRED: WIN32_ERROR =
                        WIN32_ERROR(15157u32);
                    pub const ERROR_PRI_MERGE_INVALID_FILE_NAME: WIN32_ERROR =
                        WIN32_ERROR(15158u32);
                    pub const ERROR_MRM_PACKAGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15159u32);
                    pub const ERROR_MRM_MISSING_DEFAULT_LANGUAGE: WIN32_ERROR =
                        WIN32_ERROR(15160u32);
                    pub const ERROR_MCA_INVALID_CAPABILITIES_STRING: WIN32_ERROR =
                        WIN32_ERROR(15200u32);
                    pub const ERROR_MCA_INVALID_VCP_VERSION: WIN32_ERROR = WIN32_ERROR(15201u32);
                    pub const ERROR_MCA_MONITOR_VIOLATES_MCCS_SPECIFICATION: WIN32_ERROR =
                        WIN32_ERROR(15202u32);
                    pub const ERROR_MCA_MCCS_VERSION_MISMATCH: WIN32_ERROR = WIN32_ERROR(15203u32);
                    pub const ERROR_MCA_UNSUPPORTED_MCCS_VERSION: WIN32_ERROR =
                        WIN32_ERROR(15204u32);
                    pub const ERROR_MCA_INTERNAL_ERROR: WIN32_ERROR = WIN32_ERROR(15205u32);
                    pub const ERROR_MCA_INVALID_TECHNOLOGY_TYPE_RETURNED: WIN32_ERROR =
                        WIN32_ERROR(15206u32);
                    pub const ERROR_MCA_UNSUPPORTED_COLOR_TEMPERATURE: WIN32_ERROR =
                        WIN32_ERROR(15207u32);
                    pub const ERROR_AMBIGUOUS_SYSTEM_DEVICE: WIN32_ERROR = WIN32_ERROR(15250u32);
                    pub const ERROR_SYSTEM_DEVICE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15299u32);
                    pub const ERROR_HASH_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(15300u32);
                    pub const ERROR_HASH_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(15301u32);
                    pub const ERROR_SECONDARY_IC_PROVIDER_NOT_REGISTERED: WIN32_ERROR =
                        WIN32_ERROR(15321u32);
                    pub const ERROR_GPIO_CLIENT_INFORMATION_INVALID: WIN32_ERROR =
                        WIN32_ERROR(15322u32);
                    pub const ERROR_GPIO_VERSION_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(15323u32);
                    pub const ERROR_GPIO_INVALID_REGISTRATION_PACKET: WIN32_ERROR =
                        WIN32_ERROR(15324u32);
                    pub const ERROR_GPIO_OPERATION_DENIED: WIN32_ERROR = WIN32_ERROR(15325u32);
                    pub const ERROR_GPIO_INCOMPATIBLE_CONNECT_MODE: WIN32_ERROR =
                        WIN32_ERROR(15326u32);
                    pub const ERROR_GPIO_INTERRUPT_ALREADY_UNMASKED: WIN32_ERROR =
                        WIN32_ERROR(15327u32);
                    pub const ERROR_CANNOT_SWITCH_RUNLEVEL: WIN32_ERROR = WIN32_ERROR(15400u32);
                    pub const ERROR_INVALID_RUNLEVEL_SETTING: WIN32_ERROR = WIN32_ERROR(15401u32);
                    pub const ERROR_RUNLEVEL_SWITCH_TIMEOUT: WIN32_ERROR = WIN32_ERROR(15402u32);
                    pub const ERROR_RUNLEVEL_SWITCH_AGENT_TIMEOUT: WIN32_ERROR =
                        WIN32_ERROR(15403u32);
                    pub const ERROR_RUNLEVEL_SWITCH_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(15404u32);
                    pub const ERROR_SERVICES_FAILED_AUTOSTART: WIN32_ERROR = WIN32_ERROR(15405u32);
                    pub const ERROR_COM_TASK_STOP_PENDING: WIN32_ERROR = WIN32_ERROR(15501u32);
                    pub const ERROR_INSTALL_OPEN_PACKAGE_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15600u32);
                    pub const ERROR_INSTALL_PACKAGE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(15601u32);
                    pub const ERROR_INSTALL_INVALID_PACKAGE: WIN32_ERROR = WIN32_ERROR(15602u32);
                    pub const ERROR_INSTALL_RESOLVE_DEPENDENCY_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15603u32);
                    pub const ERROR_INSTALL_OUT_OF_DISK_SPACE: WIN32_ERROR = WIN32_ERROR(15604u32);
                    pub const ERROR_INSTALL_NETWORK_FAILURE: WIN32_ERROR = WIN32_ERROR(15605u32);
                    pub const ERROR_INSTALL_REGISTRATION_FAILURE: WIN32_ERROR =
                        WIN32_ERROR(15606u32);
                    pub const ERROR_INSTALL_DEREGISTRATION_FAILURE: WIN32_ERROR =
                        WIN32_ERROR(15607u32);
                    pub const ERROR_INSTALL_CANCEL: WIN32_ERROR = WIN32_ERROR(15608u32);
                    pub const ERROR_INSTALL_FAILED: WIN32_ERROR = WIN32_ERROR(15609u32);
                    pub const ERROR_REMOVE_FAILED: WIN32_ERROR = WIN32_ERROR(15610u32);
                    pub const ERROR_PACKAGE_ALREADY_EXISTS: WIN32_ERROR = WIN32_ERROR(15611u32);
                    pub const ERROR_NEEDS_REMEDIATION: WIN32_ERROR = WIN32_ERROR(15612u32);
                    pub const ERROR_INSTALL_PREREQUISITE_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15613u32);
                    pub const ERROR_PACKAGE_REPOSITORY_CORRUPTED: WIN32_ERROR =
                        WIN32_ERROR(15614u32);
                    pub const ERROR_INSTALL_POLICY_FAILURE: WIN32_ERROR = WIN32_ERROR(15615u32);
                    pub const ERROR_PACKAGE_UPDATING: WIN32_ERROR = WIN32_ERROR(15616u32);
                    pub const ERROR_DEPLOYMENT_BLOCKED_BY_POLICY: WIN32_ERROR =
                        WIN32_ERROR(15617u32);
                    pub const ERROR_PACKAGES_IN_USE: WIN32_ERROR = WIN32_ERROR(15618u32);
                    pub const ERROR_RECOVERY_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(15619u32);
                    pub const ERROR_INVALID_STAGED_SIGNATURE: WIN32_ERROR = WIN32_ERROR(15620u32);
                    pub const ERROR_DELETING_EXISTING_APPLICATIONDATA_STORE_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15621u32);
                    pub const ERROR_INSTALL_PACKAGE_DOWNGRADE: WIN32_ERROR = WIN32_ERROR(15622u32);
                    pub const ERROR_SYSTEM_NEEDS_REMEDIATION: WIN32_ERROR = WIN32_ERROR(15623u32);
                    pub const ERROR_APPX_INTEGRITY_FAILURE_CLR_NGEN: WIN32_ERROR =
                        WIN32_ERROR(15624u32);
                    pub const ERROR_RESILIENCY_FILE_CORRUPT: WIN32_ERROR = WIN32_ERROR(15625u32);
                    pub const ERROR_INSTALL_FIREWALL_SERVICE_NOT_RUNNING: WIN32_ERROR =
                        WIN32_ERROR(15626u32);
                    pub const ERROR_PACKAGE_MOVE_FAILED: WIN32_ERROR = WIN32_ERROR(15627u32);
                    pub const ERROR_INSTALL_VOLUME_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(15628u32);
                    pub const ERROR_INSTALL_VOLUME_OFFLINE: WIN32_ERROR = WIN32_ERROR(15629u32);
                    pub const ERROR_INSTALL_VOLUME_CORRUPT: WIN32_ERROR = WIN32_ERROR(15630u32);
                    pub const ERROR_NEEDS_REGISTRATION: WIN32_ERROR = WIN32_ERROR(15631u32);
                    pub const ERROR_INSTALL_WRONG_PROCESSOR_ARCHITECTURE: WIN32_ERROR =
                        WIN32_ERROR(15632u32);
                    pub const ERROR_DEV_SIDELOAD_LIMIT_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(15633u32);
                    pub const ERROR_INSTALL_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE: WIN32_ERROR =
                        WIN32_ERROR(15634u32);
                    pub const ERROR_PACKAGE_NOT_SUPPORTED_ON_FILESYSTEM: WIN32_ERROR =
                        WIN32_ERROR(15635u32);
                    pub const ERROR_PACKAGE_MOVE_BLOCKED_BY_STREAMING: WIN32_ERROR =
                        WIN32_ERROR(15636u32);
                    pub const ERROR_INSTALL_OPTIONAL_PACKAGE_APPLICATIONID_NOT_UNIQUE: WIN32_ERROR =
                        WIN32_ERROR(15637u32);
                    pub const ERROR_PACKAGE_STAGING_ONHOLD: WIN32_ERROR = WIN32_ERROR(15638u32);
                    pub const ERROR_INSTALL_INVALID_RELATED_SET_UPDATE: WIN32_ERROR =
                        WIN32_ERROR(15639u32);
                    pub const ERROR_INSTALL_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE_FULLTRUST_CAPABILITY : WIN32_ERROR = WIN32_ERROR ( 15640u32 ) ;
                    pub const ERROR_DEPLOYMENT_BLOCKED_BY_USER_LOG_OFF: WIN32_ERROR =
                        WIN32_ERROR(15641u32);
                    pub const ERROR_PROVISION_OPTIONAL_PACKAGE_REQUIRES_MAIN_PACKAGE_PROVISIONED:
                        WIN32_ERROR = WIN32_ERROR(15642u32);
                    pub const ERROR_PACKAGES_REPUTATION_CHECK_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15643u32);
                    pub const ERROR_PACKAGES_REPUTATION_CHECK_TIMEDOUT: WIN32_ERROR =
                        WIN32_ERROR(15644u32);
                    pub const ERROR_DEPLOYMENT_OPTION_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(15645u32);
                    pub const ERROR_APPINSTALLER_ACTIVATION_BLOCKED: WIN32_ERROR =
                        WIN32_ERROR(15646u32);
                    pub const ERROR_REGISTRATION_FROM_REMOTE_DRIVE_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(15647u32);
                    pub const ERROR_APPX_RAW_DATA_WRITE_FAILED: WIN32_ERROR = WIN32_ERROR(15648u32);
                    pub const ERROR_DEPLOYMENT_BLOCKED_BY_VOLUME_POLICY_PACKAGE: WIN32_ERROR =
                        WIN32_ERROR(15649u32);
                    pub const ERROR_DEPLOYMENT_BLOCKED_BY_VOLUME_POLICY_MACHINE: WIN32_ERROR =
                        WIN32_ERROR(15650u32);
                    pub const ERROR_DEPLOYMENT_BLOCKED_BY_PROFILE_POLICY: WIN32_ERROR =
                        WIN32_ERROR(15651u32);
                    pub const ERROR_DEPLOYMENT_FAILED_CONFLICTING_MUTABLE_PACKAGE_DIRECTORY:
                        WIN32_ERROR = WIN32_ERROR(15652u32);
                    pub const ERROR_SINGLETON_RESOURCE_INSTALLED_IN_ACTIVE_USER: WIN32_ERROR =
                        WIN32_ERROR(15653u32);
                    pub const ERROR_DIFFERENT_VERSION_OF_PACKAGED_SERVICE_INSTALLED: WIN32_ERROR =
                        WIN32_ERROR(15654u32);
                    pub const ERROR_SERVICE_EXISTS_AS_NON_PACKAGED_SERVICE: WIN32_ERROR =
                        WIN32_ERROR(15655u32);
                    pub const ERROR_PACKAGED_SERVICE_REQUIRES_ADMIN_PRIVILEGES: WIN32_ERROR =
                        WIN32_ERROR(15656u32);
                    pub const ERROR_REDIRECTION_TO_DEFAULT_ACCOUNT_NOT_ALLOWED: WIN32_ERROR =
                        WIN32_ERROR(15657u32);
                    pub const ERROR_PACKAGE_LACKS_CAPABILITY_TO_DEPLOY_ON_HOST: WIN32_ERROR =
                        WIN32_ERROR(15658u32);
                    pub const ERROR_UNSIGNED_PACKAGE_INVALID_CONTENT: WIN32_ERROR =
                        WIN32_ERROR(15659u32);
                    pub const ERROR_UNSIGNED_PACKAGE_INVALID_PUBLISHER_NAMESPACE: WIN32_ERROR =
                        WIN32_ERROR(15660u32);
                    pub const ERROR_SIGNED_PACKAGE_INVALID_PUBLISHER_NAMESPACE: WIN32_ERROR =
                        WIN32_ERROR(15661u32);
                    pub const ERROR_PACKAGE_EXTERNAL_LOCATION_NOT_ALLOWED: WIN32_ERROR =
                        WIN32_ERROR(15662u32);
                    pub const ERROR_INSTALL_FULLTRUST_HOSTRUNTIME_REQUIRES_MAIN_PACKAGE_FULLTRUST_CAPABILITY : WIN32_ERROR = WIN32_ERROR ( 15663u32 ) ;
                    pub const ERROR_STATE_LOAD_STORE_FAILED: WIN32_ERROR = WIN32_ERROR(15800u32);
                    pub const ERROR_STATE_GET_VERSION_FAILED: WIN32_ERROR = WIN32_ERROR(15801u32);
                    pub const ERROR_STATE_SET_VERSION_FAILED: WIN32_ERROR = WIN32_ERROR(15802u32);
                    pub const ERROR_STATE_STRUCTURED_RESET_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15803u32);
                    pub const ERROR_STATE_OPEN_CONTAINER_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15804u32);
                    pub const ERROR_STATE_CREATE_CONTAINER_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15805u32);
                    pub const ERROR_STATE_DELETE_CONTAINER_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15806u32);
                    pub const ERROR_STATE_READ_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15807u32);
                    pub const ERROR_STATE_WRITE_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15808u32);
                    pub const ERROR_STATE_DELETE_SETTING_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15809u32);
                    pub const ERROR_STATE_QUERY_SETTING_FAILED: WIN32_ERROR = WIN32_ERROR(15810u32);
                    pub const ERROR_STATE_READ_COMPOSITE_SETTING_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15811u32);
                    pub const ERROR_STATE_WRITE_COMPOSITE_SETTING_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15812u32);
                    pub const ERROR_STATE_ENUMERATE_CONTAINER_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15813u32);
                    pub const ERROR_STATE_ENUMERATE_SETTINGS_FAILED: WIN32_ERROR =
                        WIN32_ERROR(15814u32);
                    pub const ERROR_STATE_COMPOSITE_SETTING_VALUE_SIZE_LIMIT_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(15815u32);
                    pub const ERROR_STATE_SETTING_VALUE_SIZE_LIMIT_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(15816u32);
                    pub const ERROR_STATE_SETTING_NAME_SIZE_LIMIT_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(15817u32);
                    pub const ERROR_STATE_CONTAINER_NAME_SIZE_LIMIT_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(15818u32);
                    pub const ERROR_API_UNAVAILABLE: WIN32_ERROR = WIN32_ERROR(15841u32);
                    pub const ERROR_NDIS_INTERFACE_CLOSING: WIN32_ERROR =
                        WIN32_ERROR(2150891522u32);
                    pub const ERROR_NDIS_BAD_VERSION: WIN32_ERROR = WIN32_ERROR(2150891524u32);
                    pub const ERROR_NDIS_BAD_CHARACTERISTICS: WIN32_ERROR =
                        WIN32_ERROR(2150891525u32);
                    pub const ERROR_NDIS_ADAPTER_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(2150891526u32);
                    pub const ERROR_NDIS_OPEN_FAILED: WIN32_ERROR = WIN32_ERROR(2150891527u32);
                    pub const ERROR_NDIS_DEVICE_FAILED: WIN32_ERROR = WIN32_ERROR(2150891528u32);
                    pub const ERROR_NDIS_MULTICAST_FULL: WIN32_ERROR = WIN32_ERROR(2150891529u32);
                    pub const ERROR_NDIS_MULTICAST_EXISTS: WIN32_ERROR = WIN32_ERROR(2150891530u32);
                    pub const ERROR_NDIS_MULTICAST_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(2150891531u32);
                    pub const ERROR_NDIS_REQUEST_ABORTED: WIN32_ERROR = WIN32_ERROR(2150891532u32);
                    pub const ERROR_NDIS_RESET_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(2150891533u32);
                    pub const ERROR_NDIS_NOT_SUPPORTED: WIN32_ERROR = WIN32_ERROR(2150891707u32);
                    pub const ERROR_NDIS_INVALID_PACKET: WIN32_ERROR = WIN32_ERROR(2150891535u32);
                    pub const ERROR_NDIS_ADAPTER_NOT_READY: WIN32_ERROR =
                        WIN32_ERROR(2150891537u32);
                    pub const ERROR_NDIS_INVALID_LENGTH: WIN32_ERROR = WIN32_ERROR(2150891540u32);
                    pub const ERROR_NDIS_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(2150891541u32);
                    pub const ERROR_NDIS_BUFFER_TOO_SHORT: WIN32_ERROR = WIN32_ERROR(2150891542u32);
                    pub const ERROR_NDIS_INVALID_OID: WIN32_ERROR = WIN32_ERROR(2150891543u32);
                    pub const ERROR_NDIS_ADAPTER_REMOVED: WIN32_ERROR = WIN32_ERROR(2150891544u32);
                    pub const ERROR_NDIS_UNSUPPORTED_MEDIA: WIN32_ERROR =
                        WIN32_ERROR(2150891545u32);
                    pub const ERROR_NDIS_GROUP_ADDRESS_IN_USE: WIN32_ERROR =
                        WIN32_ERROR(2150891546u32);
                    pub const ERROR_NDIS_FILE_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(2150891547u32);
                    pub const ERROR_NDIS_ERROR_READING_FILE: WIN32_ERROR =
                        WIN32_ERROR(2150891548u32);
                    pub const ERROR_NDIS_ALREADY_MAPPED: WIN32_ERROR = WIN32_ERROR(2150891549u32);
                    pub const ERROR_NDIS_RESOURCE_CONFLICT: WIN32_ERROR =
                        WIN32_ERROR(2150891550u32);
                    pub const ERROR_NDIS_MEDIA_DISCONNECTED: WIN32_ERROR =
                        WIN32_ERROR(2150891551u32);
                    pub const ERROR_NDIS_INVALID_ADDRESS: WIN32_ERROR = WIN32_ERROR(2150891554u32);
                    pub const ERROR_NDIS_INVALID_DEVICE_REQUEST: WIN32_ERROR =
                        WIN32_ERROR(2150891536u32);
                    pub const ERROR_NDIS_PAUSED: WIN32_ERROR = WIN32_ERROR(2150891562u32);
                    pub const ERROR_NDIS_INTERFACE_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(2150891563u32);
                    pub const ERROR_NDIS_UNSUPPORTED_REVISION: WIN32_ERROR =
                        WIN32_ERROR(2150891564u32);
                    pub const ERROR_NDIS_INVALID_PORT: WIN32_ERROR = WIN32_ERROR(2150891565u32);
                    pub const ERROR_NDIS_INVALID_PORT_STATE: WIN32_ERROR =
                        WIN32_ERROR(2150891566u32);
                    pub const ERROR_NDIS_LOW_POWER_STATE: WIN32_ERROR = WIN32_ERROR(2150891567u32);
                    pub const ERROR_NDIS_REINIT_REQUIRED: WIN32_ERROR = WIN32_ERROR(2150891568u32);
                    pub const ERROR_NDIS_NO_QUEUES: WIN32_ERROR = WIN32_ERROR(2150891569u32);
                    pub const ERROR_NDIS_DOT11_AUTO_CONFIG_ENABLED: WIN32_ERROR =
                        WIN32_ERROR(2150899712u32);
                    pub const ERROR_NDIS_DOT11_MEDIA_IN_USE: WIN32_ERROR =
                        WIN32_ERROR(2150899713u32);
                    pub const ERROR_NDIS_DOT11_POWER_STATE_INVALID: WIN32_ERROR =
                        WIN32_ERROR(2150899714u32);
                    pub const ERROR_NDIS_PM_WOL_PATTERN_LIST_FULL: WIN32_ERROR =
                        WIN32_ERROR(2150899715u32);
                    pub const ERROR_NDIS_PM_PROTOCOL_OFFLOAD_LIST_FULL: WIN32_ERROR =
                        WIN32_ERROR(2150899716u32);
                    pub const ERROR_NDIS_DOT11_AP_CHANNEL_CURRENTLY_NOT_AVAILABLE: WIN32_ERROR =
                        WIN32_ERROR(2150899717u32);
                    pub const ERROR_NDIS_DOT11_AP_BAND_CURRENTLY_NOT_AVAILABLE: WIN32_ERROR =
                        WIN32_ERROR(2150899718u32);
                    pub const ERROR_NDIS_DOT11_AP_CHANNEL_NOT_ALLOWED: WIN32_ERROR =
                        WIN32_ERROR(2150899719u32);
                    pub const ERROR_NDIS_DOT11_AP_BAND_NOT_ALLOWED: WIN32_ERROR =
                        WIN32_ERROR(2150899720u32);
                    pub const ERROR_NDIS_INDICATION_REQUIRED: WIN32_ERROR = WIN32_ERROR(3407873u32);
                    pub const ERROR_NDIS_OFFLOAD_POLICY: WIN32_ERROR = WIN32_ERROR(3224637455u32);
                    pub const ERROR_NDIS_OFFLOAD_CONNECTION_REJECTED: WIN32_ERROR =
                        WIN32_ERROR(3224637458u32);
                    pub const ERROR_NDIS_OFFLOAD_PATH_REJECTED: WIN32_ERROR =
                        WIN32_ERROR(3224637459u32);
                    pub const ERROR_HV_INVALID_HYPERCALL_CODE: WIN32_ERROR =
                        WIN32_ERROR(3224698882u32);
                    pub const ERROR_HV_INVALID_HYPERCALL_INPUT: WIN32_ERROR =
                        WIN32_ERROR(3224698883u32);
                    pub const ERROR_HV_INVALID_ALIGNMENT: WIN32_ERROR = WIN32_ERROR(3224698884u32);
                    pub const ERROR_HV_INVALID_PARAMETER: WIN32_ERROR = WIN32_ERROR(3224698885u32);
                    pub const ERROR_HV_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(3224698886u32);
                    pub const ERROR_HV_INVALID_PARTITION_STATE: WIN32_ERROR =
                        WIN32_ERROR(3224698887u32);
                    pub const ERROR_HV_OPERATION_DENIED: WIN32_ERROR = WIN32_ERROR(3224698888u32);
                    pub const ERROR_HV_UNKNOWN_PROPERTY: WIN32_ERROR = WIN32_ERROR(3224698889u32);
                    pub const ERROR_HV_PROPERTY_VALUE_OUT_OF_RANGE: WIN32_ERROR =
                        WIN32_ERROR(3224698890u32);
                    pub const ERROR_HV_INSUFFICIENT_MEMORY: WIN32_ERROR =
                        WIN32_ERROR(3224698891u32);
                    pub const ERROR_HV_PARTITION_TOO_DEEP: WIN32_ERROR = WIN32_ERROR(3224698892u32);
                    pub const ERROR_HV_INVALID_PARTITION_ID: WIN32_ERROR =
                        WIN32_ERROR(3224698893u32);
                    pub const ERROR_HV_INVALID_VP_INDEX: WIN32_ERROR = WIN32_ERROR(3224698894u32);
                    pub const ERROR_HV_INVALID_PORT_ID: WIN32_ERROR = WIN32_ERROR(3224698897u32);
                    pub const ERROR_HV_INVALID_CONNECTION_ID: WIN32_ERROR =
                        WIN32_ERROR(3224698898u32);
                    pub const ERROR_HV_INSUFFICIENT_BUFFERS: WIN32_ERROR =
                        WIN32_ERROR(3224698899u32);
                    pub const ERROR_HV_NOT_ACKNOWLEDGED: WIN32_ERROR = WIN32_ERROR(3224698900u32);
                    pub const ERROR_HV_INVALID_VP_STATE: WIN32_ERROR = WIN32_ERROR(3224698901u32);
                    pub const ERROR_HV_ACKNOWLEDGED: WIN32_ERROR = WIN32_ERROR(3224698902u32);
                    pub const ERROR_HV_INVALID_SAVE_RESTORE_STATE: WIN32_ERROR =
                        WIN32_ERROR(3224698903u32);
                    pub const ERROR_HV_INVALID_SYNIC_STATE: WIN32_ERROR =
                        WIN32_ERROR(3224698904u32);
                    pub const ERROR_HV_OBJECT_IN_USE: WIN32_ERROR = WIN32_ERROR(3224698905u32);
                    pub const ERROR_HV_INVALID_PROXIMITY_DOMAIN_INFO: WIN32_ERROR =
                        WIN32_ERROR(3224698906u32);
                    pub const ERROR_HV_NO_DATA: WIN32_ERROR = WIN32_ERROR(3224698907u32);
                    pub const ERROR_HV_INACTIVE: WIN32_ERROR = WIN32_ERROR(3224698908u32);
                    pub const ERROR_HV_NO_RESOURCES: WIN32_ERROR = WIN32_ERROR(3224698909u32);
                    pub const ERROR_HV_FEATURE_UNAVAILABLE: WIN32_ERROR =
                        WIN32_ERROR(3224698910u32);
                    pub const ERROR_HV_INSUFFICIENT_BUFFER: WIN32_ERROR =
                        WIN32_ERROR(3224698931u32);
                    pub const ERROR_HV_INSUFFICIENT_DEVICE_DOMAINS: WIN32_ERROR =
                        WIN32_ERROR(3224698936u32);
                    pub const ERROR_HV_CPUID_FEATURE_VALIDATION: WIN32_ERROR =
                        WIN32_ERROR(3224698940u32);
                    pub const ERROR_HV_CPUID_XSAVE_FEATURE_VALIDATION: WIN32_ERROR =
                        WIN32_ERROR(3224698941u32);
                    pub const ERROR_HV_PROCESSOR_STARTUP_TIMEOUT: WIN32_ERROR =
                        WIN32_ERROR(3224698942u32);
                    pub const ERROR_HV_SMX_ENABLED: WIN32_ERROR = WIN32_ERROR(3224698943u32);
                    pub const ERROR_HV_INVALID_LP_INDEX: WIN32_ERROR = WIN32_ERROR(3224698945u32);
                    pub const ERROR_HV_INVALID_REGISTER_VALUE: WIN32_ERROR =
                        WIN32_ERROR(3224698960u32);
                    pub const ERROR_HV_INVALID_VTL_STATE: WIN32_ERROR = WIN32_ERROR(3224698961u32);
                    pub const ERROR_HV_NX_NOT_DETECTED: WIN32_ERROR = WIN32_ERROR(3224698965u32);
                    pub const ERROR_HV_INVALID_DEVICE_ID: WIN32_ERROR = WIN32_ERROR(3224698967u32);
                    pub const ERROR_HV_INVALID_DEVICE_STATE: WIN32_ERROR =
                        WIN32_ERROR(3224698968u32);
                    pub const ERROR_HV_PENDING_PAGE_REQUESTS: WIN32_ERROR = WIN32_ERROR(3473497u32);
                    pub const ERROR_HV_PAGE_REQUEST_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224698976u32);
                    pub const ERROR_HV_INVALID_CPU_GROUP_ID: WIN32_ERROR =
                        WIN32_ERROR(3224698991u32);
                    pub const ERROR_HV_INVALID_CPU_GROUP_STATE: WIN32_ERROR =
                        WIN32_ERROR(3224698992u32);
                    pub const ERROR_HV_OPERATION_FAILED: WIN32_ERROR = WIN32_ERROR(3224698993u32);
                    pub const ERROR_HV_NOT_ALLOWED_WITH_NESTED_VIRT_ACTIVE: WIN32_ERROR =
                        WIN32_ERROR(3224698994u32);
                    pub const ERROR_HV_INSUFFICIENT_ROOT_MEMORY: WIN32_ERROR =
                        WIN32_ERROR(3224698995u32);
                    pub const ERROR_HV_EVENT_BUFFER_ALREADY_FREED: WIN32_ERROR =
                        WIN32_ERROR(3224698996u32);
                    pub const ERROR_HV_INSUFFICIENT_CONTIGUOUS_MEMORY: WIN32_ERROR =
                        WIN32_ERROR(3224698997u32);
                    pub const ERROR_HV_NOT_PRESENT: WIN32_ERROR = WIN32_ERROR(3224702976u32);
                    pub const ERROR_VID_DUPLICATE_HANDLER: WIN32_ERROR = WIN32_ERROR(3224829953u32);
                    pub const ERROR_VID_TOO_MANY_HANDLERS: WIN32_ERROR = WIN32_ERROR(3224829954u32);
                    pub const ERROR_VID_QUEUE_FULL: WIN32_ERROR = WIN32_ERROR(3224829955u32);
                    pub const ERROR_VID_HANDLER_NOT_PRESENT: WIN32_ERROR =
                        WIN32_ERROR(3224829956u32);
                    pub const ERROR_VID_INVALID_OBJECT_NAME: WIN32_ERROR =
                        WIN32_ERROR(3224829957u32);
                    pub const ERROR_VID_PARTITION_NAME_TOO_LONG: WIN32_ERROR =
                        WIN32_ERROR(3224829958u32);
                    pub const ERROR_VID_MESSAGE_QUEUE_NAME_TOO_LONG: WIN32_ERROR =
                        WIN32_ERROR(3224829959u32);
                    pub const ERROR_VID_PARTITION_ALREADY_EXISTS: WIN32_ERROR =
                        WIN32_ERROR(3224829960u32);
                    pub const ERROR_VID_PARTITION_DOES_NOT_EXIST: WIN32_ERROR =
                        WIN32_ERROR(3224829961u32);
                    pub const ERROR_VID_PARTITION_NAME_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(3224829962u32);
                    pub const ERROR_VID_MESSAGE_QUEUE_ALREADY_EXISTS: WIN32_ERROR =
                        WIN32_ERROR(3224829963u32);
                    pub const ERROR_VID_EXCEEDED_MBP_ENTRY_MAP_LIMIT: WIN32_ERROR =
                        WIN32_ERROR(3224829964u32);
                    pub const ERROR_VID_MB_STILL_REFERENCED: WIN32_ERROR =
                        WIN32_ERROR(3224829965u32);
                    pub const ERROR_VID_CHILD_GPA_PAGE_SET_CORRUPTED: WIN32_ERROR =
                        WIN32_ERROR(3224829966u32);
                    pub const ERROR_VID_INVALID_NUMA_SETTINGS: WIN32_ERROR =
                        WIN32_ERROR(3224829967u32);
                    pub const ERROR_VID_INVALID_NUMA_NODE_INDEX: WIN32_ERROR =
                        WIN32_ERROR(3224829968u32);
                    pub const ERROR_VID_NOTIFICATION_QUEUE_ALREADY_ASSOCIATED: WIN32_ERROR =
                        WIN32_ERROR(3224829969u32);
                    pub const ERROR_VID_INVALID_MEMORY_BLOCK_HANDLE: WIN32_ERROR =
                        WIN32_ERROR(3224829970u32);
                    pub const ERROR_VID_PAGE_RANGE_OVERFLOW: WIN32_ERROR =
                        WIN32_ERROR(3224829971u32);
                    pub const ERROR_VID_INVALID_MESSAGE_QUEUE_HANDLE: WIN32_ERROR =
                        WIN32_ERROR(3224829972u32);
                    pub const ERROR_VID_INVALID_GPA_RANGE_HANDLE: WIN32_ERROR =
                        WIN32_ERROR(3224829973u32);
                    pub const ERROR_VID_NO_MEMORY_BLOCK_NOTIFICATION_QUEUE: WIN32_ERROR =
                        WIN32_ERROR(3224829974u32);
                    pub const ERROR_VID_MEMORY_BLOCK_LOCK_COUNT_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(3224829975u32);
                    pub const ERROR_VID_INVALID_PPM_HANDLE: WIN32_ERROR =
                        WIN32_ERROR(3224829976u32);
                    pub const ERROR_VID_MBPS_ARE_LOCKED: WIN32_ERROR = WIN32_ERROR(3224829977u32);
                    pub const ERROR_VID_MESSAGE_QUEUE_CLOSED: WIN32_ERROR =
                        WIN32_ERROR(3224829978u32);
                    pub const ERROR_VID_VIRTUAL_PROCESSOR_LIMIT_EXCEEDED: WIN32_ERROR =
                        WIN32_ERROR(3224829979u32);
                    pub const ERROR_VID_STOP_PENDING: WIN32_ERROR = WIN32_ERROR(3224829980u32);
                    pub const ERROR_VID_INVALID_PROCESSOR_STATE: WIN32_ERROR =
                        WIN32_ERROR(3224829981u32);
                    pub const ERROR_VID_EXCEEDED_KM_CONTEXT_COUNT_LIMIT: WIN32_ERROR =
                        WIN32_ERROR(3224829982u32);
                    pub const ERROR_VID_KM_INTERFACE_ALREADY_INITIALIZED: WIN32_ERROR =
                        WIN32_ERROR(3224829983u32);
                    pub const ERROR_VID_MB_PROPERTY_ALREADY_SET_RESET: WIN32_ERROR =
                        WIN32_ERROR(3224829984u32);
                    pub const ERROR_VID_MMIO_RANGE_DESTROYED: WIN32_ERROR =
                        WIN32_ERROR(3224829985u32);
                    pub const ERROR_VID_INVALID_CHILD_GPA_PAGE_SET: WIN32_ERROR =
                        WIN32_ERROR(3224829986u32);
                    pub const ERROR_VID_RESERVE_PAGE_SET_IS_BEING_USED: WIN32_ERROR =
                        WIN32_ERROR(3224829987u32);
                    pub const ERROR_VID_RESERVE_PAGE_SET_TOO_SMALL: WIN32_ERROR =
                        WIN32_ERROR(3224829988u32);
                    pub const ERROR_VID_MBP_ALREADY_LOCKED_USING_RESERVED_PAGE: WIN32_ERROR =
                        WIN32_ERROR(3224829989u32);
                    pub const ERROR_VID_MBP_COUNT_EXCEEDED_LIMIT: WIN32_ERROR =
                        WIN32_ERROR(3224829990u32);
                    pub const ERROR_VID_SAVED_STATE_CORRUPT: WIN32_ERROR =
                        WIN32_ERROR(3224829991u32);
                    pub const ERROR_VID_SAVED_STATE_UNRECOGNIZED_ITEM: WIN32_ERROR =
                        WIN32_ERROR(3224829992u32);
                    pub const ERROR_VID_SAVED_STATE_INCOMPATIBLE: WIN32_ERROR =
                        WIN32_ERROR(3224829993u32);
                    pub const ERROR_VID_VTL_ACCESS_DENIED: WIN32_ERROR = WIN32_ERROR(3224829994u32);
                    pub const ERROR_VMCOMPUTE_TERMINATED_DURING_START: WIN32_ERROR =
                        WIN32_ERROR(3224830208u32);
                    pub const ERROR_VMCOMPUTE_IMAGE_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(3224830209u32);
                    pub const ERROR_VMCOMPUTE_HYPERV_NOT_INSTALLED: WIN32_ERROR =
                        WIN32_ERROR(3224830210u32);
                    pub const ERROR_VMCOMPUTE_OPERATION_PENDING: WIN32_ERROR =
                        WIN32_ERROR(3224830211u32);
                    pub const ERROR_VMCOMPUTE_TOO_MANY_NOTIFICATIONS: WIN32_ERROR =
                        WIN32_ERROR(3224830212u32);
                    pub const ERROR_VMCOMPUTE_INVALID_STATE: WIN32_ERROR =
                        WIN32_ERROR(3224830213u32);
                    pub const ERROR_VMCOMPUTE_UNEXPECTED_EXIT: WIN32_ERROR =
                        WIN32_ERROR(3224830214u32);
                    pub const ERROR_VMCOMPUTE_TERMINATED: WIN32_ERROR = WIN32_ERROR(3224830215u32);
                    pub const ERROR_VMCOMPUTE_CONNECT_FAILED: WIN32_ERROR =
                        WIN32_ERROR(3224830216u32);
                    pub const ERROR_VMCOMPUTE_TIMEOUT: WIN32_ERROR = WIN32_ERROR(3224830217u32);
                    pub const ERROR_VMCOMPUTE_CONNECTION_CLOSED: WIN32_ERROR =
                        WIN32_ERROR(3224830218u32);
                    pub const ERROR_VMCOMPUTE_UNKNOWN_MESSAGE: WIN32_ERROR =
                        WIN32_ERROR(3224830219u32);
                    pub const ERROR_VMCOMPUTE_UNSUPPORTED_PROTOCOL_VERSION: WIN32_ERROR =
                        WIN32_ERROR(3224830220u32);
                    pub const ERROR_VMCOMPUTE_INVALID_JSON: WIN32_ERROR =
                        WIN32_ERROR(3224830221u32);
                    pub const ERROR_VMCOMPUTE_SYSTEM_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(3224830222u32);
                    pub const ERROR_VMCOMPUTE_SYSTEM_ALREADY_EXISTS: WIN32_ERROR =
                        WIN32_ERROR(3224830223u32);
                    pub const ERROR_VMCOMPUTE_SYSTEM_ALREADY_STOPPED: WIN32_ERROR =
                        WIN32_ERROR(3224830224u32);
                    pub const ERROR_VMCOMPUTE_PROTOCOL_ERROR: WIN32_ERROR =
                        WIN32_ERROR(3224830225u32);
                    pub const ERROR_VMCOMPUTE_INVALID_LAYER: WIN32_ERROR =
                        WIN32_ERROR(3224830226u32);
                    pub const ERROR_VMCOMPUTE_WINDOWS_INSIDER_REQUIRED: WIN32_ERROR =
                        WIN32_ERROR(3224830227u32);
                    pub const ERROR_VNET_VIRTUAL_SWITCH_NAME_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(3224830464u32);
                    pub const ERROR_VID_REMOTE_NODE_PARENT_GPA_PAGES_USED: WIN32_ERROR =
                        WIN32_ERROR(2151088129u32);
                    pub const ERROR_VSMB_SAVED_STATE_FILE_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(3224830976u32);
                    pub const ERROR_VSMB_SAVED_STATE_CORRUPT: WIN32_ERROR =
                        WIN32_ERROR(3224830977u32);
                    pub const ERROR_VOLMGR_INCOMPLETE_REGENERATION: WIN32_ERROR =
                        WIN32_ERROR(2151153665u32);
                    pub const ERROR_VOLMGR_INCOMPLETE_DISK_MIGRATION: WIN32_ERROR =
                        WIN32_ERROR(2151153666u32);
                    pub const ERROR_VOLMGR_DATABASE_FULL: WIN32_ERROR = WIN32_ERROR(3224895489u32);
                    pub const ERROR_VOLMGR_DISK_CONFIGURATION_CORRUPTED: WIN32_ERROR =
                        WIN32_ERROR(3224895490u32);
                    pub const ERROR_VOLMGR_DISK_CONFIGURATION_NOT_IN_SYNC: WIN32_ERROR =
                        WIN32_ERROR(3224895491u32);
                    pub const ERROR_VOLMGR_PACK_CONFIG_UPDATE_FAILED: WIN32_ERROR =
                        WIN32_ERROR(3224895492u32);
                    pub const ERROR_VOLMGR_DISK_CONTAINS_NON_SIMPLE_VOLUME: WIN32_ERROR =
                        WIN32_ERROR(3224895493u32);
                    pub const ERROR_VOLMGR_DISK_DUPLICATE: WIN32_ERROR = WIN32_ERROR(3224895494u32);
                    pub const ERROR_VOLMGR_DISK_DYNAMIC: WIN32_ERROR = WIN32_ERROR(3224895495u32);
                    pub const ERROR_VOLMGR_DISK_ID_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895496u32);
                    pub const ERROR_VOLMGR_DISK_INVALID: WIN32_ERROR = WIN32_ERROR(3224895497u32);
                    pub const ERROR_VOLMGR_DISK_LAST_VOTER: WIN32_ERROR =
                        WIN32_ERROR(3224895498u32);
                    pub const ERROR_VOLMGR_DISK_LAYOUT_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895499u32);
                    pub const ERROR_VOLMGR_DISK_LAYOUT_NON_BASIC_BETWEEN_BASIC_PARTITIONS:
                        WIN32_ERROR = WIN32_ERROR(3224895500u32);
                    pub const ERROR_VOLMGR_DISK_LAYOUT_NOT_CYLINDER_ALIGNED: WIN32_ERROR =
                        WIN32_ERROR(3224895501u32);
                    pub const ERROR_VOLMGR_DISK_LAYOUT_PARTITIONS_TOO_SMALL: WIN32_ERROR =
                        WIN32_ERROR(3224895502u32);
                    pub const ERROR_VOLMGR_DISK_LAYOUT_PRIMARY_BETWEEN_LOGICAL_PARTITIONS:
                        WIN32_ERROR = WIN32_ERROR(3224895503u32);
                    pub const ERROR_VOLMGR_DISK_LAYOUT_TOO_MANY_PARTITIONS: WIN32_ERROR =
                        WIN32_ERROR(3224895504u32);
                    pub const ERROR_VOLMGR_DISK_MISSING: WIN32_ERROR = WIN32_ERROR(3224895505u32);
                    pub const ERROR_VOLMGR_DISK_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(3224895506u32);
                    pub const ERROR_VOLMGR_DISK_NOT_ENOUGH_SPACE: WIN32_ERROR =
                        WIN32_ERROR(3224895507u32);
                    pub const ERROR_VOLMGR_DISK_REVECTORING_FAILED: WIN32_ERROR =
                        WIN32_ERROR(3224895508u32);
                    pub const ERROR_VOLMGR_DISK_SECTOR_SIZE_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895509u32);
                    pub const ERROR_VOLMGR_DISK_SET_NOT_CONTAINED: WIN32_ERROR =
                        WIN32_ERROR(3224895510u32);
                    pub const ERROR_VOLMGR_DISK_USED_BY_MULTIPLE_MEMBERS: WIN32_ERROR =
                        WIN32_ERROR(3224895511u32);
                    pub const ERROR_VOLMGR_DISK_USED_BY_MULTIPLE_PLEXES: WIN32_ERROR =
                        WIN32_ERROR(3224895512u32);
                    pub const ERROR_VOLMGR_DYNAMIC_DISK_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(3224895513u32);
                    pub const ERROR_VOLMGR_EXTENT_ALREADY_USED: WIN32_ERROR =
                        WIN32_ERROR(3224895514u32);
                    pub const ERROR_VOLMGR_EXTENT_NOT_CONTIGUOUS: WIN32_ERROR =
                        WIN32_ERROR(3224895515u32);
                    pub const ERROR_VOLMGR_EXTENT_NOT_IN_PUBLIC_REGION: WIN32_ERROR =
                        WIN32_ERROR(3224895516u32);
                    pub const ERROR_VOLMGR_EXTENT_NOT_SECTOR_ALIGNED: WIN32_ERROR =
                        WIN32_ERROR(3224895517u32);
                    pub const ERROR_VOLMGR_EXTENT_OVERLAPS_EBR_PARTITION: WIN32_ERROR =
                        WIN32_ERROR(3224895518u32);
                    pub const ERROR_VOLMGR_EXTENT_VOLUME_LENGTHS_DO_NOT_MATCH: WIN32_ERROR =
                        WIN32_ERROR(3224895519u32);
                    pub const ERROR_VOLMGR_FAULT_TOLERANT_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(3224895520u32);
                    pub const ERROR_VOLMGR_INTERLEAVE_LENGTH_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895521u32);
                    pub const ERROR_VOLMGR_MAXIMUM_REGISTERED_USERS: WIN32_ERROR =
                        WIN32_ERROR(3224895522u32);
                    pub const ERROR_VOLMGR_MEMBER_IN_SYNC: WIN32_ERROR = WIN32_ERROR(3224895523u32);
                    pub const ERROR_VOLMGR_MEMBER_INDEX_DUPLICATE: WIN32_ERROR =
                        WIN32_ERROR(3224895524u32);
                    pub const ERROR_VOLMGR_MEMBER_INDEX_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895525u32);
                    pub const ERROR_VOLMGR_MEMBER_MISSING: WIN32_ERROR = WIN32_ERROR(3224895526u32);
                    pub const ERROR_VOLMGR_MEMBER_NOT_DETACHED: WIN32_ERROR =
                        WIN32_ERROR(3224895527u32);
                    pub const ERROR_VOLMGR_MEMBER_REGENERATING: WIN32_ERROR =
                        WIN32_ERROR(3224895528u32);
                    pub const ERROR_VOLMGR_ALL_DISKS_FAILED: WIN32_ERROR =
                        WIN32_ERROR(3224895529u32);
                    pub const ERROR_VOLMGR_NO_REGISTERED_USERS: WIN32_ERROR =
                        WIN32_ERROR(3224895530u32);
                    pub const ERROR_VOLMGR_NO_SUCH_USER: WIN32_ERROR = WIN32_ERROR(3224895531u32);
                    pub const ERROR_VOLMGR_NOTIFICATION_RESET: WIN32_ERROR =
                        WIN32_ERROR(3224895532u32);
                    pub const ERROR_VOLMGR_NUMBER_OF_MEMBERS_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895533u32);
                    pub const ERROR_VOLMGR_NUMBER_OF_PLEXES_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895534u32);
                    pub const ERROR_VOLMGR_PACK_DUPLICATE: WIN32_ERROR = WIN32_ERROR(3224895535u32);
                    pub const ERROR_VOLMGR_PACK_ID_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895536u32);
                    pub const ERROR_VOLMGR_PACK_INVALID: WIN32_ERROR = WIN32_ERROR(3224895537u32);
                    pub const ERROR_VOLMGR_PACK_NAME_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895538u32);
                    pub const ERROR_VOLMGR_PACK_OFFLINE: WIN32_ERROR = WIN32_ERROR(3224895539u32);
                    pub const ERROR_VOLMGR_PACK_HAS_QUORUM: WIN32_ERROR =
                        WIN32_ERROR(3224895540u32);
                    pub const ERROR_VOLMGR_PACK_WITHOUT_QUORUM: WIN32_ERROR =
                        WIN32_ERROR(3224895541u32);
                    pub const ERROR_VOLMGR_PARTITION_STYLE_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895542u32);
                    pub const ERROR_VOLMGR_PARTITION_UPDATE_FAILED: WIN32_ERROR =
                        WIN32_ERROR(3224895543u32);
                    pub const ERROR_VOLMGR_PLEX_IN_SYNC: WIN32_ERROR = WIN32_ERROR(3224895544u32);
                    pub const ERROR_VOLMGR_PLEX_INDEX_DUPLICATE: WIN32_ERROR =
                        WIN32_ERROR(3224895545u32);
                    pub const ERROR_VOLMGR_PLEX_INDEX_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895546u32);
                    pub const ERROR_VOLMGR_PLEX_LAST_ACTIVE: WIN32_ERROR =
                        WIN32_ERROR(3224895547u32);
                    pub const ERROR_VOLMGR_PLEX_MISSING: WIN32_ERROR = WIN32_ERROR(3224895548u32);
                    pub const ERROR_VOLMGR_PLEX_REGENERATING: WIN32_ERROR =
                        WIN32_ERROR(3224895549u32);
                    pub const ERROR_VOLMGR_PLEX_TYPE_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895550u32);
                    pub const ERROR_VOLMGR_PLEX_NOT_RAID5: WIN32_ERROR = WIN32_ERROR(3224895551u32);
                    pub const ERROR_VOLMGR_PLEX_NOT_SIMPLE: WIN32_ERROR =
                        WIN32_ERROR(3224895552u32);
                    pub const ERROR_VOLMGR_STRUCTURE_SIZE_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895553u32);
                    pub const ERROR_VOLMGR_TOO_MANY_NOTIFICATION_REQUESTS: WIN32_ERROR =
                        WIN32_ERROR(3224895554u32);
                    pub const ERROR_VOLMGR_TRANSACTION_IN_PROGRESS: WIN32_ERROR =
                        WIN32_ERROR(3224895555u32);
                    pub const ERROR_VOLMGR_UNEXPECTED_DISK_LAYOUT_CHANGE: WIN32_ERROR =
                        WIN32_ERROR(3224895556u32);
                    pub const ERROR_VOLMGR_VOLUME_CONTAINS_MISSING_DISK: WIN32_ERROR =
                        WIN32_ERROR(3224895557u32);
                    pub const ERROR_VOLMGR_VOLUME_ID_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895558u32);
                    pub const ERROR_VOLMGR_VOLUME_LENGTH_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895559u32);
                    pub const ERROR_VOLMGR_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: WIN32_ERROR =
                        WIN32_ERROR(3224895560u32);
                    pub const ERROR_VOLMGR_VOLUME_NOT_MIRRORED: WIN32_ERROR =
                        WIN32_ERROR(3224895561u32);
                    pub const ERROR_VOLMGR_VOLUME_NOT_RETAINED: WIN32_ERROR =
                        WIN32_ERROR(3224895562u32);
                    pub const ERROR_VOLMGR_VOLUME_OFFLINE: WIN32_ERROR = WIN32_ERROR(3224895563u32);
                    pub const ERROR_VOLMGR_VOLUME_RETAINED: WIN32_ERROR =
                        WIN32_ERROR(3224895564u32);
                    pub const ERROR_VOLMGR_NUMBER_OF_EXTENTS_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895565u32);
                    pub const ERROR_VOLMGR_DIFFERENT_SECTOR_SIZE: WIN32_ERROR =
                        WIN32_ERROR(3224895566u32);
                    pub const ERROR_VOLMGR_BAD_BOOT_DISK: WIN32_ERROR = WIN32_ERROR(3224895567u32);
                    pub const ERROR_VOLMGR_PACK_CONFIG_OFFLINE: WIN32_ERROR =
                        WIN32_ERROR(3224895568u32);
                    pub const ERROR_VOLMGR_PACK_CONFIG_ONLINE: WIN32_ERROR =
                        WIN32_ERROR(3224895569u32);
                    pub const ERROR_VOLMGR_NOT_PRIMARY_PACK: WIN32_ERROR =
                        WIN32_ERROR(3224895570u32);
                    pub const ERROR_VOLMGR_PACK_LOG_UPDATE_FAILED: WIN32_ERROR =
                        WIN32_ERROR(3224895571u32);
                    pub const ERROR_VOLMGR_NUMBER_OF_DISKS_IN_PLEX_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895572u32);
                    pub const ERROR_VOLMGR_NUMBER_OF_DISKS_IN_MEMBER_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895573u32);
                    pub const ERROR_VOLMGR_VOLUME_MIRRORED: WIN32_ERROR =
                        WIN32_ERROR(3224895574u32);
                    pub const ERROR_VOLMGR_PLEX_NOT_SIMPLE_SPANNED: WIN32_ERROR =
                        WIN32_ERROR(3224895575u32);
                    pub const ERROR_VOLMGR_NO_VALID_LOG_COPIES: WIN32_ERROR =
                        WIN32_ERROR(3224895576u32);
                    pub const ERROR_VOLMGR_PRIMARY_PACK_PRESENT: WIN32_ERROR =
                        WIN32_ERROR(3224895577u32);
                    pub const ERROR_VOLMGR_NUMBER_OF_DISKS_INVALID: WIN32_ERROR =
                        WIN32_ERROR(3224895578u32);
                    pub const ERROR_VOLMGR_MIRROR_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(3224895579u32);
                    pub const ERROR_VOLMGR_RAID5_NOT_SUPPORTED: WIN32_ERROR =
                        WIN32_ERROR(3224895580u32);
                    pub const ERROR_BCD_NOT_ALL_ENTRIES_IMPORTED: WIN32_ERROR =
                        WIN32_ERROR(2151219201u32);
                    pub const ERROR_BCD_TOO_MANY_ELEMENTS: WIN32_ERROR = WIN32_ERROR(3224961026u32);
                    pub const ERROR_BCD_NOT_ALL_ENTRIES_SYNCHRONIZED: WIN32_ERROR =
                        WIN32_ERROR(2151219203u32);
                    pub const ERROR_VHD_DRIVE_FOOTER_MISSING: WIN32_ERROR =
                        WIN32_ERROR(3225026561u32);
                    pub const ERROR_VHD_DRIVE_FOOTER_CHECKSUM_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(3225026562u32);
                    pub const ERROR_VHD_DRIVE_FOOTER_CORRUPT: WIN32_ERROR =
                        WIN32_ERROR(3225026563u32);
                    pub const ERROR_VHD_FORMAT_UNKNOWN: WIN32_ERROR = WIN32_ERROR(3225026564u32);
                    pub const ERROR_VHD_FORMAT_UNSUPPORTED_VERSION: WIN32_ERROR =
                        WIN32_ERROR(3225026565u32);
                    pub const ERROR_VHD_SPARSE_HEADER_CHECKSUM_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(3225026566u32);
                    pub const ERROR_VHD_SPARSE_HEADER_UNSUPPORTED_VERSION: WIN32_ERROR =
                        WIN32_ERROR(3225026567u32);
                    pub const ERROR_VHD_SPARSE_HEADER_CORRUPT: WIN32_ERROR =
                        WIN32_ERROR(3225026568u32);
                    pub const ERROR_VHD_BLOCK_ALLOCATION_FAILURE: WIN32_ERROR =
                        WIN32_ERROR(3225026569u32);
                    pub const ERROR_VHD_BLOCK_ALLOCATION_TABLE_CORRUPT: WIN32_ERROR =
                        WIN32_ERROR(3225026570u32);
                    pub const ERROR_VHD_INVALID_BLOCK_SIZE: WIN32_ERROR =
                        WIN32_ERROR(3225026571u32);
                    pub const ERROR_VHD_BITMAP_MISMATCH: WIN32_ERROR = WIN32_ERROR(3225026572u32);
                    pub const ERROR_VHD_PARENT_VHD_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(3225026573u32);
                    pub const ERROR_VHD_CHILD_PARENT_ID_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(3225026574u32);
                    pub const ERROR_VHD_CHILD_PARENT_TIMESTAMP_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(3225026575u32);
                    pub const ERROR_VHD_METADATA_READ_FAILURE: WIN32_ERROR =
                        WIN32_ERROR(3225026576u32);
                    pub const ERROR_VHD_METADATA_WRITE_FAILURE: WIN32_ERROR =
                        WIN32_ERROR(3225026577u32);
                    pub const ERROR_VHD_INVALID_SIZE: WIN32_ERROR = WIN32_ERROR(3225026578u32);
                    pub const ERROR_VHD_INVALID_FILE_SIZE: WIN32_ERROR = WIN32_ERROR(3225026579u32);
                    pub const ERROR_VIRTDISK_PROVIDER_NOT_FOUND: WIN32_ERROR =
                        WIN32_ERROR(3225026580u32);
                    pub const ERROR_VIRTDISK_NOT_VIRTUAL_DISK: WIN32_ERROR =
                        WIN32_ERROR(3225026581u32);
                    pub const ERROR_VHD_PARENT_VHD_ACCESS_DENIED: WIN32_ERROR =
                        WIN32_ERROR(3225026582u32);
                    pub const ERROR_VHD_CHILD_PARENT_SIZE_MISMATCH: WIN32_ERROR =
                        WIN32_ERROR(3225026583u32);
                    pub const ERROR_VHD_DIFFERENCING_CHAIN_CYCLE_DETECTED: WIN32_ERROR =
                        WIN32_ERROR(3225026584u32);
                    pub const ERROR_VHD_DIFFERENCING_CHAIN_ERROR_IN_PARENT: WIN32_ERROR =
                        WIN32_ERROR(3225026585u32);
                    pub const ERROR_VIRTUAL_DISK_LIMITATION: WIN32_ERROR =
                        WIN32_ERROR(3225026586u32);
                    pub const ERROR_VHD_INVALID_TYPE: WIN32_ERROR = WIN32_ERROR(3225026587u32);
                    pub const ERROR_VHD_INVALID_STATE: WIN32_ERROR = WIN32_ERROR(3225026588u32);
                    pub const ERROR_VIRTDISK_UNSUPPORTED_DISK_SECTOR_SIZE: WIN32_ERROR =
                        WIN32_ERROR(3225026589u32);
                    pub const ERROR_VIRTDISK_DISK_ALREADY_OWNED: WIN32_ERROR =
                        WIN32_ERROR(3225026590u32);
                    pub const ERROR_VIRTDISK_DISK_ONLINE_AND_WRITABLE: WIN32_ERROR =
                        WIN32_ERROR(3225026591u32);
                    pub const ERROR_CTLOG_TRACKING_NOT_INITIALIZED: WIN32_ERROR =
                        WIN32_ERROR(3225026592u32);
                    pub const ERROR_CTLOG_LOGFILE_SIZE_EXCEEDED_MAXSIZE: WIN32_ERROR =
                        WIN32_ERROR(3225026593u32);
                    pub const ERROR_CTLOG_VHD_CHANGED_OFFLINE: WIN32_ERROR =
                        WIN32_ERROR(3225026594u32);
                    pub const ERROR_CTLOG_INVALID_TRACKING_STATE: WIN32_ERROR =
                        WIN32_ERROR(3225026595u32);
                    pub const ERROR_CTLOG_INCONSISTENT_TRACKING_FILE: WIN32_ERROR =
                        WIN32_ERROR(3225026596u32);
                    pub const ERROR_VHD_RESIZE_WOULD_TRUNCATE_DATA: WIN32_ERROR =
                        WIN32_ERROR(3225026597u32);
                    pub const ERROR_VHD_COULD_NOT_COMPUTE_MINIMUM_VIRTUAL_SIZE: WIN32_ERROR =
                        WIN32_ERROR(3225026598u32);
                    pub const ERROR_VHD_ALREADY_AT_OR_BELOW_MINIMUM_VIRTUAL_SIZE: WIN32_ERROR =
                        WIN32_ERROR(3225026599u32);
                    pub const ERROR_VHD_METADATA_FULL: WIN32_ERROR = WIN32_ERROR(3225026600u32);
                    pub const ERROR_VHD_INVALID_CHANGE_TRACKING_ID: WIN32_ERROR =
                        WIN32_ERROR(3225026601u32);
                    pub const ERROR_VHD_CHANGE_TRACKING_DISABLED: WIN32_ERROR =
                        WIN32_ERROR(3225026602u32);
                    pub const ERROR_VHD_MISSING_CHANGE_TRACKING_INFORMATION: WIN32_ERROR =
                        WIN32_ERROR(3225026608u32);
                    pub const ERROR_QUERY_STORAGE_ERROR: WIN32_ERROR = WIN32_ERROR(2151284737u32);
                    impl ::std::convert::From<u32> for WIN32_ERROR {
                        fn from(value: u32) -> Self {
                            Self(value)
                        }
                    }
                    unsafe impl ::windows::Abi for WIN32_ERROR {
                        type Abi = Self;
                        type DefaultType = Self;
                    }
                    impl ::std::ops::BitOr for WIN32_ERROR {
                        type Output = Self;
                        fn bitor(self, rhs: Self) -> Self {
                            Self(self.0 | rhs.0)
                        }
                    }
                    impl ::std::ops::BitAnd for WIN32_ERROR {
                        type Output = Self;
                        fn bitand(self, rhs: Self) -> Self {
                            Self(self.0 & rhs.0)
                        }
                    }
                    impl ::std::ops::BitOrAssign for WIN32_ERROR {
                        fn bitor_assign(&mut self, rhs: Self) {
                            self.0.bitor_assign(rhs.0)
                        }
                    }
                    impl ::std::ops::BitAndAssign for WIN32_ERROR {
                        fn bitand_assign(&mut self, rhs: Self) {
                            self.0.bitand_assign(rhs.0)
                        }
                    }
                    impl ::std::ops::Not for WIN32_ERROR {
                        type Output = Self;
                        fn not(self) -> Self {
                            Self(self.0.not())
                        }
                    }
                    impl ::std::convert::From<WIN32_ERROR> for ::windows::HRESULT {
                        fn from(value: WIN32_ERROR) -> Self {
                            Self(if value.0 as i32 <= 0 {
                                value.0
                            } else {
                                (value.0 & 0x0000_FFFF) | (7 << 16) | 0x8000_0000
                            })
                        }
                    }
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Pipes {
                pub unsafe fn ConnectNamedPipe<'a>(
                    hnamedpipe: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                    lpoverlapped: *mut super::SystemServices::OVERLAPPED,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn ConnectNamedPipe(
                                hnamedpipe: super::super::Foundation::HANDLE,
                                lpoverlapped: *mut super::SystemServices::OVERLAPPED,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(ConnectNamedPipe(
                            hnamedpipe.into_param().abi(),
                            ::std::mem::transmute(lpoverlapped),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn CreateNamedPipeW<'a>(
                    lpname: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                    dwopenmode: u32,
                    dwpipemode: u32,
                    nmaxinstances: u32,
                    noutbuffersize: u32,
                    ninbuffersize: u32,
                    ndefaulttimeout: u32,
                    lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                ) -> super::super::Foundation::HANDLE {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn CreateNamedPipeW(
                                lpname: super::super::Foundation::PWSTR,
                                dwopenmode: u32,
                                dwpipemode: u32,
                                nmaxinstances: u32,
                                noutbuffersize: u32,
                                ninbuffersize: u32,
                                ndefaulttimeout: u32,
                                lpsecurityattributes : * const super::super::Security:: SECURITY_ATTRIBUTES,
                            ) -> super::super::Foundation::HANDLE;
                        }
                        ::std::mem::transmute(CreateNamedPipeW(
                            lpname.into_param().abi(),
                            ::std::mem::transmute(dwopenmode),
                            ::std::mem::transmute(dwpipemode),
                            ::std::mem::transmute(nmaxinstances),
                            ::std::mem::transmute(noutbuffersize),
                            ::std::mem::transmute(ninbuffersize),
                            ::std::mem::transmute(ndefaulttimeout),
                            ::std::mem::transmute(lpsecurityattributes),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn CreatePipe(
                    hreadpipe: *mut super::super::Foundation::HANDLE,
                    hwritepipe: *mut super::super::Foundation::HANDLE,
                    lppipeattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                    nsize: u32,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn CreatePipe(
                                hreadpipe: *mut super::super::Foundation::HANDLE,
                                hwritepipe: *mut super::super::Foundation::HANDLE,
                                lppipeattributes : * const super::super::Security:: SECURITY_ATTRIBUTES,
                                nsize: u32,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(CreatePipe(
                            ::std::mem::transmute(hreadpipe),
                            ::std::mem::transmute(hwritepipe),
                            ::std::mem::transmute(lppipeattributes),
                            ::std::mem::transmute(nsize),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn DisconnectNamedPipe<'a>(
                    hnamedpipe: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn DisconnectNamedPipe(
                                hnamedpipe: super::super::Foundation::HANDLE,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(DisconnectNamedPipe(hnamedpipe.into_param().abi()))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct NAMED_PIPE_INFO_FLAGS(pub u32);
                pub const PIPE_CLIENT_END: NAMED_PIPE_INFO_FLAGS = NAMED_PIPE_INFO_FLAGS(0u32);
                pub const PIPE_SERVER_END: NAMED_PIPE_INFO_FLAGS = NAMED_PIPE_INFO_FLAGS(1u32);
                pub const PIPE_TYPE_BYTE: NAMED_PIPE_INFO_FLAGS = NAMED_PIPE_INFO_FLAGS(0u32);
                pub const PIPE_TYPE_MESSAGE: NAMED_PIPE_INFO_FLAGS = NAMED_PIPE_INFO_FLAGS(4u32);
                impl ::std::convert::From<u32> for NAMED_PIPE_INFO_FLAGS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for NAMED_PIPE_INFO_FLAGS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for NAMED_PIPE_INFO_FLAGS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for NAMED_PIPE_INFO_FLAGS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for NAMED_PIPE_INFO_FLAGS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for NAMED_PIPE_INFO_FLAGS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::std::ops::Not for NAMED_PIPE_INFO_FLAGS {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct WAIT_NAMED_PIPE_TIME_OUT_FLAGS(pub u32);
                pub const NMPWAIT_USE_DEFAULT_WAIT: WAIT_NAMED_PIPE_TIME_OUT_FLAGS =
                    WAIT_NAMED_PIPE_TIME_OUT_FLAGS(0u32);
                pub const NMPWAIT_WAIT_FOREVER: WAIT_NAMED_PIPE_TIME_OUT_FLAGS =
                    WAIT_NAMED_PIPE_TIME_OUT_FLAGS(4294967295u32);
                impl ::std::convert::From<u32> for WAIT_NAMED_PIPE_TIME_OUT_FLAGS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for WAIT_NAMED_PIPE_TIME_OUT_FLAGS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for WAIT_NAMED_PIPE_TIME_OUT_FLAGS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for WAIT_NAMED_PIPE_TIME_OUT_FLAGS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for WAIT_NAMED_PIPE_TIME_OUT_FLAGS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for WAIT_NAMED_PIPE_TIME_OUT_FLAGS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                impl ::std::ops::Not for WAIT_NAMED_PIPE_TIME_OUT_FLAGS {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub unsafe fn WaitNamedPipeW<'a>(
                    lpnamedpipename: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                    ntimeout: WAIT_NAMED_PIPE_TIME_OUT_FLAGS,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn WaitNamedPipeW(
                                lpnamedpipename: super::super::Foundation::PWSTR,
                                ntimeout: WAIT_NAMED_PIPE_TIME_OUT_FLAGS,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(WaitNamedPipeW(
                            lpnamedpipename.into_param().abi(),
                            ::std::mem::transmute(ntimeout),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod SystemServices {
                #[derive(
                    :: std :: clone :: Clone,
                    :: std :: marker :: Copy,
                    :: std :: fmt :: Debug,
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                )]
                #[repr(transparent)]
                pub struct CHAR(pub u8);
                impl ::std::default::Default for CHAR {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                unsafe impl ::windows::Handle for CHAR {}
                unsafe impl ::windows::Abi for CHAR {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                pub unsafe fn GetOverlappedResult<'a>(
                    hfile: impl ::windows::IntoParam<'a, super::super::Foundation::HANDLE>,
                    lpoverlapped: *const OVERLAPPED,
                    lpnumberofbytestransferred: *mut u32,
                    bwait: impl ::windows::IntoParam<'a, super::super::Foundation::BOOL>,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn GetOverlappedResult(
                                hfile: super::super::Foundation::HANDLE,
                                lpoverlapped: *const OVERLAPPED,
                                lpnumberofbytestransferred: *mut u32,
                                bwait: super::super::Foundation::BOOL,
                            ) -> super::super::Foundation::BOOL;
                        }
                        ::std::mem::transmute(GetOverlappedResult(
                            hfile.into_param().abi(),
                            ::std::mem::transmute(lpoverlapped),
                            ::std::mem::transmute(lpnumberofbytestransferred),
                            bwait.into_param().abi(),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct OVERLAPPED {
                    pub Internal: usize,
                    pub InternalHigh: usize,
                    pub Anonymous: OVERLAPPED_0,
                    pub hEvent: super::super::Foundation::HANDLE,
                }
                impl OVERLAPPED {}
                impl ::std::default::Default for OVERLAPPED {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for OVERLAPPED {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for OVERLAPPED {}
                unsafe impl ::windows::Abi for OVERLAPPED {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub union OVERLAPPED_0 {
                    pub Anonymous: OVERLAPPED_0_0,
                    pub Pointer: *mut ::std::ffi::c_void,
                }
                impl OVERLAPPED_0 {}
                impl ::std::default::Default for OVERLAPPED_0 {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::cmp::PartialEq for OVERLAPPED_0 {
                    fn eq(&self, _other: &Self) -> bool {
                        unimplemented!()
                    }
                }
                impl ::std::cmp::Eq for OVERLAPPED_0 {}
                unsafe impl ::windows::Abi for OVERLAPPED_0 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct OVERLAPPED_0_0 {
                    pub Offset: u32,
                    pub OffsetHigh: u32,
                }
                impl OVERLAPPED_0_0 {}
                impl ::std::default::Default for OVERLAPPED_0_0 {
                    fn default() -> Self {
                        unsafe { ::std::mem::zeroed() }
                    }
                }
                impl ::std::fmt::Debug for OVERLAPPED_0_0 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("_Anonymous_e__Struct")
                            .field("Offset", &self.Offset)
                            .field("OffsetHigh", &self.OffsetHigh)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for OVERLAPPED_0_0 {
                    fn eq(&self, other: &Self) -> bool {
                        self.Offset == other.Offset && self.OffsetHigh == other.OffsetHigh
                    }
                }
                impl ::std::cmp::Eq for OVERLAPPED_0_0 {}
                unsafe impl ::windows::Abi for OVERLAPPED_0_0 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Threading {
                pub unsafe fn CreateEventW<'a>(
                    lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                    bmanualreset: impl ::windows::IntoParam<'a, super::super::Foundation::BOOL>,
                    binitialstate: impl ::windows::IntoParam<'a, super::super::Foundation::BOOL>,
                    lpname: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                ) -> super::super::Foundation::HANDLE {
                    #[cfg(windows)]
                    {
                        #[link(name = "kernel32")]
                        extern "system" {
                            fn CreateEventW(
                                lpeventattributes : * const super::super::Security:: SECURITY_ATTRIBUTES,
                                bmanualreset: super::super::Foundation::BOOL,
                                binitialstate: super::super::Foundation::BOOL,
                                lpname: super::super::Foundation::PWSTR,
                            ) -> super::super::Foundation::HANDLE;
                        }
                        ::std::mem::transmute(CreateEventW(
                            ::std::mem::transmute(lpeventattributes),
                            bmanualreset.into_param().abi(),
                            binitialstate.into_param().abi(),
                            lpname.into_param().abi(),
                        ))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod WindowsProgramming {
                pub const INFINITE: u32 = 4294967295u32;
                pub const PIPE_ACCESS_DUPLEX: u32 = 3u32;
                pub const PIPE_ACCESS_INBOUND: u32 = 1u32;
                pub const PIPE_ACCESS_OUTBOUND: u32 = 2u32;
                pub const PIPE_REJECT_REMOTE_CLIENTS: u32 = 8u32;
                pub const PIPE_UNLIMITED_INSTANCES: u32 = 255u32;
            }
        }
    }
}
