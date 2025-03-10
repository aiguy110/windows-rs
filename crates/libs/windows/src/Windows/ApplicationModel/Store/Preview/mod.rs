#[cfg(feature = "ApplicationModel_Store_Preview_InstallControl")]
pub mod InstallControl;
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeliveryOptimizationSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDeliveryOptimizationSettings {
    type Vtable = IDeliveryOptimizationSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for IDeliveryOptimizationSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1810fda0_e853_565e_b874_7a8a7b9a0e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeliveryOptimizationSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DownloadMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeliveryOptimizationDownloadMode) -> ::windows::core::HRESULT,
    pub DownloadModeSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeliveryOptimizationDownloadModeSource) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeliveryOptimizationSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDeliveryOptimizationSettingsStatics {
    type Vtable = IDeliveryOptimizationSettingsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IDeliveryOptimizationSettingsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c817caf_aed5_5999_b4c9_8c60898bc4f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeliveryOptimizationSettingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetCurrentSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConfigurationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreConfigurationStatics {
    type Vtable = IStoreConfigurationStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreConfigurationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x728f7fc0_8628_42ec_84a2_07780eb44d8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetSystemConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cataloghardwaremanufacturerid: *mut ::core::ffi::c_void, catalogstorecontentmodifierid: *mut ::core::ffi::c_void, systemconfigurationexpiration: super::super::super::Foundation::DateTime, cataloghardwaredescriptor: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSystemConfiguration: usize,
    pub SetMobileOperatorConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mobileoperatorid: *mut ::core::ffi::c_void, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows::core::HRESULT,
    pub SetStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HardwareManufacturerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FilterUnsupportedSystemFeaturesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, systemfeatures: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FilterUnsupportedSystemFeaturesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConfigurationStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreConfigurationStatics2 {
    type Vtable = IStoreConfigurationStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreConfigurationStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x657c4595_c8b7_4fe9_9f4c_4d71027d347e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PurchasePromptingPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PurchasePromptingPolicy: usize,
    #[cfg(feature = "Foundation")]
    pub SetPurchasePromptingPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPurchasePromptingPolicy: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConfigurationStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreConfigurationStatics3 {
    type Vtable = IStoreConfigurationStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreConfigurationStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d45f57c_f144_4cb5_9d3f_4eb05e30b6d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HasStoreWebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub HasStoreWebAccountForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    HasStoreWebAccountForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetStoreLogDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: StoreLogOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetStoreLogDataAsync: usize,
    #[cfg(feature = "System")]
    pub SetStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, webaccountid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetStoreWebAccountIdForUser: usize,
    #[cfg(feature = "System")]
    pub IsStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, webaccountid: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    IsStoreWebAccountIdForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetPurchasePromptingPolicyForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetPurchasePromptingPolicyForUser: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SetPurchasePromptingPolicyForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SetPurchasePromptingPolicyForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConfigurationStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreConfigurationStatics4 {
    type Vtable = IStoreConfigurationStatics4_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreConfigurationStatics4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20ff56d2_4ee3_4cf0_9b12_552c03310f75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetStoreWebAccountIdForUser: usize,
    pub SetEnterpriseStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub SetEnterpriseStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, webaccountid: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    SetEnterpriseStoreWebAccountIdForUser: usize,
    pub GetEnterpriseStoreWebAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetEnterpriseStoreWebAccountIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetEnterpriseStoreWebAccountIdForUser: usize,
    pub ShouldRestrictToEnterpriseStoreOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub ShouldRestrictToEnterpriseStoreOnlyForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ShouldRestrictToEnterpriseStoreOnlyForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreConfigurationStatics5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreConfigurationStatics5 {
    type Vtable = IStoreConfigurationStatics5_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreConfigurationStatics5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7613191_8fa9_49db_822b_0160e7e4e5c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreConfigurationStatics5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPinToDesktopSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPinToTaskbarSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPinToStartSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PinToDesktop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, apppackagefamilyname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub PinToDesktopForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, apppackagefamilyname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    PinToDesktopForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStoreHardwareManufacturerInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStoreHardwareManufacturerInfo {
    type Vtable = IStoreHardwareManufacturerInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IStoreHardwareManufacturerInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf292dc08_c654_43ac_a21f_34801c9d3388);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStoreHardwareManufacturerInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HardwareManufacturerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StoreContentModifierId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ModelName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ManufacturerName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePreview {
    type Vtable = IStorePreview_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePreview {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a157241_840e_49a9_bc01_5d5b01fbc8e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreview_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestProductPurchaseByProductIdAndSkuIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: *mut ::core::ffi::c_void, skuid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestProductPurchaseByProductIdAndSkuIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub LoadAddOnProductInfosAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    LoadAddOnProductInfosAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePreviewProductInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePreviewProductInfo {
    type Vtable = IStorePreviewProductInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePreviewProductInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1937dbb3_6c01_4c9d_85cd_5babaac2b351);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewProductInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProductType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SkuInfoList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SkuInfoList: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePreviewPurchaseResults(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePreviewPurchaseResults {
    type Vtable = IStorePreviewPurchaseResults_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePreviewPurchaseResults {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0daaed1_d6c5_4e53_a043_fba0d8e61231);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewPurchaseResults_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProductPurchaseStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorePreviewProductPurchaseStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorePreviewSkuInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IStorePreviewSkuInfo {
    type Vtable = IStorePreviewSkuInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IStorePreviewSkuInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81fd76e2_0b26_48d9_98ce_27461c669d6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorePreviewSkuInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SkuId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SkuType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CustomDeveloperData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrencyCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FormattedListPrice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExtendedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationCoreManagerHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebAuthenticationCoreManagerHelper {
    type Vtable = IWebAuthenticationCoreManagerHelper_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebAuthenticationCoreManagerHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06a50525_e715_4123_9276_9d6f865ba55f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerHelper_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml"))]
    pub RequestTokenWithUIElementHostingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, uielement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml")))]
    RequestTokenWithUIElementHostingAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml"))]
    pub RequestTokenWithUIElementHostingAndWebAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, uielement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml")))]
    RequestTokenWithUIElementHostingAndWebAccountAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct DeliveryOptimizationSettings(::windows::core::IUnknown);
impl DeliveryOptimizationSettings {
    pub fn DownloadMode(&self) -> ::windows::core::Result<DeliveryOptimizationDownloadMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DownloadMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DownloadModeSource(&self) -> ::windows::core::Result<DeliveryOptimizationDownloadModeSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DownloadModeSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetCurrentSettings() -> ::windows::core::Result<DeliveryOptimizationSettings> {
        Self::IDeliveryOptimizationSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentSettings)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeliveryOptimizationSettingsStatics<R, F: FnOnce(&IDeliveryOptimizationSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DeliveryOptimizationSettings, IDeliveryOptimizationSettingsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DeliveryOptimizationSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeliveryOptimizationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeliveryOptimizationSettings {}
impl ::core::fmt::Debug for DeliveryOptimizationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeliveryOptimizationSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeliveryOptimizationSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings;{1810fda0-e853-565e-b874-7a8a7b9a0e0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for DeliveryOptimizationSettings {
    type Vtable = IDeliveryOptimizationSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for DeliveryOptimizationSettings {
    const IID: ::windows::core::GUID = <IDeliveryOptimizationSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DeliveryOptimizationSettings {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.DeliveryOptimizationSettings";
}
::windows::core::interface_hierarchy!(DeliveryOptimizationSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for DeliveryOptimizationSettings {}
unsafe impl ::core::marker::Sync for DeliveryOptimizationSettings {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
pub struct StoreConfiguration;
impl StoreConfiguration {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemConfiguration(cataloghardwaremanufacturerid: &::windows::core::HSTRING, catalogstorecontentmodifierid: &::windows::core::HSTRING, systemconfigurationexpiration: super::super::super::Foundation::DateTime, cataloghardwaredescriptor: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SetSystemConfiguration)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(cataloghardwaremanufacturerid), ::core::mem::transmute_copy(catalogstorecontentmodifierid), systemconfigurationexpiration, ::core::mem::transmute_copy(cataloghardwaredescriptor)).ok() })
    }
    pub fn SetMobileOperatorConfiguration(mobileoperatorid: &::windows::core::HSTRING, appdownloadlimitinmegabytes: u32, updatedownloadlimitinmegabytes: u32) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SetMobileOperatorConfiguration)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mobileoperatorid), appdownloadlimitinmegabytes, updatedownloadlimitinmegabytes).ok() })
    }
    pub fn SetStoreWebAccountId(webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SetStoreWebAccountId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(webaccountid)).ok() })
    }
    pub fn IsStoreWebAccountId(webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStoreWebAccountId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(webaccountid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn HardwareManufacturerInfo() -> ::windows::core::Result<StoreHardwareManufacturerInfo> {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HardwareManufacturerInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FilterUnsupportedSystemFeaturesAsync<P0, E0>(systemfeatures: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StoreSystemFeature>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<StoreSystemFeature>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IStoreConfigurationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FilterUnsupportedSystemFeaturesAsync)(::windows::core::Vtable::as_raw(this), systemfeatures.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PurchasePromptingPolicy() -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        Self::IStoreConfigurationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PurchasePromptingPolicy)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPurchasePromptingPolicy<P0, E0>(value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IStoreConfigurationStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).SetPurchasePromptingPolicy)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() })
    }
    pub fn HasStoreWebAccount() -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasStoreWebAccount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn HasStoreWebAccountForUser(user: &super::super::super::System::User) -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasStoreWebAccountForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetStoreLogDataAsync(options: StoreLogOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStreamReference>> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStoreLogDataAsync)(::windows::core::Vtable::as_raw(this), options, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetStoreWebAccountIdForUser(user: &super::super::super::System::User, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics3(|this| unsafe { (::windows::core::Vtable::vtable(this).SetStoreWebAccountIdForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(webaccountid)).ok() })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn IsStoreWebAccountIdForUser(user: &super::super::super::System::User, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStoreWebAccountIdForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(webaccountid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetPurchasePromptingPolicyForUser(user: &super::super::super::System::User) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        Self::IStoreConfigurationStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPurchasePromptingPolicyForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SetPurchasePromptingPolicyForUser<P0, E0>(user: &super::super::super::System::User, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IStoreConfigurationStatics3(|this| unsafe { (::windows::core::Vtable::vtable(this).SetPurchasePromptingPolicyForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), value.try_into().map_err(|e| e.into())?.abi()).ok() })
    }
    pub fn GetStoreWebAccountId() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStoreWebAccountId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetStoreWebAccountIdForUser(user: &super::super::super::System::User) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetStoreWebAccountIdForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetEnterpriseStoreWebAccountId(webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics4(|this| unsafe { (::windows::core::Vtable::vtable(this).SetEnterpriseStoreWebAccountId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(webaccountid)).ok() })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn SetEnterpriseStoreWebAccountIdForUser(user: &super::super::super::System::User, webaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics4(|this| unsafe { (::windows::core::Vtable::vtable(this).SetEnterpriseStoreWebAccountIdForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(webaccountid)).ok() })
    }
    pub fn GetEnterpriseStoreWebAccountId() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetEnterpriseStoreWebAccountId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetEnterpriseStoreWebAccountIdForUser(user: &super::super::super::System::User) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetEnterpriseStoreWebAccountIdForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ShouldRestrictToEnterpriseStoreOnly() -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShouldRestrictToEnterpriseStoreOnly)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ShouldRestrictToEnterpriseStoreOnlyForUser(user: &super::super::super::System::User) -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShouldRestrictToEnterpriseStoreOnlyForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsPinToDesktopSupported() -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPinToDesktopSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsPinToTaskbarSupported() -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPinToTaskbarSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IsPinToStartSupported() -> ::windows::core::Result<bool> {
        Self::IStoreConfigurationStatics5(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPinToStartSupported)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PinToDesktop(apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics5(|this| unsafe { (::windows::core::Vtable::vtable(this).PinToDesktop)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(apppackagefamilyname)).ok() })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn PinToDesktopForUser(user: &super::super::super::System::User, apppackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IStoreConfigurationStatics5(|this| unsafe { (::windows::core::Vtable::vtable(this).PinToDesktopForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(apppackagefamilyname)).ok() })
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics<R, F: FnOnce(&IStoreConfigurationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StoreConfiguration, IStoreConfigurationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics2<R, F: FnOnce(&IStoreConfigurationStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StoreConfiguration, IStoreConfigurationStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics3<R, F: FnOnce(&IStoreConfigurationStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StoreConfiguration, IStoreConfigurationStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics4<R, F: FnOnce(&IStoreConfigurationStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StoreConfiguration, IStoreConfigurationStatics4> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStoreConfigurationStatics5<R, F: FnOnce(&IStoreConfigurationStatics5) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StoreConfiguration, IStoreConfigurationStatics5> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for StoreConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StoreConfiguration";
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct StoreHardwareManufacturerInfo(::windows::core::IUnknown);
impl StoreHardwareManufacturerInfo {
    pub fn HardwareManufacturerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HardwareManufacturerId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StoreContentModifierId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StoreContentModifierId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ModelName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ModelName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ManufacturerName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ManufacturerName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StoreHardwareManufacturerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StoreHardwareManufacturerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StoreHardwareManufacturerInfo {}
impl ::core::fmt::Debug for StoreHardwareManufacturerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreHardwareManufacturerInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreHardwareManufacturerInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo;{f292dc08-c654-43ac-a21f-34801c9d3388})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StoreHardwareManufacturerInfo {
    type Vtable = IStoreHardwareManufacturerInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for StoreHardwareManufacturerInfo {
    const IID: ::windows::core::GUID = <IStoreHardwareManufacturerInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StoreHardwareManufacturerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StoreHardwareManufacturerInfo";
}
::windows::core::interface_hierarchy!(StoreHardwareManufacturerInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StoreHardwareManufacturerInfo {}
unsafe impl ::core::marker::Sync for StoreHardwareManufacturerInfo {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
pub struct StorePreview;
impl StorePreview {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestProductPurchaseByProductIdAndSkuIdAsync(productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<StorePreviewPurchaseResults>> {
        Self::IStorePreview(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestProductPurchaseByProductIdAndSkuIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn LoadAddOnProductInfosAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<StorePreviewProductInfo>>> {
        Self::IStorePreview(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LoadAddOnProductInfosAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorePreview<R, F: FnOnce(&IStorePreview) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StorePreview, IStorePreview> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for StorePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreview";
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct StorePreviewProductInfo(::windows::core::IUnknown);
impl StorePreviewProductInfo {
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProductId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProductType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProductType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SkuInfoList(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<StorePreviewSkuInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SkuInfoList)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StorePreviewProductInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePreviewProductInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePreviewProductInfo {}
impl ::core::fmt::Debug for StorePreviewProductInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePreviewProductInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePreviewProductInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo;{1937dbb3-6c01-4c9d-85cd-5babaac2b351})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorePreviewProductInfo {
    type Vtable = IStorePreviewProductInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for StorePreviewProductInfo {
    const IID: ::windows::core::GUID = <IStorePreviewProductInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePreviewProductInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewProductInfo";
}
::windows::core::interface_hierarchy!(StorePreviewProductInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorePreviewProductInfo {}
unsafe impl ::core::marker::Sync for StorePreviewProductInfo {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct StorePreviewPurchaseResults(::windows::core::IUnknown);
impl StorePreviewPurchaseResults {
    pub fn ProductPurchaseStatus(&self) -> ::windows::core::Result<StorePreviewProductPurchaseStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProductPurchaseStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StorePreviewPurchaseResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePreviewPurchaseResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePreviewPurchaseResults {}
impl ::core::fmt::Debug for StorePreviewPurchaseResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePreviewPurchaseResults").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePreviewPurchaseResults {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults;{b0daaed1-d6c5-4e53-a043-fba0d8e61231})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorePreviewPurchaseResults {
    type Vtable = IStorePreviewPurchaseResults_Vtbl;
}
unsafe impl ::windows::core::Interface for StorePreviewPurchaseResults {
    const IID: ::windows::core::GUID = <IStorePreviewPurchaseResults as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePreviewPurchaseResults {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewPurchaseResults";
}
::windows::core::interface_hierarchy!(StorePreviewPurchaseResults, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorePreviewPurchaseResults {}
unsafe impl ::core::marker::Sync for StorePreviewPurchaseResults {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
pub struct StorePreviewSkuInfo(::windows::core::IUnknown);
impl StorePreviewSkuInfo {
    pub fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProductId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SkuId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SkuId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SkuType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SkuType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Description)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CustomDeveloperData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomDeveloperData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CurrencyCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrencyCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FormattedListPrice(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FormattedListPrice)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for StorePreviewSkuInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorePreviewSkuInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorePreviewSkuInfo {}
impl ::core::fmt::Debug for StorePreviewSkuInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePreviewSkuInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePreviewSkuInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo;{81fd76e2-0b26-48d9-98ce-27461c669d6c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for StorePreviewSkuInfo {
    type Vtable = IStorePreviewSkuInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for StorePreviewSkuInfo {
    const IID: ::windows::core::GUID = <IStorePreviewSkuInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorePreviewSkuInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.StorePreviewSkuInfo";
}
::windows::core::interface_hierarchy!(StorePreviewSkuInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for StorePreviewSkuInfo {}
unsafe impl ::core::marker::Sync for StorePreviewSkuInfo {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
pub struct WebAuthenticationCoreManagerHelper;
impl WebAuthenticationCoreManagerHelper {
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Authentication_Web_Core\"`, `\"UI_Xaml\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "UI_Xaml"))]
    pub fn RequestTokenWithUIElementHostingAsync<P0>(request: &super::super::super::Security::Authentication::Web::Core::WebTokenRequest, uielement: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::UI::Xaml::UIElement>>,
    {
        Self::IWebAuthenticationCoreManagerHelper(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestTokenWithUIElementHostingAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(request), uielement.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Authentication_Web_Core\"`, `\"Security_Credentials\"`, `\"UI_Xaml\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Authentication_Web_Core", feature = "Security_Credentials", feature = "UI_Xaml"))]
    pub fn RequestTokenWithUIElementHostingAndWebAccountAsync<P0>(request: &super::super::super::Security::Authentication::Web::Core::WebTokenRequest, webaccount: &super::super::super::Security::Credentials::WebAccount, uielement: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Security::Authentication::Web::Core::WebTokenRequestResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::UI::Xaml::UIElement>>,
    {
        Self::IWebAuthenticationCoreManagerHelper(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestTokenWithUIElementHostingAndWebAccountAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(request), ::core::mem::transmute_copy(webaccount), uielement.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationCoreManagerHelper<R, F: FnOnce(&IWebAuthenticationCoreManagerHelper) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebAuthenticationCoreManagerHelper, IWebAuthenticationCoreManagerHelper> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for WebAuthenticationCoreManagerHelper {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.WebAuthenticationCoreManagerHelper";
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeliveryOptimizationDownloadMode(pub i32);
impl DeliveryOptimizationDownloadMode {
    pub const Simple: Self = Self(0i32);
    pub const HttpOnly: Self = Self(1i32);
    pub const Lan: Self = Self(2i32);
    pub const Group: Self = Self(3i32);
    pub const Internet: Self = Self(4i32);
    pub const Bypass: Self = Self(5i32);
}
impl ::core::marker::Copy for DeliveryOptimizationDownloadMode {}
impl ::core::clone::Clone for DeliveryOptimizationDownloadMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeliveryOptimizationDownloadMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeliveryOptimizationDownloadMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeliveryOptimizationDownloadMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeliveryOptimizationDownloadMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeliveryOptimizationDownloadMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeliveryOptimizationDownloadModeSource(pub i32);
impl DeliveryOptimizationDownloadModeSource {
    pub const Default: Self = Self(0i32);
    pub const Policy: Self = Self(1i32);
}
impl ::core::marker::Copy for DeliveryOptimizationDownloadModeSource {}
impl ::core::clone::Clone for DeliveryOptimizationDownloadModeSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeliveryOptimizationDownloadModeSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeliveryOptimizationDownloadModeSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeliveryOptimizationDownloadModeSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeliveryOptimizationDownloadModeSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeliveryOptimizationDownloadModeSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.DeliveryOptimizationDownloadModeSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreLogOptions(pub u32);
impl StoreLogOptions {
    pub const None: Self = Self(0u32);
    pub const TryElevate: Self = Self(1u32);
}
impl ::core::marker::Copy for StoreLogOptions {}
impl ::core::clone::Clone for StoreLogOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreLogOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreLogOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreLogOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreLogOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for StoreLogOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StoreLogOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StoreLogOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StoreLogOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StoreLogOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for StoreLogOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StoreLogOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StorePreviewProductPurchaseStatus(pub i32);
impl StorePreviewProductPurchaseStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const AlreadyPurchased: Self = Self(1i32);
    pub const NotFulfilled: Self = Self(2i32);
    pub const NotPurchased: Self = Self(3i32);
}
impl ::core::marker::Copy for StorePreviewProductPurchaseStatus {}
impl ::core::clone::Clone for StorePreviewProductPurchaseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorePreviewProductPurchaseStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorePreviewProductPurchaseStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorePreviewProductPurchaseStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorePreviewProductPurchaseStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorePreviewProductPurchaseStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StorePreviewProductPurchaseStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StoreSystemFeature(pub i32);
impl StoreSystemFeature {
    pub const ArchitectureX86: Self = Self(0i32);
    pub const ArchitectureX64: Self = Self(1i32);
    pub const ArchitectureArm: Self = Self(2i32);
    pub const DirectX9: Self = Self(3i32);
    pub const DirectX10: Self = Self(4i32);
    pub const DirectX11: Self = Self(5i32);
    pub const D3D12HardwareFL11: Self = Self(6i32);
    pub const D3D12HardwareFL12: Self = Self(7i32);
    pub const Memory300MB: Self = Self(8i32);
    pub const Memory750MB: Self = Self(9i32);
    pub const Memory1GB: Self = Self(10i32);
    pub const Memory2GB: Self = Self(11i32);
    pub const CameraFront: Self = Self(12i32);
    pub const CameraRear: Self = Self(13i32);
    pub const Gyroscope: Self = Self(14i32);
    pub const Hover: Self = Self(15i32);
    pub const Magnetometer: Self = Self(16i32);
    pub const Nfc: Self = Self(17i32);
    pub const Resolution720P: Self = Self(18i32);
    pub const ResolutionWvga: Self = Self(19i32);
    pub const ResolutionWvgaOr720P: Self = Self(20i32);
    pub const ResolutionWxga: Self = Self(21i32);
    pub const ResolutionWvgaOrWxga: Self = Self(22i32);
    pub const ResolutionWxgaOr720P: Self = Self(23i32);
    pub const Memory4GB: Self = Self(24i32);
    pub const Memory6GB: Self = Self(25i32);
    pub const Memory8GB: Self = Self(26i32);
    pub const Memory12GB: Self = Self(27i32);
    pub const Memory16GB: Self = Self(28i32);
    pub const Memory20GB: Self = Self(29i32);
    pub const VideoMemory2GB: Self = Self(30i32);
    pub const VideoMemory4GB: Self = Self(31i32);
    pub const VideoMemory6GB: Self = Self(32i32);
    pub const VideoMemory1GB: Self = Self(33i32);
    pub const ArchitectureArm64: Self = Self(34i32);
}
impl ::core::marker::Copy for StoreSystemFeature {}
impl ::core::clone::Clone for StoreSystemFeature {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StoreSystemFeature {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StoreSystemFeature {
    type Abi = Self;
}
impl ::core::fmt::Debug for StoreSystemFeature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StoreSystemFeature").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StoreSystemFeature {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.StoreSystemFeature;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
