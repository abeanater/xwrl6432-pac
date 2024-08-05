#[doc = "Register `ESM_GATING4` reader"]
pub type R = crate::R<EsmGating4Spec>;
#[doc = "Register `ESM_GATING4` writer"]
pub type W = crate::W<EsmGating4Spec>;
#[doc = "Field `esm_gating` reader - 31:0\\]
bit3:0 : writing '000' will ungate the ESM_GRP3_ERROR_0 bit7:4 : writing '000' will ungate the ESM_GRP3_ERROR_1 bit31:28 : writing '000' will ungate the ESM_GRP3_ERROR_7"]
pub type EsmGatingR = crate::FieldReader<u32>;
#[doc = "Field `esm_gating` writer - 31:0\\]
bit3:0 : writing '000' will ungate the ESM_GRP3_ERROR_0 bit7:4 : writing '000' will ungate the ESM_GRP3_ERROR_1 bit31:28 : writing '000' will ungate the ESM_GRP3_ERROR_7"]
pub type EsmGatingW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
bit3:0 : writing '000' will ungate the ESM_GRP3_ERROR_0 bit7:4 : writing '000' will ungate the ESM_GRP3_ERROR_1 bit31:28 : writing '000' will ungate the ESM_GRP3_ERROR_7"]
    #[inline(always)]
    pub fn esm_gating(&self) -> EsmGatingR {
        EsmGatingR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
bit3:0 : writing '000' will ungate the ESM_GRP3_ERROR_0 bit7:4 : writing '000' will ungate the ESM_GRP3_ERROR_1 bit31:28 : writing '000' will ungate the ESM_GRP3_ERROR_7"]
    #[inline(always)]
    #[must_use]
    pub fn esm_gating(&mut self) -> EsmGatingW<EsmGating4Spec> {
        EsmGatingW::new(self, 0)
    }
}
#[doc = "ESM_GATING4\n\nYou can [`read`](crate::Reg::read) this register and get [`esm_gating4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esm_gating4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsmGating4Spec;
impl crate::RegisterSpec for EsmGating4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esm_gating4::R`](R) reader structure"]
impl crate::Readable for EsmGating4Spec {}
#[doc = "`write(|w| ..)` method takes [`esm_gating4::W`](W) writer structure"]
impl crate::Writable for EsmGating4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESM_GATING4 to value 0"]
impl crate::Resettable for EsmGating4Spec {
    const RESET_VALUE: u32 = 0;
}
