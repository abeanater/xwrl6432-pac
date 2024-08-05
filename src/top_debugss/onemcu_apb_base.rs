#[doc = "Register `ONEMCU_APB_BASE` reader"]
pub type R = crate::R<OnemcuApbBaseSpec>;
#[doc = "Register `ONEMCU_APB_BASE` writer"]
pub type W = crate::W<OnemcuApbBaseSpec>;
#[doc = "Field `ONEMCU_APB_BASE` reader - 31:0\\]
OneMCU APB Space : Start Address of ROM Table"]
pub type OnemcuApbBaseR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_APB_BASE` writer - 31:0\\]
OneMCU APB Space : Start Address of ROM Table"]
pub type OnemcuApbBaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
OneMCU APB Space : Start Address of ROM Table"]
    #[inline(always)]
    pub fn onemcu_apb_base(&self) -> OnemcuApbBaseR {
        OnemcuApbBaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
OneMCU APB Space : Start Address of ROM Table"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_apb_base(&mut self) -> OnemcuApbBaseW<OnemcuApbBaseSpec> {
        OnemcuApbBaseW::new(self, 0)
    }
}
#[doc = "Start Address of ROM Table\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_apb_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_apb_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuApbBaseSpec;
impl crate::RegisterSpec for OnemcuApbBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_apb_base::R`](R) reader structure"]
impl crate::Readable for OnemcuApbBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_apb_base::W`](W) writer structure"]
impl crate::Writable for OnemcuApbBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_APB_BASE to value 0"]
impl crate::Resettable for OnemcuApbBaseSpec {
    const RESET_VALUE: u32 = 0;
}
