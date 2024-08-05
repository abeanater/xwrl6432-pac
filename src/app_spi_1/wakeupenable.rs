#[doc = "Register `WAKEUPENABLE` reader"]
pub type R = crate::R<WakeupenableSpec>;
#[doc = "Register `WAKEUPENABLE` writer"]
pub type W = crate::W<WakeupenableSpec>;
#[doc = "Field `WKEN` reader - 0:0\\]
WakeUp functionality in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]
- (RW )"]
pub type WkenR = crate::BitReader;
#[doc = "Field `WKEN` writer - 0:0\\]
WakeUp functionality in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]
- (RW )"]
pub type WkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED_18` reader - 31:1\\]
Reads returns 0 - (RO )"]
pub type Reserved18R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_18` writer - 31:1\\]
Reads returns 0 - (RO )"]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
WakeUp functionality in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]
- (RW )"]
    #[inline(always)]
    pub fn wken(&self) -> WkenR {
        WkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reads returns 0 - (RO )"]
    #[inline(always)]
    pub fn reserved_18(&self) -> Reserved18R {
        Reserved18R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
WakeUp functionality in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]
- (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn wken(&mut self) -> WkenW<WakeupenableSpec> {
        WkenW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reads returns 0 - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_18(&mut self) -> Reserved18W<WakeupenableSpec> {
        Reserved18W::new(self, 1)
    }
}
#[doc = "The wakeup enable register allows to enable/disable the module internal sources of wakeup on event-by-event basis.\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeupenable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeupenable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupenableSpec;
impl crate::RegisterSpec for WakeupenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeupenable::R`](R) reader structure"]
impl crate::Readable for WakeupenableSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeupenable::W`](W) writer structure"]
impl crate::Writable for WakeupenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKEUPENABLE to value 0"]
impl crate::Resettable for WakeupenableSpec {
    const RESET_VALUE: u32 = 0;
}
