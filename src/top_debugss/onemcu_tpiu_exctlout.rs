#[doc = "Register `ONEMCU_TPIU_EXCTLOUT` reader"]
pub type R = crate::R<OnemcuTpiuExctloutSpec>;
#[doc = "Register `ONEMCU_TPIU_EXCTLOUT` writer"]
pub type W = crate::W<OnemcuTpiuExctloutSpec>;
#[doc = "Field `ONEMCU_TPIU_EXCTLOUT` reader - 31:0\\]
EXTCTL Out Port"]
pub type OnemcuTpiuExctloutR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_TPIU_EXCTLOUT` writer - 31:0\\]
EXTCTL Out Port"]
pub type OnemcuTpiuExctloutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
EXTCTL Out Port"]
    #[inline(always)]
    pub fn onemcu_tpiu_exctlout(&self) -> OnemcuTpiuExctloutR {
        OnemcuTpiuExctloutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
EXTCTL Out Port"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_tpiu_exctlout(&mut self) -> OnemcuTpiuExctloutW<OnemcuTpiuExctloutSpec> {
        OnemcuTpiuExctloutW::new(self, 0)
    }
}
#[doc = "EXTCTL Out Port\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_tpiu_exctlout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_tpiu_exctlout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuTpiuExctloutSpec;
impl crate::RegisterSpec for OnemcuTpiuExctloutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_tpiu_exctlout::R`](R) reader structure"]
impl crate::Readable for OnemcuTpiuExctloutSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_tpiu_exctlout::W`](W) writer structure"]
impl crate::Writable for OnemcuTpiuExctloutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_TPIU_EXCTLOUT to value 0"]
impl crate::Resettable for OnemcuTpiuExctloutSpec {
    const RESET_VALUE: u32 = 0;
}
