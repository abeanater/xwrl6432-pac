#[doc = "Register `REG18` reader"]
pub type R = crate::R<Reg18Spec>;
#[doc = "Register `REG18` writer"]
pub type W = crate::W<Reg18Spec>;
#[doc = "Field `GPADC_IFM_DONE_CLR` reader - 0:0\\]
Clear \"ifm_done_status\""]
pub type GpadcIfmDoneClrR = crate::BitReader;
#[doc = "Field `GPADC_IFM_DONE_CLR` writer - 0:0\\]
Clear \"ifm_done_status\""]
pub type GpadcIfmDoneClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:1\\]
TI reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:1\\]
TI reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear \"ifm_done_status\""]
    #[inline(always)]
    pub fn gpadc_ifm_done_clr(&self) -> GpadcIfmDoneClrR {
        GpadcIfmDoneClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
TI reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear \"ifm_done_status\""]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_ifm_done_clr(&mut self) -> GpadcIfmDoneClrW<Reg18Spec> {
        GpadcIfmDoneClrW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Reg18Spec> {
        NuW::new(self, 1)
    }
}
#[doc = "REG18\n\nYou can [`read`](crate::Reg::read) this register and get [`reg18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg18Spec;
impl crate::RegisterSpec for Reg18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg18::R`](R) reader structure"]
impl crate::Readable for Reg18Spec {}
#[doc = "`write(|w| ..)` method takes [`reg18::W`](W) writer structure"]
impl crate::Writable for Reg18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG18 to value 0"]
impl crate::Resettable for Reg18Spec {
    const RESET_VALUE: u32 = 0;
}
