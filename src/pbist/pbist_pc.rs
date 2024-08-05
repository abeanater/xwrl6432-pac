#[doc = "Register `PBIST_PC` reader"]
pub type R = crate::R<PbistPcSpec>;
#[doc = "Register `PBIST_PC` writer"]
pub type W = crate::W<PbistPcSpec>;
#[doc = "Field `PBIST_PC` reader - 4:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistPcR = crate::FieldReader;
#[doc = "Field `PBIST_PC` writer - 4:0\\]
TI Internal Register.Reserved for HW RnD"]
pub type PbistPcW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    pub fn pbist_pc(&self) -> PbistPcR {
        PbistPcR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
TI Internal Register.Reserved for HW RnD"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_pc(&mut self) -> PbistPcW<PbistPcSpec> {
        PbistPcW::new(self, 0)
    }
}
#[doc = "Program Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_pc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_pc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistPcSpec;
impl crate::RegisterSpec for PbistPcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pbist_pc::R`](R) reader structure"]
impl crate::Readable for PbistPcSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_pc::W`](W) writer structure"]
impl crate::Writable for PbistPcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PBIST_PC to value 0"]
impl crate::Resettable for PbistPcSpec {
    const RESET_VALUE: u8 = 0;
}
