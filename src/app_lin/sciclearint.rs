#[doc = "Register `SCICLEARINT` reader"]
pub type R = crate::R<SciclearintSpec>;
#[doc = "Register `SCICLEARINT` writer"]
pub type W = crate::W<SciclearintSpec>;
#[doc = "Field `CLRBRKDTINT` reader - 0:0\\]
Clear Break-detect interrupt. This bit is effective in SCI-compatible mode only. Setting this bit disables the Break-detect interrupt. This field is writable in SCI mode only."]
pub type ClrbrkdtintR = crate::BitReader;
#[doc = "Field `CLRBRKDTINT` writer - 0:0\\]
Clear Break-detect interrupt. This bit is effective in SCI-compatible mode only. Setting this bit disables the Break-detect interrupt. This field is writable in SCI mode only."]
pub type ClrbrkdtintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRWAKEUPINT` reader - 1:1\\]
Clear Wake-up interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the wake-up interrupt."]
pub type ClrwakeupintR = crate::BitReader;
#[doc = "Field `CLRWAKEUPINT` writer - 1:1\\]
Clear Wake-up interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the wake-up interrupt."]
pub type ClrwakeupintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRTIMEOUTINT` reader - 4:4\\]
Clear Timeout interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout (LIN bus idle) interrupt. This field is writable in LIN mode only."]
pub type ClrtimeoutintR = crate::BitReader;
#[doc = "Field `CLRTIMEOUTINT` writer - 4:4\\]
Clear Timeout interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout (LIN bus idle) interrupt. This field is writable in LIN mode only."]
pub type ClrtimeoutintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 5:5\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - 5:5\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRTOAWUSINT` reader - 6:6\\]
Clear Timeout After Wakeup Signal interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout after one wakeup signal interrupt. This field is writable in LIN mode only."]
pub type ClrtoawusintR = crate::BitReader;
#[doc = "Field `CLRTOAWUSINT` writer - 6:6\\]
Clear Timeout After Wakeup Signal interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout after one wakeup signal interrupt. This field is writable in LIN mode only."]
pub type ClrtoawusintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRTOA3WUSINT` reader - 7:7\\]
Clear Timeout After 3 Wakeup Signals interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout after 3 wakeup signals interrupt. This field is writable in LIN mode only."]
pub type Clrtoa3wusintR = crate::BitReader;
#[doc = "Field `CLRTOA3WUSINT` writer - 7:7\\]
Clear Timeout After 3 Wakeup Signals interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout after 3 wakeup signals interrupt. This field is writable in LIN mode only."]
pub type Clrtoa3wusintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRTXINT` reader - 8:8\\]
Clear Transmitter interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the transmitter interrupt."]
pub type ClrtxintR = crate::BitReader;
#[doc = "Field `CLRTXINT` writer - 8:8\\]
Clear Transmitter interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the transmitter interrupt."]
pub type ClrtxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRRXINT` reader - 9:9\\]
Clear Receiver interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the receiver interrupt."]
pub type ClrrxintR = crate::BitReader;
#[doc = "Field `CLRRXINT` writer - 9:9\\]
Clear Receiver interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the receiver interrupt."]
pub type ClrrxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 12:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 12:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLRIDINT` reader - 13:13\\]
Clear Identifier interrupt. This bit is effective in LIN mode only. Setting this bit disables the ID interrupt."]
pub type ClridintR = crate::BitReader;
#[doc = "Field `CLRIDINT` writer - 13:13\\]
Clear Identifier interrupt. This bit is effective in LIN mode only. Setting this bit disables the ID interrupt."]
pub type ClridintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 15:14\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 15:14\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLRTXDMA` reader - 16:16\\]
Clear transmit DMA. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the transmit DMA request."]
pub type ClrtxdmaR = crate::BitReader;
#[doc = "Field `CLRTXDMA` writer - 16:16\\]
Clear transmit DMA. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the transmit DMA request."]
pub type ClrtxdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRRXDMA` reader - 17:17\\]
Clear receiver DMA. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the receive DMA request."]
pub type ClrrxdmaR = crate::BitReader;
#[doc = "Field `CLRRXDMA` writer - 17:17\\]
Clear receiver DMA. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the receive DMA request."]
pub type ClrrxdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - 18:18\\]
Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - 18:18\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - 23:19\\]
Reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `Reserved5` writer - 23:19\\]
Reserved"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLRPEINT` reader - 24:24\\]
Clear Parity Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the parity error interrupt."]
pub type ClrpeintR = crate::BitReader;
#[doc = "Field `CLRPEINT` writer - 24:24\\]
Clear Parity Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the parity error interrupt."]
pub type ClrpeintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLROEINT` reader - 25:25\\]
Clear Overrun-Error Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the overrun interrupt."]
pub type ClroeintR = crate::BitReader;
#[doc = "Field `CLROEINT` writer - 25:25\\]
Clear Overrun-Error Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the overrun interrupt."]
pub type ClroeintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRFEINT` reader - 26:26\\]
Clear Framing-Error Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables framing-error interrupt."]
pub type ClrfeintR = crate::BitReader;
#[doc = "Field `CLRFEINT` writer - 26:26\\]
Clear Framing-Error Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables framing-error interrupt."]
pub type ClrfeintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRNREINT` reader - 27:27\\]
Clear No-Reponse-Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the no-response error interrupt. This field is writable in LIN mode only."]
pub type ClrnreintR = crate::BitReader;
#[doc = "Field `CLRNREINT` writer - 27:27\\]
Clear No-Reponse-Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the no-response error interrupt. This field is writable in LIN mode only."]
pub type ClrnreintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRISFEINT` reader - 28:28\\]
Clear Inconsistent-Sync-Field-Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the ISFE interrupt. This field is writable in LIN mode only."]
pub type ClrisfeintR = crate::BitReader;
#[doc = "Field `CLRISFEINT` writer - 28:28\\]
Clear Inconsistent-Sync-Field-Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the ISFE interrupt. This field is writable in LIN mode only."]
pub type ClrisfeintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRCEINT` reader - 29:29\\]
Clear checksum-error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the checksum-error interrupt. This field is writable in LIN mode only."]
pub type ClrceintR = crate::BitReader;
#[doc = "Field `CLRCEINT` writer - 29:29\\]
Clear checksum-error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the checksum-error interrupt. This field is writable in LIN mode only."]
pub type ClrceintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRPBEINT` reader - 30:30\\]
Clear Physical Bus Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the physical-bus error interrupt. This field is writable in LIN mode only."]
pub type ClrpbeintR = crate::BitReader;
#[doc = "Field `CLRPBEINT` writer - 30:30\\]
Clear Physical Bus Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the physical-bus error interrupt. This field is writable in LIN mode only."]
pub type ClrpbeintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRBEINT` reader - 31:31\\]
Clear Bit Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the bit error interrupt. This field is writable in LIN mode only."]
pub type ClrbeintR = crate::BitReader;
#[doc = "Field `CLRBEINT` writer - 31:31\\]
Clear Bit Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the bit error interrupt. This field is writable in LIN mode only."]
pub type ClrbeintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear Break-detect interrupt. This bit is effective in SCI-compatible mode only. Setting this bit disables the Break-detect interrupt. This field is writable in SCI mode only."]
    #[inline(always)]
    pub fn clrbrkdtint(&self) -> ClrbrkdtintR {
        ClrbrkdtintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Wake-up interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the wake-up interrupt."]
    #[inline(always)]
    pub fn clrwakeupint(&self) -> ClrwakeupintR {
        ClrwakeupintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear Timeout interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout (LIN bus idle) interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrtimeoutint(&self) -> ClrtimeoutintR {
        ClrtimeoutintR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear Timeout After Wakeup Signal interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout after one wakeup signal interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrtoawusint(&self) -> ClrtoawusintR {
        ClrtoawusintR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear Timeout After 3 Wakeup Signals interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout after 3 wakeup signals interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrtoa3wusint(&self) -> Clrtoa3wusintR {
        Clrtoa3wusintR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Transmitter interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the transmitter interrupt."]
    #[inline(always)]
    pub fn clrtxint(&self) -> ClrtxintR {
        ClrtxintR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Receiver interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the receiver interrupt."]
    #[inline(always)]
    pub fn clrrxint(&self) -> ClrrxintR {
        ClrrxintR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Clear Identifier interrupt. This bit is effective in LIN mode only. Setting this bit disables the ID interrupt."]
    #[inline(always)]
    pub fn clridint(&self) -> ClridintR {
        ClridintR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Clear transmit DMA. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the transmit DMA request."]
    #[inline(always)]
    pub fn clrtxdma(&self) -> ClrtxdmaR {
        ClrtxdmaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Clear receiver DMA. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the receive DMA request."]
    #[inline(always)]
    pub fn clrrxdma(&self) -> ClrrxdmaR {
        ClrrxdmaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear Parity Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the parity error interrupt."]
    #[inline(always)]
    pub fn clrpeint(&self) -> ClrpeintR {
        ClrpeintR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear Overrun-Error Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the overrun interrupt."]
    #[inline(always)]
    pub fn clroeint(&self) -> ClroeintR {
        ClroeintR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Clear Framing-Error Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables framing-error interrupt."]
    #[inline(always)]
    pub fn clrfeint(&self) -> ClrfeintR {
        ClrfeintR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Clear No-Reponse-Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the no-response error interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrnreint(&self) -> ClrnreintR {
        ClrnreintR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Clear Inconsistent-Sync-Field-Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the ISFE interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrisfeint(&self) -> ClrisfeintR {
        ClrisfeintR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Clear checksum-error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the checksum-error interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrceint(&self) -> ClrceintR {
        ClrceintR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Clear Physical Bus Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the physical-bus error interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrpbeint(&self) -> ClrpbeintR {
        ClrpbeintR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Clear Bit Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the bit error interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrbeint(&self) -> ClrbeintR {
        ClrbeintR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear Break-detect interrupt. This bit is effective in SCI-compatible mode only. Setting this bit disables the Break-detect interrupt. This field is writable in SCI mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrbrkdtint(&mut self) -> ClrbrkdtintW<SciclearintSpec> {
        ClrbrkdtintW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Wake-up interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn clrwakeupint(&mut self) -> ClrwakeupintW<SciclearintSpec> {
        ClrwakeupintW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear Timeout interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout (LIN bus idle) interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrtimeoutint(&mut self) -> ClrtimeoutintW<SciclearintSpec> {
        ClrtimeoutintW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SciclearintSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear Timeout After Wakeup Signal interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout after one wakeup signal interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrtoawusint(&mut self) -> ClrtoawusintW<SciclearintSpec> {
        ClrtoawusintW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear Timeout After 3 Wakeup Signals interrupt. This bit is effective in LIN mode only. Setting this bit disables the timeout after 3 wakeup signals interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrtoa3wusint(&mut self) -> Clrtoa3wusintW<SciclearintSpec> {
        Clrtoa3wusintW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Transmitter interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the transmitter interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn clrtxint(&mut self) -> ClrtxintW<SciclearintSpec> {
        ClrtxintW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Receiver interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the receiver interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn clrrxint(&mut self) -> ClrrxintW<SciclearintSpec> {
        ClrrxintW::new(self, 9)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SciclearintSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 13 - 13:13\\]
Clear Identifier interrupt. This bit is effective in LIN mode only. Setting this bit disables the ID interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn clridint(&mut self) -> ClridintW<SciclearintSpec> {
        ClridintW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SciclearintSpec> {
        Reserved3W::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Clear transmit DMA. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the transmit DMA request."]
    #[inline(always)]
    #[must_use]
    pub fn clrtxdma(&mut self) -> ClrtxdmaW<SciclearintSpec> {
        ClrtxdmaW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Clear receiver DMA. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the receive DMA request."]
    #[inline(always)]
    #[must_use]
    pub fn clrrxdma(&mut self) -> ClrrxdmaW<SciclearintSpec> {
        ClrrxdmaW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<SciclearintSpec> {
        Reserved4W::new(self, 18)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<SciclearintSpec> {
        Reserved5W::new(self, 19)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear Parity Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the parity error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn clrpeint(&mut self) -> ClrpeintW<SciclearintSpec> {
        ClrpeintW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear Overrun-Error Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables the overrun interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn clroeint(&mut self) -> ClroeintW<SciclearintSpec> {
        ClroeintW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Clear Framing-Error Interrupt. This bit is effective in LIN or SCI-compatible mode. Setting this bit disables framing-error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn clrfeint(&mut self) -> ClrfeintW<SciclearintSpec> {
        ClrfeintW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Clear No-Reponse-Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the no-response error interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrnreint(&mut self) -> ClrnreintW<SciclearintSpec> {
        ClrnreintW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Clear Inconsistent-Sync-Field-Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the ISFE interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrisfeint(&mut self) -> ClrisfeintW<SciclearintSpec> {
        ClrisfeintW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Clear checksum-error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the checksum-error interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrceint(&mut self) -> ClrceintW<SciclearintSpec> {
        ClrceintW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Clear Physical Bus Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the physical-bus error interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrpbeint(&mut self) -> ClrpbeintW<SciclearintSpec> {
        ClrpbeintW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Clear Bit Error Interrupt. This bit is effective in LIN mode only. Setting this bit disables the bit error interrupt. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrbeint(&mut self) -> ClrbeintW<SciclearintSpec> {
        ClrbeintW::new(self, 31)
    }
}
#[doc = "The SCICLEARINT register is used to disable the enabled interrupts without accessing the SCISETINT register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sciclearint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciclearint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SciclearintSpec;
impl crate::RegisterSpec for SciclearintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sciclearint::R`](R) reader structure"]
impl crate::Readable for SciclearintSpec {}
#[doc = "`write(|w| ..)` method takes [`sciclearint::W`](W) writer structure"]
impl crate::Writable for SciclearintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCICLEARINT to value 0"]
impl crate::Resettable for SciclearintSpec {
    const RESET_VALUE: u32 = 0;
}
