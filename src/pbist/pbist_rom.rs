#[doc = "Register `PBIST_ROM` reader"]
pub type R = crate::R<PbistRomSpec>;
#[doc = "Register `PBIST_ROM` writer"]
pub type W = crate::W<PbistRomSpec>;
#[doc = "Field `PBIST_ROM` reader - 1:0\\]
Rom Mask . This two-bit register sets appropriate ROM access modes for the PBIST controller. Value 0h = No information is used from ROM Value 1h = Only RAM Group information from ROM Vaule 2h = Only Algorithm information from ROM Value 3h = Both Algorithm and RAM information from ROM. This option should be selected for application self-test."]
pub type PbistRomR = crate::FieldReader;
#[doc = "Field `PBIST_ROM` writer - 1:0\\]
Rom Mask . This two-bit register sets appropriate ROM access modes for the PBIST controller. Value 0h = No information is used from ROM Value 1h = Only RAM Group information from ROM Vaule 2h = Only Algorithm information from ROM Value 3h = Both Algorithm and RAM information from ROM. This option should be selected for application self-test."]
pub type PbistRomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Rom Mask . This two-bit register sets appropriate ROM access modes for the PBIST controller. Value 0h = No information is used from ROM Value 1h = Only RAM Group information from ROM Vaule 2h = Only Algorithm information from ROM Value 3h = Both Algorithm and RAM information from ROM. This option should be selected for application self-test."]
    #[inline(always)]
    pub fn pbist_rom(&self) -> PbistRomR {
        PbistRomR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Rom Mask . This two-bit register sets appropriate ROM access modes for the PBIST controller. Value 0h = No information is used from ROM Value 1h = Only RAM Group information from ROM Vaule 2h = Only Algorithm information from ROM Value 3h = Both Algorithm and RAM information from ROM. This option should be selected for application self-test."]
    #[inline(always)]
    #[must_use]
    pub fn pbist_rom(&mut self) -> PbistRomW<PbistRomSpec> {
        PbistRomW::new(self, 0)
    }
}
#[doc = "Rom Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_rom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_rom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistRomSpec;
impl crate::RegisterSpec for PbistRomSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pbist_rom::R`](R) reader structure"]
impl crate::Readable for PbistRomSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_rom::W`](W) writer structure"]
impl crate::Writable for PbistRomSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PBIST_ROM to value 0"]
impl crate::Resettable for PbistRomSpec {
    const RESET_VALUE: u8 = 0;
}
