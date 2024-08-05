#[doc = "Register `SCIED` reader"]
pub type R = crate::R<SciedSpec>;
#[doc = "Register `SCIED` writer"]
pub type W = crate::W<SciedSpec>;
#[doc = "Field `ED` reader - 7:0\\]
Receiver Emulation Data Buffer"]
pub type EdR = crate::FieldReader;
#[doc = "Field `ED` writer - 7:0\\]
Receiver Emulation Data Buffer"]
pub type EdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Receiver Emulation Data Buffer"]
    #[inline(always)]
    pub fn ed(&self) -> EdR {
        EdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Receiver Emulation Data Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ed(&mut self) -> EdW<SciedSpec> {
        EdW::new(self, 0)
    }
}
#[doc = "Receiver Emulation Data Buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`scied::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scied::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SciedSpec;
impl crate::RegisterSpec for SciedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scied::R`](R) reader structure"]
impl crate::Readable for SciedSpec {}
#[doc = "`write(|w| ..)` method takes [`scied::W`](W) writer structure"]
impl crate::Writable for SciedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIED to value 0"]
impl crate::Resettable for SciedSpec {
    const RESET_VALUE: u32 = 0;
}
