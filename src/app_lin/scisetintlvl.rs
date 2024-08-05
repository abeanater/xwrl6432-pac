#[doc = "Register `SCISETINTLVL` reader"]
pub type R = crate::R<ScisetintlvlSpec>;
#[doc = "Register `SCISETINTLVL` writer"]
pub type W = crate::W<ScisetintlvlSpec>;
#[doc = "Field `SETBRKDTINTLVL` reader - 0:0\\]
Set Break-detect interrupt level. This bit is effective in SCI-compatible mode only. Writing to this bit maps the Break-detect interrupt level to the INT1 line. This field is writable in SCI mode only."]
pub type SetbrkdtintlvlR = crate::BitReader;
#[doc = "Field `SETBRKDTINTLVL` writer - 0:0\\]
Set Break-detect interrupt level. This bit is effective in SCI-compatible mode only. Writing to this bit maps the Break-detect interrupt level to the INT1 line. This field is writable in SCI mode only."]
pub type SetbrkdtintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETWAKEUPINTLVL` reader - 1:1\\]
Set Wake-up interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Wake-up interrupt level to the INT1 line."]
pub type SetwakeupintlvlR = crate::BitReader;
#[doc = "Field `SETWAKEUPINTLVL` writer - 1:1\\]
Set Wake-up interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Wake-up interrupt level to the INT1 line."]
pub type SetwakeupintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTIMEOUTINTLVL` reader - 4:4\\]
Set Timeout interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SettimeoutintlvlR = crate::BitReader;
#[doc = "Field `SETTIMEOUTINTLVL` writer - 4:4\\]
Set Timeout interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SettimeoutintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 5:5\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - 5:5\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTOAWUSINTLVL` reader - 6:6\\]
Set Timeout After Wakeup Signal interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the the timeout after wakeup interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SettoawusintlvlR = crate::BitReader;
#[doc = "Field `SETTOAWUSINTLVL` writer - 6:6\\]
Set Timeout After Wakeup Signal interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the the timeout after wakeup interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SettoawusintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTOA3WUSINTLVL` reader - 7:7\\]
Set Timeout After 3 Wakeup Signals interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout after 3 wakeup signals interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type Settoa3wusintlvlR = crate::BitReader;
#[doc = "Field `SETTOA3WUSINTLVL` writer - 7:7\\]
Set Timeout After 3 Wakeup Signals interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout after 3 wakeup signals interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type Settoa3wusintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTXINTLVL` reader - 8:8\\]
Set Transmitter interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the transmitter interrupt level to the INT1 line."]
pub type SettxintlvlR = crate::BitReader;
#[doc = "Field `SETTXINTLVL` writer - 8:8\\]
Set Transmitter interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the transmitter interrupt level to the INT1 line."]
pub type SettxintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETRXINTOVO` reader - 9:9\\]
Set Receiver interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the receiver interrupt level to the INT1 line."]
pub type SetrxintovoR = crate::BitReader;
#[doc = "Field `SETRXINTOVO` writer - 9:9\\]
Set Receiver interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the receiver interrupt level to the INT1 line."]
pub type SetrxintovoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 12:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 12:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SETIDINTLVL` reader - 13:13\\]
Set ID interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the ID interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetidintlvlR = crate::BitReader;
#[doc = "Field `SETIDINTLVL` writer - 13:13\\]
Set ID interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the ID interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetidintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `SETPEINTLVL` reader - 24:24\\]
Set Parity Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Parity error interrupt level to the INT1 line."]
pub type SetpeintlvlR = crate::BitReader;
#[doc = "Field `SETPEINTLVL` writer - 24:24\\]
Set Parity Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Parity error interrupt level to the INT1 line."]
pub type SetpeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETOEINTLVL` reader - 25:25\\]
Set Overrun-Error Interrupt Level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Overrun-Error interrupt level to the INT1 line."]
pub type SetoeintlvlR = crate::BitReader;
#[doc = "Field `SETOEINTLVL` writer - 25:25\\]
Set Overrun-Error Interrupt Level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Overrun-Error interrupt level to the INT1 line."]
pub type SetoeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETFEINTLVL` reader - 26:26\\]
Set Framing-Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Framing-Error interrupt level to the INT1 line."]
pub type SetfeintlvlR = crate::BitReader;
#[doc = "Field `SETFEINTLVL` writer - 26:26\\]
Set Framing-Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Framing-Error interrupt level to the INT1 line."]
pub type SetfeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETNREINTLVL` reader - 27:27\\]
Set No-Reponse-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the No-Response-Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetnreintlvlR = crate::BitReader;
#[doc = "Field `SETNREINTLVL` writer - 27:27\\]
Set No-Reponse-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the No-Response-Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetnreintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETISFEINTLVL` reader - 28:28\\]
Set Inconsistent-Sync-Field-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Inconsistent-Sync-Field-Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetisfeintlvlR = crate::BitReader;
#[doc = "Field `SETISFEINTLVL` writer - 28:28\\]
Set Inconsistent-Sync-Field-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Inconsistent-Sync-Field-Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetisfeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETCEINTLVL` reader - 29:29\\]
Set Checksum-error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Checksum-error interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetceintlvlR = crate::BitReader;
#[doc = "Field `SETCEINTLVL` writer - 29:29\\]
Set Checksum-error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Checksum-error interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetceintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPBEINTLVL` reader - 30:30\\]
Set Physical Bus Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Physical Bus Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetpbeintlvlR = crate::BitReader;
#[doc = "Field `SETPBEINTLVL` writer - 30:30\\]
Set Physical Bus Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Physical Bus Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetpbeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETBEINTLVL` reader - 31:31\\]
Set Bit Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Bit Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetbeintlvlR = crate::BitReader;
#[doc = "Field `SETBEINTLVL` writer - 31:31\\]
Set Bit Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Bit Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
pub type SetbeintlvlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Set Break-detect interrupt level. This bit is effective in SCI-compatible mode only. Writing to this bit maps the Break-detect interrupt level to the INT1 line. This field is writable in SCI mode only."]
    #[inline(always)]
    pub fn setbrkdtintlvl(&self) -> SetbrkdtintlvlR {
        SetbrkdtintlvlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set Wake-up interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Wake-up interrupt level to the INT1 line."]
    #[inline(always)]
    pub fn setwakeupintlvl(&self) -> SetwakeupintlvlR {
        SetwakeupintlvlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Set Timeout interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn settimeoutintlvl(&self) -> SettimeoutintlvlR {
        SettimeoutintlvlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Set Timeout After Wakeup Signal interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the the timeout after wakeup interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn settoawusintlvl(&self) -> SettoawusintlvlR {
        SettoawusintlvlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Set Timeout After 3 Wakeup Signals interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout after 3 wakeup signals interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn settoa3wusintlvl(&self) -> Settoa3wusintlvlR {
        Settoa3wusintlvlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Set Transmitter interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the transmitter interrupt level to the INT1 line."]
    #[inline(always)]
    pub fn settxintlvl(&self) -> SettxintlvlR {
        SettxintlvlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Set Receiver interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the receiver interrupt level to the INT1 line."]
    #[inline(always)]
    pub fn setrxintovo(&self) -> SetrxintovoR {
        SetrxintovoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Set ID interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the ID interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn setidintlvl(&self) -> SetidintlvlR {
        SetidintlvlR::new(((self.bits >> 13) & 1) != 0)
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
Set Parity Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Parity error interrupt level to the INT1 line."]
    #[inline(always)]
    pub fn setpeintlvl(&self) -> SetpeintlvlR {
        SetpeintlvlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Set Overrun-Error Interrupt Level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Overrun-Error interrupt level to the INT1 line."]
    #[inline(always)]
    pub fn setoeintlvl(&self) -> SetoeintlvlR {
        SetoeintlvlR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Set Framing-Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Framing-Error interrupt level to the INT1 line."]
    #[inline(always)]
    pub fn setfeintlvl(&self) -> SetfeintlvlR {
        SetfeintlvlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Set No-Reponse-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the No-Response-Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn setnreintlvl(&self) -> SetnreintlvlR {
        SetnreintlvlR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Set Inconsistent-Sync-Field-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Inconsistent-Sync-Field-Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn setisfeintlvl(&self) -> SetisfeintlvlR {
        SetisfeintlvlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Set Checksum-error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Checksum-error interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn setceintlvl(&self) -> SetceintlvlR {
        SetceintlvlR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Set Physical Bus Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Physical Bus Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn setpbeintlvl(&self) -> SetpbeintlvlR {
        SetpbeintlvlR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Set Bit Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Bit Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    pub fn setbeintlvl(&self) -> SetbeintlvlR {
        SetbeintlvlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Set Break-detect interrupt level. This bit is effective in SCI-compatible mode only. Writing to this bit maps the Break-detect interrupt level to the INT1 line. This field is writable in SCI mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setbrkdtintlvl(&mut self) -> SetbrkdtintlvlW<ScisetintlvlSpec> {
        SetbrkdtintlvlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set Wake-up interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Wake-up interrupt level to the INT1 line."]
    #[inline(always)]
    #[must_use]
    pub fn setwakeupintlvl(&mut self) -> SetwakeupintlvlW<ScisetintlvlSpec> {
        SetwakeupintlvlW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Set Timeout interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn settimeoutintlvl(&mut self) -> SettimeoutintlvlW<ScisetintlvlSpec> {
        SettimeoutintlvlW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<ScisetintlvlSpec> {
        Reserved1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Set Timeout After Wakeup Signal interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the the timeout after wakeup interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn settoawusintlvl(&mut self) -> SettoawusintlvlW<ScisetintlvlSpec> {
        SettoawusintlvlW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Set Timeout After 3 Wakeup Signals interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the timeout after 3 wakeup signals interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn settoa3wusintlvl(&mut self) -> Settoa3wusintlvlW<ScisetintlvlSpec> {
        Settoa3wusintlvlW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Set Transmitter interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the transmitter interrupt level to the INT1 line."]
    #[inline(always)]
    #[must_use]
    pub fn settxintlvl(&mut self) -> SettxintlvlW<ScisetintlvlSpec> {
        SettxintlvlW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Set Receiver interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the receiver interrupt level to the INT1 line."]
    #[inline(always)]
    #[must_use]
    pub fn setrxintovo(&mut self) -> SetrxintovoW<ScisetintlvlSpec> {
        SetrxintovoW::new(self, 9)
    }
    #[doc = "Bits 10:12 - 12:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<ScisetintlvlSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 13 - 13:13\\]
Set ID interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the ID interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setidintlvl(&mut self) -> SetidintlvlW<ScisetintlvlSpec> {
        SetidintlvlW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<ScisetintlvlSpec> {
        Reserved3W::new(self, 14)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<ScisetintlvlSpec> {
        Reserved4W::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<ScisetintlvlSpec> {
        Reserved5W::new(self, 18)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<ScisetintlvlSpec> {
        Reserved6W::new(self, 19)
    }
    #[doc = "Bit 24 - 24:24\\]
Set Parity Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Parity error interrupt level to the INT1 line."]
    #[inline(always)]
    #[must_use]
    pub fn setpeintlvl(&mut self) -> SetpeintlvlW<ScisetintlvlSpec> {
        SetpeintlvlW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Set Overrun-Error Interrupt Level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Overrun-Error interrupt level to the INT1 line."]
    #[inline(always)]
    #[must_use]
    pub fn setoeintlvl(&mut self) -> SetoeintlvlW<ScisetintlvlSpec> {
        SetoeintlvlW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Set Framing-Error interrupt level. This bit is effective in LIN or SCI-compatible mode. Writing to this bit maps the Framing-Error interrupt level to the INT1 line."]
    #[inline(always)]
    #[must_use]
    pub fn setfeintlvl(&mut self) -> SetfeintlvlW<ScisetintlvlSpec> {
        SetfeintlvlW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Set No-Reponse-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the No-Response-Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setnreintlvl(&mut self) -> SetnreintlvlW<ScisetintlvlSpec> {
        SetnreintlvlW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Set Inconsistent-Sync-Field-Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Inconsistent-Sync-Field-Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setisfeintlvl(&mut self) -> SetisfeintlvlW<ScisetintlvlSpec> {
        SetisfeintlvlW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Set Checksum-error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Checksum-error interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setceintlvl(&mut self) -> SetceintlvlW<ScisetintlvlSpec> {
        SetceintlvlW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Set Physical Bus Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Physical Bus Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setpbeintlvl(&mut self) -> SetpbeintlvlW<ScisetintlvlSpec> {
        SetpbeintlvlW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Set Bit Error interrupt level. This bit is effective in LIN mode only. Writing to this bit maps the Bit Error interrupt level to the INT1 line. This field is writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn setbeintlvl(&mut self) -> SetbeintlvlW<ScisetintlvlSpec> {
        SetbeintlvlW::new(self, 31)
    }
}
#[doc = "The SCISETINTLVL register is used to map individual interrupt sources to the INT1 interrupt line.\n\nYou can [`read`](crate::Reg::read) this register and get [`scisetintlvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scisetintlvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScisetintlvlSpec;
impl crate::RegisterSpec for ScisetintlvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scisetintlvl::R`](R) reader structure"]
impl crate::Readable for ScisetintlvlSpec {}
#[doc = "`write(|w| ..)` method takes [`scisetintlvl::W`](W) writer structure"]
impl crate::Writable for ScisetintlvlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCISETINTLVL to value 0"]
impl crate::Resettable for ScisetintlvlSpec {
    const RESET_VALUE: u32 = 0;
}
