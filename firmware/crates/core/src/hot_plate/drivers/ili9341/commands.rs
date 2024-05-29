#[repr(u8)]
pub enum Command {
	NoOperation = 0x00,
	SoftwareReset = 0x01,
	ReadDisplayIdentificationInformation = 0x04,
	ReadDisplayStatus = 0x09,
	ReadDisplayPowerMode = 0x0A,
	ReadDisplayMADCTL = 0x0B,
	ReadDisplayPixelFormat = 0x0C,
	ReadDisplayImageFormat = 0x0D,
	ReadDisplaySignalMode = 0x0E,
	ReadDisplaySelfDiagnosticsResult = 0x0F,

	EnterSleepMode = 0x10,
	SleepOut = 0x11,
	PartialModeOn = 0x12,
	NormalDisplayModeOn = 0x13,

	DisplayInversionOff = 0x20,
	DisplayInversionOn = 0x21,
	GammaSet = 0x26,
	DisplayOff = 0x28,
	DisplayOn = 0x29,
	ColumnAddressSet = 0x2A,
	PageAddressSet = 0x2B,
	MemoryWrite = 0x2C,
	ColorSet = 0x2D,
	MemoryRead = 0x2E,

	PartialArea = 0x30,
	VerticalScrollDefinition = 0x33,
	TearingEffectLineOff = 0x34,
	TearingEffectLineOn = 0x35,
	MemoryAccessControl = 0x36,
	VerticalScrollingStartAddress = 0x37,
	IdleModeOff = 0x38,
	IdleModeOn = 0x39,
	PixelFormatSet = 0x3A,
	WriteMemoryContinue = 0x3C,
	ReadMemoryContinue = 0x3E,

	SetTearScanline = 0x44,
	GetScanline = 0x45,

	WriteDisplayBrightness = 0x51,
	ReadDisplayBrightness = 0x52,
	WriteControlDisplay = 0x53,
	ReadControlDisplay = 0x54,
	WriteContentAdaptiveBrightnessControl = 0x55,
	ReadContentAdaptiveBrightnessControl = 0x56,

	WriteCABCMinimumBrightness = 0x5E,
	ReadCABCMinimumBrightness = 0x5F,

	ReadID1 = 0xDA,
	ReadID2 = 0xDB,
	ReadID3 = 0xDC,
	ReadID4 = 0xD3,

	// ---------------------------------------------------------------
	RGBInterfaceSignalControl = 0xB0,
	FrameRateControlWhileNormal = 0xB1,
	FrameRateControlWhileIdle = 0xB2,
	FrameRateControlWhilePartial = 0xB3,
	DisplayInversionControl = 0xB4,
	BlankingPorchControl = 0xB5,
	DisplayFunctionControl = 0xB6,
	EntryModeSet = 0xB7,
	BacklightControl1 = 0xB8,
	BacklightControl2 = 0xB9,
	BacklightControl3 = 0xBA,
	BacklightControl4 = 0xBB,
	BacklightControl5 = 0xBC,
	BacklightControl7 = 0xBE,
	BacklightControl8 = 0xBF,
	// ---------------------------------------------------------------
	PowerControl1 = 0xC0,
	PowerControl2 = 0xC1,
	VCOMControl1 = 0xC5,
	VCOMControl2 = 0xC7,

	NVMemoryWrite = 0xD0,
	NVMemoryProtectionKey = 0xD1,
	NVMemoryStatusRead = 0xD2,

	// ---------------------------------------------------------------
	PositiveGammaCorrection = 0xE0,
	NegativeGammaCorrection = 0xE1,
	DigitalGammaControl = 0xE2,
	DigitalGammaControl2 = 0xE3,

	InterfaceControl = 0xF6,

	PowerControlA = 0xCB,
	PowerControlB = 0xCF,

	DriverTimingControlA = 0xE8,
	DriverTimingControlB = 0xEA,

	PowerOnSequenceControl = 0xED,
	Enable3G = 0xF2,
	PumpRatioControl = 0xF7,
}
