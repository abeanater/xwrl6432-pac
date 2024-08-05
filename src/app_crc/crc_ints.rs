#[doc = "Register `CRC_INTS` reader"]
pub type R = crate::R<CrcIntsSpec>;
#[doc = "Register `CRC_INTS` writer"]
pub type W = crate::W<CrcIntsSpec>;
#[doc = "Field `Reserved1` reader - 0:0\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - 0:0\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CRCFAILENS` reader - 1:1\\]
Channel 1 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch1CrcfailensR = crate::BitReader;
#[doc = "Field `CH1_CRCFAILENS` writer - 1:1\\]
Channel 1 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch1CrcfailensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_OVERENS` reader - 2:2\\]
Channel 1 Overrun Interrupt Enable Bit. Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch1OverensR = crate::BitReader;
#[doc = "Field `CH1_OVERENS` writer - 2:2\\]
Channel 1 Overrun Interrupt Enable Bit. Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch1OverensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_UNDERENS` reader - 3:3\\]
Channel 1 Underrun Interrupt Enable Bit. Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch1UnderensR = crate::BitReader;
#[doc = "Field `CH1_UNDERENS` writer - 3:3\\]
Channel 1 Underrun Interrupt Enable Bit. Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch1UnderensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TIMEOUTENS` reader - 4:4\\]
Channel 1 Timeout Interrupt Enable Bit. Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch1TimeoutensR = crate::BitReader;
#[doc = "Field `CH1_TIMEOUTENS` writer - 4:4\\]
Channel 1 Timeout Interrupt Enable Bit. Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch1TimeoutensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 8:5\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 8:5\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH2_CRCFAILENS` reader - 9:9\\]
Channel 2 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch2CrcfailensR = crate::BitReader;
#[doc = "Field `CH2_CRCFAILENS` writer - 9:9\\]
Channel 2 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch2CrcfailensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_OVERENS` reader - 10:10\\]
Channel 2 Overrun Interrupt Enable Bit. Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch2OverensR = crate::BitReader;
#[doc = "Field `CH2_OVERENS` writer - 10:10\\]
Channel 2 Overrun Interrupt Enable Bit. Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch2OverensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_UNDERENS` reader - 11:11\\]
Channel 2 Underrun Interrupt Enable Bit. Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch2UnderensR = crate::BitReader;
#[doc = "Field `CH2_UNDERENS` writer - 11:11\\]
Channel 2 Underrun Interrupt Enable Bit. Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch2UnderensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TIMEOUTENS` reader - 12:12\\]
Channel 2 Timeout Interrupt Enable Bit. Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch2TimeoutensR = crate::BitReader;
#[doc = "Field `CH2_TIMEOUTENS` writer - 12:12\\]
Channel 2 Timeout Interrupt Enable Bit. Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch2TimeoutensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 16:13\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 16:13\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU15` reader - 17:17\\]
Reserved"]
pub type Nu15R = crate::BitReader;
#[doc = "Field `NU15` writer - 17:17\\]
Reserved"]
pub type Nu15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU16` reader - 18:18\\]
Reserved"]
pub type Nu16R = crate::BitReader;
#[doc = "Field `NU16` writer - 18:18\\]
Reserved"]
pub type Nu16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU17` reader - 19:19\\]
Reserved"]
pub type Nu17R = crate::BitReader;
#[doc = "Field `NU17` writer - 19:19\\]
Reserved"]
pub type Nu17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU18` reader - 20:20\\]
Reserved"]
pub type Nu18R = crate::BitReader;
#[doc = "Field `NU18` writer - 20:20\\]
Reserved"]
pub type Nu18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - 24:21\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - 24:21\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU19` reader - 25:25\\]
Reserved"]
pub type Nu19R = crate::BitReader;
#[doc = "Field `NU19` writer - 25:25\\]
Reserved"]
pub type Nu19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU20` reader - 26:26\\]
Reserved"]
pub type Nu20R = crate::BitReader;
#[doc = "Field `NU20` writer - 26:26\\]
Reserved"]
pub type Nu20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU21` reader - 27:27\\]
Reserved"]
pub type Nu21R = crate::BitReader;
#[doc = "Field `NU21` writer - 27:27\\]
Reserved"]
pub type Nu21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU22` reader - 28:28\\]
Reserved"]
pub type Nu22R = crate::BitReader;
#[doc = "Field `NU22` writer - 28:28\\]
Reserved"]
pub type Nu22W<'a, REG> = crate::BitWriter<'a, REG>;
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
Channel 1 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    pub fn ch1_crcfailens(&self) -> Ch1CrcfailensR {
        Ch1CrcfailensR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 Overrun Interrupt Enable Bit. Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    pub fn ch1_overens(&self) -> Ch1OverensR {
        Ch1OverensR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 Underrun Interrupt Enable Bit. Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ch1_underens(&self) -> Ch1UnderensR {
        Ch1UnderensR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Timeout Interrupt Enable Bit. Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    pub fn ch1_timeoutens(&self) -> Ch1TimeoutensR {
        Ch1TimeoutensR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    pub fn ch2_crcfailens(&self) -> Ch2CrcfailensR {
        Ch2CrcfailensR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 Overrun Interrupt Enable Bit. Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    pub fn ch2_overens(&self) -> Ch2OverensR {
        Ch2OverensR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 Underrun Interrupt Enable Bit. Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ch2_underens(&self) -> Ch2UnderensR {
        Ch2UnderensR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 Timeout Interrupt Enable Bit. Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    pub fn ch2_timeoutens(&self) -> Ch2TimeoutensR {
        Ch2TimeoutensR::new(((self.bits >> 12) & 1) != 0)
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
    pub fn nu15(&self) -> Nu15R {
        Nu15R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    pub fn nu16(&self) -> Nu16R {
        Nu16R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved"]
    #[inline(always)]
    pub fn nu17(&self) -> Nu17R {
        Nu17R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved"]
    #[inline(always)]
    pub fn nu18(&self) -> Nu18R {
        Nu18R::new(((self.bits >> 20) & 1) != 0)
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
    pub fn nu19(&self) -> Nu19R {
        Nu19R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved"]
    #[inline(always)]
    pub fn nu20(&self) -> Nu20R {
        Nu20R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Reserved"]
    #[inline(always)]
    pub fn nu21(&self) -> Nu21R {
        Nu21R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Reserved"]
    #[inline(always)]
    pub fn nu22(&self) -> Nu22R {
        Nu22R::new(((self.bits >> 28) & 1) != 0)
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
    pub fn reserved1(&mut self) -> Reserved1W<CrcIntsSpec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_crcfailens(&mut self) -> Ch1CrcfailensW<CrcIntsSpec> {
        Ch1CrcfailensW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 Overrun Interrupt Enable Bit. Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_overens(&mut self) -> Ch1OverensW<CrcIntsSpec> {
        Ch1OverensW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 Underrun Interrupt Enable Bit. Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_underens(&mut self) -> Ch1UnderensW<CrcIntsSpec> {
        Ch1UnderensW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Timeout Interrupt Enable Bit. Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_timeoutens(&mut self) -> Ch1TimeoutensW<CrcIntsSpec> {
        Ch1TimeoutensW::new(self, 4)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<CrcIntsSpec> {
        Reserved2W::new(self, 5)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_crcfailens(&mut self) -> Ch2CrcfailensW<CrcIntsSpec> {
        Ch2CrcfailensW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 Overrun Interrupt Enable Bit. Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_overens(&mut self) -> Ch2OverensW<CrcIntsSpec> {
        Ch2OverensW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 Underrun Interrupt Enable Bit. Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_underens(&mut self) -> Ch2UnderensW<CrcIntsSpec> {
        Ch2UnderensW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 Timeout Interrupt Enable Bit. Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_timeoutens(&mut self) -> Ch2TimeoutensW<CrcIntsSpec> {
        Ch2TimeoutensW::new(self, 12)
    }
    #[doc = "Bits 13:16 - 16:13\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<CrcIntsSpec> {
        Reserved3W::new(self, 13)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu15(&mut self) -> Nu15W<CrcIntsSpec> {
        Nu15W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu16(&mut self) -> Nu16W<CrcIntsSpec> {
        Nu16W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu17(&mut self) -> Nu17W<CrcIntsSpec> {
        Nu17W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu18(&mut self) -> Nu18W<CrcIntsSpec> {
        Nu18W::new(self, 20)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<CrcIntsSpec> {
        Reserved4W::new(self, 21)
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu19(&mut self) -> Nu19W<CrcIntsSpec> {
        Nu19W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu20(&mut self) -> Nu20W<CrcIntsSpec> {
        Nu20W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu21(&mut self) -> Nu21W<CrcIntsSpec> {
        Nu21W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu22(&mut self) -> Nu22W<CrcIntsSpec> {
        Nu22W::new(self, 28)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<CrcIntsSpec> {
        Reserved5W::new(self, 29)
    }
}
#[doc = "Write one to a bit to enable a interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ints::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ints::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcIntsSpec;
impl crate::RegisterSpec for CrcIntsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_ints::R`](R) reader structure"]
impl crate::Readable for CrcIntsSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_ints::W`](W) writer structure"]
impl crate::Writable for CrcIntsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_INTS to value 0"]
impl crate::Resettable for CrcIntsSpec {
    const RESET_VALUE: u32 = 0;
}
