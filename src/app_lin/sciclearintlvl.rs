#[doc = "Register `SCICLEARINTLVL` reader"]
pub type R = crate::R<SciclearintlvlSpec>;
#[doc = "Register `SCICLEARINTLVL` writer"]
pub type W = crate::W<SciclearintlvlSpec>;
#[doc = "Field `CLRBRKDTINTLVL` reader - 0:0\\]
Clear Break-detect interrupt level. This bit is effective in SCI-compatible mode only. Writing to this bit maps the Break-detect interrupt level to the INT0 line. This field is writable in SCI mode only."]
pub type ClrbrkdtintlvlR = crate::BitReader;
#[doc = "Field `CLRBRKDTINTLVL` writer - 0:0\\]
Clear Break-detect interrupt level. This bit is effective in SCI-compatible mode only. Writing to this bit maps the Break-detect interrupt level to the INT0 line. This field is writable in SCI mode only."]
pub type ClrbrkdtintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRWAKEUPINTLVL` reader - 1:1\\]
Clear Wake-up interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Wake-up interrupt level to the INT0 line."]
pub type ClrwakeupintlvlR = crate::BitReader;
#[doc = "Field `CLRWAKEUPINTLVL` writer - 1:1\\]
Clear Wake-up interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Wake-up interrupt level to the INT0 line."]
pub type ClrwakeupintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRTIMEOUTINTLVL` reader - 4:4\\]
Clear Timeout interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrtimeoutintlvlR = crate::BitReader;
#[doc = "Field `CLRTIMEOUTINTLVL` writer - 4:4\\]
Clear Timeout interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrtimeoutintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 5:5\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - 5:5\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRTOAWUSINTLVL` reader - 6:6\\]
Clear Timeout After Wakeup Signal interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the the timeout after wakeup interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrtoawusintlvlR = crate::BitReader;
#[doc = "Field `CLRTOAWUSINTLVL` writer - 6:6\\]
Clear Timeout After Wakeup Signal interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the the timeout after wakeup interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrtoawusintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRTOA3WUSINTLVL` reader - 7:7\\]
Clear Timeout After 3 Wakeup Signals interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout after 3 wakeup signals interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type Clrtoa3wusintlvlR = crate::BitReader;
#[doc = "Field `CLRTOA3WUSINTLVL` writer - 7:7\\]
Clear Timeout After 3 Wakeup Signals interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout after 3 wakeup signals interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type Clrtoa3wusintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRTXINTLVL` reader - 8:8\\]
Clear Transmitter interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the transmitter interrupt level to the INT0 line."]
pub type ClrtxintlvlR = crate::BitReader;
#[doc = "Field `CLRTXINTLVL` writer - 8:8\\]
Clear Transmitter interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the transmitter interrupt level to the INT0 line."]
pub type ClrtxintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRRXINTLVL` reader - 9:9\\]
Clear Receiver interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the receiver interrupt level to the INT0 line."]
pub type ClrrxintlvlR = crate::BitReader;
#[doc = "Field `CLRRXINTLVL` writer - 9:9\\]
Clear Receiver interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the receiver interrupt level to the INT0 line."]
pub type ClrrxintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 12:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 12:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLRIDINTLVL` reader - 13:13\\]
Clear ID interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the ID interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClridintlvlR = crate::BitReader;
#[doc = "Field `CLRIDINTLVL` writer - 13:13\\]
Clear ID interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the ID interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClridintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 15:14\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 15:14\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved4` reader - 17:16\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - 17:16\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved5` reader - 18:18\\]
Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - 18:18\\]
Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - 23:19\\]
Reserved"]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `Reserved6` writer - 23:19\\]
Reserved"]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLRPEINTLVL` reader - 24:24\\]
Clear Parity Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Parity Error interrupt level to the INT0 line."]
pub type ClrpeintlvlR = crate::BitReader;
#[doc = "Field `CLRPEINTLVL` writer - 24:24\\]
Clear Parity Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Parity Error interrupt level to the INT0 line."]
pub type ClrpeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLROEINTLVL` reader - 25:25\\]
Clear Overrun-Error Interrupt Level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Overrun-Error interrupt level to the INT0 line."]
pub type ClroeintlvlR = crate::BitReader;
#[doc = "Field `CLROEINTLVL` writer - 25:25\\]
Clear Overrun-Error Interrupt Level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Overrun-Error interrupt level to the INT0 line."]
pub type ClroeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRFEINTLVL` reader - 26:26\\]
Clear Framing-Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Framing-Error interrupt level to the INT0 line."]
pub type ClrfeintlvlR = crate::BitReader;
#[doc = "Field `CLRFEINTLVL` writer - 26:26\\]
Clear Framing-Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Framing-Error interrupt level to the INT0 line."]
pub type ClrfeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRNREINTLVL` reader - 27:27\\]
Clear No-Reponse-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the No-Response-Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrnreintlvlR = crate::BitReader;
#[doc = "Field `CLRNREINTLVL` writer - 27:27\\]
Clear No-Reponse-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the No-Response-Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrnreintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRISFEINTLVL` reader - 28:28\\]
Clear Inconsistent-Sync-Field-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Inconsistent-Sync-Field-Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrisfeintlvlR = crate::BitReader;
#[doc = "Field `CLRISFEINTLVL` writer - 28:28\\]
Clear Inconsistent-Sync-Field-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Inconsistent-Sync-Field-Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrisfeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRCEINTLVL` reader - 29:29\\]
Clear Checksum-error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Checksum-error interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrceintlvlR = crate::BitReader;
#[doc = "Field `CLRCEINTLVL` writer - 29:29\\]
Clear Checksum-error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Checksum-error interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrceintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRPBEINTLVL` reader - 30:30\\]
Clear Physical Bus Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Physical Bus Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrpbeintlvlR = crate::BitReader;
#[doc = "Field `CLRPBEINTLVL` writer - 30:30\\]
Clear Physical Bus Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Physical Bus Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrpbeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRBEINTLVL` reader - 31:31\\]
Clear Bit Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Bit Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrbeintlvlR = crate::BitReader;
#[doc = "Field `CLRBEINTLVL` writer - 31:31\\]
Clear Bit Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Bit Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
pub type ClrbeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear Break-detect interrupt level. This bit is effective in SCI-compatible mode only. Writing to this bit maps the Break-detect interrupt level to the INT0 line. This field is writable in SCI mode only."]
    #[inline(always)]
    pub fn clrbrkdtintlvl(&self) -> ClrbrkdtintlvlR {
        ClrbrkdtintlvlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Wake-up interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Wake-up interrupt level to the INT0 line."]
    #[inline(always)]
    pub fn clrwakeupintlvl(&self) -> ClrwakeupintlvlR {
        ClrwakeupintlvlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear Timeout interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrtimeoutintlvl(&self) -> ClrtimeoutintlvlR {
        ClrtimeoutintlvlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear Timeout After Wakeup Signal interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the the timeout after wakeup interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrtoawusintlvl(&self) -> ClrtoawusintlvlR {
        ClrtoawusintlvlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear Timeout After 3 Wakeup Signals interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout after 3 wakeup signals interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrtoa3wusintlvl(&self) -> Clrtoa3wusintlvlR {
        Clrtoa3wusintlvlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Transmitter interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the transmitter interrupt level to the INT0 line."]
    #[inline(always)]
    pub fn clrtxintlvl(&self) -> ClrtxintlvlR {
        ClrtxintlvlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Receiver interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the receiver interrupt level to the INT0 line."]
    #[inline(always)]
    pub fn clrrxintlvl(&self) -> ClrrxintlvlR {
        ClrrxintlvlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Clear ID interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the ID interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clridintlvl(&self) -> ClridintlvlR {
        ClridintlvlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear Parity Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Parity Error interrupt level to the INT0 line."]
    #[inline(always)]
    pub fn clrpeintlvl(&self) -> ClrpeintlvlR {
        ClrpeintlvlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear Overrun-Error Interrupt Level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Overrun-Error interrupt level to the INT0 line."]
    #[inline(always)]
    pub fn clroeintlvl(&self) -> ClroeintlvlR {
        ClroeintlvlR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Clear Framing-Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Framing-Error interrupt level to the INT0 line."]
    #[inline(always)]
    pub fn clrfeintlvl(&self) -> ClrfeintlvlR {
        ClrfeintlvlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Clear No-Reponse-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the No-Response-Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrnreintlvl(&self) -> ClrnreintlvlR {
        ClrnreintlvlR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Clear Inconsistent-Sync-Field-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Inconsistent-Sync-Field-Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrisfeintlvl(&self) -> ClrisfeintlvlR {
        ClrisfeintlvlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Clear Checksum-error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Checksum-error interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrceintlvl(&self) -> ClrceintlvlR {
        ClrceintlvlR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Clear Physical Bus Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Physical Bus Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrpbeintlvl(&self) -> ClrpbeintlvlR {
        ClrpbeintlvlR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Clear Bit Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Bit Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn clrbeintlvl(&self) -> ClrbeintlvlR {
        ClrbeintlvlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear Break-detect interrupt level. This bit is effective in SCI-compatible mode only. Writing to this bit maps the Break-detect interrupt level to the INT0 line. This field is writable in SCI mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrbrkdtintlvl(&mut self) -> ClrbrkdtintlvlW<SciclearintlvlSpec> {
        ClrbrkdtintlvlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Wake-up interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Wake-up interrupt level to the INT0 line."]
    #[inline(always)]
    #[must_use]
    pub fn clrwakeupintlvl(&mut self) -> ClrwakeupintlvlW<SciclearintlvlSpec> {
        ClrwakeupintlvlW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear Timeout interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrtimeoutintlvl(&mut self) -> ClrtimeoutintlvlW<SciclearintlvlSpec> {
        ClrtimeoutintlvlW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SciclearintlvlSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear Timeout After Wakeup Signal interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the the timeout after wakeup interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrtoawusintlvl(&mut self) -> ClrtoawusintlvlW<SciclearintlvlSpec> {
        ClrtoawusintlvlW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear Timeout After 3 Wakeup Signals interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout after 3 wakeup signals interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrtoa3wusintlvl(&mut self) -> Clrtoa3wusintlvlW<SciclearintlvlSpec> {
        Clrtoa3wusintlvlW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Transmitter interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the transmitter interrupt level to the INT0 line."]
    #[inline(always)]
    #[must_use]
    pub fn clrtxintlvl(&mut self) -> ClrtxintlvlW<SciclearintlvlSpec> {
        ClrtxintlvlW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Receiver interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the receiver interrupt level to the INT0 line."]
    #[inline(always)]
    #[must_use]
    pub fn clrrxintlvl(&mut self) -> ClrrxintlvlW<SciclearintlvlSpec> {
        ClrrxintlvlW::new(self, 9)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SciclearintlvlSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 13 - 13:13\\]
Clear ID interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the ID interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clridintlvl(&mut self) -> ClridintlvlW<SciclearintlvlSpec> {
        ClridintlvlW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SciclearintlvlSpec> {
        Reserved3W::new(self, 14)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<SciclearintlvlSpec> {
        Reserved4W::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<SciclearintlvlSpec> {
        Reserved5W::new(self, 18)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<SciclearintlvlSpec> {
        Reserved6W::new(self, 19)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear Parity Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Parity Error interrupt level to the INT0 line."]
    #[inline(always)]
    #[must_use]
    pub fn clrpeintlvl(&mut self) -> ClrpeintlvlW<SciclearintlvlSpec> {
        ClrpeintlvlW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear Overrun-Error Interrupt Level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Overrun-Error interrupt level to the INT0 line."]
    #[inline(always)]
    #[must_use]
    pub fn clroeintlvl(&mut self) -> ClroeintlvlW<SciclearintlvlSpec> {
        ClroeintlvlW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Clear Framing-Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Framing-Error interrupt level to the INT0 line."]
    #[inline(always)]
    #[must_use]
    pub fn clrfeintlvl(&mut self) -> ClrfeintlvlW<SciclearintlvlSpec> {
        ClrfeintlvlW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Clear No-Reponse-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the No-Response-Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrnreintlvl(&mut self) -> ClrnreintlvlW<SciclearintlvlSpec> {
        ClrnreintlvlW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Clear Inconsistent-Sync-Field-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Inconsistent-Sync-Field-Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrisfeintlvl(&mut self) -> ClrisfeintlvlW<SciclearintlvlSpec> {
        ClrisfeintlvlW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Clear Checksum-error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Checksum-error interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrceintlvl(&mut self) -> ClrceintlvlW<SciclearintlvlSpec> {
        ClrceintlvlW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Clear Physical Bus Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Physical Bus Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrpbeintlvl(&mut self) -> ClrpbeintlvlW<SciclearintlvlSpec> {
        ClrpbeintlvlW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Clear Bit Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Bit Error interrupt level to the INT0 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn clrbeintlvl(&mut self) -> ClrbeintlvlW<SciclearintlvlSpec> {
        ClrbeintlvlW::new(self, 31)
    }
}
#[doc = "The SCICLEARINTLVL register is used to map individual interrupt sources to the INT0 line.\n\nYou can [`read`](crate::Reg::read) this register and get [`sciclearintlvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciclearintlvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SciclearintlvlSpec;
impl crate::RegisterSpec for SciclearintlvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sciclearintlvl::R`](R) reader structure"]
impl crate::Readable for SciclearintlvlSpec {}
#[doc = "`write(|w| ..)` method takes [`sciclearintlvl::W`](W) writer structure"]
impl crate::Writable for SciclearintlvlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCICLEARINTLVL to value 0"]
impl crate::Resettable for SciclearintlvlSpec {
    const RESET_VALUE: u32 = 0;
}
