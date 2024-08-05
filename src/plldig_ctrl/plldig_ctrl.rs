#[doc = "Register `PLLDIG_CTRL` reader"]
pub type R = crate::R<PlldigCtrlSpec>;
#[doc = "Register `PLLDIG_CTRL` writer"]
pub type W = crate::W<PlldigCtrlSpec>;
#[doc = "Field `cfg_plldig_ctrl` reader - 31:0\\]
PLL DIG test controls"]
pub type CfgPlldigCtrlR = crate::FieldReader<u32>;
#[doc = "Field `cfg_plldig_ctrl` writer - 31:0\\]
PLL DIG test controls"]
pub type CfgPlldigCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
PLL DIG test controls"]
    #[inline(always)]
    pub fn cfg_plldig_ctrl(&self) -> CfgPlldigCtrlR {
        CfgPlldigCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
PLL DIG test controls"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_plldig_ctrl(&mut self) -> CfgPlldigCtrlW<PlldigCtrlSpec> {
        CfgPlldigCtrlW::new(self, 0)
    }
}
#[doc = "PLLDIG_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`plldig_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plldig_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlldigCtrlSpec;
impl crate::RegisterSpec for PlldigCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plldig_ctrl::R`](R) reader structure"]
impl crate::Readable for PlldigCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`plldig_ctrl::W`](W) writer structure"]
impl crate::Writable for PlldigCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLLDIG_CTRL to value 0"]
impl crate::Resettable for PlldigCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
