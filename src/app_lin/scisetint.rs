#[doc = "Register `SCISETINT` reader"]
pub type R = crate::R<ScisetintSpec>;
#[doc = "Register `SCISETINT` writer"]
pub type W = crate::W<ScisetintSpec>;
#[doc = "Field `SETBRKDTINT` reader - 0:0\\]
Set break-detect interrupt. This bit is effective in SCI-compatible mode only. Setting this bit enables the SCI/LIN to generate an interrupt if a break condition is detected on the LINRX pin. This field is writable in SCI mode only."]
pub type SetbrkdtintR = crate::BitReader;
#[doc = "Field `SETBRKDTINT` writer - 0:0\\]
Set break-detect interrupt. This bit is effective in SCI-compatible mode only. Setting this bit enables the SCI/LIN to generate an interrupt if a break condition is detected on the LINRX pin. This field is writable in SCI mode only."]
pub type SetbrkdtintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETWAKEUPINT` reader - 1:1\\]
Set wake-up interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN to generate a wake-up interrupt and thereby exit low-power mode. The wake-up interrupt is asserted on falling edge of the wake-up pulse. If enabled, the wake-up interrupt is asserted when local low-power mode is requested while the receiver is busy or if a low level is detected on the SCIRX pin during low-power mode. Wake-up interrupt is not asserted upon a wakeup pulse if the module is not in power down mode."]
pub type SetwakeupintR = crate::BitReader;
#[doc = "Field `SETWAKEUPINT` writer - 1:1\\]
Set wake-up interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN to generate a wake-up interrupt and thereby exit low-power mode. The wake-up interrupt is asserted on falling edge of the wake-up pulse. If enabled, the wake-up interrupt is asserted when local low-power mode is requested while the receiver is busy or if a low level is detected on the SCIRX pin during low-power mode. Wake-up interrupt is not asserted upon a wakeup pulse if the module is not in power down mode."]
pub type SetwakeupintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTIMEOUTINT` reader - 4:4\\]
Set timeout interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when no LIN bus activity (bus idle) occurs for at least 4 seconds. This field is writable in LIN mode only."]
pub type SettimeoutintR = crate::BitReader;
#[doc = "Field `SETTIMEOUTINT` writer - 4:4\\]
Set timeout interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when no LIN bus activity (bus idle) occurs for at least 4 seconds. This field is writable in LIN mode only."]
pub type SettimeoutintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 5:5\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - 5:5\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTOAWUSINT` reader - 6:6\\]
Set Timeout After Wakeup Signal interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when there is a timeout after one wakeup signal has been sent. This field is writable in LIN mode only."]
pub type SettoawusintR = crate::BitReader;
#[doc = "Field `SETTOAWUSINT` writer - 6:6\\]
Set Timeout After Wakeup Signal interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when there is a timeout after one wakeup signal has been sent. This field is writable in LIN mode only."]
pub type SettoawusintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTOA3WUSINT` reader - 7:7\\]
Set Timeout After 3 Wakeup Signals interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when there is a timeout after 3 wakeup signals have been sent. This field is writable in LIN mode only."]
pub type Settoa3wusintR = crate::BitReader;
#[doc = "Field `SETTOA3WUSINT` writer - 7:7\\]
Set Timeout After 3 Wakeup Signals interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when there is a timeout after 3 wakeup signals have been sent. This field is writable in LIN mode only."]
pub type Settoa3wusintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXINT` reader - 8:8\\]
Set Transmitter interrupt. Setting this bit enables the SCI/LIN to generate a transmit interrupt as data is being transferred from SCITD to SCITXSHF and the TXRDY bit is being set."]
pub type SettxintR = crate::BitReader;
#[doc = "Field `SETTXINT` writer - 8:8\\]
Set Transmitter interrupt. Setting this bit enables the SCI/LIN to generate a transmit interrupt as data is being transferred from SCITD to SCITXSHF and the TXRDY bit is being set."]
pub type SettxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRXINT` reader - 9:9\\]
Set Receiver interrupt. Setting this bit enables the SCI/LIN to generate a receive interrupt after a frame has been completely received and the data is being transferred from SCIRXSHF to SCIRD."]
pub type SetrxintR = crate::BitReader;
#[doc = "Field `SETRXINT` writer - 9:9\\]
Set Receiver interrupt. Setting this bit enables the SCI/LIN to generate a receive interrupt after a frame has been completely received and the data is being transferred from SCIRXSHF to SCIRD."]
pub type SetrxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 12:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 12:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SETIDINT` reader - 13:13\\]
Set Identification interrupt. This bit is effective in LIN mode only. This bit is set to enable interrupt once a valid matching identifier is received."]
pub type SetidintR = crate::BitReader;
#[doc = "Field `SETIDINT` writer - 13:13\\]
Set Identification interrupt. This bit is effective in LIN mode only. This bit is set to enable interrupt once a valid matching identifier is received."]
pub type SetidintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 15:14\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 15:14\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SET_TX_DMA` reader - 16:16\\]
Set transmit DMA. This bit is effective in LIN or SCI-compatible mode. To enable DMA requests for the transmitter, this bit must be set. If it is cleared, interrupt requests are generated depending on SETTXINT."]
pub type SetTxDmaR = crate::BitReader;
#[doc = "Field `SET_TX_DMA` writer - 16:16\\]
Set transmit DMA. This bit is effective in LIN or SCI-compatible mode. To enable DMA requests for the transmitter, this bit must be set. If it is cleared, interrupt requests are generated depending on SETTXINT."]
pub type SetTxDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_RX_DMA` reader - 17:17\\]
Set receiver DMA. This bit is effective in LIN or SCI-compatible mode. To enable DMA requests for the receiver this bit must be set. If it is cleared, interrupt requests are generated depending on SETRXINT."]
pub type SetRxDmaR = crate::BitReader;
#[doc = "Field `SET_RX_DMA` writer - 17:17\\]
Set receiver DMA. This bit is effective in LIN or SCI-compatible mode. To enable DMA requests for the receiver this bit must be set. If it is cleared, interrupt requests are generated depending on SETRXINT."]
pub type SetRxDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_RX_DMA_ALL` reader - 18:18\\]
Set receiver DMA for Address &amp; Data frames. This bit is effective in LIN or SCI-compatible mode. To enable RX DMA request for address and data frames this bit must be set. If it is cleared, RX interrupt request is generated for address frames and DMA requests are generated for data frames."]
pub type SetRxDmaAllR = crate::BitReader;
#[doc = "Field `SET_RX_DMA_ALL` writer - 18:18\\]
Set receiver DMA for Address &amp; Data frames. This bit is effective in LIN or SCI-compatible mode. To enable RX DMA request for address and data frames this bit must be set. If it is cleared, RX interrupt request is generated for address frames and DMA requests are generated for data frames."]
pub type SetRxDmaAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - 23:19\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - 23:19\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SETPEINT` reader - 24:24\\]
Set parity interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when a parity error occurs."]
pub type SetpeintR = crate::BitReader;
#[doc = "Field `SETPEINT` writer - 24:24\\]
Set parity interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when a parity error occurs."]
pub type SetpeintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETOEINT` reader - 25:25\\]
Set overrun-error interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when an overrun error occurs."]
pub type SetoeintR = crate::BitReader;
#[doc = "Field `SETOEINT` writer - 25:25\\]
Set overrun-error interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when an overrun error occurs."]
pub type SetoeintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETFEINT` reader - 26:26\\]
Set framing-error interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when a framing error occurs."]
pub type SetfeintR = crate::BitReader;
#[doc = "Field `SETFEINT` writer - 26:26\\]
Set framing-error interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when a framing error occurs."]
pub type SetfeintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETNREINT` reader - 27:27\\]
Set no-response-error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when a no-response error occurs. This field is writable in LIN mode only."]
pub type SetnreintR = crate::BitReader;
#[doc = "Field `SETNREINT` writer - 27:27\\]
Set no-response-error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when a no-response error occurs. This field is writable in LIN mode only."]
pub type SetnreintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETISFEINT` reader - 28:28\\]
Set inconsistent-sync-field-error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is an inconsistent sync field error. This field is writable in LIN mode only."]
pub type SetisfeintR = crate::BitReader;
#[doc = "Field `SETISFEINT` writer - 28:28\\]
Set inconsistent-sync-field-error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is an inconsistent sync field error. This field is writable in LIN mode only."]
pub type SetisfeintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETCEINT` reader - 29:29\\]
Set checksum-error Interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is a checksum error. This field is writable in LIN mode only."]
pub type SetceintR = crate::BitReader;
#[doc = "Field `SETCEINT` writer - 29:29\\]
Set checksum-error Interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is a checksum error. This field is writable in LIN mode only."]
pub type SetceintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPBEINT` reader - 30:30\\]
Set physical bus error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when a physical bus error occurs. This field is writable in LIN mode only."]
pub type SetpbeintR = crate::BitReader;
#[doc = "Field `SETPBEINT` writer - 30:30\\]
Set physical bus error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when a physical bus error occurs. This field is writable in LIN mode only."]
pub type SetpbeintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETBEINT` reader - 31:31\\]
Set bit error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is a bit error. This field is writable in LIN mode only."]
pub type SetbeintR = crate::BitReader;
#[doc = "Field `SETBEINT` writer - 31:31\\]
Set bit error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is a bit error. This field is writable in LIN mode only."]
pub type SetbeintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Set break-detect interrupt. This bit is effective in SCI-compatible mode only. Setting this bit enables the SCI/LIN to generate an interrupt if a break condition is detected on the LINRX pin. This field is writable in SCI mode only."]
    #[inline(always)]
    pub fn setbrkdtint(&self) -> SetbrkdtintR {
        SetbrkdtintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set wake-up interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN to generate a wake-up interrupt and thereby exit low-power mode. The wake-up interrupt is asserted on falling edge of the wake-up pulse. If enabled, the wake-up interrupt is asserted when local low-power mode is requested while the receiver is busy or if a low level is detected on the SCIRX pin during low-power mode. Wake-up interrupt is not asserted upon a wakeup pulse if the module is not in power down mode."]
    #[inline(always)]
    pub fn setwakeupint(&self) -> SetwakeupintR {
        SetwakeupintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Set timeout interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when no LIN bus activity (bus idle) occurs for at least 4 seconds. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn settimeoutint(&self) -> SettimeoutintR {
        SettimeoutintR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Set Timeout After Wakeup Signal interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when there is a timeout after one wakeup signal has been sent. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn settoawusint(&self) -> SettoawusintR {
        SettoawusintR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Set Timeout After 3 Wakeup Signals interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when there is a timeout after 3 wakeup signals have been sent. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn settoa3wusint(&self) -> Settoa3wusintR {
        Settoa3wusintR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Set Transmitter interrupt. Setting this bit enables the SCI/LIN to generate a transmit interrupt as data is being transferred from SCITD to SCITXSHF and the TXRDY bit is being set."]
    #[inline(always)]
    pub fn settxint(&self) -> SettxintR {
        SettxintR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Set Receiver interrupt. Setting this bit enables the SCI/LIN to generate a receive interrupt after a frame has been completely received and the data is being transferred from SCIRXSHF to SCIRD."]
    #[inline(always)]
    pub fn setrxint(&self) -> SetrxintR {
        SetrxintR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Set Identification interrupt. This bit is effective in LIN mode only. This bit is set to enable interrupt once a valid matching identifier is received."]
    #[inline(always)]
    pub fn setidint(&self) -> SetidintR {
        SetidintR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Set transmit DMA. This bit is effective in LIN or SCI-compatible mode. To enable DMA requests for the transmitter, this bit must be set. If it is cleared, interrupt requests are generated depending on SETTXINT."]
    #[inline(always)]
    pub fn set_tx_dma(&self) -> SetTxDmaR {
        SetTxDmaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Set receiver DMA. This bit is effective in LIN or SCI-compatible mode. To enable DMA requests for the receiver this bit must be set. If it is cleared, interrupt requests are generated depending on SETRXINT."]
    #[inline(always)]
    pub fn set_rx_dma(&self) -> SetRxDmaR {
        SetRxDmaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Set receiver DMA for Address &amp; Data frames. This bit is effective in LIN or SCI-compatible mode. To enable RX DMA request for address and data frames this bit must be set. If it is cleared, RX interrupt request is generated for address frames and DMA requests are generated for data frames."]
    #[inline(always)]
    pub fn set_rx_dma_all(&self) -> SetRxDmaAllR {
        SetRxDmaAllR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Set parity interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when a parity error occurs."]
    #[inline(always)]
    pub fn setpeint(&self) -> SetpeintR {
        SetpeintR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Set overrun-error interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when an overrun error occurs."]
    #[inline(always)]
    pub fn setoeint(&self) -> SetoeintR {
        SetoeintR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Set framing-error interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when a framing error occurs."]
    #[inline(always)]
    pub fn setfeint(&self) -> SetfeintR {
        SetfeintR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Set no-response-error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when a no-response error occurs. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn setnreint(&self) -> SetnreintR {
        SetnreintR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Set inconsistent-sync-field-error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is an inconsistent sync field error. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn setisfeint(&self) -> SetisfeintR {
        SetisfeintR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Set checksum-error Interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is a checksum error. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn setceint(&self) -> SetceintR {
        SetceintR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Set physical bus error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when a physical bus error occurs. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn setpbeint(&self) -> SetpbeintR {
        SetpbeintR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Set bit error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is a bit error. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn setbeint(&self) -> SetbeintR {
        SetbeintR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Set break-detect interrupt. This bit is effective in SCI-compatible mode only. Setting this bit enables the SCI/LIN to generate an interrupt if a break condition is detected on the LINRX pin. This field is writable in SCI mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setbrkdtint(&mut self) -> SetbrkdtintW<ScisetintSpec> {
        SetbrkdtintW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set wake-up interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN to generate a wake-up interrupt and thereby exit low-power mode. The wake-up interrupt is asserted on falling edge of the wake-up pulse. If enabled, the wake-up interrupt is asserted when local low-power mode is requested while the receiver is busy or if a low level is detected on the SCIRX pin during low-power mode. Wake-up interrupt is not asserted upon a wakeup pulse if the module is not in power down mode."]
    #[inline(always)]
    #[must_use]
    pub fn setwakeupint(&mut self) -> SetwakeupintW<ScisetintSpec> {
        SetwakeupintW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Set timeout interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when no LIN bus activity (bus idle) occurs for at least 4 seconds. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn settimeoutint(&mut self) -> SettimeoutintW<ScisetintSpec> {
        SettimeoutintW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<ScisetintSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Set Timeout After Wakeup Signal interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when there is a timeout after one wakeup signal has been sent. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn settoawusint(&mut self) -> SettoawusintW<ScisetintSpec> {
        SettoawusintW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Set Timeout After 3 Wakeup Signals interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN to generate an interrupt when there is a timeout after 3 wakeup signals have been sent. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn settoa3wusint(&mut self) -> Settoa3wusintW<ScisetintSpec> {
        Settoa3wusintW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Set Transmitter interrupt. Setting this bit enables the SCI/LIN to generate a transmit interrupt as data is being transferred from SCITD to SCITXSHF and the TXRDY bit is being set."]
    #[inline(always)]
    #[must_use]
    pub fn settxint(&mut self) -> SettxintW<ScisetintSpec> {
        SettxintW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Set Receiver interrupt. Setting this bit enables the SCI/LIN to generate a receive interrupt after a frame has been completely received and the data is being transferred from SCIRXSHF to SCIRD."]
    #[inline(always)]
    #[must_use]
    pub fn setrxint(&mut self) -> SetrxintW<ScisetintSpec> {
        SetrxintW::new(self, 9)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<ScisetintSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 13 - 13:13\\]
Set Identification interrupt. This bit is effective in LIN mode only. This bit is set to enable interrupt once a valid matching identifier is received."]
    #[inline(always)]
    #[must_use]
    pub fn setidint(&mut self) -> SetidintW<ScisetintSpec> {
        SetidintW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<ScisetintSpec> {
        Reserved3W::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Set transmit DMA. This bit is effective in LIN or SCI-compatible mode. To enable DMA requests for the transmitter, this bit must be set. If it is cleared, interrupt requests are generated depending on SETTXINT."]
    #[inline(always)]
    #[must_use]
    pub fn set_tx_dma(&mut self) -> SetTxDmaW<ScisetintSpec> {
        SetTxDmaW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Set receiver DMA. This bit is effective in LIN or SCI-compatible mode. To enable DMA requests for the receiver this bit must be set. If it is cleared, interrupt requests are generated depending on SETRXINT."]
    #[inline(always)]
    #[must_use]
    pub fn set_rx_dma(&mut self) -> SetRxDmaW<ScisetintSpec> {
        SetRxDmaW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Set receiver DMA for Address &amp; Data frames. This bit is effective in LIN or SCI-compatible mode. To enable RX DMA request for address and data frames this bit must be set. If it is cleared, RX interrupt request is generated for address frames and DMA requests are generated for data frames."]
    #[inline(always)]
    #[must_use]
    pub fn set_rx_dma_all(&mut self) -> SetRxDmaAllW<ScisetintSpec> {
        SetRxDmaAllW::new(self, 18)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<ScisetintSpec> {
        Reserved4W::new(self, 19)
    }
    #[doc = "Bit 24 - 24:24\\]
Set parity interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when a parity error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn setpeint(&mut self) -> SetpeintW<ScisetintSpec> {
        SetpeintW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Set overrun-error interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when an overrun error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn setoeint(&mut self) -> SetoeintW<ScisetintSpec> {
        SetoeintW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Set framing-error interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit enables the SCI/LIN module to generate an interrupt when a framing error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn setfeint(&mut self) -> SetfeintW<ScisetintSpec> {
        SetfeintW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Set no-response-error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when a no-response error occurs. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setnreint(&mut self) -> SetnreintW<ScisetintSpec> {
        SetnreintW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Set inconsistent-sync-field-error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is an inconsistent sync field error. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setisfeint(&mut self) -> SetisfeintW<ScisetintSpec> {
        SetisfeintW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Set checksum-error Interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is a checksum error. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setceint(&mut self) -> SetceintW<ScisetintSpec> {
        SetceintW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Set physical bus error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when a physical bus error occurs. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setpbeint(&mut self) -> SetpbeintW<ScisetintSpec> {
        SetpbeintW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Set bit error interrupt. This bit is effective in LIN mode only. Setting this bit enables the SCI/LIN module to generate an interrupt when there is a bit error. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setbeint(&mut self) -> SetbeintW<ScisetintSpec> {
        SetbeintW::new(self, 31)
    }
}
#[doc = "The SCISETINT register is used to enable the various interrupts available in the LIN module.\n\nYou can [`read`](crate::Reg::read) this register and get [`scisetint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scisetint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScisetintSpec;
impl crate::RegisterSpec for ScisetintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scisetint::R`](R) reader structure"]
impl crate::Readable for ScisetintSpec {}
#[doc = "`write(|w| ..)` method takes [`scisetint::W`](W) writer structure"]
impl crate::Writable for ScisetintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCISETINT to value 0"]
impl crate::Resettable for ScisetintSpec {
    const RESET_VALUE: u32 = 0;
}
