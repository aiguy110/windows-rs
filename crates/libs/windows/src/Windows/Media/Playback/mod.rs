#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IBackgroundMediaPlayerStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IBackgroundMediaPlayerStatics {
    type Vtable = IBackgroundMediaPlayerStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IBackgroundMediaPlayerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x856ddbc1_55f7_471f_a0f2_68ac4c904592);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundMediaPlayerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Current: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MessageReceivedFromBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MessageReceivedFromBackground: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMessageReceivedFromBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMessageReceivedFromBackground: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MessageReceivedFromForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MessageReceivedFromForeground: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMessageReceivedFromForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMessageReceivedFromForeground: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SendMessageToBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SendMessageToBackground: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub SendMessageToForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    SendMessageToForeground: usize,
    #[cfg(feature = "deprecated")]
    pub IsMediaPlaying: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsMediaPlaying: usize,
    #[cfg(feature = "deprecated")]
    pub Shutdown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Shutdown: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICurrentMediaPlaybackItemChangedEventArgs {
    type Vtable = ICurrentMediaPlaybackItemChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICurrentMediaPlaybackItemChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1743a892_5c43_4a15_967a_572d2d0f26c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NewItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OldItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICurrentMediaPlaybackItemChangedEventArgs2 {
    type Vtable = ICurrentMediaPlaybackItemChangedEventArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICurrentMediaPlaybackItemChangedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d80a51e_996e_40a9_be48_e66ec90b2b7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentMediaPlaybackItemChangedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackItemChangedReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreak(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaBreak {
    type Vtable = IMediaBreak_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaBreak {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x714be270_0def_4ebc_a489_6b34930e1558);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreak_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PlaybackList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PresentationPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PresentationPosition: usize,
    pub InsertionMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaBreakInsertionMethod) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomProperties: usize,
    pub CanStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakEndedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaBreakEndedEventArgs {
    type Vtable = IMediaBreakEndedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaBreakEndedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32b93276_1c5d_4fee_8732_236dc3a88580);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakEndedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MediaBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaBreakFactory {
    type Vtable = IMediaBreakFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaBreakFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4516e002_18e0_4079_8b5f_d33495c15d2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, insertionmethod: MediaBreakInsertionMethod, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateWithPresentationPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, insertionmethod: MediaBreakInsertionMethod, presentationposition: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithPresentationPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaBreakManager {
    type Vtable = IMediaBreakManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaBreakManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa854ddb1_feb4_4d9b_9d97_0fdbe58e5e39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BreaksSeekedOver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BreaksSeekedOver: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBreaksSeekedOver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBreaksSeekedOver: usize,
    #[cfg(feature = "Foundation")]
    pub BreakStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BreakStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBreakStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBreakStarted: usize,
    #[cfg(feature = "Foundation")]
    pub BreakEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BreakEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBreakEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBreakEnded: usize,
    #[cfg(feature = "Foundation")]
    pub BreakSkipped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BreakSkipped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBreakSkipped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBreakSkipped: usize,
    pub CurrentBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PlaybackSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PlayBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SkipCurrentBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakSchedule(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaBreakSchedule {
    type Vtable = IMediaBreakSchedule_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaBreakSchedule {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa19a5813_98b6_41d8_83da_f971d22b7bba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakSchedule_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ScheduleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScheduleChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveScheduleChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveScheduleChanged: usize,
    pub InsertMidrollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediabreak: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveMidrollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediabreak: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MidrollBreaks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MidrollBreaks: usize,
    pub SetPrerollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PrerollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPostrollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PostrollBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PlaybackItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakSeekedOverEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaBreakSeekedOverEventArgs {
    type Vtable = IMediaBreakSeekedOverEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaBreakSeekedOverEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5aa6746_0606_4492_b9d3_c3c8fde0a4ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakSeekedOverEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SeekedOverBreaks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SeekedOverBreaks: usize,
    #[cfg(feature = "Foundation")]
    pub OldPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OldPosition: usize,
    #[cfg(feature = "Foundation")]
    pub NewPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakSkippedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaBreakSkippedEventArgs {
    type Vtable = IMediaBreakSkippedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaBreakSkippedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ee94c05_2f54_4a3e_a3ab_24c3b270b4a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakSkippedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MediaBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBreakStartedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaBreakStartedEventArgs {
    type Vtable = IMediaBreakStartedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaBreakStartedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa87efe71_dfd4_454a_956e_0a4a648395f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBreakStartedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MediaBreak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Playback\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IMediaEnginePlaybackSource(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl IMediaEnginePlaybackSource {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CurrentItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentItem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetPlaybackSource<P0, E0>(&self, source: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IMediaPlaybackSource>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPlaybackSource)(::windows::core::Vtable::as_raw(this), source.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
#[cfg(feature = "deprecated")]
::windows::core::interface_hierarchy!(IMediaEnginePlaybackSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IMediaEnginePlaybackSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IMediaEnginePlaybackSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IMediaEnginePlaybackSource {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IMediaEnginePlaybackSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaEnginePlaybackSource").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for IMediaEnginePlaybackSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5c1d0ba7-3856-48b9-8dc6-244bf107bf8c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Vtable for IMediaEnginePlaybackSource {
    type Vtable = IMediaEnginePlaybackSource_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IMediaEnginePlaybackSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c1d0ba7_3856_48b9_8dc6_244bf107bf8c);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEnginePlaybackSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub CurrentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentItem: usize,
    #[cfg(feature = "deprecated")]
    pub SetPlaybackSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPlaybackSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaItemDisplayProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaItemDisplayProperties {
    type Vtable = IMediaItemDisplayProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaItemDisplayProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e3c1b48_7097_4384_a217_c1291dfa8c16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaItemDisplayProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaPlaybackType) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaPlaybackType) -> ::windows::core::HRESULT,
    pub MusicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VideoProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub ClearAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManager {
    type Vtable = IMediaPlaybackCommandManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5acee5a6_5cb6_4a5a_8521_cc86b1c1ed37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub MediaPlayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PlayBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PauseBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NextBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PreviousBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FastForwardBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RewindBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShuffleBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AutoRepeatModeBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PositionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RateBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PlayReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlayReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlayReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlayReceived: usize,
    #[cfg(feature = "Foundation")]
    pub PauseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PauseReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePauseReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePauseReceived: usize,
    #[cfg(feature = "Foundation")]
    pub NextReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NextReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNextReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNextReceived: usize,
    #[cfg(feature = "Foundation")]
    pub PreviousReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreviousReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePreviousReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePreviousReceived: usize,
    #[cfg(feature = "Foundation")]
    pub FastForwardReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FastForwardReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFastForwardReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFastForwardReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RewindReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RewindReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRewindReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRewindReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ShuffleReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShuffleReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShuffleReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShuffleReceived: usize,
    #[cfg(feature = "Foundation")]
    pub AutoRepeatModeReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoRepeatModeReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAutoRepeatModeReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAutoRepeatModeReceived: usize,
    #[cfg(feature = "Foundation")]
    pub PositionReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RateReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RateReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRateReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRateReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d6f4f23_5230_4411_a0e9_bad94c2a045c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AutoRepeatMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaPlaybackAutoRepeatMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerCommandBehavior(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManagerCommandBehavior {
    type Vtable = IMediaPlaybackCommandManagerCommandBehavior_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerCommandBehavior {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x786c1e78_ce78_4a10_afd6_843fcbb90c2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerCommandBehavior_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CommandManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub EnablingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaCommandEnablingRule) -> ::windows::core::HRESULT,
    pub SetEnablingRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaCommandEnablingRule) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsEnabledChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerFastForwardReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30f064d9_b491_4d0a_bc21_3098bd1332e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerNextReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManagerNextReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerNextReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerNextReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1504433_a2b0_45d4_b9de_5f42ac14a839);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerNextReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPauseReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManagerPauseReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPauseReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerPauseReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ceccd1c_c25c_4221_b16c_c3c98ce012d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPauseReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPlayReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManagerPlayReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPlayReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerPlayReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9af0004e_578b_4c56_a006_16159d888a48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPlayReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPositionReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManagerPositionReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPositionReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerPositionReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5591a754_d627_4bdd_a90d_86a015b24902);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPositionReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerPreviousReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManagerPreviousReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x525e3081_4632_4f76_99b1_d771623f6287);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerRateReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManagerRateReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRateReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerRateReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18ea3939_4a16_4169_8b05_3eb9f5ff78eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerRateReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerRewindReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManagerRewindReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRewindReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerRewindReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f085947_a3c0_425d_aaef_97ba7898b141);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerRewindReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackCommandManagerShuffleReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackCommandManagerShuffleReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50a05cef_63ee_4a96_b7b5_fee08b9ff90c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsShuffleRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackItem {
    type Vtable = IMediaPlaybackItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x047097d2_e4af_48ab_b283_6929e674ece2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AudioTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AudioTracksChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAudioTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAudioTracksChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub VideoTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VideoTracksChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoTracksChanged: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TimedMetadataTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TimedMetadataTracksChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTimedMetadataTracksChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTimedMetadataTracksChanged: usize,
    #[cfg(feature = "Media_Core")]
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    Source: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub AudioTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    AudioTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub VideoTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    VideoTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub TimedMetadataTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    TimedMetadataTracks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItem2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackItem2 {
    type Vtable = IMediaPlaybackItem2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackItem2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd859d171_d7ef_4b81_ac1f_f40493cbb091);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItem2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BreakSchedule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub DurationLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DurationLimit: usize,
    pub CanSkip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanSkip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub GetDisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ApplyDisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItem3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackItem3 {
    type Vtable = IMediaPlaybackItem3_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackItem3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d328220_b80a_4d09_9ff8_f87094a1c831);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItem3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsDisabledInPlaybackList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDisabledInPlaybackList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub TotalDownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub AutoLoadedDisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AutoLoadedDisplayPropertyKind) -> ::windows::core::HRESULT,
    pub SetAutoLoadedDisplayProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AutoLoadedDisplayPropertyKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemError(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackItemError {
    type Vtable = IMediaPlaybackItemError_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackItemError {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69fbef2b_dcd6_4df9_a450_dbf4c6f1c2c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemError_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackItemErrorCode) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackItemFactory {
    type Vtable = IMediaPlaybackItemFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackItemFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7133fce1_1769_4ff9_a7c1_38d2c4d42360);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackItemFactory2 {
    type Vtable = IMediaPlaybackItemFactory2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackItemFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd77cdf3a_b947_4972_b35d_adfb931a71e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFactory2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub CreateWithStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, starttime: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))]
    CreateWithStartTime: usize,
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub CreateWithStartTimeAndDurationLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, starttime: super::super::Foundation::TimeSpan, durationlimit: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Core")))]
    CreateWithStartTimeAndDurationLimit: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackItemFailedEventArgs {
    type Vtable = IMediaPlaybackItemFailedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackItemFailedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7703134a_e9a7_47c3_862c_c656d30683d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemFailedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemOpenedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackItemOpenedEventArgs {
    type Vtable = IMediaPlaybackItemOpenedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackItemOpenedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbd9bd82_3037_4fbe_ae8f_39fc39edf4ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemOpenedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackItemStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackItemStatics {
    type Vtable = IMediaPlaybackItemStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackItemStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b1be7f4_4345_403c_8a67_f5de91df4c86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackItemStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub FindFromMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    FindFromMediaSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackList(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackList {
    type Vtable = IMediaPlaybackList_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f77ee9c_dc42_4e26_a98d_7850df8ec925);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackList_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ItemFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemFailed: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentItemChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentItemChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentItemChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentItemChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ItemOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemOpened: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
    pub AutoRepeatEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutoRepeatEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShuffleEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CurrentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentItemIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MovePrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemindex: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackList2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackList2 {
    type Vtable = IMediaPlaybackList2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackList2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e09b478_600a_4274_a14b_0b6723d0f48b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackList2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MaxPrefetchTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPrefetchTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPrefetchTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPrefetchTime: usize,
    pub StartingItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStartingItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ShuffledItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShuffledItems: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetShuffledItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetShuffledItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackList3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackList3 {
    type Vtable = IMediaPlaybackList3_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackList3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd24bba9_bc47_4463_aa90_c18b7e5ffde1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackList3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MaxPlayedItemsToKeepOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPlayedItemsToKeepOpen: usize,
    #[cfg(feature = "Foundation")]
    pub SetMaxPlayedItemsToKeepOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxPlayedItemsToKeepOpen: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackSession {
    type Vtable = IMediaPlaybackSession_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc32b683d_0407_41ba_8946_8b345a5a5435);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PlaybackStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlaybackRateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlaybackRateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlaybackRateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlaybackRateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SeekCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SeekCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSeekCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSeekCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub BufferingStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferingStarted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBufferingStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBufferingStarted: usize,
    #[cfg(feature = "Foundation")]
    pub BufferingEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferingEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBufferingEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBufferingEnded: usize,
    #[cfg(feature = "Foundation")]
    pub BufferingProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferingProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBufferingProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBufferingProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadProgressChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadProgressChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NaturalDurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NaturalDurationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNaturalDurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNaturalDurationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePositionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePositionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub NaturalVideoSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NaturalVideoSizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNaturalVideoSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNaturalVideoSizeChanged: usize,
    pub MediaPlayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NaturalDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NaturalDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
    pub PlaybackState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackState) -> ::windows::core::HRESULT,
    pub CanSeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanPause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub BufferingProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub DownloadProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub NaturalVideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub NaturalVideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NormalizedSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NormalizedSourceRect: usize,
    #[cfg(feature = "Foundation")]
    pub SetNormalizedSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNormalizedSourceRect: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub StereoscopicVideoPackingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    StereoscopicVideoPackingMode: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetStereoscopicVideoPackingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetStereoscopicVideoPackingMode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSession2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackSession2 {
    type Vtable = IMediaPlaybackSession2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackSession2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8ba7c79_1fc8_4097_ad70_c0fa18cc0050);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSession2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub BufferedRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferedRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBufferedRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBufferedRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub PlayedRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PlayedRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePlayedRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePlayedRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SeekableRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SeekableRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSeekableRangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSeekableRangesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SupportedPlaybackRatesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SupportedPlaybackRatesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSupportedPlaybackRatesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSupportedPlaybackRatesChanged: usize,
    pub SphericalVideoProjection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsMirroring: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBufferedRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBufferedRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPlayedRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPlayedRanges: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSeekableRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSeekableRanges: usize,
    pub IsSupportedPlaybackRateRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rate1: f64, rate2: f64, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSession3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackSession3 {
    type Vtable = IMediaPlaybackSession3_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackSession3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ba2b41a_a3e2_405f_b77b_a4812c238b66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSession3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub PlaybackRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    PlaybackRotation: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetPlaybackRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::MediaRotation) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetPlaybackRotation: usize,
    pub GetOutputDegradationPolicyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSessionBufferingStartedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackSessionBufferingStartedEventArgs {
    type Vtable = IMediaPlaybackSessionBufferingStartedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackSessionBufferingStartedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd6aafed_74e2_43b5_b115_76236c33791a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSessionBufferingStartedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPlaybackInterruption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSessionOutputDegradationPolicyState(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackSessionOutputDegradationPolicyState {
    type Vtable = IMediaPlaybackSessionOutputDegradationPolicyState_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackSessionOutputDegradationPolicyState {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x558e727d_f633_49f9_965a_abaa1db709be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSessionOutputDegradationPolicyState_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub VideoConstrictionReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlaybackSessionVideoConstrictionReason) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct IMediaPlaybackSource(::windows::core::IUnknown);
impl IMediaPlaybackSource {}
::windows::core::interface_hierarchy!(IMediaPlaybackSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IMediaPlaybackSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMediaPlaybackSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaPlaybackSource {}
impl ::core::fmt::Debug for IMediaPlaybackSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaPlaybackSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMediaPlaybackSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ef9dc2bc-9317-4696-b051-2bad643177b5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IMediaPlaybackSource {
    type Vtable = IMediaPlaybackSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef9dc2bc_9317_4696_b051_2bad643177b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackSphericalVideoProjection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackSphericalVideoProjection {
    type Vtable = IMediaPlaybackSphericalVideoProjection_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackSphericalVideoProjection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd405b37c_6f0e_4661_b8ee_d487ba9752d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSphericalVideoProjection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub FrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    FrameFormat: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub SetFrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SetFrameFormat: usize,
    pub HorizontalFieldOfViewInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetHorizontalFieldOfViewInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ViewOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ViewOrientation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetViewOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetViewOrientation: usize,
    pub ProjectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SphericalVideoProjectionMode) -> ::windows::core::HRESULT,
    pub SetProjectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SphericalVideoProjectionMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlaybackTimedMetadataTrackList(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlaybackTimedMetadataTrackList {
    type Vtable = IMediaPlaybackTimedMetadataTrackList_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlaybackTimedMetadataTrackList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b41319_bbfb_46a3_9372_9c9c744b9438);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackTimedMetadataTrackList_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub PresentationModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    PresentationModeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePresentationModeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePresentationModeChanged: usize,
    pub GetPresentationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT,
    pub SetPresentationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayer {
    type Vtable = IMediaPlayer_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x381a83cb_6fff_499b_8d64_2885dfc1249e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AutoPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub NaturalDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    NaturalDuration: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    Position: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetPosition: usize,
    #[cfg(feature = "deprecated")]
    pub BufferingProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BufferingProgress: usize,
    #[cfg(feature = "deprecated")]
    pub CurrentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CurrentState: usize,
    #[cfg(feature = "deprecated")]
    pub CanSeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanSeek: usize,
    #[cfg(feature = "deprecated")]
    pub CanPause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    CanPause: usize,
    pub IsLoopingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsLoopingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsProtected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsProtected: usize,
    pub IsMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub PlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlaybackRate: usize,
    #[cfg(feature = "deprecated")]
    pub SetPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetPlaybackRate: usize,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub PlaybackMediaMarkers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    PlaybackMediaMarkers: usize,
    #[cfg(feature = "Foundation")]
    pub MediaOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaOpened: usize,
    #[cfg(feature = "Foundation")]
    pub MediaEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaEnded: usize,
    #[cfg(feature = "Foundation")]
    pub MediaFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MediaFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMediaFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMediaFailed: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub CurrentStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    CurrentStateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveCurrentStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveCurrentStateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub PlaybackMediaMarkerReached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    PlaybackMediaMarkerReached: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemovePlaybackMediaMarkerReached: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemovePlaybackMediaMarkerReached: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub MediaPlayerRateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    MediaPlayerRateChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveMediaPlayerRateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveMediaPlayerRateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub VolumeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VolumeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVolumeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVolumeChanged: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SeekCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SeekCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveSeekCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveSeekCompleted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub BufferingStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    BufferingStarted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveBufferingStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveBufferingStarted: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub BufferingEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    BufferingEnded: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub RemoveBufferingEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    RemoveBufferingEnded: usize,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub SetUriSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    SetUriSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayer2 {
    type Vtable = IMediaPlayer2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c841218_2123_4fc5_9082_2f883f77bdf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SystemMediaTransportControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AudioCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerAudioCategory) -> ::windows::core::HRESULT,
    pub SetAudioCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlayerAudioCategory) -> ::windows::core::HRESULT,
    pub AudioDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerAudioDeviceType) -> ::windows::core::HRESULT,
    pub SetAudioDeviceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MediaPlayerAudioDeviceType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayer3 {
    type Vtable = IMediaPlayer3_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayer3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee0660da_031b_4feb_bd9b_92e0a0a8d299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub IsMutedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsMutedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsMutedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsMutedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SourceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceChanged: usize,
    pub AudioBalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetAudioBalance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub RealTimePlayback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRealTimePlayback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub StereoscopicVideoRenderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StereoscopicVideoRenderMode) -> ::windows::core::HRESULT,
    pub SetStereoscopicVideoRenderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StereoscopicVideoRenderMode) -> ::windows::core::HRESULT,
    pub BreakManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CommandManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub AudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    AudioDevice: usize,
    #[cfg(feature = "Devices_Enumeration")]
    pub SetAudioDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    SetAudioDevice: usize,
    pub TimelineController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTimelineController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TimelineControllerPositionOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimelineControllerPositionOffset: usize,
    #[cfg(feature = "Foundation")]
    pub SetTimelineControllerPositionOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTimelineControllerPositionOffset: usize,
    pub PlaybackSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StepForwardOneFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StepBackwardOneFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_Casting")]
    pub GetAsCastingSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Casting"))]
    GetAsCastingSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayer4 {
    type Vtable = IMediaPlayer4_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayer4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80035db0_7448_4770_afcf_2a57450914c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetSurfaceSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSurfaceSize: usize,
    #[cfg(feature = "UI_Composition")]
    pub GetSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetSurface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer5(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayer5 {
    type Vtable = IMediaPlayer5_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayer5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcfe537fd_f86a_4446_bf4d_c8e792b7b4b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub VideoFrameAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoFrameAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVideoFrameAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVideoFrameAvailable: usize,
    pub IsVideoFrameServerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsVideoFrameServerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CopyFrameToVideoSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CopyFrameToVideoSurface: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CopyFrameToVideoSurfaceWithTargetRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, targetrectangle: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CopyFrameToVideoSurfaceWithTargetRectangle: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CopyFrameToStereoscopicVideoSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationlefteye: *mut ::core::ffi::c_void, destinationrighteye: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CopyFrameToStereoscopicVideoSurfaces: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer6(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayer6 {
    type Vtable = IMediaPlayer6_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayer6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0caa086_ae65_414c_b010_8bc55f00e692);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer6_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SubtitleFrameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubtitleFrameChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSubtitleFrameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSubtitleFrameChanged: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub RenderSubtitlesToSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    RenderSubtitlesToSurface: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub RenderSubtitlesToSurfaceWithTargetRectangle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destination: *mut ::core::ffi::c_void, targetrectangle: super::super::Foundation::Rect, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    RenderSubtitlesToSurfaceWithTargetRectangle: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayer7(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayer7 {
    type Vtable = IMediaPlayer7_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayer7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d1dc478_4500_4531_b3f4_777a71491f7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer7_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Audio")]
    pub AudioStateMonitor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))]
    AudioStateMonitor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerDataReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayerDataReceivedEventArgs {
    type Vtable = IMediaPlayerDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayerDataReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc75a9405_c801_412a_835b_83fc0e622a8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerDataReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Data: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerEffects(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayerEffects {
    type Vtable = IMediaPlayerEffects_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayerEffects {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85a1deda_cab6_4cc0_8be3_6035f4de2591);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerEffects_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddAudioEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: *mut ::core::ffi::c_void, effectoptional: bool, configuration: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAudioEffect: usize,
    pub RemoveAllEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerEffects2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayerEffects2 {
    type Vtable = IMediaPlayerEffects2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayerEffects2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa419a79_1bbe_46c5_ae1f_8ee69fb3c2c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerEffects2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddVideoEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: *mut ::core::ffi::c_void, effectoptional: bool, effectconfiguration: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddVideoEffect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerFailedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayerFailedEventArgs {
    type Vtable = IMediaPlayerFailedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayerFailedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2744e9b9_a7e3_4f16_bac4_7914ebc08301);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerFailedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaPlayerError) -> ::windows::core::HRESULT,
    pub ExtendedErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub ErrorMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerRateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayerRateChangedEventArgs {
    type Vtable = IMediaPlayerRateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayerRateChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40600d58_3b61_4bb2_989f_fc65608b6cab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerRateChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NewRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayerSource {
    type Vtable = IMediaPlayerSource_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayerSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd4f8897_1423_4c3e_82c5_0fb1af94f715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Protection")]
    pub ProtectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    ProtectionManager: usize,
    #[cfg(feature = "Media_Protection")]
    pub SetProtectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    SetProtectionManager: usize,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub SetFileSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    SetFileSource: usize,
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub SetStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_Streams", feature = "deprecated")))]
    SetStreamSource: usize,
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub SetMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Media_Core", feature = "deprecated")))]
    SetMediaSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerSource2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayerSource2 {
    type Vtable = IMediaPlayerSource2_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayerSource2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82449b9f_7322_4c0b_b03b_3e69a48260c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerSource2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaPlayerSurface(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMediaPlayerSurface {
    type Vtable = IMediaPlayerSurface_Vtbl;
}
unsafe impl ::windows::core::Interface for IMediaPlayerSurface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ed653bc_b736_49c3_830b_764a3845313a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerSurface_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CompositionSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CompositionSurface: usize,
    #[cfg(feature = "UI_Composition")]
    pub Compositor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Compositor: usize,
    pub MediaPlayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackMediaMarker(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlaybackMediaMarker {
    type Vtable = IPlaybackMediaMarker_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlaybackMediaMarker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4d22f5c_3c1c_4444_b6b9_778b0422d41a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarker_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Time: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Time: usize,
    pub MediaMarkerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackMediaMarkerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlaybackMediaMarkerFactory {
    type Vtable = IPlaybackMediaMarkerFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlaybackMediaMarkerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c530a78_e0ae_4e1a_a8c8_e23f982a937b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateFromTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromTime: usize,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan, mediamarkettype: *mut ::core::ffi::c_void, text: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackMediaMarkerReachedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlaybackMediaMarkerReachedEventArgs {
    type Vtable = IPlaybackMediaMarkerReachedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlaybackMediaMarkerReachedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x578cd1b9_90e2_4e60_abc4_8740b01f6196);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerReachedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PlaybackMediaMarker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPlaybackMediaMarkerSequence(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPlaybackMediaMarkerSequence {
    type Vtable = IPlaybackMediaMarkerSequence_Vtbl;
}
unsafe impl ::windows::core::Interface for IPlaybackMediaMarkerSequence {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2810cee_638b_46cf_8817_1d111fe9d8c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerSequence_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Insert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataPresentationModeChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ITimedMetadataPresentationModeChangedEventArgs {
    type Vtable = ITimedMetadataPresentationModeChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ITimedMetadataPresentationModeChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1636099_65df_45ae_8cef_dc0b53fdc2bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataPresentationModeChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Core")]
    pub Track: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Core"))]
    Track: usize,
    pub OldPresentationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT,
    pub NewPresentationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackPresentationMode) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Media_Playback\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
pub struct BackgroundMediaPlayer;
#[cfg(feature = "deprecated")]
impl BackgroundMediaPlayer {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Current() -> ::windows::core::Result<MediaPlayer> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Current)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn MessageReceivedFromBackground(value: &super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageReceivedFromBackground)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveMessageReceivedFromBackground(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveMessageReceivedFromBackground)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn MessageReceivedFromForeground(value: &super::super::Foundation::EventHandler<MediaPlayerDataReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageReceivedFromForeground)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveMessageReceivedFromForeground(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveMessageReceivedFromForeground)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn SendMessageToBackground(value: &super::super::Foundation::Collections::ValueSet) -> ::windows::core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SendMessageToBackground)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn SendMessageToForeground(value: &super::super::Foundation::Collections::ValueSet) -> ::windows::core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).SendMessageToForeground)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsMediaPlaying() -> ::windows::core::Result<bool> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMediaPlaying)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Shutdown() -> ::windows::core::Result<()> {
        Self::IBackgroundMediaPlayerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).Shutdown)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IBackgroundMediaPlayerStatics<R, F: FnOnce(&IBackgroundMediaPlayerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BackgroundMediaPlayer, IBackgroundMediaPlayerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for BackgroundMediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.BackgroundMediaPlayer";
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct CurrentMediaPlaybackItemChangedEventArgs(::windows::core::IUnknown);
impl CurrentMediaPlaybackItemChangedEventArgs {
    pub fn NewItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewItem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OldItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OldItem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Reason(&self) -> ::windows::core::Result<MediaPlaybackItemChangedReason> {
        let this = &::windows::core::Interface::cast::<ICurrentMediaPlaybackItemChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CurrentMediaPlaybackItemChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrentMediaPlaybackItemChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentMediaPlaybackItemChangedEventArgs {}
impl ::core::fmt::Debug for CurrentMediaPlaybackItemChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentMediaPlaybackItemChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CurrentMediaPlaybackItemChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.CurrentMediaPlaybackItemChangedEventArgs;{1743a892-5c43-4a15-967a-572d2d0f26c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CurrentMediaPlaybackItemChangedEventArgs {
    type Vtable = ICurrentMediaPlaybackItemChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CurrentMediaPlaybackItemChangedEventArgs {
    const IID: ::windows::core::GUID = <ICurrentMediaPlaybackItemChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CurrentMediaPlaybackItemChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.CurrentMediaPlaybackItemChangedEventArgs";
}
::windows::core::interface_hierarchy!(CurrentMediaPlaybackItemChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CurrentMediaPlaybackItemChangedEventArgs {}
unsafe impl ::core::marker::Sync for CurrentMediaPlaybackItemChangedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaBreak(::windows::core::IUnknown);
impl MediaBreak {
    pub fn PlaybackList(&self) -> ::windows::core::Result<MediaPlaybackList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackList)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PresentationPosition(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PresentationPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn InsertionMethod(&self) -> ::windows::core::Result<MediaBreakInsertionMethod> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InsertionMethod)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanStart(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanStart)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCanStart(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCanStart)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Create(insertionmethod: MediaBreakInsertionMethod) -> ::windows::core::Result<MediaBreak> {
        Self::IMediaBreakFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), insertionmethod, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithPresentationPosition(insertionmethod: MediaBreakInsertionMethod, presentationposition: super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaBreak> {
        Self::IMediaBreakFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithPresentationPosition)(::windows::core::Vtable::as_raw(this), insertionmethod, presentationposition, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaBreakFactory<R, F: FnOnce(&IMediaBreakFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaBreak, IMediaBreakFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MediaBreak {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreak {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreak {}
impl ::core::fmt::Debug for MediaBreak {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreak").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreak {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreak;{714be270-0def-4ebc-a489-6b34930e1558})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaBreak {
    type Vtable = IMediaBreak_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaBreak {
    const IID: ::windows::core::GUID = <IMediaBreak as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaBreak {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreak";
}
::windows::core::interface_hierarchy!(MediaBreak, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaBreak {}
unsafe impl ::core::marker::Sync for MediaBreak {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaBreakEndedEventArgs(::windows::core::IUnknown);
impl MediaBreakEndedEventArgs {
    pub fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaBreak)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBreakEndedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakEndedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakEndedEventArgs {}
impl ::core::fmt::Debug for MediaBreakEndedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakEndedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakEndedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakEndedEventArgs;{32b93276-1c5d-4fee-8732-236dc3a88580})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaBreakEndedEventArgs {
    type Vtable = IMediaBreakEndedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaBreakEndedEventArgs {
    const IID: ::windows::core::GUID = <IMediaBreakEndedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaBreakEndedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakEndedEventArgs";
}
::windows::core::interface_hierarchy!(MediaBreakEndedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaBreakEndedEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakEndedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaBreakManager(::windows::core::IUnknown);
impl MediaBreakManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BreaksSeekedOver(&self, handler: &super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSeekedOverEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BreaksSeekedOver)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBreaksSeekedOver(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveBreaksSeekedOver)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BreakStarted(&self, handler: &super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakStartedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BreakStarted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBreakStarted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveBreakStarted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BreakEnded(&self, handler: &super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakEndedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BreakEnded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBreakEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveBreakEnded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BreakSkipped(&self, handler: &super::super::Foundation::TypedEventHandler<MediaBreakManager, MediaBreakSkippedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BreakSkipped)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBreakSkipped(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveBreakSkipped)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn CurrentBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentBreak)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PlaybackSession(&self) -> ::windows::core::Result<MediaPlaybackSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackSession)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PlayBreak(&self, value: &MediaBreak) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).PlayBreak)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SkipCurrentBreak(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SkipCurrentBreak)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaBreakManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakManager {}
impl ::core::fmt::Debug for MediaBreakManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakManager;{a854ddb1-feb4-4d9b-9d97-0fdbe58e5e39})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaBreakManager {
    type Vtable = IMediaBreakManager_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaBreakManager {
    const IID: ::windows::core::GUID = <IMediaBreakManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaBreakManager {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakManager";
}
::windows::core::interface_hierarchy!(MediaBreakManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaBreakManager {}
unsafe impl ::core::marker::Sync for MediaBreakManager {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaBreakSchedule(::windows::core::IUnknown);
impl MediaBreakSchedule {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScheduleChanged(&self, handler: &super::super::Foundation::TypedEventHandler<MediaBreakSchedule, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScheduleChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScheduleChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveScheduleChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn InsertMidrollBreak(&self, mediabreak: &MediaBreak) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).InsertMidrollBreak)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mediabreak)).ok() }
    }
    pub fn RemoveMidrollBreak(&self, mediabreak: &MediaBreak) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMidrollBreak)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mediabreak)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MidrollBreaks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MidrollBreaks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPrerollBreak(&self, value: &MediaBreak) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrerollBreak)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PrerollBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrerollBreak)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPostrollBreak(&self, value: &MediaBreak) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPostrollBreak)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PostrollBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PostrollBreak)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PlaybackItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackItem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBreakSchedule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakSchedule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakSchedule {}
impl ::core::fmt::Debug for MediaBreakSchedule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakSchedule").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakSchedule {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakSchedule;{a19a5813-98b6-41d8-83da-f971d22b7bba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaBreakSchedule {
    type Vtable = IMediaBreakSchedule_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaBreakSchedule {
    const IID: ::windows::core::GUID = <IMediaBreakSchedule as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaBreakSchedule {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSchedule";
}
::windows::core::interface_hierarchy!(MediaBreakSchedule, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaBreakSchedule {}
unsafe impl ::core::marker::Sync for MediaBreakSchedule {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaBreakSeekedOverEventArgs(::windows::core::IUnknown);
impl MediaBreakSeekedOverEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SeekedOverBreaks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaBreak>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SeekedOverBreaks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OldPosition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OldPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NewPosition(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewPosition)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBreakSeekedOverEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakSeekedOverEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakSeekedOverEventArgs {}
impl ::core::fmt::Debug for MediaBreakSeekedOverEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakSeekedOverEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakSeekedOverEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakSeekedOverEventArgs;{e5aa6746-0606-4492-b9d3-c3c8fde0a4ea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaBreakSeekedOverEventArgs {
    type Vtable = IMediaBreakSeekedOverEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaBreakSeekedOverEventArgs {
    const IID: ::windows::core::GUID = <IMediaBreakSeekedOverEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaBreakSeekedOverEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSeekedOverEventArgs";
}
::windows::core::interface_hierarchy!(MediaBreakSeekedOverEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaBreakSeekedOverEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakSeekedOverEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaBreakSkippedEventArgs(::windows::core::IUnknown);
impl MediaBreakSkippedEventArgs {
    pub fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaBreak)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBreakSkippedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakSkippedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakSkippedEventArgs {}
impl ::core::fmt::Debug for MediaBreakSkippedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakSkippedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakSkippedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakSkippedEventArgs;{6ee94c05-2f54-4a3e-a3ab-24c3b270b4a3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaBreakSkippedEventArgs {
    type Vtable = IMediaBreakSkippedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaBreakSkippedEventArgs {
    const IID: ::windows::core::GUID = <IMediaBreakSkippedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaBreakSkippedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakSkippedEventArgs";
}
::windows::core::interface_hierarchy!(MediaBreakSkippedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaBreakSkippedEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakSkippedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaBreakStartedEventArgs(::windows::core::IUnknown);
impl MediaBreakStartedEventArgs {
    pub fn MediaBreak(&self) -> ::windows::core::Result<MediaBreak> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaBreak)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaBreakStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaBreakStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBreakStartedEventArgs {}
impl ::core::fmt::Debug for MediaBreakStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaBreakStartedEventArgs;{a87efe71-dfd4-454a-956e-0a4a648395f8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaBreakStartedEventArgs {
    type Vtable = IMediaBreakStartedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaBreakStartedEventArgs {
    const IID: ::windows::core::GUID = <IMediaBreakStartedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaBreakStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaBreakStartedEventArgs";
}
::windows::core::interface_hierarchy!(MediaBreakStartedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaBreakStartedEventArgs {}
unsafe impl ::core::marker::Sync for MediaBreakStartedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaItemDisplayProperties(::windows::core::IUnknown);
impl MediaItemDisplayProperties {
    pub fn Type(&self) -> ::windows::core::Result<super::MediaPlaybackType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: super::MediaPlaybackType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetType)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MusicProperties(&self) -> ::windows::core::Result<super::MusicDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MusicProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn VideoProperties(&self) -> ::windows::core::Result<super::VideoDisplayProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VideoProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Thumbnail)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail(&self, value: &super::super::Storage::Streams::RandomAccessStreamReference) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetThumbnail)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ClearAll(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ClearAll)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for MediaItemDisplayProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaItemDisplayProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaItemDisplayProperties {}
impl ::core::fmt::Debug for MediaItemDisplayProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaItemDisplayProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaItemDisplayProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaItemDisplayProperties;{1e3c1b48-7097-4384-a217-c1291dfa8c16})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaItemDisplayProperties {
    type Vtable = IMediaItemDisplayProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaItemDisplayProperties {
    const IID: ::windows::core::GUID = <IMediaItemDisplayProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaItemDisplayProperties {
    const NAME: &'static str = "Windows.Media.Playback.MediaItemDisplayProperties";
}
::windows::core::interface_hierarchy!(MediaItemDisplayProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaItemDisplayProperties {}
unsafe impl ::core::marker::Sync for MediaItemDisplayProperties {}
#[doc = "*Required features: `\"Media_Playback\"`, `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
#[repr(transparent)]
pub struct MediaPlaybackAudioTrackList(::windows::core::IUnknown);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl MediaPlaybackAudioTrackList {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::Core::AudioTrack>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::Core::AudioTrack>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn SelectedIndexChanged(&self, handler: &super::super::Foundation::TypedEventHandler<super::Core::ISingleSelectMediaTrackList, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectedIndexChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn RemoveSelectedIndexChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSelectedIndexChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSelectedIndex)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn SelectedIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectedIndex)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::Core::AudioTrack> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn IndexOf(&self, value: &super::Core::AudioTrack, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<super::Core::AudioTrack>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::clone::Clone for MediaPlaybackAudioTrackList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::cmp::PartialEq for MediaPlaybackAudioTrackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::cmp::Eq for MediaPlaybackAudioTrackList {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::fmt::Debug for MediaPlaybackAudioTrackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackAudioTrackList").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::windows::core::RuntimeType for MediaPlaybackAudioTrackList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackAudioTrackList;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Media.Core.AudioTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::windows::core::Vtable for MediaPlaybackAudioTrackList {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<super::Core::AudioTrack>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::windows::core::Interface for MediaPlaybackAudioTrackList {
    const IID: ::windows::core::GUID = <super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack> as ::windows::core::Interface>::IID;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::windows::core::RuntimeName for MediaPlaybackAudioTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackAudioTrackList";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::iter::IntoIterator for MediaPlaybackAudioTrackList {
    type Item = super::Core::AudioTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::iter::IntoIterator for &MediaPlaybackAudioTrackList {
    type Item = super::Core::AudioTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
::windows::core::interface_hierarchy!(MediaPlaybackAudioTrackList, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<MediaPlaybackAudioTrackList> for super::super::Foundation::Collections::IIterable<super::Core::AudioTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackAudioTrackList> for super::super::Foundation::Collections::IIterable<super::Core::AudioTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackAudioTrackList> for ::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::Core::AudioTrack>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<MediaPlaybackAudioTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackAudioTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackAudioTrackList> for ::windows::core::InParam<super::Core::ISingleSelectMediaTrackList> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<MediaPlaybackAudioTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackAudioTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackAudioTrackList> for ::windows::core::InParam<super::super::Foundation::Collections::IVectorView<super::Core::AudioTrack>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackAudioTrackList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::core::marker::Send for MediaPlaybackAudioTrackList {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::core::marker::Sync for MediaPlaybackAudioTrackList {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManager(::windows::core::IUnknown);
impl MediaPlaybackCommandManager {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaPlayer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PlayBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlayBehavior)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PauseBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PauseBehavior)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NextBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextBehavior)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PreviousBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousBehavior)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FastForwardBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FastForwardBehavior)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RewindBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RewindBehavior)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ShuffleBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShuffleBehavior)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AutoRepeatModeBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoRepeatModeBehavior)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PositionBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionBehavior)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RateBehavior(&self) -> ::windows::core::Result<MediaPlaybackCommandManagerCommandBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RateBehavior)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlayReceived(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPlayReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlayReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlayReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePlayReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PauseReceived(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPauseReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PauseReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePauseReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePauseReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NextReceived(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerNextReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NextReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNextReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNextReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreviousReceived(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPreviousReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePreviousReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePreviousReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FastForwardReceived(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerFastForwardReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FastForwardReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFastForwardReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFastForwardReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RewindReceived(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRewindReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RewindReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRewindReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRewindReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShuffleReceived(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerShuffleReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShuffleReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShuffleReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveShuffleReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AutoRepeatModeReceived(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoRepeatModeReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAutoRepeatModeReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAutoRepeatModeReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PositionReceived(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerPositionReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePositionReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RateReceived(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManager, MediaPlaybackCommandManagerRateReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RateReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRateReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRateReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManager {}
impl ::core::fmt::Debug for MediaPlaybackCommandManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManager;{5acee5a6-5cb6-4a5a-8521-cc86b1c1ed37})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManager {
    type Vtable = IMediaPlaybackCommandManager_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManager {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManager {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManager";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManager {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManager {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AutoRepeatMode(&self) -> ::windows::core::Result<super::MediaPlaybackAutoRepeatMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoRepeatMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs;{3d6f4f23-5230-4411-a0e9-bad94c2a045c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerAutoRepeatModeReceivedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerCommandBehavior(::windows::core::IUnknown);
impl MediaPlaybackCommandManagerCommandBehavior {
    pub fn CommandManager(&self) -> ::windows::core::Result<MediaPlaybackCommandManager> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CommandManager)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EnablingRule(&self) -> ::windows::core::Result<MediaCommandEnablingRule> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnablingRule)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetEnablingRule(&self, value: MediaCommandEnablingRule) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEnablingRule)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsEnabledChanged(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackCommandManagerCommandBehavior, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabledChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsEnabledChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveIsEnabledChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerCommandBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerCommandBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerCommandBehavior {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerCommandBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerCommandBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerCommandBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerCommandBehavior;{786c1e78-ce78-4a10-afd6-843fcbb90c2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManagerCommandBehavior {
    type Vtable = IMediaPlaybackCommandManagerCommandBehavior_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerCommandBehavior {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManagerCommandBehavior as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerCommandBehavior {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerCommandBehavior";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManagerCommandBehavior, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerCommandBehavior {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerCommandBehavior {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerFastForwardReceivedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerFastForwardReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerFastForwardReceivedEventArgs;{30f064d9-b491-4d0a-bc21-3098bd1332e9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerFastForwardReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManagerFastForwardReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerFastForwardReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManagerFastForwardReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerFastForwardReceivedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerNextReceivedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackCommandManagerNextReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerNextReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerNextReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerNextReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerNextReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerNextReceivedEventArgs;{e1504433-a2b0-45d4-b9de-5f42ac14a839})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManagerNextReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerNextReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerNextReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManagerNextReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerNextReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerNextReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManagerNextReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerNextReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerNextReceivedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPauseReceivedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackCommandManagerPauseReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerPauseReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerPauseReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPauseReceivedEventArgs;{5ceccd1c-c25c-4221-b16c-c3c98ce012d6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPauseReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManagerPauseReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerPauseReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPauseReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManagerPauseReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPauseReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPauseReceivedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPlayReceivedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackCommandManagerPlayReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerPlayReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerPlayReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPlayReceivedEventArgs;{9af0004e-578b-4c56-a006-16159d888a48})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPlayReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManagerPlayReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerPlayReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPlayReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManagerPlayReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPlayReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPlayReceivedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPositionReceivedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackCommandManagerPositionReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerPositionReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerPositionReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPositionReceivedEventArgs;{5591a754-d627-4bdd-a90d-86a015b24902})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPositionReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManagerPositionReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerPositionReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPositionReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManagerPositionReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPositionReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPositionReceivedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerPreviousReceivedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerPreviousReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerPreviousReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerPreviousReceivedEventArgs;{525e3081-4632-4f76-99b1-d771623f6287})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerPreviousReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManagerPreviousReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerPreviousReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerPreviousReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManagerPreviousReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerPreviousReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerPreviousReceivedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerRateReceivedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackCommandManagerRateReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PlaybackRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerRateReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerRateReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerRateReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerRateReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerRateReceivedEventArgs;{18ea3939-4a16-4169-8b05-3eb9f5ff78eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManagerRateReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRateReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerRateReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManagerRateReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerRateReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerRateReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManagerRateReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerRateReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerRateReceivedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerRewindReceivedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackCommandManagerRewindReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerRewindReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerRewindReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerRewindReceivedEventArgs;{9f085947-a3c0-425d-aaef-97ba7898b141})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerRewindReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManagerRewindReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerRewindReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerRewindReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManagerRewindReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerRewindReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerRewindReceivedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackCommandManagerShuffleReceivedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsShuffleRequested(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsShuffleRequested)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackCommandManagerShuffleReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackCommandManagerShuffleReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackCommandManagerShuffleReceivedEventArgs;{50a05cef-63ee-4a96-b7b5-fee08b9ff90c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    type Vtable = IMediaPlaybackCommandManagerShuffleReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackCommandManagerShuffleReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackCommandManagerShuffleReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackCommandManagerShuffleReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackCommandManagerShuffleReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackCommandManagerShuffleReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackCommandManagerShuffleReceivedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackItem(::windows::core::IUnknown);
impl MediaPlaybackItem {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AudioTracksChanged(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioTracksChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAudioTracksChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAudioTracksChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn VideoTracksChanged(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VideoTracksChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoTracksChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveVideoTracksChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TimedMetadataTracksChanged(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackItem, super::super::Foundation::Collections::IVectorChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimedMetadataTracksChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTimedMetadataTracksChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveTimedMetadataTracksChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn Source(&self) -> ::windows::core::Result<super::Core::MediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Source)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn AudioTracks(&self) -> ::windows::core::Result<MediaPlaybackAudioTrackList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioTracks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn VideoTracks(&self) -> ::windows::core::Result<MediaPlaybackVideoTrackList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VideoTracks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn TimedMetadataTracks(&self) -> ::windows::core::Result<MediaPlaybackTimedMetadataTrackList> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimedMetadataTracks)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BreakSchedule(&self) -> ::windows::core::Result<MediaBreakSchedule> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BreakSchedule)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DurationLimit(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DurationLimit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanSkip(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanSkip)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCanSkip(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetCanSkip)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetDisplayProperties(&self) -> ::windows::core::Result<MediaItemDisplayProperties> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDisplayProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ApplyDisplayProperties(&self, value: &MediaItemDisplayProperties) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).ApplyDisplayProperties)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsDisabledInPlaybackList(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDisabledInPlaybackList)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsDisabledInPlaybackList(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsDisabledInPlaybackList)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn TotalDownloadProgress(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TotalDownloadProgress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AutoLoadedDisplayProperties(&self) -> ::windows::core::Result<AutoLoadedDisplayPropertyKind> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoLoadedDisplayProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAutoLoadedDisplayProperties(&self, value: AutoLoadedDisplayPropertyKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackItem3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAutoLoadedDisplayProperties)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn Create(source: &super::Core::MediaSource) -> ::windows::core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateWithStartTime(source: &super::Core::MediaSource, starttime: super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithStartTime)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source), starttime, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn CreateWithStartTimeAndDurationLimit(source: &super::Core::MediaSource, starttime: super::super::Foundation::TimeSpan, durationlimit: super::super::Foundation::TimeSpan) -> ::windows::core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemFactory2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithStartTimeAndDurationLimit)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source), starttime, durationlimit, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn FindFromMediaSource(source: &super::Core::MediaSource) -> ::windows::core::Result<MediaPlaybackItem> {
        Self::IMediaPlaybackItemStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FindFromMediaSource)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaPlaybackItemFactory<R, F: FnOnce(&IMediaPlaybackItemFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaPlaybackItemFactory2<R, F: FnOnce(&IMediaPlaybackItemFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemFactory2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaPlaybackItemStatics<R, F: FnOnce(&IMediaPlaybackItemStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaPlaybackItem, IMediaPlaybackItemStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MediaPlaybackItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackItem {}
impl ::core::fmt::Debug for MediaPlaybackItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItem;{047097d2-e4af-48ab-b283-6929e674ece2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackItem {
    type Vtable = IMediaPlaybackItem_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackItem {
    const IID: ::windows::core::GUID = <IMediaPlaybackItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackItem {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItem";
}
::windows::core::interface_hierarchy!(MediaPlaybackItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MediaPlaybackItem> for IMediaPlaybackSource {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackItem) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaPlaybackItem> for IMediaPlaybackSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackItem) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MediaPlaybackItem> for ::windows::core::InParam<IMediaPlaybackSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackItem) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackItem {}
unsafe impl ::core::marker::Sync for MediaPlaybackItem {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackItemError(::windows::core::IUnknown);
impl MediaPlaybackItemError {
    pub fn ErrorCode(&self) -> ::windows::core::Result<MediaPlaybackItemErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedError)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackItemError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackItemError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackItemError {}
impl ::core::fmt::Debug for MediaPlaybackItemError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItemError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItemError;{69fbef2b-dcd6-4df9-a450-dbf4c6f1c2c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackItemError {
    type Vtable = IMediaPlaybackItemError_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackItemError {
    const IID: ::windows::core::GUID = <IMediaPlaybackItemError as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackItemError {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemError";
}
::windows::core::interface_hierarchy!(MediaPlaybackItemError, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackItemError {}
unsafe impl ::core::marker::Sync for MediaPlaybackItemError {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackItemFailedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackItemFailedEventArgs {
    pub fn Item(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Item)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Error(&self) -> ::windows::core::Result<MediaPlaybackItemError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Error)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackItemFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackItemFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackItemFailedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackItemFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItemFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItemFailedEventArgs;{7703134a-e9a7-47c3-862c-c656d30683d4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackItemFailedEventArgs {
    type Vtable = IMediaPlaybackItemFailedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackItemFailedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackItemFailedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackItemFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemFailedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackItemFailedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackItemFailedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackItemFailedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackItemOpenedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackItemOpenedEventArgs {
    pub fn Item(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Item)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackItemOpenedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackItemOpenedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackItemOpenedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackItemOpenedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemOpenedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItemOpenedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackItemOpenedEventArgs;{cbd9bd82-3037-4fbe-ae8f-39fc39edf4ef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackItemOpenedEventArgs {
    type Vtable = IMediaPlaybackItemOpenedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackItemOpenedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackItemOpenedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackItemOpenedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackItemOpenedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackItemOpenedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackItemOpenedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackItemOpenedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackList(::windows::core::IUnknown);
impl MediaPlaybackList {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaPlaybackList, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemFailed(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemFailedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemFailed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveItemFailed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentItemChanged(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackList, CurrentMediaPlaybackItemChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentItemChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentItemChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCurrentItemChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemOpened(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackList, MediaPlaybackItemOpenedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemOpened)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemOpened(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveItemOpened)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IObservableVector<MediaPlaybackItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Items)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AutoRepeatEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoRepeatEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAutoRepeatEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAutoRepeatEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ShuffleEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShuffleEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetShuffleEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetShuffleEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CurrentItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentItem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CurrentItemIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentItemIndex)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MoveNext(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoveNext)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MovePrevious(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MovePrevious)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MoveTo(&self, itemindex: u32) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoveTo)(::windows::core::Vtable::as_raw(this), itemindex, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPrefetchTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxPrefetchTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxPrefetchTime<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxPrefetchTime)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn StartingItem(&self) -> ::windows::core::Result<MediaPlaybackItem> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartingItem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStartingItem(&self, value: &MediaPlaybackItem) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStartingItem)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShuffledItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MediaPlaybackItem>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShuffledItems)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetShuffledItems<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<MediaPlaybackItem>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetShuffledItems)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPlayedItemsToKeepOpen(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxPlayedItemsToKeepOpen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxPlayedItemsToKeepOpen<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackList3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetMaxPlayedItemsToKeepOpen)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaPlaybackList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackList {}
impl ::core::fmt::Debug for MediaPlaybackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackList;{7f77ee9c-dc42-4e26-a98d-7850df8ec925})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackList {
    type Vtable = IMediaPlaybackList_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackList {
    const IID: ::windows::core::GUID = <IMediaPlaybackList as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackList";
}
::windows::core::interface_hierarchy!(MediaPlaybackList, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MediaPlaybackList> for IMediaPlaybackSource {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MediaPlaybackList> for IMediaPlaybackSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MediaPlaybackList> for ::windows::core::InParam<IMediaPlaybackSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MediaPlaybackList {}
unsafe impl ::core::marker::Sync for MediaPlaybackList {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackSession(::windows::core::IUnknown);
impl MediaPlaybackSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackStateChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackStateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePlaybackStateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlaybackRateChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackRateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlaybackRateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePlaybackRateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SeekCompleted(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SeekCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSeekCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSeekCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferingStarted(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferingStarted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBufferingStarted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveBufferingStarted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferingEnded(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferingEnded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBufferingEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveBufferingEnded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferingProgressChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferingProgressChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBufferingProgressChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveBufferingProgressChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadProgressChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DownloadProgressChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDownloadProgressChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDownloadProgressChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NaturalDurationChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalDurationChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNaturalDurationChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNaturalDurationChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PositionChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PositionChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePositionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePositionChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NaturalVideoSizeChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalVideoSizeChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNaturalVideoSizeChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNaturalVideoSizeChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaPlayer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NaturalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPosition(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPosition)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PlaybackState(&self) -> ::windows::core::Result<MediaPlaybackState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanSeek(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanSeek)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanPause(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanPause)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsProtected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsProtected)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PlaybackRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPlaybackRate)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BufferingProgress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferingProgress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DownloadProgress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DownloadProgress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NaturalVideoHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalVideoHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NaturalVideoWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalVideoWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NormalizedSourceRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NormalizedSourceRect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetNormalizedSourceRect(&self, value: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNormalizedSourceRect)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn StereoscopicVideoPackingMode(&self) -> ::windows::core::Result<super::MediaProperties::StereoscopicVideoPackingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StereoscopicVideoPackingMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetStereoscopicVideoPackingMode(&self, value: super::MediaProperties::StereoscopicVideoPackingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetStereoscopicVideoPackingMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferedRangesChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferedRangesChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBufferedRangesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveBufferedRangesChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PlayedRangesChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlayedRangesChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePlayedRangesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePlayedRangesChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SeekableRangesChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SeekableRangesChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSeekableRangesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSeekableRangesChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SupportedPlaybackRatesChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlaybackSession, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedPlaybackRatesChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSupportedPlaybackRatesChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSupportedPlaybackRatesChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn SphericalVideoProjection(&self) -> ::windows::core::Result<MediaPlaybackSphericalVideoProjection> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SphericalVideoProjection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsMirroring(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMirroring)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsMirroring(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsMirroring)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBufferedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBufferedRanges)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPlayedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPlayedRanges)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSeekableRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaTimeRange>> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSeekableRanges)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsSupportedPlaybackRateRange(&self, rate1: f64, rate2: f64) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSupportedPlaybackRateRange)(::windows::core::Vtable::as_raw(this), rate1, rate2, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn PlaybackRotation(&self) -> ::windows::core::Result<super::MediaProperties::MediaRotation> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackRotation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetPlaybackRotation(&self, value: super::MediaProperties::MediaRotation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPlaybackRotation)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetOutputDegradationPolicyState(&self) -> ::windows::core::Result<MediaPlaybackSessionOutputDegradationPolicyState> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackSession3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOutputDegradationPolicyState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackSession {}
impl ::core::fmt::Debug for MediaPlaybackSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSession;{c32b683d-0407-41ba-8946-8b345a5a5435})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackSession {
    type Vtable = IMediaPlaybackSession_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackSession {
    const IID: ::windows::core::GUID = <IMediaPlaybackSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackSession {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSession";
}
::windows::core::interface_hierarchy!(MediaPlaybackSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackSession {}
unsafe impl ::core::marker::Sync for MediaPlaybackSession {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackSessionBufferingStartedEventArgs(::windows::core::IUnknown);
impl MediaPlaybackSessionBufferingStartedEventArgs {
    pub fn IsPlaybackInterruption(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPlaybackInterruption)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackSessionBufferingStartedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackSessionBufferingStartedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackSessionBufferingStartedEventArgs {}
impl ::core::fmt::Debug for MediaPlaybackSessionBufferingStartedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackSessionBufferingStartedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackSessionBufferingStartedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSessionBufferingStartedEventArgs;{cd6aafed-74e2-43b5-b115-76236c33791a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackSessionBufferingStartedEventArgs {
    type Vtable = IMediaPlaybackSessionBufferingStartedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackSessionBufferingStartedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlaybackSessionBufferingStartedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackSessionBufferingStartedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSessionBufferingStartedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlaybackSessionBufferingStartedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackSessionBufferingStartedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlaybackSessionBufferingStartedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackSessionOutputDegradationPolicyState(::windows::core::IUnknown);
impl MediaPlaybackSessionOutputDegradationPolicyState {
    pub fn VideoConstrictionReason(&self) -> ::windows::core::Result<MediaPlaybackSessionVideoConstrictionReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VideoConstrictionReason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlaybackSessionOutputDegradationPolicyState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackSessionOutputDegradationPolicyState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackSessionOutputDegradationPolicyState {}
impl ::core::fmt::Debug for MediaPlaybackSessionOutputDegradationPolicyState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackSessionOutputDegradationPolicyState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackSessionOutputDegradationPolicyState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSessionOutputDegradationPolicyState;{558e727d-f633-49f9-965a-abaa1db709be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackSessionOutputDegradationPolicyState {
    type Vtable = IMediaPlaybackSessionOutputDegradationPolicyState_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackSessionOutputDegradationPolicyState {
    const IID: ::windows::core::GUID = <IMediaPlaybackSessionOutputDegradationPolicyState as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackSessionOutputDegradationPolicyState {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSessionOutputDegradationPolicyState";
}
::windows::core::interface_hierarchy!(MediaPlaybackSessionOutputDegradationPolicyState, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackSessionOutputDegradationPolicyState {}
unsafe impl ::core::marker::Sync for MediaPlaybackSessionOutputDegradationPolicyState {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlaybackSphericalVideoProjection(::windows::core::IUnknown);
impl MediaPlaybackSphericalVideoProjection {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn FrameFormat(&self) -> ::windows::core::Result<super::MediaProperties::SphericalVideoFrameFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SetFrameFormat(&self, value: super::MediaProperties::SphericalVideoFrameFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFrameFormat)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn HorizontalFieldOfViewInDegrees(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HorizontalFieldOfViewInDegrees)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHorizontalFieldOfViewInDegrees(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHorizontalFieldOfViewInDegrees)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ViewOrientation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Quaternion> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ViewOrientation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetViewOrientation(&self, value: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetViewOrientation)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ProjectionMode(&self) -> ::windows::core::Result<SphericalVideoProjectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProjectionMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetProjectionMode(&self, value: SphericalVideoProjectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetProjectionMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for MediaPlaybackSphericalVideoProjection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlaybackSphericalVideoProjection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlaybackSphericalVideoProjection {}
impl ::core::fmt::Debug for MediaPlaybackSphericalVideoProjection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackSphericalVideoProjection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackSphericalVideoProjection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackSphericalVideoProjection;{d405b37c-6f0e-4661-b8ee-d487ba9752d5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlaybackSphericalVideoProjection {
    type Vtable = IMediaPlaybackSphericalVideoProjection_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlaybackSphericalVideoProjection {
    const IID: ::windows::core::GUID = <IMediaPlaybackSphericalVideoProjection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlaybackSphericalVideoProjection {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackSphericalVideoProjection";
}
::windows::core::interface_hierarchy!(MediaPlaybackSphericalVideoProjection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlaybackSphericalVideoProjection {}
unsafe impl ::core::marker::Sync for MediaPlaybackSphericalVideoProjection {}
#[doc = "*Required features: `\"Media_Playback\"`, `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
#[repr(transparent)]
pub struct MediaPlaybackTimedMetadataTrackList(::windows::core::IUnknown);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl MediaPlaybackTimedMetadataTrackList {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::Core::TimedMetadataTrack>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn PresentationModeChanged(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlaybackTimedMetadataTrackList, TimedMetadataPresentationModeChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PresentationModeChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePresentationModeChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePresentationModeChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn GetPresentationMode(&self, index: u32) -> ::windows::core::Result<TimedMetadataTrackPresentationMode> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPresentationMode)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPresentationMode(&self, index: u32, value: TimedMetadataTrackPresentationMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlaybackTimedMetadataTrackList>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPresentationMode)(::windows::core::Vtable::as_raw(this), index, value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::Core::TimedMetadataTrack> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn IndexOf(&self, value: &super::Core::TimedMetadataTrack, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<super::Core::TimedMetadataTrack>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::clone::Clone for MediaPlaybackTimedMetadataTrackList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::cmp::PartialEq for MediaPlaybackTimedMetadataTrackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::cmp::Eq for MediaPlaybackTimedMetadataTrackList {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::fmt::Debug for MediaPlaybackTimedMetadataTrackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackTimedMetadataTrackList").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::windows::core::RuntimeType for MediaPlaybackTimedMetadataTrackList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackTimedMetadataTrackList;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Media.Core.TimedMetadataTrack;{9e6aed9e-f67a-49a9-b330-cf03b0e9cf07})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::windows::core::Vtable for MediaPlaybackTimedMetadataTrackList {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<super::Core::TimedMetadataTrack>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::windows::core::Interface for MediaPlaybackTimedMetadataTrackList {
    const IID: ::windows::core::GUID = <super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack> as ::windows::core::Interface>::IID;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::windows::core::RuntimeName for MediaPlaybackTimedMetadataTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackTimedMetadataTrackList";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::iter::IntoIterator for MediaPlaybackTimedMetadataTrackList {
    type Item = super::Core::TimedMetadataTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::iter::IntoIterator for &MediaPlaybackTimedMetadataTrackList {
    type Item = super::Core::TimedMetadataTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
::windows::core::interface_hierarchy!(MediaPlaybackTimedMetadataTrackList, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<MediaPlaybackTimedMetadataTrackList> for super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackTimedMetadataTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackTimedMetadataTrackList> for super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackTimedMetadataTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackTimedMetadataTrackList> for ::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataTrack>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackTimedMetadataTrackList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<MediaPlaybackTimedMetadataTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackTimedMetadataTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackTimedMetadataTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackTimedMetadataTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackTimedMetadataTrackList> for ::windows::core::InParam<super::super::Foundation::Collections::IVectorView<super::Core::TimedMetadataTrack>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackTimedMetadataTrackList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::core::marker::Send for MediaPlaybackTimedMetadataTrackList {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::core::marker::Sync for MediaPlaybackTimedMetadataTrackList {}
#[doc = "*Required features: `\"Media_Playback\"`, `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
#[repr(transparent)]
pub struct MediaPlaybackVideoTrackList(::windows::core::IUnknown);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl MediaPlaybackVideoTrackList {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::Core::VideoTrack>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::Core::VideoTrack>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn SelectedIndexChanged(&self, handler: &super::super::Foundation::TypedEventHandler<super::Core::ISingleSelectMediaTrackList, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectedIndexChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Core"))]
    pub fn RemoveSelectedIndexChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSelectedIndexChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSelectedIndex)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn SelectedIndex(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::Core::ISingleSelectMediaTrackList>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SelectedIndex)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<super::Core::VideoTrack> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn IndexOf(&self, value: &super::Core::VideoTrack, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<super::Core::VideoTrack>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::clone::Clone for MediaPlaybackVideoTrackList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::cmp::PartialEq for MediaPlaybackVideoTrackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::cmp::Eq for MediaPlaybackVideoTrackList {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::fmt::Debug for MediaPlaybackVideoTrackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackVideoTrackList").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::windows::core::RuntimeType for MediaPlaybackVideoTrackList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlaybackVideoTrackList;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Media.Core.VideoTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::windows::core::Vtable for MediaPlaybackVideoTrackList {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<super::Core::VideoTrack>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::windows::core::Interface for MediaPlaybackVideoTrackList {
    const IID: ::windows::core::GUID = <super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack> as ::windows::core::Interface>::IID;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::windows::core::RuntimeName for MediaPlaybackVideoTrackList {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlaybackVideoTrackList";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::iter::IntoIterator for MediaPlaybackVideoTrackList {
    type Item = super::Core::VideoTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::iter::IntoIterator for &MediaPlaybackVideoTrackList {
    type Item = super::Core::VideoTrack;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
::windows::core::interface_hierarchy!(MediaPlaybackVideoTrackList, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<MediaPlaybackVideoTrackList> for super::super::Foundation::Collections::IIterable<super::Core::VideoTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackVideoTrackList> for super::super::Foundation::Collections::IIterable<super::Core::VideoTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackVideoTrackList> for ::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::Core::VideoTrack>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<MediaPlaybackVideoTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackVideoTrackList> for super::Core::ISingleSelectMediaTrackList {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackVideoTrackList> for ::windows::core::InParam<super::Core::ISingleSelectMediaTrackList> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<MediaPlaybackVideoTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackVideoTrackList> for super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
impl ::core::convert::TryFrom<&MediaPlaybackVideoTrackList> for ::windows::core::InParam<super::super::Foundation::Collections::IVectorView<super::Core::VideoTrack>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlaybackVideoTrackList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::core::marker::Send for MediaPlaybackVideoTrackList {}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
unsafe impl ::core::marker::Sync for MediaPlaybackVideoTrackList {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlayer(::windows::core::IUnknown);
impl MediaPlayer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MediaPlayer, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn AutoPlay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoPlay)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAutoPlay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAutoPlay)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn NaturalDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NaturalDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetPosition(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPosition)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn BufferingProgress(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferingProgress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CurrentState(&self) -> ::windows::core::Result<MediaPlayerState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CanSeek(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanSeek)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn CanPause(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanPause)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsLoopingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsLoopingEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsLoopingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsLoopingEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsProtected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsProtected)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsMuted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMuted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsMuted(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsMuted)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PlaybackRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetPlaybackRate(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPlaybackRate)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Volume(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Volume)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetVolume(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVolume)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn PlaybackMediaMarkers(&self) -> ::windows::core::Result<PlaybackMediaMarkerSequence> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackMediaMarkers)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MediaOpened(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaOpened)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaOpened(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMediaOpened)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MediaEnded(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaEnded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMediaEnded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MediaFailed(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerFailedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaFailed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMediaFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMediaFailed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn CurrentStateChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CurrentStateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveCurrentStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCurrentStateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn PlaybackMediaMarkerReached(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, PlaybackMediaMarkerReachedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackMediaMarkerReached)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemovePlaybackMediaMarkerReached(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePlaybackMediaMarkerReached)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn MediaPlayerRateChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, MediaPlayerRateChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaPlayerRateChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveMediaPlayerRateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMediaPlayerRateChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VolumeChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VolumeChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVolumeChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveVolumeChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SeekCompleted(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SeekCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveSeekCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSeekCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn BufferingStarted(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferingStarted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveBufferingStarted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveBufferingStarted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn BufferingEnded(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BufferingEnded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn RemoveBufferingEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveBufferingEnded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn Play(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Play)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Pause)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn SetUriSource(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUriSource)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SystemMediaTransportControls(&self) -> ::windows::core::Result<super::SystemMediaTransportControls> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SystemMediaTransportControls)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AudioCategory(&self) -> ::windows::core::Result<MediaPlayerAudioCategory> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioCategory)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAudioCategory(&self, value: MediaPlayerAudioCategory) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAudioCategory)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AudioDeviceType(&self) -> ::windows::core::Result<MediaPlayerAudioDeviceType> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioDeviceType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAudioDeviceType(&self, value: MediaPlayerAudioDeviceType) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAudioDeviceType)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsMutedChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMutedChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsMutedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveIsMutedChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceChanged(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSourceChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn AudioBalance(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioBalance)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAudioBalance(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAudioBalance)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn RealTimePlayback(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RealTimePlayback)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRealTimePlayback(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetRealTimePlayback)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn StereoscopicVideoRenderMode(&self) -> ::windows::core::Result<StereoscopicVideoRenderMode> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StereoscopicVideoRenderMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetStereoscopicVideoRenderMode(&self, value: StereoscopicVideoRenderMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStereoscopicVideoRenderMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BreakManager(&self) -> ::windows::core::Result<MediaBreakManager> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BreakManager)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CommandManager(&self) -> ::windows::core::Result<MediaPlaybackCommandManager> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CommandManager)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn AudioDevice(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioDevice)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn SetAudioDevice(&self, value: &super::super::Devices::Enumeration::DeviceInformation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetAudioDevice)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TimelineController(&self) -> ::windows::core::Result<super::MediaTimelineController> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimelineController)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTimelineController(&self, value: &super::MediaTimelineController) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetTimelineController)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimelineControllerPositionOffset(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TimelineControllerPositionOffset)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTimelineControllerPositionOffset(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetTimelineControllerPositionOffset)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PlaybackSession(&self) -> ::windows::core::Result<MediaPlaybackSession> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackSession)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StepForwardOneFrame(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StepForwardOneFrame)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn StepBackwardOneFrame(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).StepBackwardOneFrame)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Casting\"`*"]
    #[cfg(feature = "Media_Casting")]
    pub fn GetAsCastingSource(&self) -> ::windows::core::Result<super::Casting::CastingSource> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAsCastingSource)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSurfaceSize(&self, size: super::super::Foundation::Size) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSurfaceSize)(::windows::core::Vtable::as_raw(this), size).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn GetSurface(&self, compositor: &super::super::UI::Composition::Compositor) -> ::windows::core::Result<MediaPlayerSurface> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSurface)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(compositor), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VideoFrameAvailable(&self, value: &super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VideoFrameAvailable)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVideoFrameAvailable(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveVideoFrameAvailable)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn IsVideoFrameServerEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVideoFrameServerEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsVideoFrameServerEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsVideoFrameServerEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CopyFrameToVideoSurface<P0, E0>(&self, destination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).CopyFrameToVideoSurface)(::windows::core::Vtable::as_raw(this), destination.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CopyFrameToVideoSurfaceWithTargetRectangle<P0, E0>(&self, destination: P0, targetrectangle: super::super::Foundation::Rect) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).CopyFrameToVideoSurfaceWithTargetRectangle)(::windows::core::Vtable::as_raw(this), destination.try_into().map_err(|e| e.into())?.abi(), targetrectangle).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CopyFrameToStereoscopicVideoSurfaces<P0, E0, P1, E1>(&self, destinationlefteye: P0, destinationrighteye: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).CopyFrameToStereoscopicVideoSurfaces)(::windows::core::Vtable::as_raw(this), destinationlefteye.try_into().map_err(|e| e.into())?.abi(), destinationrighteye.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SubtitleFrameChanged(&self, handler: &super::super::Foundation::TypedEventHandler<MediaPlayer, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SubtitleFrameChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSubtitleFrameChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSubtitleFrameChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn RenderSubtitlesToSurface<P0, E0>(&self, destination: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenderSubtitlesToSurface)(::windows::core::Vtable::as_raw(this), destination.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn RenderSubtitlesToSurfaceWithTargetRectangle<P0, E0>(&self, destination: P0, targetrectangle: super::super::Foundation::Rect) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlayer6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RenderSubtitlesToSurfaceWithTargetRectangle)(::windows::core::Vtable::as_raw(this), destination.try_into().map_err(|e| e.into())?.abi(), targetrectangle, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Audio\"`*"]
    #[cfg(feature = "Media_Audio")]
    pub fn AudioStateMonitor(&self) -> ::windows::core::Result<super::Audio::AudioStateMonitor> {
        let this = &::windows::core::Interface::cast::<IMediaPlayer7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AudioStateMonitor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddAudioEffect<P0, E0>(&self, activatableclassid: &::windows::core::HSTRING, effectoptional: bool, configuration: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IPropertySet>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlayerEffects>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).AddAudioEffect)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(activatableclassid), effectoptional, configuration.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn RemoveAllEffects(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerEffects>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAllEffects)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddVideoEffect<P0, E0>(&self, activatableclassid: &::windows::core::HSTRING, effectoptional: bool, effectconfiguration: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IPropertySet>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlayerEffects2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).AddVideoEffect)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(activatableclassid), effectoptional, effectconfiguration.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Protection\"`*"]
    #[cfg(feature = "Media_Protection")]
    pub fn ProtectionManager(&self) -> ::windows::core::Result<super::Protection::MediaProtectionManager> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProtectionManager)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Protection\"`*"]
    #[cfg(feature = "Media_Protection")]
    pub fn SetProtectionManager(&self, value: &super::Protection::MediaProtectionManager) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetProtectionManager)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn SetFileSource<P0, E0>(&self, file: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetFileSource)(::windows::core::Vtable::as_raw(this), file.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage_Streams", feature = "deprecated"))]
    pub fn SetStreamSource<P0, E0>(&self, stream: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStreamSource)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Core\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Media_Core", feature = "deprecated"))]
    pub fn SetMediaSource<P0, E0>(&self, source: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::Core::IMediaSource>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetMediaSource)(::windows::core::Vtable::as_raw(this), source.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn Source(&self) -> ::windows::core::Result<IMediaPlaybackSource> {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Source)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSource<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IMediaPlaybackSource>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IMediaPlayerSource2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSource)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
impl ::core::clone::Clone for MediaPlayer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlayer {}
impl ::core::fmt::Debug for MediaPlayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayer;{381a83cb-6fff-499b-8d64-2885dfc1249e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlayer {
    type Vtable = IMediaPlayer_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlayer {
    const IID: ::windows::core::GUID = <IMediaPlayer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayer";
}
::windows::core::interface_hierarchy!(MediaPlayer, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaPlayer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlayer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaPlayer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlayer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaPlayer> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlayer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MediaPlayer {}
unsafe impl ::core::marker::Sync for MediaPlayer {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlayerDataReceivedEventArgs(::windows::core::IUnknown);
impl MediaPlayerDataReceivedEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlayerDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlayerDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlayerDataReceivedEventArgs {}
impl ::core::fmt::Debug for MediaPlayerDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerDataReceivedEventArgs;{c75a9405-c801-412a-835b-83fc0e622a8e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlayerDataReceivedEventArgs {
    type Vtable = IMediaPlayerDataReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlayerDataReceivedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlayerDataReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlayerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerDataReceivedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlayerDataReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlayerDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlayerDataReceivedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlayerFailedEventArgs(::windows::core::IUnknown);
impl MediaPlayerFailedEventArgs {
    pub fn Error(&self) -> ::windows::core::Result<MediaPlayerError> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Error)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlayerFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlayerFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlayerFailedEventArgs {}
impl ::core::fmt::Debug for MediaPlayerFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerFailedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerFailedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerFailedEventArgs;{2744e9b9-a7e3-4f16-bac4-7914ebc08301})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlayerFailedEventArgs {
    type Vtable = IMediaPlayerFailedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlayerFailedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlayerFailedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlayerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerFailedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlayerFailedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlayerFailedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlayerFailedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlayerRateChangedEventArgs(::windows::core::IUnknown);
impl MediaPlayerRateChangedEventArgs {
    pub fn NewRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewRate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlayerRateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlayerRateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlayerRateChangedEventArgs {}
impl ::core::fmt::Debug for MediaPlayerRateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerRateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerRateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerRateChangedEventArgs;{40600d58-3b61-4bb2-989f-fc65608b6cab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlayerRateChangedEventArgs {
    type Vtable = IMediaPlayerRateChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlayerRateChangedEventArgs {
    const IID: ::windows::core::GUID = <IMediaPlayerRateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlayerRateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerRateChangedEventArgs";
}
::windows::core::interface_hierarchy!(MediaPlayerRateChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for MediaPlayerRateChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaPlayerRateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct MediaPlayerSurface(::windows::core::IUnknown);
impl MediaPlayerSurface {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn CompositionSurface(&self) -> ::windows::core::Result<super::super::UI::Composition::ICompositionSurface> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompositionSurface)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Composition\"`*"]
    #[cfg(feature = "UI_Composition")]
    pub fn Compositor(&self) -> ::windows::core::Result<super::super::UI::Composition::Compositor> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Compositor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MediaPlayer(&self) -> ::windows::core::Result<MediaPlayer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaPlayer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for MediaPlayerSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MediaPlayerSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaPlayerSurface {}
impl ::core::fmt::Debug for MediaPlayerSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerSurface {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.MediaPlayerSurface;{0ed653bc-b736-49c3-830b-764a3845313a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MediaPlayerSurface {
    type Vtable = IMediaPlayerSurface_Vtbl;
}
unsafe impl ::windows::core::Interface for MediaPlayerSurface {
    const IID: ::windows::core::GUID = <IMediaPlayerSurface as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MediaPlayerSurface {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerSurface";
}
::windows::core::interface_hierarchy!(MediaPlayerSurface, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MediaPlayerSurface> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MediaPlayerSurface) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaPlayerSurface> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlayerSurface) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MediaPlayerSurface> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MediaPlayerSurface) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MediaPlayerSurface {}
unsafe impl ::core::marker::Sync for MediaPlayerSurface {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct PlaybackMediaMarker(::windows::core::IUnknown);
impl PlaybackMediaMarker {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Time(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Time)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MediaMarkerType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MediaMarkerType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromTime(value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<PlaybackMediaMarker> {
        Self::IPlaybackMediaMarkerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFromTime)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create(value: super::super::Foundation::TimeSpan, mediamarkettype: &::windows::core::HSTRING, text: &::windows::core::HSTRING) -> ::windows::core::Result<PlaybackMediaMarker> {
        Self::IPlaybackMediaMarkerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), value, ::core::mem::transmute_copy(mediamarkettype), ::core::mem::transmute_copy(text), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPlaybackMediaMarkerFactory<R, F: FnOnce(&IPlaybackMediaMarkerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PlaybackMediaMarker, IPlaybackMediaMarkerFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PlaybackMediaMarker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackMediaMarker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackMediaMarker {}
impl ::core::fmt::Debug for PlaybackMediaMarker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackMediaMarker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackMediaMarker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.PlaybackMediaMarker;{c4d22f5c-3c1c-4444-b6b9-778b0422d41a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlaybackMediaMarker {
    type Vtable = IPlaybackMediaMarker_Vtbl;
}
unsafe impl ::windows::core::Interface for PlaybackMediaMarker {
    const IID: ::windows::core::GUID = <IPlaybackMediaMarker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlaybackMediaMarker {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarker";
}
::windows::core::interface_hierarchy!(PlaybackMediaMarker, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PlaybackMediaMarker {}
unsafe impl ::core::marker::Sync for PlaybackMediaMarker {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct PlaybackMediaMarkerReachedEventArgs(::windows::core::IUnknown);
impl PlaybackMediaMarkerReachedEventArgs {
    pub fn PlaybackMediaMarker(&self) -> ::windows::core::Result<PlaybackMediaMarker> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PlaybackMediaMarker)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PlaybackMediaMarkerReachedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackMediaMarkerReachedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackMediaMarkerReachedEventArgs {}
impl ::core::fmt::Debug for PlaybackMediaMarkerReachedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackMediaMarkerReachedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackMediaMarkerReachedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.PlaybackMediaMarkerReachedEventArgs;{578cd1b9-90e2-4e60-abc4-8740b01f6196})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlaybackMediaMarkerReachedEventArgs {
    type Vtable = IPlaybackMediaMarkerReachedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PlaybackMediaMarkerReachedEventArgs {
    const IID: ::windows::core::GUID = <IPlaybackMediaMarkerReachedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlaybackMediaMarkerReachedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarkerReachedEventArgs";
}
::windows::core::interface_hierarchy!(PlaybackMediaMarkerReachedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PlaybackMediaMarkerReachedEventArgs {}
unsafe impl ::core::marker::Sync for PlaybackMediaMarkerReachedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct PlaybackMediaMarkerSequence(::windows::core::IUnknown);
impl PlaybackMediaMarkerSequence {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<PlaybackMediaMarker>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<PlaybackMediaMarker>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Insert(&self, value: &PlaybackMediaMarker) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Insert)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PlaybackMediaMarkerSequence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PlaybackMediaMarkerSequence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PlaybackMediaMarkerSequence {}
impl ::core::fmt::Debug for PlaybackMediaMarkerSequence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PlaybackMediaMarkerSequence").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PlaybackMediaMarkerSequence {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.PlaybackMediaMarkerSequence;{f2810cee-638b-46cf-8817-1d111fe9d8c4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PlaybackMediaMarkerSequence {
    type Vtable = IPlaybackMediaMarkerSequence_Vtbl;
}
unsafe impl ::windows::core::Interface for PlaybackMediaMarkerSequence {
    const IID: ::windows::core::GUID = <IPlaybackMediaMarkerSequence as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PlaybackMediaMarkerSequence {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarkerSequence";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PlaybackMediaMarkerSequence {
    type Item = PlaybackMediaMarker;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PlaybackMediaMarkerSequence {
    type Item = PlaybackMediaMarker;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows::core::interface_hierarchy!(PlaybackMediaMarkerSequence, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PlaybackMediaMarkerSequence> for super::super::Foundation::Collections::IIterable<PlaybackMediaMarker> {
    type Error = ::windows::core::Error;
    fn try_from(value: PlaybackMediaMarkerSequence) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlaybackMediaMarkerSequence> for super::super::Foundation::Collections::IIterable<PlaybackMediaMarker> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlaybackMediaMarkerSequence) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PlaybackMediaMarkerSequence> for ::windows::core::InParam<super::super::Foundation::Collections::IIterable<PlaybackMediaMarker>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PlaybackMediaMarkerSequence) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for PlaybackMediaMarkerSequence {}
unsafe impl ::core::marker::Sync for PlaybackMediaMarkerSequence {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
pub struct TimedMetadataPresentationModeChangedEventArgs(::windows::core::IUnknown);
impl TimedMetadataPresentationModeChangedEventArgs {
    #[doc = "*Required features: `\"Media_Core\"`*"]
    #[cfg(feature = "Media_Core")]
    pub fn Track(&self) -> ::windows::core::Result<super::Core::TimedMetadataTrack> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Track)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OldPresentationMode(&self) -> ::windows::core::Result<TimedMetadataTrackPresentationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OldPresentationMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NewPresentationMode(&self) -> ::windows::core::Result<TimedMetadataTrackPresentationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewPresentationMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for TimedMetadataPresentationModeChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimedMetadataPresentationModeChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataPresentationModeChangedEventArgs {}
impl ::core::fmt::Debug for TimedMetadataPresentationModeChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataPresentationModeChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedMetadataPresentationModeChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Playback.TimedMetadataPresentationModeChangedEventArgs;{d1636099-65df-45ae-8cef-dc0b53fdc2bb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for TimedMetadataPresentationModeChangedEventArgs {
    type Vtable = ITimedMetadataPresentationModeChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for TimedMetadataPresentationModeChangedEventArgs {
    const IID: ::windows::core::GUID = <ITimedMetadataPresentationModeChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for TimedMetadataPresentationModeChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.TimedMetadataPresentationModeChangedEventArgs";
}
::windows::core::interface_hierarchy!(TimedMetadataPresentationModeChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TimedMetadataPresentationModeChangedEventArgs {}
unsafe impl ::core::marker::Sync for TimedMetadataPresentationModeChangedEventArgs {}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutoLoadedDisplayPropertyKind(pub i32);
impl AutoLoadedDisplayPropertyKind {
    pub const None: Self = Self(0i32);
    pub const MusicOrVideo: Self = Self(1i32);
    pub const Music: Self = Self(2i32);
    pub const Video: Self = Self(3i32);
}
impl ::core::marker::Copy for AutoLoadedDisplayPropertyKind {}
impl ::core::clone::Clone for AutoLoadedDisplayPropertyKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoLoadedDisplayPropertyKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AutoLoadedDisplayPropertyKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for AutoLoadedDisplayPropertyKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoLoadedDisplayPropertyKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutoLoadedDisplayPropertyKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.AutoLoadedDisplayPropertyKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FailedMediaStreamKind(pub i32);
impl FailedMediaStreamKind {
    pub const Unknown: Self = Self(0i32);
    pub const Audio: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for FailedMediaStreamKind {}
impl ::core::clone::Clone for FailedMediaStreamKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FailedMediaStreamKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FailedMediaStreamKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for FailedMediaStreamKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FailedMediaStreamKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FailedMediaStreamKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.FailedMediaStreamKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaBreakInsertionMethod(pub i32);
impl MediaBreakInsertionMethod {
    pub const Interrupt: Self = Self(0i32);
    pub const Replace: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaBreakInsertionMethod {}
impl ::core::clone::Clone for MediaBreakInsertionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaBreakInsertionMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaBreakInsertionMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaBreakInsertionMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBreakInsertionMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaBreakInsertionMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaBreakInsertionMethod;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaCommandEnablingRule(pub i32);
impl MediaCommandEnablingRule {
    pub const Auto: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
    pub const Never: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaCommandEnablingRule {}
impl ::core::clone::Clone for MediaCommandEnablingRule {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaCommandEnablingRule {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaCommandEnablingRule {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaCommandEnablingRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCommandEnablingRule").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaCommandEnablingRule {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaCommandEnablingRule;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaPlaybackItemChangedReason(pub i32);
impl MediaPlaybackItemChangedReason {
    pub const InitialItem: Self = Self(0i32);
    pub const EndOfStream: Self = Self(1i32);
    pub const Error: Self = Self(2i32);
    pub const AppRequested: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaPlaybackItemChangedReason {}
impl ::core::clone::Clone for MediaPlaybackItemChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackItemChangedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaPlaybackItemChangedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackItemChangedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemChangedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItemChangedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackItemChangedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaPlaybackItemErrorCode(pub i32);
impl MediaPlaybackItemErrorCode {
    pub const None: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const DecodeError: Self = Self(3i32);
    pub const SourceNotSupportedError: Self = Self(4i32);
    pub const EncryptionError: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaPlaybackItemErrorCode {}
impl ::core::clone::Clone for MediaPlaybackItemErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackItemErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaPlaybackItemErrorCode {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackItemErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackItemErrorCode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackItemErrorCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackItemErrorCode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaPlaybackSessionVideoConstrictionReason(pub i32);
impl MediaPlaybackSessionVideoConstrictionReason {
    pub const None: Self = Self(0i32);
    pub const VirtualMachine: Self = Self(1i32);
    pub const UnsupportedDisplayAdapter: Self = Self(2i32);
    pub const UnsignedDriver: Self = Self(3i32);
    pub const FrameServerEnabled: Self = Self(4i32);
    pub const OutputProtectionFailed: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaPlaybackSessionVideoConstrictionReason {}
impl ::core::clone::Clone for MediaPlaybackSessionVideoConstrictionReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackSessionVideoConstrictionReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaPlaybackSessionVideoConstrictionReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackSessionVideoConstrictionReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackSessionVideoConstrictionReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackSessionVideoConstrictionReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackSessionVideoConstrictionReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaPlaybackState(pub i32);
impl MediaPlaybackState {
    pub const None: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlaybackState {}
impl ::core::clone::Clone for MediaPlaybackState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlaybackState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaPlaybackState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlaybackState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlaybackState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlaybackState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlaybackState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaPlayerAudioCategory(pub i32);
impl MediaPlayerAudioCategory {
    pub const Other: Self = Self(0i32);
    pub const Communications: Self = Self(3i32);
    pub const Alerts: Self = Self(4i32);
    pub const SoundEffects: Self = Self(5i32);
    pub const GameEffects: Self = Self(6i32);
    pub const GameMedia: Self = Self(7i32);
    pub const GameChat: Self = Self(8i32);
    pub const Speech: Self = Self(9i32);
    pub const Movie: Self = Self(10i32);
    pub const Media: Self = Self(11i32);
}
impl ::core::marker::Copy for MediaPlayerAudioCategory {}
impl ::core::clone::Clone for MediaPlayerAudioCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlayerAudioCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaPlayerAudioCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlayerAudioCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerAudioCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerAudioCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerAudioCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaPlayerAudioDeviceType(pub i32);
impl MediaPlayerAudioDeviceType {
    pub const Console: Self = Self(0i32);
    pub const Multimedia: Self = Self(1i32);
    pub const Communications: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaPlayerAudioDeviceType {}
impl ::core::clone::Clone for MediaPlayerAudioDeviceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlayerAudioDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaPlayerAudioDeviceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlayerAudioDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerAudioDeviceType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerAudioDeviceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerAudioDeviceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaPlayerError(pub i32);
impl MediaPlayerError {
    pub const Unknown: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const DecodingError: Self = Self(3i32);
    pub const SourceNotSupported: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaPlayerError {}
impl ::core::clone::Clone for MediaPlayerError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaPlayerError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MediaPlayerError {
    type Abi = Self;
}
impl ::core::fmt::Debug for MediaPlayerError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MediaPlayerError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerError;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaPlayerState(pub i32);
#[cfg(feature = "deprecated")]
impl MediaPlayerState {
    pub const Closed: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Buffering: Self = Self(2i32);
    pub const Playing: Self = Self(3i32);
    pub const Paused: Self = Self(4i32);
    pub const Stopped: Self = Self(5i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for MediaPlayerState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for MediaPlayerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for MediaPlayerState {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for MediaPlayerState {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for MediaPlayerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPlayerState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for MediaPlayerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.MediaPlayerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SphericalVideoProjectionMode(pub i32);
impl SphericalVideoProjectionMode {
    pub const Spherical: Self = Self(0i32);
    pub const Flat: Self = Self(1i32);
}
impl ::core::marker::Copy for SphericalVideoProjectionMode {}
impl ::core::clone::Clone for SphericalVideoProjectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SphericalVideoProjectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SphericalVideoProjectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for SphericalVideoProjectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SphericalVideoProjectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SphericalVideoProjectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.SphericalVideoProjectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct StereoscopicVideoRenderMode(pub i32);
impl StereoscopicVideoRenderMode {
    pub const Mono: Self = Self(0i32);
    pub const Stereo: Self = Self(1i32);
}
impl ::core::marker::Copy for StereoscopicVideoRenderMode {}
impl ::core::clone::Clone for StereoscopicVideoRenderMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StereoscopicVideoRenderMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StereoscopicVideoRenderMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for StereoscopicVideoRenderMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StereoscopicVideoRenderMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StereoscopicVideoRenderMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.StereoscopicVideoRenderMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Playback\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedMetadataTrackPresentationMode(pub i32);
impl TimedMetadataTrackPresentationMode {
    pub const Disabled: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const ApplicationPresented: Self = Self(2i32);
    pub const PlatformPresented: Self = Self(3i32);
}
impl ::core::marker::Copy for TimedMetadataTrackPresentationMode {}
impl ::core::clone::Clone for TimedMetadataTrackPresentationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedMetadataTrackPresentationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TimedMetadataTrackPresentationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for TimedMetadataTrackPresentationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackPresentationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TimedMetadataTrackPresentationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Playback.TimedMetadataTrackPresentationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
