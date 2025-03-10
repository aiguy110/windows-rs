#[cfg(feature = "Media_Protection_PlayReady")]
pub mod PlayReady;
#[doc(hidden)]
#[repr(transparent)]
pub struct IComponentLoadFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IComponentLoadFailedEventArgs {
    type Vtable = IComponentLoadFailedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IComponentLoadFailedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95972e93_7746_417e_8495_f031bbc5862c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentLoadFailedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Information: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Completion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IComponentRenewalStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IComponentRenewalStatics {
    type Vtable = IComponentRenewalStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IComponentRenewalStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ffbcd67_b795_48c5_8b7b_a7c4efe202e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentRenewalStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RenewSystemComponentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, information: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenewSystemComponentsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdcpSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHdcpSession {
    type Vtable = IHdcpSession_Vtbl;
}
unsafe impl ::windows::core::Interface for IHdcpSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x718845e9_64d7_426d_809b_1be461941a2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdcpSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEffectiveProtectionAtLeast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protection: HdcpProtection, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetEffectiveProtection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetEffectiveProtection: usize,
    #[cfg(feature = "Foundation")]
    pub SetDesiredMinProtectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protection: HdcpProtection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredMinProtectionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ProtectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectionChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProtectionManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaProtectionManager {
    type Vtable = IMediaProtectionManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaProtectionManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45694947_c741_434b_a79e_474c12d93d2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ServiceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RebootNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RebootNeeded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRebootNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRebootNeeded: usize,
    #[cfg(feature = "Foundation")]
    pub ComponentLoadFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ComponentLoadFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveComponentLoadFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveComponentLoadFailed: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProtectionPMPServer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaProtectionPMPServer {
    type Vtable = IMediaProtectionPMPServer_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaProtectionPMPServer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c111226_7b26_4d31_95bb_9c1b08ef7fc0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionPMPServer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProtectionPMPServerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaProtectionPMPServerFactory {
    type Vtable = IMediaProtectionPMPServerFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaProtectionPMPServerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x602c8e5e_f7d2_487e_af91_dbc4252b2182);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionPMPServerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreatePMPServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreatePMPServer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProtectionServiceCompletion(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaProtectionServiceCompletion {
    type Vtable = IMediaProtectionServiceCompletion_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaProtectionServiceCompletion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b5cca18_cfd5_44ee_a2ed_df76010c14b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionServiceCompletion_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, success: bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct IMediaProtectionServiceRequest(::windows::core::IUnknown);
impl IMediaProtectionServiceRequest {
    pub fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionSystem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IMediaProtectionServiceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IMediaProtectionServiceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaProtectionServiceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaProtectionServiceRequest {}
impl ::core::fmt::Debug for IMediaProtectionServiceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaProtectionServiceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMediaProtectionServiceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b1de0ea6-2094-478d-87a4-8b95200f85c6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IMediaProtectionServiceRequest {
    type Vtable = IMediaProtectionServiceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaProtectionServiceRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1de0ea6_2094_478d_87a4_8b95200f85c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionServiceRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProtectionSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProtectionCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProtectionCapabilities {
    type Vtable = IProtectionCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for IProtectionCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7ac5d7e_7480_4d29_a464_7bcd913dd8e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsTypeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut ::core::ffi::c_void, keysystem: *mut ::core::ffi::c_void, result__: *mut ProtectionCapabilityResult) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevocationAndRenewalInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRevocationAndRenewalInformation {
    type Vtable = IRevocationAndRenewalInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for IRevocationAndRenewalInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3a1937b_2501_439e_a6e7_6fc95e175fcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevocationAndRenewalInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRevocationAndRenewalItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRevocationAndRenewalItem {
    type Vtable = IRevocationAndRenewalItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IRevocationAndRenewalItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3099c20c_3cf0_49ea_902d_caf32d2dde2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevocationAndRenewalItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reasons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RevocationAndRenewalReasons) -> ::windows::core::HRESULT,
    pub HeaderHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PublicKeyHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RenewalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServiceRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IServiceRequestedEventArgs {
    type Vtable = IServiceRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IServiceRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34283baf_abb4_4fc1_bd89_93f106573a49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Completion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IServiceRequestedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IServiceRequestedEventArgs2 {
    type Vtable = IServiceRequestedEventArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IServiceRequestedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x553c69d6_fafe_4128_8dfa_130e398a13a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceRequestedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Playback")]
    pub MediaPlaybackItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    MediaPlaybackItem: usize,
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct ComponentLoadFailedEventArgs(::windows::core::IUnknown);
impl ComponentLoadFailedEventArgs {
    pub fn Information(&self) -> ::windows::core::Result<RevocationAndRenewalInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Information)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Completion(&self) -> ::windows::core::Result<MediaProtectionServiceCompletion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Completion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ComponentLoadFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ComponentLoadFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ComponentLoadFailedEventArgs {}
impl ::core::fmt::Debug for ComponentLoadFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ComponentLoadFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ComponentLoadFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ComponentLoadFailedEventArgs;{95972e93-7746-417e-8495-f031bbc5862c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ComponentLoadFailedEventArgs {
    type Vtable = IComponentLoadFailedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ComponentLoadFailedEventArgs {
    const IID: ::windows::core::GUID = <IComponentLoadFailedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ComponentLoadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.ComponentLoadFailedEventArgs";
}
::windows::core::interface_hierarchy!(ComponentLoadFailedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ComponentLoadFailedEventArgs {}
unsafe impl ::core::marker::Sync for ComponentLoadFailedEventArgs {}
#[doc = "*Required features: `\"Media_Protection\"`*"]
pub struct ComponentRenewal;
impl ComponentRenewal {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenewSystemComponentsAsync(information: &RevocationAndRenewalInformation) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<RenewalStatus, u32>> {
        Self::IComponentRenewalStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenewSystemComponentsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(information), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IComponentRenewalStatics<R, F: FnOnce(&IComponentRenewalStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ComponentRenewal, IComponentRenewalStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ComponentRenewal {
    const NAME: &'static str = "Windows.Media.Protection.ComponentRenewal";
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct HdcpSession(::windows::core::IUnknown);
impl HdcpSession {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HdcpSession, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn IsEffectiveProtectionAtLeast(&self, protection: HdcpProtection) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEffectiveProtectionAtLeast)(::windows::core::Vtable::as_raw(this), protection, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetEffectiveProtection(&self) -> ::windows::core::Result<super::super::Foundation::IReference<HdcpProtection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetEffectiveProtection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredMinProtectionAsync(&self, protection: HdcpProtection) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HdcpSetProtectionResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetDesiredMinProtectionAsync)(::windows::core::Vtable::as_raw(this), protection, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtectionChanged(&self, handler: &super::super::Foundation::TypedEventHandler<HdcpSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveProtectionChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for HdcpSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HdcpSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HdcpSession {}
impl ::core::fmt::Debug for HdcpSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcpSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdcpSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.HdcpSession;{718845e9-64d7-426d-809b-1be461941a2a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HdcpSession {
    type Vtable = IHdcpSession_Vtbl;
}
unsafe impl ::windows::core::Interface for HdcpSession {
    const IID: ::windows::core::GUID = <IHdcpSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HdcpSession {
    const NAME: &'static str = "Windows.Media.Protection.HdcpSession";
}
::windows::core::interface_hierarchy!(HdcpSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HdcpSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HdcpSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HdcpSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HdcpSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HdcpSession> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HdcpSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HdcpSession {}
unsafe impl ::core::marker::Sync for HdcpSession {}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct MediaProtectionManager(::windows::core::IUnknown);
impl MediaProtectionManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaProtectionManager, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceRequested(&self, handler: &ServiceRequestedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceRequested(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveServiceRequested)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RebootNeeded(&self, handler: &RebootNeededEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RebootNeeded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRebootNeeded(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRebootNeeded)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ComponentLoadFailed(&self, handler: &ComponentLoadFailedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ComponentLoadFailed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveComponentLoadFailed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveComponentLoadFailed)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaProtectionManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaProtectionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaProtectionManager {}
impl ::core::fmt::Debug for MediaProtectionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaProtectionManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaProtectionManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionManager;{45694947-c741-434b-a79e-474c12d93d2f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaProtectionManager {
    type Vtable = IMediaProtectionManager_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaProtectionManager {
    const IID: ::windows::core::GUID = <IMediaProtectionManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaProtectionManager {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionManager";
}
::windows::core::interface_hierarchy!(MediaProtectionManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaProtectionManager {}
unsafe impl ::core::marker::Sync for MediaProtectionManager {}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct MediaProtectionPMPServer(::windows::core::IUnknown);
impl MediaProtectionPMPServer {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Properties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePMPServer<P0, E0>(pproperties: P0) -> ::windows::core::Result<MediaProtectionPMPServer>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IPropertySet>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMediaProtectionPMPServerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreatePMPServer)(::windows::core::Vtable::as_raw(this), pproperties.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaProtectionPMPServerFactory<R, F: FnOnce(&IMediaProtectionPMPServerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaProtectionPMPServer, IMediaProtectionPMPServerFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MediaProtectionPMPServer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaProtectionPMPServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaProtectionPMPServer {}
impl ::core::fmt::Debug for MediaProtectionPMPServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaProtectionPMPServer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaProtectionPMPServer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionPMPServer;{0c111226-7b26-4d31-95bb-9c1b08ef7fc0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaProtectionPMPServer {
    type Vtable = IMediaProtectionPMPServer_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaProtectionPMPServer {
    const IID: ::windows::core::GUID = <IMediaProtectionPMPServer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaProtectionPMPServer {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionPMPServer";
}
::windows::core::interface_hierarchy!(MediaProtectionPMPServer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaProtectionPMPServer {}
unsafe impl ::core::marker::Sync for MediaProtectionPMPServer {}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct MediaProtectionServiceCompletion(::windows::core::IUnknown);
impl MediaProtectionServiceCompletion {
    pub fn Complete(&self, success: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this), success).ok() }
    }
}
impl ::core::clone::Clone for MediaProtectionServiceCompletion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaProtectionServiceCompletion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaProtectionServiceCompletion {}
impl ::core::fmt::Debug for MediaProtectionServiceCompletion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaProtectionServiceCompletion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaProtectionServiceCompletion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionServiceCompletion;{8b5cca18-cfd5-44ee-a2ed-df76010c14b5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaProtectionServiceCompletion {
    type Vtable = IMediaProtectionServiceCompletion_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaProtectionServiceCompletion {
    const IID: ::windows::core::GUID = <IMediaProtectionServiceCompletion as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaProtectionServiceCompletion {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionServiceCompletion";
}
::windows::core::interface_hierarchy!(MediaProtectionServiceCompletion, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaProtectionServiceCompletion {}
unsafe impl ::core::marker::Sync for MediaProtectionServiceCompletion {}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct ProtectionCapabilities(::windows::core::IUnknown);
impl ProtectionCapabilities {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ProtectionCapabilities, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsTypeSupported(&self, r#type: &::windows::core::HSTRING, keysystem: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionCapabilityResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsTypeSupported)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(r#type), ::core::mem::transmute_copy(keysystem), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ProtectionCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProtectionCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtectionCapabilities {}
impl ::core::fmt::Debug for ProtectionCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectionCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ProtectionCapabilities;{c7ac5d7e-7480-4d29-a464-7bcd913dd8e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ProtectionCapabilities {
    type Vtable = IProtectionCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for ProtectionCapabilities {
    const IID: ::windows::core::GUID = <IProtectionCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProtectionCapabilities {
    const NAME: &'static str = "Windows.Media.Protection.ProtectionCapabilities";
}
::windows::core::interface_hierarchy!(ProtectionCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProtectionCapabilities {}
unsafe impl ::core::marker::Sync for ProtectionCapabilities {}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct RevocationAndRenewalInformation(::windows::core::IUnknown);
impl RevocationAndRenewalInformation {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<RevocationAndRenewalItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Items)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RevocationAndRenewalInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RevocationAndRenewalInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RevocationAndRenewalInformation {}
impl ::core::fmt::Debug for RevocationAndRenewalInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevocationAndRenewalInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RevocationAndRenewalInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.RevocationAndRenewalInformation;{f3a1937b-2501-439e-a6e7-6fc95e175fcf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RevocationAndRenewalInformation {
    type Vtable = IRevocationAndRenewalInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for RevocationAndRenewalInformation {
    const IID: ::windows::core::GUID = <IRevocationAndRenewalInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RevocationAndRenewalInformation {
    const NAME: &'static str = "Windows.Media.Protection.RevocationAndRenewalInformation";
}
::windows::core::interface_hierarchy!(RevocationAndRenewalInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RevocationAndRenewalInformation {}
unsafe impl ::core::marker::Sync for RevocationAndRenewalInformation {}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct RevocationAndRenewalItem(::windows::core::IUnknown);
impl RevocationAndRenewalItem {
    pub fn Reasons(&self) -> ::windows::core::Result<RevocationAndRenewalReasons> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reasons)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HeaderHash(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeaderHash)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PublicKeyHash(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublicKeyHash)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RenewalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenewalId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RevocationAndRenewalItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RevocationAndRenewalItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RevocationAndRenewalItem {}
impl ::core::fmt::Debug for RevocationAndRenewalItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevocationAndRenewalItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RevocationAndRenewalItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.RevocationAndRenewalItem;{3099c20c-3cf0-49ea-902d-caf32d2dde2c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RevocationAndRenewalItem {
    type Vtable = IRevocationAndRenewalItem_Vtbl;
}
unsafe impl ::windows::core::Interface for RevocationAndRenewalItem {
    const IID: ::windows::core::GUID = <IRevocationAndRenewalItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RevocationAndRenewalItem {
    const NAME: &'static str = "Windows.Media.Protection.RevocationAndRenewalItem";
}
::windows::core::interface_hierarchy!(RevocationAndRenewalItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RevocationAndRenewalItem {}
unsafe impl ::core::marker::Sync for RevocationAndRenewalItem {}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct ServiceRequestedEventArgs(::windows::core::IUnknown);
impl ServiceRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<IMediaProtectionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Completion(&self) -> ::windows::core::Result<MediaProtectionServiceCompletion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Completion)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Playback\"`*"]
    #[cfg(feature = "Media_Playback")]
    pub fn MediaPlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows::core::Interface::cast::<IServiceRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaPlaybackItem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ServiceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ServiceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServiceRequestedEventArgs {}
impl ::core::fmt::Debug for ServiceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServiceRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ServiceRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ServiceRequestedEventArgs;{34283baf-abb4-4fc1-bd89-93f106573a49})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ServiceRequestedEventArgs {
    type Vtable = IServiceRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ServiceRequestedEventArgs {
    const IID: ::windows::core::GUID = <IServiceRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ServiceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.ServiceRequestedEventArgs";
}
::windows::core::interface_hierarchy!(ServiceRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ServiceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ServiceRequestedEventArgs {}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GraphicsTrustStatus(pub i32);
impl GraphicsTrustStatus {
    pub const TrustNotRequired: Self = Self(0i32);
    pub const TrustEstablished: Self = Self(1i32);
    pub const EnvironmentNotSupported: Self = Self(2i32);
    pub const DriverNotSupported: Self = Self(3i32);
    pub const DriverSigningFailure: Self = Self(4i32);
    pub const UnknownFailure: Self = Self(5i32);
}
impl ::core::marker::Copy for GraphicsTrustStatus {}
impl ::core::clone::Clone for GraphicsTrustStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GraphicsTrustStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GraphicsTrustStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GraphicsTrustStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GraphicsTrustStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GraphicsTrustStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.GraphicsTrustStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HdcpProtection(pub i32);
impl HdcpProtection {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const OnWithTypeEnforcement: Self = Self(2i32);
}
impl ::core::marker::Copy for HdcpProtection {}
impl ::core::clone::Clone for HdcpProtection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdcpProtection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HdcpProtection {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdcpProtection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcpProtection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdcpProtection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.HdcpProtection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HdcpSetProtectionResult(pub i32);
impl HdcpSetProtectionResult {
    pub const Success: Self = Self(0i32);
    pub const TimedOut: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const UnknownFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for HdcpSetProtectionResult {}
impl ::core::clone::Clone for HdcpSetProtectionResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdcpSetProtectionResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HdcpSetProtectionResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdcpSetProtectionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdcpSetProtectionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdcpSetProtectionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.HdcpSetProtectionResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProtectionCapabilityResult(pub i32);
impl ProtectionCapabilityResult {
    pub const NotSupported: Self = Self(0i32);
    pub const Maybe: Self = Self(1i32);
    pub const Probably: Self = Self(2i32);
}
impl ::core::marker::Copy for ProtectionCapabilityResult {}
impl ::core::clone::Clone for ProtectionCapabilityResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionCapabilityResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ProtectionCapabilityResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ProtectionCapabilityResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionCapabilityResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProtectionCapabilityResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.ProtectionCapabilityResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RenewalStatus(pub i32);
impl RenewalStatus {
    pub const NotStarted: Self = Self(0i32);
    pub const UpdatesInProgress: Self = Self(1i32);
    pub const UserCancelled: Self = Self(2i32);
    pub const AppComponentsMayNeedUpdating: Self = Self(3i32);
    pub const NoComponentsFound: Self = Self(4i32);
}
impl ::core::marker::Copy for RenewalStatus {}
impl ::core::clone::Clone for RenewalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RenewalStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RenewalStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for RenewalStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RenewalStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RenewalStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.RenewalStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RevocationAndRenewalReasons(pub u32);
impl RevocationAndRenewalReasons {
    pub const UserModeComponentLoad: Self = Self(1u32);
    pub const KernelModeComponentLoad: Self = Self(2u32);
    pub const AppComponent: Self = Self(4u32);
    pub const GlobalRevocationListLoadFailed: Self = Self(16u32);
    pub const InvalidGlobalRevocationListSignature: Self = Self(32u32);
    pub const GlobalRevocationListAbsent: Self = Self(4096u32);
    pub const ComponentRevoked: Self = Self(8192u32);
    pub const InvalidComponentCertificateExtendedKeyUse: Self = Self(16384u32);
    pub const ComponentCertificateRevoked: Self = Self(32768u32);
    pub const InvalidComponentCertificateRoot: Self = Self(65536u32);
    pub const ComponentHighSecurityCertificateRevoked: Self = Self(131072u32);
    pub const ComponentLowSecurityCertificateRevoked: Self = Self(262144u32);
    pub const BootDriverVerificationFailed: Self = Self(1048576u32);
    pub const ComponentSignedWithTestCertificate: Self = Self(16777216u32);
    pub const EncryptionFailure: Self = Self(268435456u32);
}
impl ::core::marker::Copy for RevocationAndRenewalReasons {}
impl ::core::clone::Clone for RevocationAndRenewalReasons {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RevocationAndRenewalReasons {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RevocationAndRenewalReasons {
    type Abi = Self;
}
impl ::core::fmt::Debug for RevocationAndRenewalReasons {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RevocationAndRenewalReasons").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RevocationAndRenewalReasons {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RevocationAndRenewalReasons {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RevocationAndRenewalReasons {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RevocationAndRenewalReasons {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RevocationAndRenewalReasons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for RevocationAndRenewalReasons {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.RevocationAndRenewalReasons;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct ComponentLoadFailedEventHandler(pub ::windows::core::IUnknown);
impl ComponentLoadFailedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ComponentLoadFailedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ComponentLoadFailedEventHandlerBox::<F> { vtable: &ComponentLoadFailedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, sender: &MediaProtectionManager, e: &ComponentLoadFailedEventArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(e)).ok() }
    }
}
#[repr(C)]
struct ComponentLoadFailedEventHandlerBox<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ComponentLoadFailedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ComponentLoadFailedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ComponentLoadFailedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> ComponentLoadFailedEventHandlerBox<F> {
    const VTABLE: ComponentLoadFailedEventHandler_Vtbl = ComponentLoadFailedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<ComponentLoadFailedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&e)).into()
    }
}
impl ::core::clone::Clone for ComponentLoadFailedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ComponentLoadFailedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ComponentLoadFailedEventHandler {}
impl ::core::fmt::Debug for ComponentLoadFailedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ComponentLoadFailedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ComponentLoadFailedEventHandler {
    type Vtable = ComponentLoadFailedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for ComponentLoadFailedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95da643c_6db9_424b_86ca_091af432081c);
}
unsafe impl ::windows::core::RuntimeType for ComponentLoadFailedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{95da643c-6db9-424b-86ca-091af432081c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ComponentLoadFailedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct RebootNeededEventHandler(pub ::windows::core::IUnknown);
impl RebootNeededEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&MediaProtectionManager>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RebootNeededEventHandlerBox::<F> { vtable: &RebootNeededEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, sender: &MediaProtectionManager) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender)).ok() }
    }
}
#[repr(C)]
struct RebootNeededEventHandlerBox<F: FnMut(::core::option::Option<&MediaProtectionManager>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const RebootNeededEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(::core::option::Option<&MediaProtectionManager>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> RebootNeededEventHandlerBox<F> {
    const VTABLE: RebootNeededEventHandler_Vtbl = RebootNeededEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<RebootNeededEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&sender)).into()
    }
}
impl ::core::clone::Clone for RebootNeededEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RebootNeededEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RebootNeededEventHandler {}
impl ::core::fmt::Debug for RebootNeededEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RebootNeededEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for RebootNeededEventHandler {
    type Vtable = RebootNeededEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for RebootNeededEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64e12a45_973b_4a3a_b260_91898a49a82c);
}
unsafe impl ::windows::core::RuntimeType for RebootNeededEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{64e12a45-973b-4a3a-b260-91898a49a82c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RebootNeededEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Protection\"`*"]
#[repr(transparent)]
pub struct ServiceRequestedEventHandler(pub ::windows::core::IUnknown);
impl ServiceRequestedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ServiceRequestedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ServiceRequestedEventHandlerBox::<F> { vtable: &ServiceRequestedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, sender: &MediaProtectionManager, e: &ServiceRequestedEventArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(e)).ok() }
    }
}
#[repr(C)]
struct ServiceRequestedEventHandlerBox<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ServiceRequestedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ServiceRequestedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(::core::option::Option<&MediaProtectionManager>, ::core::option::Option<&ServiceRequestedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> ServiceRequestedEventHandlerBox<F> {
    const VTABLE: ServiceRequestedEventHandler_Vtbl = ServiceRequestedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<ServiceRequestedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&e)).into()
    }
}
impl ::core::clone::Clone for ServiceRequestedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ServiceRequestedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ServiceRequestedEventHandler {}
impl ::core::fmt::Debug for ServiceRequestedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ServiceRequestedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for ServiceRequestedEventHandler {
    type Vtable = ServiceRequestedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for ServiceRequestedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2d690ba_cac9_48e1_95c0_d38495a84055);
}
unsafe impl ::windows::core::RuntimeType for ServiceRequestedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d2d690ba-cac9-48e1-95c0-d38495a84055}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ServiceRequestedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
