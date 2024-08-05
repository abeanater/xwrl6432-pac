#[doc = "Register `REG17` reader"]
pub type R = crate::R<Reg17Spec>;
#[doc = "Register `REG17` writer"]
pub type W = crate::W<Reg17Spec>;
#[doc = "Field `GPADC_IFM_DONE_STATUS` reader - 0:0\\]
Test completion status in IFM mode.Used for FW polling"]
pub type GpadcIfmDoneStatusR = crate::BitReader;
#[doc = "Field `GPADC_IFM_DONE_STATUS` writer - 0:0\\]
Test completion status in IFM mode.Used for FW polling"]
pub type GpadcIfmDoneStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - 31:1\\]
TI reserved"]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - 31:1\\]
TI reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Test completion status in IFM mode.Used for FW polling"]
    #[inline(always)]
    pub fn gpadc_ifm_done_status(&self) -> GpadcIfmDoneStatusR {
        GpadcIfmDoneStatusR::new((self.bits & 1) != 0)
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
Test completion status in IFM mode.Used for FW polling"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_ifm_done_status(&mut self) -> GpadcIfmDoneStatusW<Reg17Spec> {
        GpadcIfmDoneStatusW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Reg17Spec> {
        NuW::new(self, 1)
    }
}
#[doc = "REG17\n\nYou can [`read`](crate::Reg::read) this register and get [`reg17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg17Spec;
impl crate::RegisterSpec for Reg17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg17::R`](R) reader structure"]
impl crate::Readable for Reg17Spec {}
#[doc = "`write(|w| ..)` method takes [`reg17::W`](W) writer structure"]
impl crate::Writable for Reg17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG17 to value 0"]
impl crate::Resettable for Reg17Spec {
    const RESET_VALUE: u32 = 0;
}
