#[doc = "Register `CRC_INTR` reader"]
pub type R = crate::R<CrcIntrSpec>;
#[doc = "Register `CRC_INTR` writer"]
pub type W = crate::W<CrcIntrSpec>;
#[doc = "Field `Reserved1` reader - 0:0\\]
Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - 0:0\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CRCFAILENR` reader - 1:1\\]
Channel 1 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt disable"]
pub type Ch1CrcfailenrR = crate::BitReader;
#[doc = "Field `CH1_CRCFAILENR` writer - 1:1\\]
Channel 1 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt disable"]
pub type Ch1CrcfailenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_OVERENR` reader - 2:2\\]
Channel 1 Overrun Interrupt Disable Bit. Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt disable"]
pub type Ch1OverenrR = crate::BitReader;
#[doc = "Field `CH1_OVERENR` writer - 2:2\\]
Channel 1 Overrun Interrupt Disable Bit. Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt disable"]
pub type Ch1OverenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_UNDERENR` reader - 3:3\\]
Channel 1 Underrun Interrupt Disable Bit. Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/dis- able). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt disable"]
pub type Ch1UnderenrR = crate::BitReader;
#[doc = "Field `CH1_UNDERENR` writer - 3:3\\]
Channel 1 Underrun Interrupt Disable Bit. Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/dis- able). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt disable"]
pub type Ch1UnderenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TIMEOUTENR` reader - 4:4\\]
Channel 1 Timeout Interrupt Disable Bit. Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt disable"]
pub type Ch1TimeoutenrR = crate::BitReader;
#[doc = "Field `CH1_TIMEOUTENR` writer - 4:4\\]
Channel 1 Timeout Interrupt Disable Bit. Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt disable"]
pub type Ch1TimeoutenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 8:5\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 8:5\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH2_CRCFAILENR` reader - 9:9\\]
Channel 2 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt disable"]
pub type Ch2CrcfailenrR = crate::BitReader;
#[doc = "Field `CH2_CRCFAILENR` writer - 9:9\\]
Channel 2 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt disable"]
pub type Ch2CrcfailenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_OVERENR` reader - 10:10\\]
Channel 2 Overrun Interrupt Disable Bit. Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt disable"]
pub type Ch2OverenrR = crate::BitReader;
#[doc = "Field `CH2_OVERENR` writer - 10:10\\]
Channel 2 Overrun Interrupt Disable Bit. Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt disable"]
pub type Ch2OverenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_UNDERENR` reader - 11:11\\]
Channel 2 Underrun Interrupt Disable Bit. Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/dis- able). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt disable"]
pub type Ch2UnderenrR = crate::BitReader;
#[doc = "Field `CH2_UNDERENR` writer - 11:11\\]
Channel 2 Underrun Interrupt Disable Bit. Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/dis- able). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt disable"]
pub type Ch2UnderenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TIMEOUTENR` reader - 12:12\\]
Channel 2 Timeout Interrupt Disable Bit. Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt disable"]
pub type Ch2TimeoutenrR = crate::BitReader;
#[doc = "Field `CH2_TIMEOUTENR` writer - 12:12\\]
Channel 2 Timeout Interrupt Disable Bit. Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt disable"]
pub type Ch2TimeoutenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 16:13\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 16:13\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU23` reader - 17:17\\]
Reserved"]
pub type Nu23R = crate::BitReader;
#[doc = "Field `NU23` writer - 17:17\\]
Reserved"]
pub type Nu23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU24` reader - 18:18\\]
Reserved"]
pub type Nu24R = crate::BitReader;
#[doc = "Field `NU24` writer - 18:18\\]
Reserved"]
pub type Nu24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU25` reader - 19:19\\]
Reserved"]
pub type Nu25R = crate::BitReader;
#[doc = "Field `NU25` writer - 19:19\\]
Reserved"]
pub type Nu25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU26` reader - 20:20\\]
Reserved"]
pub type Nu26R = crate::BitReader;
#[doc = "Field `NU26` writer - 20:20\\]
Reserved"]
pub type Nu26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - 24:21\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - 24:21\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU27` reader - 25:25\\]
Reserved"]
pub type Nu27R = crate::BitReader;
#[doc = "Field `NU27` writer - 25:25\\]
Reserved"]
pub type Nu27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU28` reader - 26:26\\]
Reserved"]
pub type Nu28R = crate::BitReader;
#[doc = "Field `NU28` writer - 26:26\\]
Reserved"]
pub type Nu28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU29` reader - 27:27\\]
Reserved"]
pub type Nu29R = crate::BitReader;
#[doc = "Field `NU29` writer - 27:27\\]
Reserved"]
pub type Nu29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU30` reader - 28:28\\]
Reserved"]
pub type Nu30R = crate::BitReader;
#[doc = "Field `NU30` writer - 28:28\\]
Reserved"]
pub type Nu30W<'a, REG> = crate::BitWriter<'a, REG>;
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
Channel 1 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt disable"]
    #[inline(always)]
    pub fn ch1_crcfailenr(&self) -> Ch1CrcfailenrR {
        Ch1CrcfailenrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 Overrun Interrupt Disable Bit. Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt disable"]
    #[inline(always)]
    pub fn ch1_overenr(&self) -> Ch1OverenrR {
        Ch1OverenrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 Underrun Interrupt Disable Bit. Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/dis- able). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt disable"]
    #[inline(always)]
    pub fn ch1_underenr(&self) -> Ch1UnderenrR {
        Ch1UnderenrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Timeout Interrupt Disable Bit. Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt disable"]
    #[inline(always)]
    pub fn ch1_timeoutenr(&self) -> Ch1TimeoutenrR {
        Ch1TimeoutenrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt disable"]
    #[inline(always)]
    pub fn ch2_crcfailenr(&self) -> Ch2CrcfailenrR {
        Ch2CrcfailenrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 Overrun Interrupt Disable Bit. Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt disable"]
    #[inline(always)]
    pub fn ch2_overenr(&self) -> Ch2OverenrR {
        Ch2OverenrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 Underrun Interrupt Disable Bit. Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/dis- able). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt disable"]
    #[inline(always)]
    pub fn ch2_underenr(&self) -> Ch2UnderenrR {
        Ch2UnderenrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 Timeout Interrupt Disable Bit. Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt disable"]
    #[inline(always)]
    pub fn ch2_timeoutenr(&self) -> Ch2TimeoutenrR {
        Ch2TimeoutenrR::new(((self.bits >> 12) & 1) != 0)
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
    pub fn nu23(&self) -> Nu23R {
        Nu23R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    pub fn nu24(&self) -> Nu24R {
        Nu24R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved"]
    #[inline(always)]
    pub fn nu25(&self) -> Nu25R {
        Nu25R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved"]
    #[inline(always)]
    pub fn nu26(&self) -> Nu26R {
        Nu26R::new(((self.bits >> 20) & 1) != 0)
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
    pub fn nu27(&self) -> Nu27R {
        Nu27R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved"]
    #[inline(always)]
    pub fn nu28(&self) -> Nu28R {
        Nu28R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Reserved"]
    #[inline(always)]
    pub fn nu29(&self) -> Nu29R {
        Nu29R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Reserved"]
    #[inline(always)]
    pub fn nu30(&self) -> Nu30R {
        Nu30R::new(((self.bits >> 28) & 1) != 0)
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
    pub fn reserved1(&mut self) -> Reserved1W<CrcIntrSpec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_crcfailenr(&mut self) -> Ch1CrcfailenrW<CrcIntrSpec> {
        Ch1CrcfailenrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 Overrun Interrupt Disable Bit. Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_overenr(&mut self) -> Ch1OverenrW<CrcIntrSpec> {
        Ch1OverenrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 Underrun Interrupt Disable Bit. Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/dis- able). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_underenr(&mut self) -> Ch1UnderenrW<CrcIntrSpec> {
        Ch1UnderenrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Timeout Interrupt Disable Bit. Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_timeoutenr(&mut self) -> Ch1TimeoutenrW<CrcIntrSpec> {
        Ch1TimeoutenrW::new(self, 4)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<CrcIntrSpec> {
        Reserved2W::new(self, 5)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_crcfailenr(&mut self) -> Ch2CrcfailenrW<CrcIntrSpec> {
        Ch2CrcfailenrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 Overrun Interrupt Disable Bit. Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_overenr(&mut self) -> Ch2OverenrW<CrcIntrSpec> {
        Ch2OverenrW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 Underrun Interrupt Disable Bit. Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/dis- able). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_underenr(&mut self) -> Ch2UnderenrW<CrcIntrSpec> {
        Ch2UnderenrW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 Timeout Interrupt Disable Bit. Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_timeoutenr(&mut self) -> Ch2TimeoutenrW<CrcIntrSpec> {
        Ch2TimeoutenrW::new(self, 12)
    }
    #[doc = "Bits 13:16 - 16:13\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<CrcIntrSpec> {
        Reserved3W::new(self, 13)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu23(&mut self) -> Nu23W<CrcIntrSpec> {
        Nu23W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu24(&mut self) -> Nu24W<CrcIntrSpec> {
        Nu24W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu25(&mut self) -> Nu25W<CrcIntrSpec> {
        Nu25W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu26(&mut self) -> Nu26W<CrcIntrSpec> {
        Nu26W::new(self, 20)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<CrcIntrSpec> {
        Reserved4W::new(self, 21)
    }
    #[doc = "Bit 25 - 25:25\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu27(&mut self) -> Nu27W<CrcIntrSpec> {
        Nu27W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu28(&mut self) -> Nu28W<CrcIntrSpec> {
        Nu28W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu29(&mut self) -> Nu29W<CrcIntrSpec> {
        Nu29W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu30(&mut self) -> Nu30W<CrcIntrSpec> {
        Nu30W::new(self, 28)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<CrcIntrSpec> {
        Reserved5W::new(self, 29)
    }
}
#[doc = "Write one to a bit to disable a interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcIntrSpec;
impl crate::RegisterSpec for CrcIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_intr::R`](R) reader structure"]
impl crate::Readable for CrcIntrSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_intr::W`](W) writer structure"]
impl crate::Writable for CrcIntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_INTR to value 0"]
impl crate::Resettable for CrcIntrSpec {
    const RESET_VALUE: u32 = 0;
}
