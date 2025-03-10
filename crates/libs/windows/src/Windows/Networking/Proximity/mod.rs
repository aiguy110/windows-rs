#[doc(hidden)]
#[repr(transparent)]
pub struct IConnectionRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IConnectionRequestedEventArgs {
    type Vtable = IConnectionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IConnectionRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb6891ae_4f1e_4c66_bd0d_46924a942e08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PeerInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerFinderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPeerFinderStatics {
    type Vtable = IPeerFinderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPeerFinderStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x914b3b61_f6e1_47c4_a14c_148a1903d0c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerFinderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowBluetooth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowBluetooth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowInfrastructure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowInfrastructure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AllowWiFiDirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowWiFiDirect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SupportedDiscoveryTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PeerDiscoveryTypes) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AlternateIdentities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AlternateIdentities: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartWithMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peermessage: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TriggeredConnectionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TriggeredConnectionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTriggeredConnectionStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTriggeredConnectionStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConnectionRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConnectionRequested: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllPeersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllPeersAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub ConnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peerinformation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    ConnectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerFinderStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPeerFinderStatics2 {
    type Vtable = IPeerFinderStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPeerFinderStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6e73c65_fdd0_4b0b_9312_866408935d82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerFinderStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Role: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PeerRole) -> ::windows::core::HRESULT,
    pub SetRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PeerRole) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DiscoveryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscoveryData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetDiscoveryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetDiscoveryData: usize,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPeerInformation {
    type Vtable = IPeerInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for IPeerInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20024f08_9fff_45f4_b6e9_408b2ebef373);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerInformation3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPeerInformation3 {
    type Vtable = IPeerInformation3_Vtbl;
}
unsafe impl ::windows::core::Interface for IPeerInformation3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb20f612a_dbd0_40f8_95bd_2d4209c7836f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformation3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub DiscoveryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscoveryData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerInformationWithHostAndService(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPeerInformationWithHostAndService {
    type Vtable = IPeerInformationWithHostAndService_Vtbl;
}
unsafe impl ::windows::core::Interface for IPeerInformationWithHostAndService {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc7ccad_1b70_4e8b_92db_bbe781419308);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerInformationWithHostAndService_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPeerWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPeerWatcher {
    type Vtable = IPeerWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for IPeerWatcher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cee21f8_2fa6_4679_9691_03c94a420f34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPeerWatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PeerWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximityDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProximityDevice {
    type Vtable = IProximityDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for IProximityDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa8a552_f6e1_4329_a0fc_ab6b0fd28262);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SubscribeForMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: *mut ::core::ffi::c_void, messagereceivedhandler: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub PublishMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub PublishMessageWithCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, messagetransmittedhandler: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PublishBinaryMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublishBinaryMessage: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PublishBinaryMessageWithCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messagetype: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, messagetransmittedhandler: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PublishBinaryMessageWithCallback: usize,
    #[cfg(feature = "Foundation")]
    pub PublishUriMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PublishUriMessage: usize,
    #[cfg(feature = "Foundation")]
    pub PublishUriMessageWithCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, messagetransmittedhandler: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PublishUriMessageWithCallback: usize,
    pub StopSubscribingForMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscriptionid: i64) -> ::windows::core::HRESULT,
    pub StopPublishingMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeviceArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arrivedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceArrived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceArrived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceArrived: usize,
    #[cfg(feature = "Foundation")]
    pub DeviceDeparted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, departedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceDeparted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceDeparted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceDeparted: usize,
    pub MaxMessageBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub BitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximityDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProximityDeviceStatics {
    type Vtable = IProximityDeviceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IProximityDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x914ba01d_f6e1_47c4_a14c_148a1903d0c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProximityMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IProximityMessage {
    type Vtable = IProximityMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IProximityMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefab0782_f6e1_4675_a045_d8e320c24808);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProximityMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MessageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SubscriptionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    pub DataAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITriggeredConnectionStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITriggeredConnectionStateChangedEventArgs {
    type Vtable = ITriggeredConnectionStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ITriggeredConnectionStateChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6a780ad_f6e1_4d54_96e2_33f620bca88a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITriggeredConnectionStateChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TriggeredConnectState) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub Socket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    Socket: usize,
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct ConnectionRequestedEventArgs(::windows::core::IUnknown);
impl ConnectionRequestedEventArgs {
    pub fn PeerInformation(&self) -> ::windows::core::Result<PeerInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PeerInformation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ConnectionRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConnectionRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConnectionRequestedEventArgs {}
impl ::core::fmt::Debug for ConnectionRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectionRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConnectionRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ConnectionRequestedEventArgs;{eb6891ae-4f1e-4c66-bd0d-46924a942e08})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ConnectionRequestedEventArgs {
    type Vtable = IConnectionRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ConnectionRequestedEventArgs {
    const IID: ::windows::core::GUID = <IConnectionRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.ConnectionRequestedEventArgs";
}
::windows::core::interface_hierarchy!(ConnectionRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ConnectionRequestedEventArgs {}
unsafe impl ::core::marker::Sync for ConnectionRequestedEventArgs {}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
pub struct PeerFinder;
impl PeerFinder {
    pub fn AllowBluetooth() -> ::windows::core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowBluetooth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetAllowBluetooth(value: bool) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SetAllowBluetooth)(::windows::core::Vtable::as_raw(this), value).ok() })
    }
    pub fn AllowInfrastructure() -> ::windows::core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowInfrastructure)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetAllowInfrastructure(value: bool) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SetAllowInfrastructure)(::windows::core::Vtable::as_raw(this), value).ok() })
    }
    pub fn AllowWiFiDirect() -> ::windows::core::Result<bool> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowWiFiDirect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetAllowWiFiDirect(value: bool) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SetAllowWiFiDirect)(::windows::core::Vtable::as_raw(this), value).ok() })
    }
    pub fn DisplayName() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetDisplayName(value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SetDisplayName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() })
    }
    pub fn SupportedDiscoveryTypes() -> ::windows::core::Result<PeerDiscoveryTypes> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedDiscoveryTypes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AlternateIdentities() -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlternateIdentities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Start() -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    pub fn StartWithMessage(peermessage: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).StartWithMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(peermessage)).ok() })
    }
    pub fn Stop() -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TriggeredConnectionStateChanged(handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TriggeredConnectionStateChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TriggeredConnectionStateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTriggeredConnectionStateChanged(cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveTriggeredConnectionStateChanged)(::windows::core::Vtable::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionRequested(handler: &super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ConnectionRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConnectionRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConnectionRequested(cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveConnectionRequested)(::windows::core::Vtable::as_raw(this), cookie).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllPeersAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PeerInformation>>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindAllPeersAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub fn ConnectAsync(peerinformation: &PeerInformation) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Sockets::StreamSocket>> {
        Self::IPeerFinderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConnectAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(peerinformation), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Role() -> ::windows::core::Result<PeerRole> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Role)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetRole(value: PeerRole) -> ::windows::core::Result<()> {
        Self::IPeerFinderStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).SetRole)(::windows::core::Vtable::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DiscoveryData() -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DiscoveryData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetDiscoveryData<P0, E0>(value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPeerFinderStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).SetDiscoveryData)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() })
    }
    pub fn CreateWatcher() -> ::windows::core::Result<PeerWatcher> {
        Self::IPeerFinderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWatcher)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPeerFinderStatics<R, F: FnOnce(&IPeerFinderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PeerFinder, IPeerFinderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IPeerFinderStatics2<R, F: FnOnce(&IPeerFinderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PeerFinder, IPeerFinderStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PeerFinder {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerFinder";
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct PeerInformation(::windows::core::IUnknown);
impl PeerInformation {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPeerInformation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DiscoveryData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows::core::Interface::cast::<IPeerInformation3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DiscoveryData)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HostName(&self) -> ::windows::core::Result<super::HostName> {
        let this = &::windows::core::Interface::cast::<IPeerInformationWithHostAndService>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HostName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPeerInformationWithHostAndService>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PeerInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PeerInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PeerInformation {}
impl ::core::fmt::Debug for PeerInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeerInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PeerInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.PeerInformation;{20024f08-9fff-45f4-b6e9-408b2ebef373})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PeerInformation {
    type Vtable = IPeerInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for PeerInformation {
    const IID: ::windows::core::GUID = <IPeerInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PeerInformation {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerInformation";
}
::windows::core::interface_hierarchy!(PeerInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PeerInformation {}
unsafe impl ::core::marker::Sync for PeerInformation {}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct PeerWatcher(::windows::core::IUnknown);
impl PeerWatcher {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added(&self, handler: &super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Added)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAdded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed(&self, handler: &super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Removed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemoved)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated(&self, handler: &super::super::Foundation::TypedEventHandler<PeerWatcher, PeerInformation>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Updated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUpdated)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted(&self, handler: &super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnumerationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveEnumerationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped(&self, handler: &super::super::Foundation::TypedEventHandler<PeerWatcher, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stopped)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStopped)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<PeerWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PeerWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PeerWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PeerWatcher {}
impl ::core::fmt::Debug for PeerWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeerWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PeerWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.PeerWatcher;{3cee21f8-2fa6-4679-9691-03c94a420f34})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PeerWatcher {
    type Vtable = IPeerWatcher_Vtbl;
}
unsafe impl ::windows::core::Interface for PeerWatcher {
    const IID: ::windows::core::GUID = <IPeerWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PeerWatcher {
    const NAME: &'static str = "Windows.Networking.Proximity.PeerWatcher";
}
::windows::core::interface_hierarchy!(PeerWatcher, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PeerWatcher {}
unsafe impl ::core::marker::Sync for PeerWatcher {}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct ProximityDevice(::windows::core::IUnknown);
impl ProximityDevice {
    pub fn SubscribeForMessage(&self, messagetype: &::windows::core::HSTRING, messagereceivedhandler: &MessageReceivedHandler) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubscribeForMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(messagetype), ::core::mem::transmute_copy(messagereceivedhandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PublishMessage(&self, messagetype: &::windows::core::HSTRING, message: &::windows::core::HSTRING) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublishMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(messagetype), ::core::mem::transmute_copy(message), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PublishMessageWithCallback(&self, messagetype: &::windows::core::HSTRING, message: &::windows::core::HSTRING, messagetransmittedhandler: &MessageTransmittedHandler) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublishMessageWithCallback)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(messagetype), ::core::mem::transmute_copy(message), ::core::mem::transmute_copy(messagetransmittedhandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PublishBinaryMessage<P0, E0>(&self, messagetype: &::windows::core::HSTRING, message: P0) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublishBinaryMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(messagetype), message.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PublishBinaryMessageWithCallback<P0, E0>(&self, messagetype: &::windows::core::HSTRING, message: P0, messagetransmittedhandler: &MessageTransmittedHandler) -> ::windows::core::Result<i64>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublishBinaryMessageWithCallback)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(messagetype), message.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(messagetransmittedhandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PublishUriMessage(&self, message: &super::super::Foundation::Uri) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublishUriMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(message), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PublishUriMessageWithCallback(&self, message: &super::super::Foundation::Uri, messagetransmittedhandler: &MessageTransmittedHandler) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PublishUriMessageWithCallback)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(message), ::core::mem::transmute_copy(messagetransmittedhandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StopSubscribingForMessage(&self, subscriptionid: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StopSubscribingForMessage)(::windows::core::Vtable::as_raw(this), subscriptionid).ok() }
    }
    pub fn StopPublishingMessage(&self, messageid: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StopPublishingMessage)(::windows::core::Vtable::as_raw(this), messageid).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeviceArrived(&self, arrivedhandler: &DeviceArrivedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceArrived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(arrivedhandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceArrived(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDeviceArrived)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeviceDeparted(&self, departedhandler: &DeviceDepartedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceDeparted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(departedhandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceDeparted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDeviceDeparted)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    pub fn MaxMessageBytes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxMessageBytes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitsPerSecond(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitsPerSecond)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeviceId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDefault() -> ::windows::core::Result<ProximityDevice> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn FromId(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<ProximityDevice> {
        Self::IProximityDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProximityDeviceStatics<R, F: FnOnce(&IProximityDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ProximityDevice, IProximityDeviceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ProximityDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProximityDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximityDevice {}
impl ::core::fmt::Debug for ProximityDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximityDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProximityDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ProximityDevice;{efa8a552-f6e1-4329-a0fc-ab6b0fd28262})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ProximityDevice {
    type Vtable = IProximityDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for ProximityDevice {
    const IID: ::windows::core::GUID = <IProximityDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProximityDevice {
    const NAME: &'static str = "Windows.Networking.Proximity.ProximityDevice";
}
::windows::core::interface_hierarchy!(ProximityDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProximityDevice {}
unsafe impl ::core::marker::Sync for ProximityDevice {}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct ProximityMessage(::windows::core::IUnknown);
impl ProximityMessage {
    pub fn MessageType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SubscriptionId(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubscriptionId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DataAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataAsString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ProximityMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ProximityMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProximityMessage {}
impl ::core::fmt::Debug for ProximityMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProximityMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ProximityMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.ProximityMessage;{efab0782-f6e1-4675-a045-d8e320c24808})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ProximityMessage {
    type Vtable = IProximityMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for ProximityMessage {
    const IID: ::windows::core::GUID = <IProximityMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ProximityMessage {
    const NAME: &'static str = "Windows.Networking.Proximity.ProximityMessage";
}
::windows::core::interface_hierarchy!(ProximityMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ProximityMessage {}
unsafe impl ::core::marker::Sync for ProximityMessage {}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct TriggeredConnectionStateChangedEventArgs(::windows::core::IUnknown);
impl TriggeredConnectionStateChangedEventArgs {
    pub fn State(&self) -> ::windows::core::Result<TriggeredConnectState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).State)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn Socket(&self) -> ::windows::core::Result<super::Sockets::StreamSocket> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Socket)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for TriggeredConnectionStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TriggeredConnectionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TriggeredConnectionStateChangedEventArgs {}
impl ::core::fmt::Debug for TriggeredConnectionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TriggeredConnectionStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TriggeredConnectionStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs;{c6a780ad-f6e1-4d54-96e2-33f620bca88a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TriggeredConnectionStateChangedEventArgs {
    type Vtable = ITriggeredConnectionStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for TriggeredConnectionStateChangedEventArgs {
    const IID: ::windows::core::GUID = <ITriggeredConnectionStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TriggeredConnectionStateChangedEventArgs {
    const NAME: &'static str = "Windows.Networking.Proximity.TriggeredConnectionStateChangedEventArgs";
}
::windows::core::interface_hierarchy!(TriggeredConnectionStateChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TriggeredConnectionStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for TriggeredConnectionStateChangedEventArgs {}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PeerDiscoveryTypes(pub u32);
impl PeerDiscoveryTypes {
    pub const None: Self = Self(0u32);
    pub const Browse: Self = Self(1u32);
    pub const Triggered: Self = Self(2u32);
}
impl ::core::marker::Copy for PeerDiscoveryTypes {}
impl ::core::clone::Clone for PeerDiscoveryTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PeerDiscoveryTypes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PeerDiscoveryTypes {
    type Abi = Self;
}
impl ::core::fmt::Debug for PeerDiscoveryTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeerDiscoveryTypes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PeerDiscoveryTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PeerDiscoveryTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PeerDiscoveryTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PeerDiscoveryTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PeerDiscoveryTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PeerDiscoveryTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerDiscoveryTypes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PeerRole(pub i32);
impl PeerRole {
    pub const Peer: Self = Self(0i32);
    pub const Host: Self = Self(1i32);
    pub const Client: Self = Self(2i32);
}
impl ::core::marker::Copy for PeerRole {}
impl ::core::clone::Clone for PeerRole {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PeerRole {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PeerRole {
    type Abi = Self;
}
impl ::core::fmt::Debug for PeerRole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeerRole").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PeerRole {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerRole;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PeerWatcherStatus(pub i32);
impl PeerWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for PeerWatcherStatus {}
impl ::core::clone::Clone for PeerWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PeerWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PeerWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PeerWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PeerWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PeerWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.PeerWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TriggeredConnectState(pub i32);
impl TriggeredConnectState {
    pub const PeerFound: Self = Self(0i32);
    pub const Listening: Self = Self(1i32);
    pub const Connecting: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
    pub const Canceled: Self = Self(4i32);
    pub const Failed: Self = Self(5i32);
}
impl ::core::marker::Copy for TriggeredConnectState {}
impl ::core::clone::Clone for TriggeredConnectState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TriggeredConnectState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TriggeredConnectState {
    type Abi = Self;
}
impl ::core::fmt::Debug for TriggeredConnectState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TriggeredConnectState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TriggeredConnectState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.Proximity.TriggeredConnectState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct DeviceArrivedEventHandler(pub ::windows::core::IUnknown);
impl DeviceArrivedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DeviceArrivedEventHandlerBox::<F> { vtable: &DeviceArrivedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, sender: &ProximityDevice) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender)).ok() }
    }
}
#[repr(C)]
struct DeviceArrivedEventHandlerBox<F: FnMut(::core::option::Option<&ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DeviceArrivedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(::core::option::Option<&ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DeviceArrivedEventHandlerBox<F> {
    const VTABLE: DeviceArrivedEventHandler_Vtbl = DeviceArrivedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<DeviceArrivedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
impl ::core::clone::Clone for DeviceArrivedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceArrivedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceArrivedEventHandler {}
impl ::core::fmt::Debug for DeviceArrivedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceArrivedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for DeviceArrivedEventHandler {
    type Vtable = DeviceArrivedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for DeviceArrivedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa9da69_f6e1_49c9_a49e_8e0fc58fb911);
}
unsafe impl ::windows::core::RuntimeType for DeviceArrivedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{efa9da69-f6e1-49c9-a49e-8e0fc58fb911}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DeviceArrivedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct DeviceDepartedEventHandler(pub ::windows::core::IUnknown);
impl DeviceDepartedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DeviceDepartedEventHandlerBox::<F> { vtable: &DeviceDepartedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, sender: &ProximityDevice) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender)).ok() }
    }
}
#[repr(C)]
struct DeviceDepartedEventHandlerBox<F: FnMut(::core::option::Option<&ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DeviceDepartedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(::core::option::Option<&ProximityDevice>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DeviceDepartedEventHandlerBox<F> {
    const VTABLE: DeviceDepartedEventHandler_Vtbl = DeviceDepartedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<DeviceDepartedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
impl ::core::clone::Clone for DeviceDepartedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceDepartedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceDepartedEventHandler {}
impl ::core::fmt::Debug for DeviceDepartedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceDepartedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for DeviceDepartedEventHandler {
    type Vtable = DeviceDepartedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for DeviceDepartedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa9da69_f6e2_49c9_a49e_8e0fc58fb911);
}
unsafe impl ::windows::core::RuntimeType for DeviceDepartedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{efa9da69-f6e2-49c9-a49e-8e0fc58fb911}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DeviceDepartedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct MessageReceivedHandler(pub ::windows::core::IUnknown);
impl MessageReceivedHandler {
    pub fn new<F: FnMut(::core::option::Option<&ProximityDevice>, ::core::option::Option<&ProximityMessage>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MessageReceivedHandlerBox::<F> { vtable: &MessageReceivedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, sender: &ProximityDevice, message: &ProximityMessage) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(message)).ok() }
    }
}
#[repr(C)]
struct MessageReceivedHandlerBox<F: FnMut(::core::option::Option<&ProximityDevice>, ::core::option::Option<&ProximityMessage>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MessageReceivedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(::core::option::Option<&ProximityDevice>, ::core::option::Option<&ProximityMessage>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MessageReceivedHandlerBox<F> {
    const VTABLE: MessageReceivedHandler_Vtbl = MessageReceivedHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<MessageReceivedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&message)).into()
    }
}
impl ::core::clone::Clone for MessageReceivedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MessageReceivedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageReceivedHandler {}
impl ::core::fmt::Debug for MessageReceivedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageReceivedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for MessageReceivedHandler {
    type Vtable = MessageReceivedHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for MessageReceivedHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefab0782_f6e2_4675_a045_d8e320c24808);
}
unsafe impl ::windows::core::RuntimeType for MessageReceivedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{efab0782-f6e2-4675-a045-d8e320c24808}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct MessageReceivedHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Networking_Proximity\"`*"]
#[repr(transparent)]
pub struct MessageTransmittedHandler(pub ::windows::core::IUnknown);
impl MessageTransmittedHandler {
    pub fn new<F: FnMut(::core::option::Option<&ProximityDevice>, i64) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MessageTransmittedHandlerBox::<F> { vtable: &MessageTransmittedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, sender: &ProximityDevice, messageid: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(sender), messageid).ok() }
    }
}
#[repr(C)]
struct MessageTransmittedHandlerBox<F: FnMut(::core::option::Option<&ProximityDevice>, i64) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MessageTransmittedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(::core::option::Option<&ProximityDevice>, i64) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MessageTransmittedHandlerBox<F> {
    const VTABLE: MessageTransmittedHandler_Vtbl = MessageTransmittedHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<MessageTransmittedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, messageid: i64) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&sender), messageid).into()
    }
}
impl ::core::clone::Clone for MessageTransmittedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MessageTransmittedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageTransmittedHandler {}
impl ::core::fmt::Debug for MessageTransmittedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageTransmittedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for MessageTransmittedHandler {
    type Vtable = MessageTransmittedHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for MessageTransmittedHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefaa0b4a_f6e2_4d7d_856c_78fc8efc021e);
}
unsafe impl ::windows::core::RuntimeType for MessageTransmittedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{efaa0b4a-f6e2-4d7d-856c-78fc8efc021e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct MessageTransmittedHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, messageid: i64) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
