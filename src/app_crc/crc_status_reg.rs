#[doc = "Register `CRC_STATUS_REG` reader"]
pub type R = crate::R<CrcStatusRegSpec>;
#[doc = "Register `CRC_STATUS_REG` writer"]
pub type W = crate::W<CrcStatusRegSpec>;
#[doc = "Field `Reserved1` reader - 0:0\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - 0:0\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CRCFAIL` reader - 1:1\\]
Channel 1 CRC Compare Fail Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch1CrcfailR = crate::BitReader;
#[doc = "Field `CH1_CRCFAIL` writer - 1:1\\]
Channel 1 CRC Compare Fail Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch1CrcfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_OVER` reader - 2:2\\]
Channel 1 CRC Overrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch1OverR = crate::BitReader;
#[doc = "Field `CH1_OVER` writer - 2:2\\]
Channel 1 CRC Overrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch1OverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_UNDER` reader - 3:3\\]
Channel 1 CRC Underrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch1UnderR = crate::BitReader;
#[doc = "Field `CH1_UNDER` writer - 3:3\\]
Channel 1 CRC Underrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch1UnderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TIMEOUT` reader - 4:4\\]
Channel 1 CRC Timeout Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch1TimeoutR = crate::BitReader;
#[doc = "Field `CH1_TIMEOUT` writer - 4:4\\]
Channel 1 CRC Timeout Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch1TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 8:5\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 8:5\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH2_CRCFAIL` reader - 9:9\\]
Channel 2 CRC Compare Fail Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch2CrcfailR = crate::BitReader;
#[doc = "Field `CH2_CRCFAIL` writer - 9:9\\]
Channel 2 CRC Compare Fail Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
pub type Ch2CrcfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_OVER` reader - 10:10\\]
Channel 2 CRC Overrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch2OverR = crate::BitReader;
#[doc = "Field `CH2_OVER` writer - 10:10\\]
Channel 2 CRC Overrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
pub type Ch2OverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_UNDER` reader - 11:11\\]
Channel 2 CRC Underrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch2UnderR = crate::BitReader;
#[doc = "Field `CH2_UNDER` writer - 11:11\\]
Channel 2 CRC Underrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
pub type Ch2UnderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TIMEOUT` reader - 12:12\\]
Channel 2 CRC Timeout Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch2TimeoutR = crate::BitReader;
#[doc = "Field `CH2_TIMEOUT` writer - 12:12\\]
Channel 2 CRC Timeout Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
pub type Ch2TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 16:13\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 16:13\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU31` reader - 17:17\\]
Reserved"]
pub type Nu31R = crate::BitReader;
#[doc = "Field `NU31` writer - 17:17\\]
Reserved"]
pub type Nu31W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU32` reader - 18:18\\]
Reserved"]
pub type Nu32R = crate::BitReader;
#[doc = "Field `NU32` writer - 18:18\\]
Reserved"]
pub type Nu32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU33` reader - 19:19\\]
Reserved"]
pub type Nu33R = crate::BitReader;
#[doc = "Field `NU33` writer - 19:19\\]
Reserved"]
pub type Nu33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU34` reader - 20:20\\]
Reserved"]
pub type Nu34R = crate::BitReader;
#[doc = "Field `NU34` writer - 20:20\\]
Reserved"]
pub type Nu34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - 24:21\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - 24:21\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU35` reader - 25:25\\]
Reserved"]
pub type Nu35R = crate::BitReader;
#[doc = "Field `NU35` writer - 25:25\\]
Reserved"]
pub type Nu35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU36` reader - 26:26\\]
Reserved"]
pub type Nu36R = crate::BitReader;
#[doc = "Field `NU36` writer - 26:26\\]
Reserved"]
pub type Nu36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU37` reader - 27:27\\]
Reserved"]
pub type Nu37R = crate::BitReader;
#[doc = "Field `NU37` writer - 27:27\\]
Reserved"]
pub type Nu37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU38` reader - 28:28\\]
Reserved"]
pub type Nu38R = crate::BitReader;
#[doc = "Field `NU38` writer - 28:28\\]
Reserved"]
pub type Nu38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - 31:29\\]
Reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `Reserved5` writer - 31:29\\]
Reserved"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 CRC Compare Fail Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    pub fn ch1_crcfail(&self) -> Ch1CrcfailR {
        Ch1CrcfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 CRC Overrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    pub fn ch1_over(&self) -> Ch1OverR {
        Ch1OverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 CRC Underrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    pub fn ch1_under(&self) -> Ch1UnderR {
        Ch1UnderR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 CRC Timeout Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    pub fn ch1_timeout(&self) -> Ch1TimeoutR {
        Ch1TimeoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Compare Fail Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    pub fn ch2_crcfail(&self) -> Ch2CrcfailR {
        Ch2CrcfailR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 CRC Overrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    pub fn ch2_over(&self) -> Ch2OverR {
        Ch2OverR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 CRC Underrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    pub fn ch2_under(&self) -> Ch2UnderR {
        Ch2UnderR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 CRC Timeout Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    pub fn ch2_timeout(&self) -> Ch2TimeoutR {
        Ch2TimeoutR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - 16:13\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved"]
    #[inline(always)]
    pub fn nu31(&self) -> Nu31R {
        Nu31R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    pub fn nu32(&self) -> Nu32R {
        Nu32R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved"]
    #[inline(always)]
    pub fn nu33(&self) -> Nu33R {
        Nu33R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved"]
    #[inline(always)]
    pub fn nu34(&self) -> Nu34R {
        Nu34R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved"]
    #[inline(always)]
    pub fn nu35(&self) -> Nu35R {
        Nu35R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved"]
    #[inline(always)]
    pub fn nu36(&self) -> Nu36R {
        Nu36R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Reserved"]
    #[inline(always)]
    pub fn nu37(&self) -> Nu37R {
        Nu37R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Reserved"]
    #[inline(always)]
    pub fn nu38(&self) -> Nu38R {
        Nu38R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcStatusRegSpec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 CRC Compare Fail Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_crcfail(&mut self) -> Ch1CrcfailW<CrcStatusRegSpec> {
        Ch1CrcfailW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 CRC Overrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_over(&mut self) -> Ch1OverW<CrcStatusRegSpec> {
        Ch1OverW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 CRC Underrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_under(&mut self) -> Ch1UnderW<CrcStatusRegSpec> {
        Ch1UnderW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 CRC Timeout Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_timeout(&mut self) -> Ch1TimeoutW<CrcStatusRegSpec> {
        Ch1TimeoutW::new(self, 4)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<CrcStatusRegSpec> {
        Reserved2W::new(self, 5)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Compare Fail Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only. 0 = No CRC compare fail interrupt is active 1 = CRC compare fail interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_crcfail(&mut self) -> Ch2CrcfailW<CrcStatusRegSpec> {
        Ch2CrcfailW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 CRC Overrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode 0 = No overrun interrupt is active 1 = Overrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_over(&mut self) -> Ch2OverW<CrcStatusRegSpec> {
        Ch2OverW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 CRC Underrun Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode only 0 = No underrun interrupt is active 1 = Underrun interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_under(&mut self) -> Ch2UnderW<CrcStatusRegSpec> {
        Ch2UnderW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 CRC Timeout Status Flag. This bit is cleared by writing a ΓÇÖ1ΓÇÖ to it only. Writing ΓÇÖ0ΓÇÖ has no effect. This bit is set in AUTO mode. 0 = No timeout interrupt is active 1 = Timeout interrupt is active"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_timeout(&mut self) -> Ch2TimeoutW<CrcStatusRegSpec> {
        Ch2TimeoutW::new(self, 12)
    }
    #[doc = "Bits 13:16 - 16:13\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<CrcStatusRegSpec> {
        Reserved3W::new(self, 13)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu31(&mut self) -> Nu31W<CrcStatusRegSpec> {
        Nu31W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu32(&mut self) -> Nu32W<CrcStatusRegSpec> {
        Nu32W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu33(&mut self) -> Nu33W<CrcStatusRegSpec> {
        Nu33W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu34(&mut self) -> Nu34W<CrcStatusRegSpec> {
        Nu34W::new(self, 20)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<CrcStatusRegSpec> {
        Reserved4W::new(self, 21)
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu35(&mut self) -> Nu35W<CrcStatusRegSpec> {
        Nu35W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu36(&mut self) -> Nu36W<CrcStatusRegSpec> {
        Nu36W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu37(&mut self) -> Nu37W<CrcStatusRegSpec> {
        Nu37W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu38(&mut self) -> Nu38W<CrcStatusRegSpec> {
        Nu38W::new(self, 28)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<CrcStatusRegSpec> {
        Reserved5W::new(self, 29)
    }
}
#[doc = "Contains interrupt flags for different types of interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcStatusRegSpec;
impl crate::RegisterSpec for CrcStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_status_reg::R`](R) reader structure"]
impl crate::Readable for CrcStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_status_reg::W`](W) writer structure"]
impl crate::Writable for CrcStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_STATUS_REG to value 0"]
impl crate::Resettable for CrcStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
