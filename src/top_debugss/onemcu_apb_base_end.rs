#[doc = "Register `ONEMCU_APB_BASE_END` reader"]
pub type R = crate::R<OnemcuApbBaseEndSpec>;
#[doc = "Register `ONEMCU_APB_BASE_END` writer"]
pub type W = crate::W<OnemcuApbBaseEndSpec>;
#[doc = "Field `ONEMCU_APB_BASE_END` reader - 31:0\\]
OneMCU APB Space : Endt Address of ROM Table"]
pub type OnemcuApbBaseEndR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_APB_BASE_END` writer - 31:0\\]
OneMCU APB Space : Endt Address of ROM Table"]
pub type OnemcuApbBaseEndW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
OneMCU APB Space : Endt Address of ROM Table"]
    #[inline(always)]
    pub fn onemcu_apb_base_end(&self) -> OnemcuApbBaseEndR {
        OnemcuApbBaseEndR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
OneMCU APB Space : Endt Address of ROM Table"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_apb_base_end(&mut self) -> OnemcuApbBaseEndW<OnemcuApbBaseEndSpec> {
        OnemcuApbBaseEndW::new(self, 0)
    }
}
#[doc = "End Address of ROM Table\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_apb_base_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_apb_base_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuApbBaseEndSpec;
impl crate::RegisterSpec for OnemcuApbBaseEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_apb_base_end::R`](R) reader structure"]
impl crate::Readable for OnemcuApbBaseEndSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_apb_base_end::W`](W) writer structure"]
impl crate::Writable for OnemcuApbBaseEndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_APB_BASE_END to value 0"]
impl crate::Resettable for OnemcuApbBaseEndSpec {
    const RESET_VALUE: u32 = 0;
}
