#[doc = "Register `dft_config_reg` reader"]
pub type R = crate::R<DftConfigRegSpec>;
#[doc = "Register `dft_config_reg` writer"]
pub type W = crate::W<DftConfigRegSpec>;
#[doc = "Field `ctrl` reader - 31:0\\]
bitundefined: dft_dmled_status_obs_sel"]
pub type CtrlR = crate::FieldReader<u32>;
#[doc = "Field `ctrl` writer - 31:0\\]
bitundefined: dft_dmled_status_obs_sel"]
pub type CtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
bitundefined: dft_dmled_status_obs_sel"]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
bitundefined: dft_dmled_status_obs_sel"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl(&mut self) -> CtrlW<DftConfigRegSpec> {
        CtrlW::new(self, 0)
    }
}
#[doc = "dft_config_reg\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_config_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_config_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftConfigRegSpec;
impl crate::RegisterSpec for DftConfigRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dft_config_reg::R`](R) reader structure"]
impl crate::Readable for DftConfigRegSpec {}
#[doc = "`write(|w| ..)` method takes [`dft_config_reg::W`](W) writer structure"]
impl crate::Writable for DftConfigRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dft_config_reg to value 0"]
impl crate::Resettable for DftConfigRegSpec {
    const RESET_VALUE: u32 = 0;
}
