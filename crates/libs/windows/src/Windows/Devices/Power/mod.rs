#[doc(hidden)]
#[repr(transparent)]
pub struct IBattery(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBattery {
    type Vtable = IBattery_Vtbl;
}
unsafe impl ::windows::core::Interface for IBattery {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc894fc6_0072_47c8_8b5d_614aaa7a437e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBattery_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReportUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReportUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBatteryReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBatteryReport {
    type Vtable = IBatteryReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IBatteryReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9858c3a_4e13_420a_a8d0_24f18f395401);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBatteryReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ChargeRateInMilliwatts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChargeRateInMilliwatts: usize,
    #[cfg(feature = "Foundation")]
    pub DesignCapacityInMilliwattHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesignCapacityInMilliwattHours: usize,
    #[cfg(feature = "Foundation")]
    pub FullChargeCapacityInMilliwattHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FullChargeCapacityInMilliwattHours: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingCapacityInMilliwattHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingCapacityInMilliwattHours: usize,
    #[cfg(feature = "System_Power")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::Power::BatteryStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System_Power"))]
    Status: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBatteryStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBatteryStatics {
    type Vtable = IBatteryStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IBatteryStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79cd72b6_9e5e_4452_bea6_dfcd541e597f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBatteryStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AggregateBattery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Power\"`*"]
#[repr(transparent)]
pub struct Battery(::windows::core::IUnknown);
impl Battery {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetReport(&self) -> ::windows::core::Result<BatteryReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetReport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportUpdated(&self, handler: &super::super::Foundation::TypedEventHandler<Battery, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportUpdated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReportUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveReportUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn AggregateBattery() -> ::windows::core::Result<Battery> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AggregateBattery)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Battery>> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBatteryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBatteryStatics<R, F: FnOnce(&IBatteryStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Battery, IBatteryStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Battery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Battery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Battery {}
impl ::core::fmt::Debug for Battery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Battery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Battery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Power.Battery;{bc894fc6-0072-47c8-8b5d-614aaa7a437e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Battery {
    type Vtable = IBattery_Vtbl;
}
unsafe impl ::windows::core::Interface for Battery {
    const IID: ::windows::core::GUID = <IBattery as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Battery {
    const NAME: &'static str = "Windows.Devices.Power.Battery";
}
::windows::core::interface_hierarchy!(Battery, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Battery {}
unsafe impl ::core::marker::Sync for Battery {}
#[doc = "*Required features: `\"Devices_Power\"`*"]
#[repr(transparent)]
pub struct BatteryReport(::windows::core::IUnknown);
impl BatteryReport {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChargeRateInMilliwatts(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChargeRateInMilliwatts)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesignCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DesignCapacityInMilliwattHours)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FullChargeCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FullChargeCapacityInMilliwattHours)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingCapacityInMilliwattHours(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemainingCapacityInMilliwattHours)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System_Power\"`*"]
    #[cfg(feature = "System_Power")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::System::Power::BatteryStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BatteryReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BatteryReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BatteryReport {}
impl ::core::fmt::Debug for BatteryReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BatteryReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BatteryReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Power.BatteryReport;{c9858c3a-4e13-420a-a8d0-24f18f395401})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BatteryReport {
    type Vtable = IBatteryReport_Vtbl;
}
unsafe impl ::windows::core::Interface for BatteryReport {
    const IID: ::windows::core::GUID = <IBatteryReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BatteryReport {
    const NAME: &'static str = "Windows.Devices.Power.BatteryReport";
}
::windows::core::interface_hierarchy!(BatteryReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BatteryReport {}
unsafe impl ::core::marker::Sync for BatteryReport {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
