#[doc = "Register `SCIGCR1` reader"]
pub type R = crate::R<Scigcr1Spec>;
#[doc = "Register `SCIGCR1` writer"]
pub type W = crate::W<Scigcr1Spec>;
#[doc = "Field `COMMMODE` reader - 0:0\\]
SCI/LIN communication mode bit. In compatibility mode, it selects the SCI communication mode. In LIN mode it selects length control option for ID-field bits ID4 and ID5."]
pub type CommmodeR = crate::BitReader;
#[doc = "Field `COMMMODE` writer - 0:0\\]
SCI/LIN communication mode bit. In compatibility mode, it selects the SCI communication mode. In LIN mode it selects length control option for ID-field bits ID4 and ID5."]
pub type CommmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMINGMODE` reader - 1:1\\]
SCI timing mode bit. This bit is effective in SCI-compatible mode only. It must be set to 1 when the SCI mode is used. This bit configures the SCI for asynchronous operation."]
pub type TimingmodeR = crate::BitReader;
#[doc = "Field `TIMINGMODE` writer - 1:1\\]
SCI timing mode bit. This bit is effective in SCI-compatible mode only. It must be set to 1 when the SCI mode is used. This bit configures the SCI for asynchronous operation."]
pub type TimingmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITYENA` reader - 2:2\\]
Parity enable. Enables or disables the parity function."]
pub type ParityenaR = crate::BitReader;
#[doc = "Field `PARITYENA` writer - 2:2\\]
Parity enable. Enables or disables the parity function."]
pub type ParityenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY` reader - 3:3\\]
SCI parity odd/even selection. This bit is effective in SCI-compatible mode only. If the PARITY ENA bit (SCIGCR1.2) is set, PARITY designates odd or even parity. The parity bit is calculated based on the data bits in each frame and the address bit (in address-bit mode). The start and stop fields in the frame are not included in the parity calculation. This field is writable in SCI mode only."]
pub type ParityR = crate::BitReader;
#[doc = "Field `PARITY` writer - 3:3\\]
SCI parity odd/even selection. This bit is effective in SCI-compatible mode only. If the PARITY ENA bit (SCIGCR1.2) is set, PARITY designates odd or even parity. The parity bit is calculated based on the data bits in each frame and the address bit (in address-bit mode). The start and stop fields in the frame are not included in the parity calculation. This field is writable in SCI mode only."]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - 4:4\\]
SCI number of stop bits. This bit is effective in SCI-compatible mode only. Note: The receiver checks for only one stop bit. However in idle-line mode, the receiver waits until the end of the second stop bit (if STOP = 1) to begin checking for an idle period. This field is writable in SCI mode only."]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - 4:4\\]
SCI number of stop bits. This bit is effective in SCI-compatible mode only. Note: The receiver checks for only one stop bit. However in idle-line mode, the receiver waits until the end of the second stop bit (if STOP = 1) to begin checking for an idle period. This field is writable in SCI mode only."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MASTER` reader - 5:5\\]
SCI internal clock enable or LIN Master/Slave configuration. In the SCI mode, this bit enables the clock to the SCI module. In LIN mode, this bit determines whether a LIN node is a slave or master."]
pub type ClkMasterR = crate::BitReader;
#[doc = "Field `CLK_MASTER` writer - 5:5\\]
SCI internal clock enable or LIN Master/Slave configuration. In the SCI mode, this bit enables the clock to the SCI module. In LIN mode, this bit determines whether a LIN node is a slave or master."]
pub type ClkMasterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINMODE` reader - 6:6\\]
LIN mode This bit controls the mode of operation of the module. Writable Only in privilege mode"]
pub type LinmodeR = crate::BitReader;
#[doc = "Field `LINMODE` writer - 6:6\\]
LIN mode This bit controls the mode of operation of the module. Writable Only in privilege mode"]
pub type LinmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWnRST` reader - 7:7\\]
Software reset (active low). This bit is effective in LIN or SCI-compatible mode. The SCI/LIN should only be configured while SWnRST = 0. Only the following configuration bits can be changed in runtime (i.e., while SWnRESET = 1): - STOP EXT Frame (SCIGCR1\\[13\\]) - CC bit (SCIGCR2\\[17\\]) - SC bit (SCIGCR2\\[16\\])"]
pub type SwnRstR = crate::BitReader;
#[doc = "Field `SWnRST` writer - 7:7\\]
Software reset (active low). This bit is effective in LIN or SCI-compatible mode. The SCI/LIN should only be configured while SWnRST = 0. Only the following configuration bits can be changed in runtime (i.e., while SWnRESET = 1): - STOP EXT Frame (SCIGCR1\\[13\\]) - CC bit (SCIGCR2\\[17\\]) - SC bit (SCIGCR2\\[16\\])"]
pub type SwnRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - 8:8\\]
SCI sleep. SCI compatibility mode only. In a multiprocessor configuration, this bit controls the receive sleep function. Clearing this bit brings the SCI out of sleep mode. The receiver still operates when the SLEEP bit is set; however, RXRDY is updated and SCIRD is loaded with new data only when an address frame is detected. The remaining receiver status flags are updated and an error interrupt is requested if the corresponding interrupt enable bit is set, regardless of the value of the SLEEP bit. In this way, if an error is detected on the receive data line while the SCI is asleep, software can promptly deal with the error condition. The SLEEP bit is not automatically cleared when an address byte is detected. This field is writable in SCI mode only."]
pub type SleepR = crate::BitReader;
#[doc = "Field `SLEEP` writer - 8:8\\]
SCI sleep. SCI compatibility mode only. In a multiprocessor configuration, this bit controls the receive sleep function. Clearing this bit brings the SCI out of sleep mode. The receiver still operates when the SLEEP bit is set; however, RXRDY is updated and SCIRD is loaded with new data only when an address frame is detected. The remaining receiver status flags are updated and an error interrupt is requested if the corresponding interrupt enable bit is set, regardless of the value of the SLEEP bit. In this way, if an error is detected on the receive data line while the SCI is asleep, software can promptly deal with the error condition. The SLEEP bit is not automatically cleared when an address byte is detected. This field is writable in SCI mode only."]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADAPT` reader - 9:9\\]
Adapt mode enable. This mode is effective in LIN mode only. This bit has an effect during the detection of the Sync Field. There are two LIN protocol bit rate modes that could be enabled with this bit according to the Node capability file definition: automatic or select. Software and network configuration will decide which of the previous two modes. When this bit is cleared, the LIN 2.0 protocol fixed bit rate should be used. If the ADAPT bit is set, a LIN slave node detecting the baudrate will compare it to the prescalers in BRSR register and update it if they are different. The BRSR register will be updated with the new value. If this bit is not set there will be no adjustment to the BRSR register. This field is writable in LIN mode only."]
pub type AdaptR = crate::BitReader;
#[doc = "Field `ADAPT` writer - 9:9\\]
Adapt mode enable. This mode is effective in LIN mode only. This bit has an effect during the detection of the Sync Field. There are two LIN protocol bit rate modes that could be enabled with this bit according to the Node capability file definition: automatic or select. Software and network configuration will decide which of the previous two modes. When this bit is cleared, the LIN 2.0 protocol fixed bit rate should be used. If the ADAPT bit is set, a LIN slave node detecting the baudrate will compare it to the prescalers in BRSR register and update it if they are different. The BRSR register will be updated with the new value. If this bit is not set there will be no adjustment to the BRSR register. This field is writable in LIN mode only."]
pub type AdaptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBUFMODE` reader - 10:10\\]
Multibuffer mode. This bit is effective in LIN or SCI-compatible mode. This bit controls receive/transmit buffer usage, that is, whether the RX/TX multibuffers are used or a single register, RD0/TD0, is used."]
pub type MbufmodeR = crate::BitReader;
#[doc = "Field `MBUFMODE` writer - 10:10\\]
Multibuffer mode. This bit is effective in LIN or SCI-compatible mode. This bit controls receive/transmit buffer usage, that is, whether the RX/TX multibuffers are used or a single register, RD0/TD0, is used."]
pub type MbufmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTYPE` reader - 11:11\\]
Checksum type. This bit is effective in LIN mode only. This bit controls the type of checksum to be used: classic or enhanced."]
pub type CtypeR = crate::BitReader;
#[doc = "Field `CTYPE` writer - 11:11\\]
Checksum type. This bit is effective in LIN mode only. This bit controls the type of checksum to be used: classic or enhanced."]
pub type CtypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HGENCTRL` reader - 12:12\\]
HGEN control bit. This bit is effective in LIN mode only. This bit controls the type of mask filtering comparison."]
pub type HgenctrlR = crate::BitReader;
#[doc = "Field `HGENCTRL` writer - 12:12\\]
HGEN control bit. This bit is effective in LIN mode only. This bit controls the type of mask filtering comparison."]
pub type HgenctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPEXTFRAME` reader - 13:13\\]
Stop extended frame communication. This bit is effective in LIN mode only. This bit can be written only during extended frame communication. When the extended frame communication is stopped, this bit is cleared automatically."]
pub type StopextframeR = crate::BitReader;
#[doc = "Field `STOPEXTFRAME` writer - 13:13\\]
Stop extended frame communication. This bit is effective in LIN mode only. This bit can be written only during extended frame communication. When the extended frame communication is stopped, this bit is cleared automatically."]
pub type StopextframeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - 16:16\\]
Loopback bit. This bit is effective in LIN or SCI-compatible mode. The self-checking option for the SCI/LIN can be selected with this bit. If the LINTX and LINRX pins are configured with SCI/LIN functionality, then the LINTX pin is internally connected to the LINRX pin. Externally, during loop back operation, the LINTX pin outputs a high value and the LINRX pin is in a high-impedance state. If this bit value is changed while the SCI/LIN is transmitting or receiving data, errors may result."]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - 16:16\\]
Loopback bit. This bit is effective in LIN or SCI-compatible mode. The self-checking option for the SCI/LIN can be selected with this bit. If the LINTX and LINRX pins are configured with SCI/LIN functionality, then the LINTX pin is internally connected to the LINRX pin. Externally, during loop back operation, the LINTX pin outputs a high value and the LINRX pin is in a high-impedance state. If this bit value is changed while the SCI/LIN is transmitting or receiving data, errors may result."]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - 17:17\\]
Continue on suspend. This bit has an effect only when a program is being debugged with an emulator, and it determines how the SCI/LIN operates when the program is suspended. This bit affects the LIN counters. When this bit is set, the counters are not stopped during debug. When this bit is cleared, the counters are stopped during debug."]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - 17:17\\]
Continue on suspend. This bit has an effect only when a program is being debugged with an emulator, and it determines how the SCI/LIN operates when the program is suspended. This bit affects the LIN counters. When this bit is set, the counters are not stopped during debug. When this bit is cleared, the counters are stopped during debug."]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 23:18\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 23:18\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RXENA` reader - 24:24\\]
Receive enable. This bit is effective in LIN or SCI-compatible mode. RXENA allows or prevents the transfer of data from SCIRXSHF to SCIRD or the receive multibuffers. Note: Clearing RXENA stops received characters from being transferred into the receive buffer or multi-buffers, prevents the RX status flags (see Table 7) from being updated by receive data, and inhibits both receive and error interrupts. However, the shift register continues to assemble data regardless of the state of RXENA. Note: If RXENA is cleared before the time the reception of a frame is complete, the data from the frame is not transferred into the receive buffer. Note: If RXENA is set before the time the reception of a frame is complete, the data from the frame is transferred into the receive buffer. If RXENA is set while SCIRXSHF is in the process of assembling a frame, the status flags are not guaranteed to be accurate for that frame. To ensure that the status flags correctly reflect what was detected on the bus during a particular frame, RXENA should be set before the detection of that frame"]
pub type RxenaR = crate::BitReader;
#[doc = "Field `RXENA` writer - 24:24\\]
Receive enable. This bit is effective in LIN or SCI-compatible mode. RXENA allows or prevents the transfer of data from SCIRXSHF to SCIRD or the receive multibuffers. Note: Clearing RXENA stops received characters from being transferred into the receive buffer or multi-buffers, prevents the RX status flags (see Table 7) from being updated by receive data, and inhibits both receive and error interrupts. However, the shift register continues to assemble data regardless of the state of RXENA. Note: If RXENA is cleared before the time the reception of a frame is complete, the data from the frame is not transferred into the receive buffer. Note: If RXENA is set before the time the reception of a frame is complete, the data from the frame is transferred into the receive buffer. If RXENA is set while SCIRXSHF is in the process of assembling a frame, the status flags are not guaranteed to be accurate for that frame. To ensure that the status flags correctly reflect what was detected on the bus during a particular frame, RXENA should be set before the detection of that frame"]
pub type RxenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXENA` reader - 25:25\\]
Transmit enable. This bit is effective in LIN and SCI modes. Data is transferred from SCITD or the TDy (with y=0, 1,...7) buffers in LIN mode to the SCITXSHF shift out register only when the TXENA bit is set. Note: Data written to SCITD or the transmit multi-buffer before TXENA is set is not transmitted. If TXENA is cleared while transmission is ongoing, the data previously written to SCITD is sent (including the checksum byte in LIN mode)."]
pub type TxenaR = crate::BitReader;
#[doc = "Field `TXENA` writer - 25:25\\]
Transmit enable. This bit is effective in LIN and SCI modes. Data is transferred from SCITD or the TDy (with y=0, 1,...7) buffers in LIN mode to the SCITXSHF shift out register only when the TXENA bit is set. Note: Data written to SCITD or the transmit multi-buffer before TXENA is set is not transmitted. If TXENA is cleared while transmission is ongoing, the data previously written to SCITD is sent (including the checksum byte in LIN mode)."]
pub type TxenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 31:26\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 31:26\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SCI/LIN communication mode bit. In compatibility mode, it selects the SCI communication mode. In LIN mode it selects length control option for ID-field bits ID4 and ID5."]
    #[inline(always)]
    pub fn commmode(&self) -> CommmodeR {
        CommmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SCI timing mode bit. This bit is effective in SCI-compatible mode only. It must be set to 1 when the SCI mode is used. This bit configures the SCI for asynchronous operation."]
    #[inline(always)]
    pub fn timingmode(&self) -> TimingmodeR {
        TimingmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Parity enable. Enables or disables the parity function."]
    #[inline(always)]
    pub fn parityena(&self) -> ParityenaR {
        ParityenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
SCI parity odd/even selection. This bit is effective in SCI-compatible mode only. If the PARITY ENA bit (SCIGCR1.2) is set, PARITY designates odd or even parity. The parity bit is calculated based on the data bits in each frame and the address bit (in address-bit mode). The start and stop fields in the frame are not included in the parity calculation. This field is writable in SCI mode only."]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
SCI number of stop bits. This bit is effective in SCI-compatible mode only. Note: The receiver checks for only one stop bit. However in idle-line mode, the receiver waits until the end of the second stop bit (if STOP = 1) to begin checking for an idle period. This field is writable in SCI mode only."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
SCI internal clock enable or LIN Master/Slave configuration. In the SCI mode, this bit enables the clock to the SCI module. In LIN mode, this bit determines whether a LIN node is a slave or master."]
    #[inline(always)]
    pub fn clk_master(&self) -> ClkMasterR {
        ClkMasterR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
LIN mode This bit controls the mode of operation of the module. Writable Only in privilege mode"]
    #[inline(always)]
    pub fn linmode(&self) -> LinmodeR {
        LinmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software reset (active low). This bit is effective in LIN or SCI-compatible mode. The SCI/LIN should only be configured while SWnRST = 0. Only the following configuration bits can be changed in runtime (i.e., while SWnRESET = 1): - STOP EXT Frame (SCIGCR1\\[13\\]) - CC bit (SCIGCR2\\[17\\]) - SC bit (SCIGCR2\\[16\\])"]
    #[inline(always)]
    pub fn swn_rst(&self) -> SwnRstR {
        SwnRstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
SCI sleep. SCI compatibility mode only. In a multiprocessor configuration, this bit controls the receive sleep function. Clearing this bit brings the SCI out of sleep mode. The receiver still operates when the SLEEP bit is set; however, RXRDY is updated and SCIRD is loaded with new data only when an address frame is detected. The remaining receiver status flags are updated and an error interrupt is requested if the corresponding interrupt enable bit is set, regardless of the value of the SLEEP bit. In this way, if an error is detected on the receive data line while the SCI is asleep, software can promptly deal with the error condition. The SLEEP bit is not automatically cleared when an address byte is detected. This field is writable in SCI mode only."]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Adapt mode enable. This mode is effective in LIN mode only. This bit has an effect during the detection of the Sync Field. There are two LIN protocol bit rate modes that could be enabled with this bit according to the Node capability file definition: automatic or select. Software and network configuration will decide which of the previous two modes. When this bit is cleared, the LIN 2.0 protocol fixed bit rate should be used. If the ADAPT bit is set, a LIN slave node detecting the baudrate will compare it to the prescalers in BRSR register and update it if they are different. The BRSR register will be updated with the new value. If this bit is not set there will be no adjustment to the BRSR register. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn adapt(&self) -> AdaptR {
        AdaptR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Multibuffer mode. This bit is effective in LIN or SCI-compatible mode. This bit controls receive/transmit buffer usage, that is, whether the RX/TX multibuffers are used or a single register, RD0/TD0, is used."]
    #[inline(always)]
    pub fn mbufmode(&self) -> MbufmodeR {
        MbufmodeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Checksum type. This bit is effective in LIN mode only. This bit controls the type of checksum to be used: classic or enhanced."]
    #[inline(always)]
    pub fn ctype(&self) -> CtypeR {
        CtypeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
HGEN control bit. This bit is effective in LIN mode only. This bit controls the type of mask filtering comparison."]
    #[inline(always)]
    pub fn hgenctrl(&self) -> HgenctrlR {
        HgenctrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Stop extended frame communication. This bit is effective in LIN mode only. This bit can be written only during extended frame communication. When the extended frame communication is stopped, this bit is cleared automatically."]
    #[inline(always)]
    pub fn stopextframe(&self) -> StopextframeR {
        StopextframeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Loopback bit. This bit is effective in LIN or SCI-compatible mode. The self-checking option for the SCI/LIN can be selected with this bit. If the LINTX and LINRX pins are configured with SCI/LIN functionality, then the LINTX pin is internally connected to the LINRX pin. Externally, during loop back operation, the LINTX pin outputs a high value and the LINRX pin is in a high-impedance state. If this bit value is changed while the SCI/LIN is transmitting or receiving data, errors may result."]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Continue on suspend. This bit has an effect only when a program is being debugged with an emulator, and it determines how the SCI/LIN operates when the program is suspended. This bit affects the LIN counters. When this bit is set, the counters are not stopped during debug. When this bit is cleared, the counters are stopped during debug."]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Receive enable. This bit is effective in LIN or SCI-compatible mode. RXENA allows or prevents the transfer of data from SCIRXSHF to SCIRD or the receive multibuffers. Note: Clearing RXENA stops received characters from being transferred into the receive buffer or multi-buffers, prevents the RX status flags (see Table 7) from being updated by receive data, and inhibits both receive and error interrupts. However, the shift register continues to assemble data regardless of the state of RXENA. Note: If RXENA is cleared before the time the reception of a frame is complete, the data from the frame is not transferred into the receive buffer. Note: If RXENA is set before the time the reception of a frame is complete, the data from the frame is transferred into the receive buffer. If RXENA is set while SCIRXSHF is in the process of assembling a frame, the status flags are not guaranteed to be accurate for that frame. To ensure that the status flags correctly reflect what was detected on the bus during a particular frame, RXENA should be set before the detection of that frame"]
    #[inline(always)]
    pub fn rxena(&self) -> RxenaR {
        RxenaR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Transmit enable. This bit is effective in LIN and SCI modes. Data is transferred from SCITD or the TDy (with y=0, 1,...7) buffers in LIN mode to the SCITXSHF shift out register only when the TXENA bit is set. Note: Data written to SCITD or the transmit multi-buffer before TXENA is set is not transmitted. If TXENA is cleared while transmission is ongoing, the data previously written to SCITD is sent (including the checksum byte in LIN mode)."]
    #[inline(always)]
    pub fn txena(&self) -> TxenaR {
        TxenaR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SCI/LIN communication mode bit. In compatibility mode, it selects the SCI communication mode. In LIN mode it selects length control option for ID-field bits ID4 and ID5."]
    #[inline(always)]
    #[must_use]
    pub fn commmode(&mut self) -> CommmodeW<Scigcr1Spec> {
        CommmodeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SCI timing mode bit. This bit is effective in SCI-compatible mode only. It must be set to 1 when the SCI mode is used. This bit configures the SCI for asynchronous operation."]
    #[inline(always)]
    #[must_use]
    pub fn timingmode(&mut self) -> TimingmodeW<Scigcr1Spec> {
        TimingmodeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Parity enable. Enables or disables the parity function."]
    #[inline(always)]
    #[must_use]
    pub fn parityena(&mut self) -> ParityenaW<Scigcr1Spec> {
        ParityenaW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
SCI parity odd/even selection. This bit is effective in SCI-compatible mode only. If the PARITY ENA bit (SCIGCR1.2) is set, PARITY designates odd or even parity. The parity bit is calculated based on the data bits in each frame and the address bit (in address-bit mode). The start and stop fields in the frame are not included in the parity calculation. This field is writable in SCI mode only."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<Scigcr1Spec> {
        ParityW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
SCI number of stop bits. This bit is effective in SCI-compatible mode only. Note: The receiver checks for only one stop bit. However in idle-line mode, the receiver waits until the end of the second stop bit (if STOP = 1) to begin checking for an idle period. This field is writable in SCI mode only."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<Scigcr1Spec> {
        StopW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
SCI internal clock enable or LIN Master/Slave configuration. In the SCI mode, this bit enables the clock to the SCI module. In LIN mode, this bit determines whether a LIN node is a slave or master."]
    #[inline(always)]
    #[must_use]
    pub fn clk_master(&mut self) -> ClkMasterW<Scigcr1Spec> {
        ClkMasterW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
LIN mode This bit controls the mode of operation of the module. Writable Only in privilege mode"]
    #[inline(always)]
    #[must_use]
    pub fn linmode(&mut self) -> LinmodeW<Scigcr1Spec> {
        LinmodeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software reset (active low). This bit is effective in LIN or SCI-compatible mode. The SCI/LIN should only be configured while SWnRST = 0. Only the following configuration bits can be changed in runtime (i.e., while SWnRESET = 1): - STOP EXT Frame (SCIGCR1\\[13\\]) - CC bit (SCIGCR2\\[17\\]) - SC bit (SCIGCR2\\[16\\])"]
    #[inline(always)]
    #[must_use]
    pub fn swn_rst(&mut self) -> SwnRstW<Scigcr1Spec> {
        SwnRstW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
SCI sleep. SCI compatibility mode only. In a multiprocessor configuration, this bit controls the receive sleep function. Clearing this bit brings the SCI out of sleep mode. The receiver still operates when the SLEEP bit is set; however, RXRDY is updated and SCIRD is loaded with new data only when an address frame is detected. The remaining receiver status flags are updated and an error interrupt is requested if the corresponding interrupt enable bit is set, regardless of the value of the SLEEP bit. In this way, if an error is detected on the receive data line while the SCI is asleep, software can promptly deal with the error condition. The SLEEP bit is not automatically cleared when an address byte is detected. This field is writable in SCI mode only."]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SleepW<Scigcr1Spec> {
        SleepW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Adapt mode enable. This mode is effective in LIN mode only. This bit has an effect during the detection of the Sync Field. There are two LIN protocol bit rate modes that could be enabled with this bit according to the Node capability file definition: automatic or select. Software and network configuration will decide which of the previous two modes. When this bit is cleared, the LIN 2.0 protocol fixed bit rate should be used. If the ADAPT bit is set, a LIN slave node detecting the baudrate will compare it to the prescalers in BRSR register and update it if they are different. The BRSR register will be updated with the new value. If this bit is not set there will be no adjustment to the BRSR register. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn adapt(&mut self) -> AdaptW<Scigcr1Spec> {
        AdaptW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Multibuffer mode. This bit is effective in LIN or SCI-compatible mode. This bit controls receive/transmit buffer usage, that is, whether the RX/TX multibuffers are used or a single register, RD0/TD0, is used."]
    #[inline(always)]
    #[must_use]
    pub fn mbufmode(&mut self) -> MbufmodeW<Scigcr1Spec> {
        MbufmodeW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Checksum type. This bit is effective in LIN mode only. This bit controls the type of checksum to be used: classic or enhanced."]
    #[inline(always)]
    #[must_use]
    pub fn ctype(&mut self) -> CtypeW<Scigcr1Spec> {
        CtypeW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
HGEN control bit. This bit is effective in LIN mode only. This bit controls the type of mask filtering comparison."]
    #[inline(always)]
    #[must_use]
    pub fn hgenctrl(&mut self) -> HgenctrlW<Scigcr1Spec> {
        HgenctrlW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Stop extended frame communication. This bit is effective in LIN mode only. This bit can be written only during extended frame communication. When the extended frame communication is stopped, this bit is cleared automatically."]
    #[inline(always)]
    #[must_use]
    pub fn stopextframe(&mut self) -> StopextframeW<Scigcr1Spec> {
        StopextframeW::new(self, 13)
    }
    #[doc = "Bit 16 - 16:16\\]
Loopback bit. This bit is effective in LIN or SCI-compatible mode. The self-checking option for the SCI/LIN can be selected with this bit. If the LINTX and LINRX pins are configured with SCI/LIN functionality, then the LINTX pin is internally connected to the LINRX pin. Externally, during loop back operation, the LINTX pin outputs a high value and the LINRX pin is in a high-impedance state. If this bit value is changed while the SCI/LIN is transmitting or receiving data, errors may result."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LoopbackW<Scigcr1Spec> {
        LoopbackW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Continue on suspend. This bit has an effect only when a program is being debugged with an emulator, and it determines how the SCI/LIN operates when the program is suspended. This bit affects the LIN counters. When this bit is set, the counters are not stopped during debug. When this bit is cleared, the counters are stopped during debug."]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> ContW<Scigcr1Spec> {
        ContW::new(self, 17)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scigcr1Spec> {
        Reserved1W::new(self, 18)
    }
    #[doc = "Bit 24 - 24:24\\]
Receive enable. This bit is effective in LIN or SCI-compatible mode. RXENA allows or prevents the transfer of data from SCIRXSHF to SCIRD or the receive multibuffers. Note: Clearing RXENA stops received characters from being transferred into the receive buffer or multi-buffers, prevents the RX status flags (see Table 7) from being updated by receive data, and inhibits both receive and error interrupts. However, the shift register continues to assemble data regardless of the state of RXENA. Note: If RXENA is cleared before the time the reception of a frame is complete, the data from the frame is not transferred into the receive buffer. Note: If RXENA is set before the time the reception of a frame is complete, the data from the frame is transferred into the receive buffer. If RXENA is set while SCIRXSHF is in the process of assembling a frame, the status flags are not guaranteed to be accurate for that frame. To ensure that the status flags correctly reflect what was detected on the bus during a particular frame, RXENA should be set before the detection of that frame"]
    #[inline(always)]
    #[must_use]
    pub fn rxena(&mut self) -> RxenaW<Scigcr1Spec> {
        RxenaW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Transmit enable. This bit is effective in LIN and SCI modes. Data is transferred from SCITD or the TDy (with y=0, 1,...7) buffers in LIN mode to the SCITXSHF shift out register only when the TXENA bit is set. Note: Data written to SCITD or the transmit multi-buffer before TXENA is set is not transmitted. If TXENA is cleared while transmission is ongoing, the data previously written to SCITD is sent (including the checksum byte in LIN mode)."]
    #[inline(always)]
    #[must_use]
    pub fn txena(&mut self) -> TxenaW<Scigcr1Spec> {
        TxenaW::new(self, 25)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Scigcr1Spec> {
        Reserved2W::new(self, 26)
    }
}
#[doc = "The SCIGCR1 register defines the frame format, protocol, and communication mode used by the SCI.\n\nYou can [`read`](crate::Reg::read) this register and get [`scigcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scigcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scigcr1Spec;
impl crate::RegisterSpec for Scigcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scigcr1::R`](R) reader structure"]
impl crate::Readable for Scigcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`scigcr1::W`](W) writer structure"]
impl crate::Writable for Scigcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIGCR1 to value 0"]
impl crate::Resettable for Scigcr1Spec {
    const RESET_VALUE: u32 = 0;
}
